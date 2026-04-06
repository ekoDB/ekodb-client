make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.66s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("_5UYay5HiBflYCvBlPIqL059Sf-VDCOwtw-EiRtybDz-sqXkGlGiDpBuCaV1lzM0ZzHv5C34g7XCgKdwTz0-5A")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("_5UYay5HiBflYCvBlPIqL059Sf-VDCOwtw-EiRtybDz-sqXkGlGiDpBuCaV1lzM0ZzHv5C34g7XCgKdwTz0-5A"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("_5UYay5HiBflYCvBlPIqL059Sf-VDCOwtw-EiRtybDz-sqXkGlGiDpBuCaV1lzM0ZzHv5C34g7XCgKdwTz0-5A"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("_5UYay5HiBflYCvBlPIqL059Sf-VDCOwtw-EiRtybDz-sqXkGlGiDpBuCaV1lzM0ZzHv5C34g7XCgKdwTz0-5A"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "XIRI1bUm4XF0vKdRdr_ajMzn25G7r7wEIh_A79ht4EHm1G0QAT709vSsKX2lA0f2my_sOrIuvmhdEhrSnCBiGA"

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1775447293262794000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "XIRI1bUm4XF0vKdRdr_ajMzn25G7r7wEIh_A79ht4EHm1G0QAT709vSsKX2lA0f2my_sOrIuvmhdEhrSnCBiGA",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "p8-TbJ5C183-_saQl4hJiJwN0VU1iMjWqBIaFTUQP1922yA1KQyjNKGP8y6MsvICiOyE5qZnYVFMJZxuhe7sdQ"

=== List Collections ===
Total collections: 31
Sample collections: ["audit__ek0_testing", "cached_database_metrics", "certificate_diagnostics_testing", "websocket_test", "deployments", "cached_zones", "certificate_diagnostics_development", "deployment_analytics", "cached_locations", "cached_machine_types", "users", "demo_collection", "billing_details", "user_metadata", "user_analytics", "certificate_diagnostics_staging", "module_version_checks", "system_analysis", "test_collection", "cached_deployment_logs", "cached_module_versions", "deployment_operations", "deployment_queue", "deployment_keys", "invoices", "batch_users", "ip_locations", "deployment_costs", "zone_locations", "distributed_locks", "certificate_diagnostics_production"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "d0HnnS9YoaHHGTvUKEakTtfD5dzLWh-2eYUKwpuVD2AGeBtiHcyt3bWf5rFqWEddmq_BxeyEz5V6B8MksmkBYw"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "q0Di_bA58t5AyaWEEDZSpspGbJkB0V_XsImcH-9OQEmgsY5uapb_aa9LqCTVL-d0XLZhI32lJlanjRlp958-Nw"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "WVOxpnbDydfyr6PuDGL0pFkJdYQhEvMLHPTwmW_uk3KxZ8TOUi9QRf6b8cGFeeF-nCqE-YQ0sAvaIHOaJPYS0A"

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("ebpBM3zl9DkNpJE4_aLqkhZsJCSl2ZquDHuyjCAw7H95OhlpEQT9laTqmSA9Js3UWUhDyMjjCgdcXxtXnAmTcA")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "ebpBM3zl9DkNpJE4_aLqkhZsJCSl2ZquDHuyjCAw7H95OhlpEQT9laTqmSA9Js3UWUhDyMjjCgdcXxtXnAmTcA"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-04-06T04:48:53.111346Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Functions Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: Vk0O3_xaX574Wq3Q6AzzBOn-8j86kIxNCUYJyC5n6OgcX3YrRAnT-_JyFYpu0mbNI8pO2StEoNoXhQ2zm3mGKQ
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: "7MwQ4npwbBMEbC2pPKPfItgSAzOXN35D9wTPeswdrJYKnzwAF1vQQk1SLInPALfOh5TXg_WuKxso-4GilFVt_A"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Function saved: B-uC_x-j-RxHH5lTbATyLGL48L2Sc5AK3wEs0wKEj0IGgMaU5OvN3agPefRYPoUpcpElJdWTDM3GW-2oSW7BBA
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: CYAtDTVFUFkfDMCb2o1QFYTd8skMC--iw3ZdHHpunnbFLEwIM5hE6_C_jpQOiRjx9FH-gxYYICQfIPtxOWg5aw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60.0,"count":5,"max_score":100,"status":"active"}
   {"avg_score":50.0,"count":5,"max_score":90,"status":"inactive"}

📝 Example 4: Function Management

📋 Total scripts: 4
🔍 Retrieved script: Get Active Users
✏️  function updated
🗑️  function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: xHR6FQuOoChmK7x4h6L2Fn_GAqKwJDkwl0_oiJyA2259XL6qO_alFz5prBmsPIfQFyG8Ns1kjDU7k_uay8gqug
Created Bob: $500 - ID: f3E74i3WlbYfgTd_1Zsnnel5zRk15qxEm7bmmn78Zx2FtESG507zdXhQjGN2U61QajSDByn5EL1Wy2jZlgXACA

=== Example 1: Begin Transaction ===
Transaction ID: 1c642583-b2fa-4fbd-a12d-b38a9e16ea61

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: {"type":"Integer","value":800}
Bob: {"type":"Integer","value":700}

=== Example 5: Rollback ===
New transaction: 74458469-64c7-4f35-bd7f-82a37cef560a
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("-lS7meWC82vnoEfRw7DGQfKXbWwl3sYCZJSxrnzXPeGobewTlUp_4lGt7H8myq28a-fuTU0XrDrchUi5ckHO7A")})

=== Find by ID ===
Found: Record({"id": String("-lS7meWC82vnoEfRw7DGQfKXbWwl3sYCZJSxrnzXPeGobewTlUp_4lGt7H8myq28a-fuTU0XrDrchUi5ckHO7A"), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-04-06T03:49:11.564754Z")}), "name": Object({"type": String("String"), "value": String("Test Record")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-04-06T03:49:11.564754Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "name": Object({"value": String("Test Record"), "type": String("String")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "id": String("-lS7meWC82vnoEfRw7DGQfKXbWwl3sYCZJSxrnzXPeGobewTlUp_4lGt7H8myq28a-fuTU0XrDrchUi5ckHO7A"), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "created_at": Object({"type": String("DateTime"), "value": String("2026-04-06T03:49:11.564754Z")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")})})]

=== Update Document ===
Updated: Record({"embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "id": String("-lS7meWC82vnoEfRw7DGQfKXbWwl3sYCZJSxrnzXPeGobewTlUp_4lGt7H8myq28a-fuTU0XrDrchUi5ckHO7A"), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "name": Object({"type": String("String"), "value": String("Updated Record")}), "value": Object({"type": String("Integer"), "value": Integer(100)}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "created_at": Object({"type": String("DateTime"), "value": String("2026-04-06T03:49:11.564754Z")})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: jC5hPtcEGyOKkveg1PF1mHXRaNpCn8MDs5vHek8DrhFcC3l55gc1eHB4GrvimrbIW2yEjumG1trLWVI700oKGQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  active: true
  name: "WebSocket Test Record"
  id: "jC5hPtcEGyOKkveg1PF1mHXRaNpCn8MDs5vHek8DrhFcC3l55gc1eHB4GrvimrbIW2yEjumG1trLWVI700oKGQ"
  value: 42

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "sAzBXmtDLoKODsqLWT9P0K-sXUNZhHcjjv2l1vFkzujMWMa3ChFAPn9HrEwXTL0KNGmVrQhP_PyCxwSRscV1qA"

=== List Collections ===
Total collections: 35
Sample collections: ["audit__ek0_testing", "cached_database_metrics", "certificate_diagnostics_testing", "websocket_test", "deployments"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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
  cache:product:3: Record({"price": Float(49.989999999999995), "name": String("Product 3")})

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 8

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  cache:product:1: deleted
  cache:product:2: deleted
  cache:product:3: deleted

✓ All KV operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: GA73HNcYhdTnCRfwdv6gIgIY5JzXqEqi0ORF_SoiowR8_EPYHQuZR1YNE-83EW7Gqhi8kmVXrdddnE5ccsR9IA
Created Bob: $500 - ID: ELwKHwwuMaFaDYubc9FYbID2aGxFKvDloAiXzZM6acElUgRkc39vYW2GOsO8nh8URcVRuuJXO-ORB4azf3vonA

=== Example 1: Begin Transaction ===
Transaction ID: 6a1edc5e-4154-4feb-978e-3427aeb39ed0

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: b91e2000-c68c-47e3-83bf-71a509f14963
Status before rollback: "Active"
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

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
  - Some(Object({"value": String("Charlie"), "type": String("String")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"value": String("Bob"), "type": String("String")}))
  2. Some(Object({"value": String("David"), "type": String("String")}))
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_search`
=== ekoDB Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Found 3 results for 'programming'
  1. Score: 19.8000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 6.6000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})
  3. Score: 6.6000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})

=== Fuzzy Search ===
✓ Found 3 results for 'progamming' (typo)
  1. Score: 1.6500 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 0.5500 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})
  3. Score: 0.5500 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})

=== Field-Specific Search ===
✓ Found 4 results in title/description
  1. Score: 4.0000
     Title: Some(Object {"type": String("String"), "value": String("Machine Learning Basics")})
     Matched: ["description", "title", "description.value", "title.value"]
  2. Score: 2.0000
     Title: Some(Object {"type": String("String"), "value": String("Python for Data Science")})
     Matched: ["description", "description.value"]
  3. Score: 1.0000
     Title: Some(Object {"type": String("String"), "value": String("Database Design")})
     Matched: ["description.value", "description"]
  4. Score: 1.0000
     Title: Some(Object {"type": String("String"), "value": String("Rust Programming")})
     Matched: ["description.value", "description"]

=== Weighted Search ===
✓ Found 2 results with field weights
  1. Score: 23.1000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})
  2. Score: 3.3000 - Some(Object {"type": String("String"), "value": String("Machine Learning Basics")})

=== Advanced Search Options ===
✓ Found 1 results with stemming
  1. Score: 19.8000 - Some(Object {"type": String("String"), "value": String("Database Design")})

=== Search with Limit ===
✓ Limited to 2 results (requested 2)
  1. Score: 19.8000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 6.6000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 2ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("q5hq99zT09h3kHgokCYIPIBK0AFtoqQ6GXCQRF88WbwsXD_SVIwEs5Aa19NkgQ0boyQNEiCbt21Lc5OHBw_6rA"))
✓ Inserted user 2: Some(String("qyfP7_wOAbH4RxJpq064e3udEuKT-KO9ZNchcDI98xRZNO8wTvG7cP_qXDqFR9VvigKnY4I5XXdirIvx8MZVCQ"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - age: Integer
  - title: String
    (required)
  - email: String
    (required)
  - status: String

=== Listing Collections ===
✓ Total collections: 35
  Sample: ["audit__ek0_testing", "cached_database_metrics", "schema_client_rust", "certificate_diagnostics_testing", "websocket_test"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "hTBVRA-kVlqh9C3wZFds6OUd57zDEclqcq5iTId-tZhDTgsuNSiw45f-gLCqLHW1edBDys9LPKfCmgNj2SnyvA"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("mLTjROeXZ6X_pVUcFsfNn8XXXEvS82KYHyIwu9o-bt-vMZLUlCYum73WXOsZc6LtWpIUXU_oN4LSusdmBg2I2Q"))

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("3sHaY1K75AUIQM9TBmxrS3x2544l2nGZb8SgTDY3qEOZZlvSxlsFiWy6867oNC49uq5hSZ8TDPyvI0WAIVj97w"))

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
✓ Edge cache script created: EhEkcC0d6UWfiqu6Q_nhumdEAOthGSec2pCRIG_ZcAUUcDD7hbbRtYtgok_46xaPw1faMEnZa0DGidg4LKY0Og

Call 1: Cache lookup
Response time: 14ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 13ms
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Functions Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: HLsFkaK1vDpawVNdbwPME29JxjYGYhTa-4ySejP3a48GyMjCBj1EFsp_iecwm6fYXHfxY_XxpgN4OuYbIJDQ-A
📊 Found 22 active users

📝 Example 2: Parameterized Function

✅ Function saved: ppx2IFtGW3Rt-wlu3vLtYxdD1_TaEyQ8btHOZqufFfsQEyHRGs-hH1r5pH815-sdMNzTRCBPUx_zl_w1_Umy3w
📊 Found 22 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: saM99nLK_ax6gRCwr5J0HE3ahv5G6tLXFmC-Sa5jx_D1MYbgf3lfN82kqs2W9L3SAktVwCjNYgDqHBnXz67OQw
📊 Statistics: 3 groups

📝 Example 4: function Management

📋 Total scripts: 6
🔍 Retrieved script: Get Active Users
✏️  function updated
🗑️  function deleted

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
   ⏱️  Duration: 214.302958ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "id": 1,
    "address": {
      "zipcode": "92998-3874",
      "geo": {
        "lat": "-37.3159",
        "lng": "81.1496"
      },
      "street": "Kulas Light",
      "suite": "A...

Second call (cache hit - from cache):
   ⏱️  Duration: 38.895542ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "email": "Sincere@april.biz",
    "id": 1,
    "username": "Bret",
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "bs": "harness real-time e-m...
   🚀 Cache speedup: 5.6x faster!

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions_complete`
🚀 ekoDB Rust Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: Zaj78GrVAtgew9SxVlVjniHnMIQIZNwIZr20uQm4xbbB2ak4Hc1sxUQdQyH-zWUcfaMYqgWsAi95d3EE0QzZbQ
📊 Found 2 product groups
   Record({"count": Integer(2), "category": String("Furniture"), "avg_price": Float(474.0)})
   Record({"avg_price": Float(575.6666666666666), "category": String("Electronics"), "count": Integer(3)})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   Record({"count": Integer(2), "category": String("Furniture")})
   Record({"category": String("Electronics"), "count": Integer(3)})
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: function with Parameter Definition

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("u-nRGaKjZv5Z5LW3J5rvTSDIyJcEtxOplTcvveqeUbcfulFiAeNqW8d8woMYWG40ER4ipL1e7F9R-sflhB46_w"))
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: D06_N0CQqHSbxtYTRzgT-G39kH_lBHEPWUUFLi33h0E2dHxj1xj7J8cpC7Y5Vjv5srZa2QdL2UONhQ0CVJWc3Q
📊 function executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: EE_hdnvfP060-8GBhMjeV8dgbaOHKiN4be7gcN3YbEib4kV5ESuqWC7sC9BY9dwK3mm1s4tPZbeiHsusAe42yQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: pEkRlp2RUO9xPuN7RGDrM5T5wxe9XMEuE54MyHHefjnjISF_Wl94BbwC_iqp0FlL5WJH4-sVtDZo6uWvHi4xbQ
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_rs (pMoUzuFXrI5YpENeXWO_ZPYJ3whle_olrTJ5rviOvhV2_m3vXkyI6_gCvs1owcL6eB2vVaO4hGLHiySAMXLNTQ)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 41ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native (QuWXMMMplC5GR1TFdUAoQVtzlMTsPs6v2ik4swmvvxo6Xdq0DTflsNRSPNl487tGrWP7Tws1nKvC2BlZunkHgQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 113ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 20ms
  Speedup: 5.7x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (7G-Y9NOJ9twliQZIDECBNV1m75oaL_xNk6WrfjrY1Hv5vnSCfjnaoec9vJ5X1AJzsIx6T9LJs4nFLT2Ivvwvew)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (IeSMfeesfXDokb1R23LJa2LcQEvldcGDCqxFX8GTFiWs1xNAhK1kl8W5aeeSh0hTohtq4P6Pfpm04rSzjgssig)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (a-wOvMAyVIckdipdgPnPe181SNhQsm5SXd_CJtKl1eEwNvi_HZet7WouPbMird5Jnxw6zji6MXEvRSVoehpotA)
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

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/examples/client_functions_advanced`
🚀 ekoDB Rust Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   Record({"avg_price": Float(365.6666666666667), "count": Integer(3), "category": String("Furniture")})
   Record({"count": Integer(5), "category": String("Electronics"), "avg_price": Float(367.0)})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits:

1. **Fast Similarity Search**: They enable quick retrieval of similar items based on vector representations, which is essential for applications like recommendation systems and image search.

2. **Handling Unstructured Data**: Vector databases excel at processing unstructured data, such as text, images, and audio, by converting them into embeddings.

3. **Scalability**: Designed to handle large volumes of high-dimensional data efficiently, they can scale to accommodate growing datasets.

4. **Real-time Processing**: Many vector databases support real-time queries, allowing for immediate insights from data.

5. **Integration with Machine Learning**: They facilitate easy integration with ML models for tasks such as feature extraction, making it easier to deploy AI applications.

6. **Advanced Query Capabilities**: Support for complex queries involving distance metrics and nearest neighbor searches.

7. **Flexibility**: They can work with various data types and structures, adapting to different use cases.

8. **High Performance**: Optimized for high-dimensional space searches, ensuring faster query responses compared to traditional databases.
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_functions_crud`
🚀 ekoDB Rust CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   Record({"status": String("active"), "count": Integer(7)})
   Record({"count": Integer(3), "status": String("inactive")})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Vector Databases Explained (Database)
   2. Database Design Principles (Database)
   3. Natural Language Processing (AI)
   4. Introduction to Machine Learning (AI)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   Record({"category": String("AI"), "count": Integer(2)})
   Record({"count": Integer(3), "category": String("Database")})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: TfKmmOMR_2POqQKuPE-ULopkgjXNfSUIAYaLdZXTvnxQXfHoloZdTGSH9hr6eSm1IwL5Mn52hzhdMgkxpKxTWA

=== Sending Chat Message ===
Message ID: hLR3mtebmp2f1ULt9EBmUFPgst4La589Vfq_2Fz-g8bywMlcBaLnyOR3MQ9xo_bif6gs7uoCegtuOcTnRQOaBw

=== AI Response ===
Response 1: ekoDB is a high-performance database designed to support various AI-powered functionalities. It is equipped with intelligent caching, real-time capabilities, and integrates AI features.

### Key Features of ekoDB:
1. **AI Chat Integration**: The chat feature allows users to query their database using natural language and receive responses that are contextually relevant.
2. **Full-Text Search**: ekoDB supports full-text search capabilities, enabling users to perform text-based searches efficiently.
3. **Vector Search**: The database also provides vector search features, allowing users to find similar records based on semantic meaning rather than just keyword matching.
4. **Hybrid Search**: ekoDB offers hybrid search functionality, which combines both text and vector search methods, automatically retrieving context to enhance search results.

Overall, ekoDB aims to facilitate seamless data interaction through its diverse and intelligent search capabilities.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["category", "title", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("D116tYQ1QgQptXU4Mk0o-z4WJxBmZz52oznV2uelTHg7_nidYA1NGOCOL3-v9D6W59umWzIDfPhEbBFPjruo7Q"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("J0knTgazOvPIMwi_5qHx_HLZ7iNCYsHY9ZZiLm9d9yRtwYkIMc41A2XBxniBPxrTLMwWMWnDqOzpQyGRRtVuJw"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("dlo-d7CZZoEJs1BE3yCNAgFTCSZ_QjXwgjlEzN1FFqBUAesWmpsN0is__jgG38xQnz-opmXLLJLsRWPRp25oMw"), "title": String("Search Features")}

Execution Time: 11903ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.46s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: fMrTYfIKxXyRdWRVNWnHhXzievNyCHlme8irzIUTUy9psBOQeeKsWytqdSVgWWXMmBAvpVAdGhnQiS9c8RuzGg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or have other questions, feel free to ask!

✓ Second message sent
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any other questions or need more information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: DhA8DyhZtR0FuoyBL1w-RIlzIpVPoqizfcdl22D8cB31svLhft8zl4DkRsY6AgPDTTr2LuqVa1Oz4p5MeuX2cw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 6

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 5

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 5UXzj18M6q9HTsNd9G6KNym3g3rukd_sJJzGSvP0zCDTIPcZAL3z2EzKjldUqeILYAuQzKhCP3SA99CndHkKIA
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Product Name**: ekoDB
- **Description**: A high-performance database product with AI capabilities
- **Price**: $99

If you need more information or additional products, please let me know!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: JGWWbw36faNCuP-J1n8ATz5IFTXSoFXYeh8nYWBgPhmpztZqKNcz_d_yrX8bgjrxsx5NPU9qXqu-Foe2FP5vug
  Parent: 5UXzj18M6q9HTsNd9G6KNym3g3rukd_sJJzGSvP0zCDTIPcZAL3z2EzKjldUqeILYAuQzKhCP3SA99CndHkKIA

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: JGWWbw36faNCuP-J1n8ATz5IFTXSoFXYeh8nYWBgPhmpztZqKNcz_d_yrX8bgjrxsx5NPU9qXqu-Foe2FP5vug (Untitled)
  Session 2: 5UXzj18M6q9HTsNd9G6KNym3g3rukd_sJJzGSvP0zCDTIPcZAL3z2EzKjldUqeILYAuQzKhCP3SA99CndHkKIA (Untitled)
  Session 3: DhA8DyhZtR0FuoyBL1w-RIlzIpVPoqizfcdl22D8cB31svLhft8zl4DkRsY6AgPDTTr2LuqVa1Oz4p5MeuX2cw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: JGWWbw36faNCuP-J1n8ATz5IFTXSoFXYeh8nYWBgPhmpztZqKNcz_d_yrX8bgjrxsx5NPU9qXqu-Foe2FP5vug

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/examples/client_chat_models`
✓ Client created

=== Get All Chat Models ===
Available chat models by provider:

OpenAI models (122):
  - gpt-4-0613
  - gpt-4
  - gpt-3.5-turbo
  - gpt-5.4-mini
  - gpt-5.4
  - gpt-5.4-nano-2026-03-17
  - gpt-5.4-nano
  - gpt-5.4-mini-2026-03-17
  - davinci-002
  - babbage-002
  - gpt-3.5-turbo-instruct
  - gpt-3.5-turbo-instruct-0914
  - dall-e-3
  - dall-e-2
  - gpt-3.5-turbo-1106
  - tts-1-hd
  - tts-1-1106
  - tts-1-hd-1106
  - text-embedding-3-small
  - text-embedding-3-large
  - gpt-3.5-turbo-0125
  - gpt-4-turbo
  - gpt-4-turbo-2024-04-09
  - gpt-4o
  - gpt-4o-2024-05-13
  - gpt-4o-mini-2024-07-18
  - gpt-4o-mini
  - gpt-4o-2024-08-06
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
  - gpt-4o-mini-tts-2025-12-15
  - gpt-realtime-mini-2025-12-15
  - gpt-audio-mini-2025-12-15
  - chatgpt-image-latest
  - gpt-5.2-codex
  - gpt-5.3-codex
  - gpt-realtime-1.5
  - gpt-audio-1.5
  - gpt-4o-search-preview
  - gpt-4o-search-preview-2025-03-11
  - gpt-5.3-chat-latest
  - gpt-5.4-2026-03-05
  - gpt-5.4-pro
  - gpt-5.4-pro-2026-03-05
  - gpt-3.5-turbo-16k
  - tts-1
  - whisper-1
  - text-embedding-ada-002

Anthropic models (9):
  - claude-sonnet-4-6
  - claude-opus-4-6
  - claude-opus-4-5-20251101
  - claude-haiku-4-5-20251001
  - claude-sonnet-4-5-20250929
  - claude-opus-4-1-20250805
  - claude-opus-4-20250514
  - claude-sonnet-4-20250514
  - claude-3-haiku-20240307

Perplexity models (5):
  - sonar
  - sonar-pro
  - sonar-deep-research
  - sonar-reasoning
  - sonar-reasoning-pro

=== Get Models for Specific Provider ===

openai models (122):
  - gpt-4-0613
  - gpt-4
  - gpt-3.5-turbo
  - gpt-5.4-mini
  - gpt-5.4
  ... and 117 more

anthropic models (9):
  - claude-sonnet-4-6
  - claude-opus-4-6
  - claude-opus-4-5-20251101
  - claude-haiku-4-5-20251001
  - claude-sonnet-4-5-20250929
  ... and 4 more

perplexity models (5):
  - sonar
  - sonar-pro
  - sonar-deep-research
  - sonar-reasoning
  - sonar-reasoning-pro

✓ Chat Models API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_user_functions`
✓ Client created

=== Create User Function ===
Created user function with ID: lbFNfCE_O5pdzcgyJQR6h4I1ODraRgkth7yYLfakG-EXoHACSYMOEgUlClOr_OhYmH608o95deAISdgNK6Sb5w

=== Get User Function ===
Retrieved: get_active_users_rs - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 13 user functions:
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_users_by_status: Get Users By Status
  - validate_user: Check if user exists
  - get_verified_user: Get verified and validated user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_high_scoring_active_users: Get High Scoring Active Users
  - get_active_users_rs: Get Active Users
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_active_users: Get Active Users (Updated)
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_slim_user: Validate and slim down user

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("sZC3rRuxSPVs8Ni8ejZg178utJVKBJdV8gFkyeGB556lxvk3b9io2fgzm8nVCv2SXnqJh5Y3NiHLV0u15xdsvA")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("Y1kTIRoGjwqopQPG5lyWsRR3w_c4hsdZmifnBlRNuiZAVlh0rn9LwpC26R--PK4LjghoBOaZiIq05qB4VueHCQ")})
✓ Second upsert (update): Record({"id": String("95kpsN6TH_MkL3aY2VmzlQWllvTa6LCUgtJIIEwCz3EBFQmtkYtYTLHDgzQv2YOOVz6FQXMXvucvtlaag1Cu9g")})

=== Find One Operation ===
✓ Found user by email: Record({"active": Object({"value": Boolean(true), "type": String("Boolean")}), "name": Object({"value": String("Alice Johnson"), "type": String("String")}), "email": Object({"type": String("String"), "value": String("alice@example.com")}), "age": Object({"type": String("Integer"), "value": Integer(28)}), "id": String("sZC3rRuxSPVs8Ni8ejZg178utJVKBJdV8gFkyeGB556lxvk3b9io2fgzm8nVCv2SXnqJh5Y3NiHLV0u15xdsvA")})
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: true
✓ Fake record exists: false (should be false)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 8 records (expected 7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/bypass_ripple_example`
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: Record({"id": String("q1hsIDB-1wJxc08ubtuNZi9JdPi3W4JaN60nbceu98HQWiRFFk7emqbQT53Xy-Jd84pOigVmMyNI4WpFLGpuIQ")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("-tmBpzDlBAKfay3QFIW0ilZVNRHFxw80NEDd2bElL7vXNEPsHR4DrZcMrx1YjlcjXsAhnvapptPODPtyj3saVg")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"price": Object({"value": Integer(150), "type": String("Integer")}), "id": String("q1hsIDB-1wJxc08ubtuNZi9JdPi3W4JaN60nbceu98HQWiRFFk7emqbQT53Xy-Jd84pOigVmMyNI4WpFLGpuIQ"), "name": Object({"value": String("Product 1"), "type": String("String")})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("15yjWdSF42wuAvcgND8vd9RN1bOXliVgKaBJAEc_uIp5cg0VV1UbXu-kS3RZimLL-ixJbOipDST4DHfI6DLPKA")})

✅ All bypass_ripple operations completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/projection_example`
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["email", "name", "id"]

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["id", "user_role", "age", "email", "avatar_url", "status", "created_at", "bio", "name"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["age", "created_at", "email", "bio", "id", "name", "password", "avatar_url", "status", "api_key", "secret_token", "user_role"]
  Projected query:
    - 3 fields per record
    - Fields: ["id", "email", "name"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ [32mRust client examples complete![0m
