make test-examples
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
✅ [32mAll Rust integration tests complete![0m
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'Fw3-_Rt2nOv_VytvoIJrkihPJOFqsjTagKWK7P1tGujgqACAAcMWT9h5TYTArWqcRDBW_TOJhv99CyvxfajyVw'}

=== Find by ID ===
Found: {'id': 'Fw3-_Rt2nOv_VytvoIJrkihPJOFqsjTagKWK7P1tGujgqACAAcMWT9h5TYTArWqcRDBW_TOJhv99CyvxfajyVw', 'name': {'type': 'String', 'value': 'Test Record'}, 'value': {'type': 'Integer', 'value': 42}, 'active': {'value': True, 'type': 'Boolean'}}

=== Find with Query ===
Found documents: [{'name': {'value': 'Test Record', 'type': 'String'}, 'id': 'Fw3-_Rt2nOv_VytvoIJrkihPJOFqsjTagKWK7P1tGujgqACAAcMWT9h5TYTArWqcRDBW_TOJhv99CyvxfajyVw', 'value': {'value': 42, 'type': 'Integer'}, 'active': {'type': 'Boolean', 'value': True}}]

=== Update Document ===
Updated: {'id': 'Fw3-_Rt2nOv_VytvoIJrkihPJOFqsjTagKWK7P1tGujgqACAAcMWT9h5TYTArWqcRDBW_TOJhv99CyvxfajyVw', 'name': {'type': 'String', 'value': 'Updated Record'}, 'value': {'type': 'Integer', 'value': 100}, 'active': {'value': True, 'type': 'Boolean'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: YqPd6d25_D4tl6fKjbje3AExz8arcSNbcmeUwMRvFm_j2tAzd2q0DMFW65DwWrroSYcvnXyFUI5AqnR8pXWDOg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "733342103",
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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "YqPd6d25_D4tl6fKjbje3AExz8arcSNbcmeUwMRvFm_j2tAzd2q0DMFW65DwWrroSYcvnXyFUI5AqnR8pXWDOg",
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
🚀 ekoDB Functions Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: 2TI3pMvGWHGM4R8ZXH0NNmbUcSgfms_FsSICVosoy7UnsT0J4dwSONk-rc2O52cbc33xmtBuXVhSzC6Qu2bfpw
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: RRilQeDMCNYbOdgQw_kxAFZ9nY0TaTJKwgsUaKIxiZEbpFskMMBBYMBK4S74b7gy9TKQlZNz3iybkB9CVlG0sA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: cu5hlx_loozYBlpNrjISMBy2UgjlXT9sUmaZvGLTW0kLRDVVVLrMx902nzB51nUDsJbzP3X7VE3e6zelqvcv0A
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'max_score': 100, 'status': 'active', 'avg_score': 60.0, 'count': 15}
   {'max_score': 90, 'status': 'inactive', 'count': 15, 'avg_score': 50.0}

📝 Example 4: Function Management

📋 Total scripts: 15
🔍 Retrieved script: Get Active Users
✏️  function updated
🗑️  function deleted

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
Collection created with first record: n9PDwyX_m0H9j25NNMmk4B6LB0Eaih065EgXQmJ-q2Uvu6sQrDLod-KMEAPFBf-IHjr9wnSnBeifTtjZlwFPXQ

=== List Collections ===
Total collections: 38
Sample collections: ['audit__ek0_testing', 'cached_database_metrics', 'chat_configurations__ek0_testing', 'certificate_diagnostics_testing', 'websocket_test']

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
Created Alice: $1000 - ID: PfNVTWa0T-2yLTYm8xmzoXlppstqhC3d258UCIfWSuMKDGeEVxLqytfaohaRrh_hZfobGw58DctCZpJza6nHcw
Created Bob: $500 - ID: 9fnquSUpw5R8IWhRhzpOAXe-rGzqJ025Y9glyxqhqMUql1XuC4x4S7vdHqme0tHCYgf2dv9UNlcQjKA9YEChlA

=== Example 1: Begin Transaction ===
Transaction ID: e709ad6f-1354-4030-878f-7dd85485887c

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'value': 700, 'type': 'Integer'}

=== Example 5: Rollback ===
New transaction: 20017572-7c3b-4ae3-8cf8-2664834ead5e
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Qj15G19gTDuJm06DTxNjNGMOkVJSUFWFZvecRec0929ycNf11S8WocB6Zq4nuZhy82wQ0pvlybYVUBSOuBZgbQ
Created Bob: $500 - ID: XFifvakqWa09RRFCQVvOWf21MK_HtCP6bO5WGUVYasr5bNykrd6r2AyM33vkNzUQ4btAjLpVwBaKJZv8cyuw1Q

=== Example 1: Begin Transaction ===
Transaction ID: 7fff0e6b-34d2-4482-a790-deb4989ee5d9

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to NON-DURABLE mode...
   ✓ Config updated: durable_operations=false


============================================================
🔥 TEST 2: Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: KgKmkafStrjwd9oDcDjUX7Gi1HjgYlhSN25Z-qmL887zgKb2mgQxBhWGiWIXnAM8iyXcBEMqFvjZiAZ09l9Rjw
Created Bob: $500 - ID: ojAcgzcOdwspdaSwi1_GOsvPnlidAR-d7iKjtlw27G5rckwU_UMOEZKarepVBjt56GX0uWzAbzbn1IBt54GjTA

=== Example 1: Begin Transaction ===
Transaction ID: 0f43350b-6577-40f9-954e-2e0fc342b94b

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'type': 'Integer', 'value': 700}

=== Example 5: Rollback ===
New transaction: 144674a2-8f83-402f-92cc-6b46e1421ef0
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 8hXUMeBz74WV84WITNylvsEVL9iFC__fQxu96Gr6O37PTKjrWGRnudWBYQ0_pgPgRS8eW7f4MkxSRvH8xpVHSA
Created Bob: $500 - ID: RMXMz9qXjURLRzNsvew0D-JTKGok6laRp3SItRP4M0Fq7sE31M8vkxY0ITxrV-8mra3593sZQlE63jy_Wt3qQQ

=== Example 1: Begin Transaction ===
Transaction ID: 46e62229-8bc0-4fee-981b-6a4ee70c1b2f

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'value': 800, 'type': 'Integer'}
Bob: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to DURABLE mode...
   ✓ Config updated: durable_operations=true


============================================================
🔥 TEST 3: Durable Mode (durable=true)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: go1QaGxYZswxzU_grWKkfy5Z_VRmDQTmFpviI-ib88Fu5J6DOI1BTSmt2qan7ZbSJ8my0vKbQ-IdpBJtsT7HtA
Created Bob: $500 - ID: zXBI8vIFrjaCAyT3nYK0TLLRXef000BUEGjuKdymoWs1UF6OOZ0X4lrNnpIwCVygkIcaU5J4mqZFfBn6kb8woA

=== Example 1: Begin Transaction ===
Transaction ID: 6529e0e6-652e-4271-a075-52cb3ffac91d

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'value': 800, 'type': 'Integer'}
Bob: ${'type': 'Integer', 'value': 700}

=== Example 5: Rollback ===
New transaction: 0b1ab438-bf9a-4b43-83c8-24b9303180db
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: pGvJWDiJaPbKTe8axErTueQ6uGo-2slb5dMFcUZlm3DuCQhS6iODjgDXZt-b-9Ga2co8_X7IN4ZCSDoUtx_J7g
Created Bob: $500 - ID: RN33X7dkZBZxixEH037Rfkf_Cctx_JFb97Sq4FMmhU1DHfmb2VcF9OYlUPWuRT9f7g1kN0_UHfQcpDpuM4SVFA

=== Example 1: Begin Transaction ===
Transaction ID: 149e9ad2-41fc-445f-b31d-253c90032b70

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'value': 800, 'type': 'Integer'}
Bob: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ Deleted test accounts


🔄 Restoring original configuration...
   ✓ Config restored: durable_operations=True


============================================================
✅ ALL TESTS PASSED - Transactions successful
============================================================

🚀 ekoDB Complete CRUD Functions Example
============================================================
Demonstrates:
  • Insert + Verify (using Query)
  • Query + Update Status + Verify
  • Query + Update Credits + Verify
  • Query Before Delete + Delete + Verify Gone

Each function shows Functions chaining with proper verification
============================================================
============================================================
📝 function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: bvIi8swkOlVokQUn3GpQ6oenZ4SHfeSbyYCA0FKWK_lxnJD8uqMIsxXTgm-_Z6pL2f9XCAt09pLBOjgEMLZAwg

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: wkzUMS5cSBG96Plz3wsOAHO6dEgjb02qqRF157ExGruJ3R-WQRLHw1mznIb_INaFTdfyFty1OfwRpeHvMNL20Q
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 2TMAFmat65msTjEYy_S4T_Ju_mzSLaeO1vbc-6YqeLTS5QfjFafA4ERs8yGOnHRVv98WvaPLc1ApX0esWHTcbw

2️⃣ Calling function (Query + Update + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'value': 'active', 'type': 'String'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: OCdkOYs7ee_bpzmQWR6jK_1vJ4EE8pikbugE4zdjNXDKT6wYhS7Mw30Bpsm1bDylLm-BStAupK8g4G-sNrDNdA

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: lURapt1V4-oTIQ9LugHOg_Lt2FxVtIhQRMq5ys4hv3BGuyMW-VrJP8XQTi4c4kicADSO5uWmDvFQClwrzHwV5g

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: bvIi8swkOlVokQUn3GpQ...
   ✅ Deleted script: 2TMAFmat65msTjEYy_S4...
   ✅ Deleted script: OCdkOYs7ee_bpzmQWR6j...
   ✅ Deleted script: lURapt1V4-oTIQ9LugHO...
   ✅ Deleted collection: users

============================================================
✅ Complete CRUD Functions Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Functions chain Functions together
   ✅ Each function demonstrates operation + verification
   ✅ Parameters make functions reusable
   ✅ Verification is built into the function itself
   ✅ Complete CRUD lifecycle in 4 focused functions
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
  Output: Document ID = vkZLHBa4GdEI5rqqPglVDludtjCn54yWjk87RZXw3_4c7IpmUNEIPYTQucCw_vhygdp9g1kb_-dxSjpMbdhZPg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(vkZLHBa4GdEI5rqqPglVDludtjCn54yWjk87RZXw3_4c7IpmUNEIPYTQucCw_vhygdp9g1kb_-dxSjpMbdhZPg)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(vkZLHBa4GdEI5rqqPglVDludtjCn54yWjk87RZXw3_4c7IpmUNEIPYTQucCw_vhygdp9g1kb_-dxSjpMbdhZPg)
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
  Output: Document ID = 9xbV1l0rrGf51wMkwr2MRqeHDrAQo9ZgGKbB9xMEwoK37qU7Bed16SIHqaWAgIePMksWA0tC1btM1jD6NF5rfQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(9xbV1l0rrGf51wMkwr2MRqeHDrAQo9ZgGKbB9xMEwoK37qU7Bed16SIHqaWAgIePMksWA0tC1btM1jD6NF5rfQ)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(9xbV1l0rrGf51wMkwr2MRqeHDrAQo9ZgGKbB9xMEwoK37qU7Bed16SIHqaWAgIePMksWA0tC1btM1jD6NF5rfQ)
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
=== Running crud_functions.py ===[0m
[32m✓ crud_functions.py completed successfully[0m
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
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.18s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.15.2-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.15.2-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.15.2
    Uninstalling ekodb_client-0.15.2:
      Successfully uninstalled ekodb_client-0.15.2
Successfully installed ekodb-client-0.15.2
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'rftuStF0eDBdt_rbmgtvd2K9_GwYMPyMktLk48507wrJRUZ5eZlC-HrOndAfTJRJmCejRNS_1f8X0UHg8zUVqA'}

=== Find by ID ===
Found: {'value': {'value': 42, 'type': 'Integer'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'price': {'value': 99.99, 'type': 'Float'}, 'id': 'rftuStF0eDBdt_rbmgtvd2K9_GwYMPyMktLk48507wrJRUZ5eZlC-HrOndAfTJRJmCejRNS_1f8X0UHg8zUVqA', 'name': {'type': 'String', 'value': 'Test Record'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'created_at': {'type': 'String', 'value': '2026-04-05T23:50:41.116896'}, 'active': {'type': 'Boolean', 'value': True}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-04-05 23:50:41.116896
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'value': 42, 'data': 'aGVsbG8gd29ybGQ=', 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'tags': ['tag1', 'tag2', 'tag3'], 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'price': 99.99, 'id': 'rftuStF0eDBdt_rbmgtvd2K9_GwYMPyMktLk48507wrJRUZ5eZlC-HrOndAfTJRJmCejRNS_1f8X0UHg8zUVqA', 'name': 'Test Record', 'categories': ['electronics', 'computers'], 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'created_at': '2026-04-05T23:50:41.116896', 'active': True}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'id': 'rftuStF0eDBdt_rbmgtvd2K9_GwYMPyMktLk48507wrJRUZ5eZlC-HrOndAfTJRJmCejRNS_1f8X0UHg8zUVqA', 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'value': {'type': 'Integer', 'value': 100}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'created_at': {'value': '2026-04-05T23:50:41.116896', 'type': 'String'}, 'active': {'value': True, 'type': 'Boolean'}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'price': {'value': 99.99, 'type': 'Float'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: hqRl--vcMPU5-RLMkj7kaOKE5ePNrd1vmUodBew0JkZv-UWXBksXORfuIWeaAz85C1Y-robimmnepRlw8WKRug

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
Collection created with first record: "_ST8BBYn-3nh1iqM2USCP0mSox5uOTOpQ2RIldfR6lejxNk_5OcpTGNdVU14rQbN9lJ1Os6EqHWkak1CKWZrCg"

=== List Collections ===
Total collections: 37
Sample collections: ['audit__ek0_testing', 'cached_database_metrics', 'chat_configurations__ek0_testing', 'certificate_diagnostics_testing', 'websocket_test']

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
  cache:product:2: {'name': 'Product 2', 'price': 39.99}
  cache:product:3: {'name': 'Product 3', 'price': 49.99}

=== KV Exists ===
Key exists: True

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 13

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
Created Alice: $1000 - ID: Oazy-KtM8hvK-POlGt1Nb1olUF8q3OP8k1M5WopI6qqBJ1hA8O-lwWZ8E7NKBhZhPz4NtKpblk1daHwQcAl7VQ
Created Bob: $500 - ID: Sok9lz8LgXxo9gfgqG-WslCUsOhgXT8bqixPRrL3ZHrp9QOLdDnVSbJDAaCbdn4AeL-wWl4ikjH_2PzyX_ZiPw

=== Example 1: Begin Transaction ===
Transaction ID: 2592b160-ddd9-41a0-84ab-657f6dba5ea4

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 7e1712de-7e05-4541-a1be-dbe16de780c8
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
  1. Score: 25.740, Matched: name.value, email, email.value, name
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, bio.value, title, title.value
  2. Score: 26.400, Matched: bio, title.value, bio.value, title
  3. Score: 26.400, Matched: title, bio, title.value, bio.value
  4. Score: 26.400, Matched: title.value, title, bio.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio, title.value, bio.value
  2. Score: 39.600, Matched: bio.value, title.value, title, bio
  3. Score: 39.600, Matched: title, bio.value, bio, title.value
  4. Score: 39.600, Matched: bio.value, title, title.value, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.759
  2. Score: 0.744
  3. Score: 0.727

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.363, Matched: title.value, content, title, content.value
  2. Score: 1.380, Matched: title.value, content.value, title, content
  3. Score: 0.372, Matched: 

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
✓ Inserted document: pNzOrst0kZBjP_EJD8GS3lUWL9wVfHQC1RC5ANcUYUmq4i4K8ajqErogqyoHj4vUV2v8tYWDNYAqSsLbvDT1Rg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: m4nm0viqQ3JuW9kQXp4AyURxxRY_g4lq7ouUVPY9KWPQTrVUlH5AWwdkuit3cZnej8Y7mubhoa5ShYW70OpZ0Q

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
✓ Inserted document with TTL: rj-qrETvuCUDFE7MLTN12yzZlDFMpMmZCosZuWsOLrmp5QSYa2jlff52CxpClINM2fG7mTFwGsBtMe-eS5HVlg

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
✓ Edge cache script created: CdcE2LurmuA7PsGb84P4kdiAMmnFtzOyFLoPz0BLoKvUc3Jrp7wJAnlbtEcseiphRtXpuIrAiqzbWSrAJrNvvg

Call 1: Cache miss (fetches from API)
Response time: 519ms
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 7.8,
          "time": "2026-04-06T03:45"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.01347064971923828,
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
Response time: 41ms (12.8x faster!)
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 7.8,
          "time": "2026-04-06T03:45"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.01347064971923828,
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

🚀 ekoDB Functions Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: YL7byismqgHdxwqMqGV8KeWkHlQp3C3B6eSEU6i5_5tV6eN5uv1nS7rqDWNeLQu19fC957N8XZ9H3PsVn9BE_g
📊 Found 10 active users

📝 Example 2: Parameterized Function

✅ Function saved: doSE8aIcTMfX3w-GmWO2VZTT9uuRnzXaT_kG8oWgHFPmTFDdTw8192bmfGq2WxtM4Tf3RMc33SBpyimsS4_4jw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: oab9x4U9oiuVV2EnjK90xDkUl4uELE-XCji2gaIx5CJ1q91x5GKwq_r168mbPhsTXvQRX-OdZMQVMjuEzPLDTw
📊 Statistics: 2 groups
   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

📝 Example 4: function Management

📋 Total scripts: 18
🔍 Retrieved script: Get Active Users
✏️  function updated
🗑️  function deleted

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
   ⏱️  Duration: 39.7ms
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
   ⏱️  Duration: 40.2ms
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
🚀 ekoDB Python Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: S08XmzmxLipw2N8kviaYch2P5Xjhw5bo-8CqAUkI2gTjR8M3S4pFh2WnKqyZU6sb-HL8P-_NIFQfTM2lhjA1Jg
📊 Found 2 product groups
   {'avg_price': 575.6666666666666, 'category': 'Electronics', 'count': 3}
   {'avg_price': 474.0, 'category': 'Furniture', 'count': 2}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {'category': 'Furniture', 'count': 2}
   {'category': 'Electronics', 'count': 3}
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
🚀 ekoDB Python KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: dpw2q4rVMHNjDWf_365aKzcpfDftcUL-YDpme9AsIlJUNpWWOy-fpQxot8XXTv1v-XjlErB-zE3TahHsV-Gg4w
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
🗑️  Deleted session

📝 Example 4: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 5: Combined Wrapped Types + KV Usage

✅ Inserted order: HiDzt1YmH61jxbjBB-FbcVzwtaPS85Kn0Q7IgmIQYWKFVLh2vgOn8aaVlo5pq0rXgKTdbLmCIbETZKHFk2I_qA
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-04-06T03:50:44.567511+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (5F-2p8LjTsw-3AIWx9Do0e0-zndM3WIYQUjl-FAMvGEEwdq1wfswO2XIW0MWyr6_N0upjraq9kQzENP7zOGa9A)

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
Response time: 106ms (served from cache)
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
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🚀 ekoDB Python Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native (th6hsL7C24iBl8tM8WB6tz-aUbPAM4vWqywtyXTcFf6T1AjJzw4z9vwl8JX_YdVZU_5Hje0ODejLKTnOqpoR8Q)

First call (cache miss - will fetch from GitHub API):
  Response time: 123ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 16ms
  Speedup: 7.6x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (PUsBzTT8smpYcQOqIRIcCEsL8pQjkUee4pQuH9YU33trX3AYzaC9ihxHz5wN3w-ajPdYqOg-r9jBc7ya7NVORw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (PLYcZJPmPgL5oqU3b2mXSXayZYBvShVXddl4pHzKNQIwbwv-sZb5TgCvb8brhxTJH6y4VdZEEP3GsBy09VBmjQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (vL02hya0fy3Qcsi2n20_99sIXSqHZq_4XHyk_5nH0l5Ey6JF8d-_exV8Tk2DdZK-T5JCYiePBviygriykqMqnw)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

================================================================================
✅ Key Benefits of Native SWR:
✅ Single function: Replaces 4-function cache-aside pattern
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)
Native SWR:     SWR → Insert (2 functions)
Result:         60% fewer functions, cleaner code, same behavior 🎯

🧹 Cleaning up...
✓ Deleted 4 test scripts

✅ All examples completed!
🚀 ekoDB Python Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   {'avg_price': 367.0, 'category': 'Electronics', 'count': 5}
   {'avg_price': 365.6666666666667, 'category': 'Furniture', 'count': 3}
⏱️  Execution time: 0ms

📝 Example 3: Count Total Products

✅ Function saved
📊 Total products: [{'total': 8}]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB Python AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They excel at searching for similar items based on vector embeddings, making them ideal for applications like recommendation systems and image retrieval.

2. **High-dimensional Data Handling**: Vector databases can manage and query high-dimensional data efficiently, which is common in AI and machine learning.

3. **Scalability**: Many vector databases are designed to scale horizontally, accommodating growing datasets without significant performance degradation.

4. **Real-time Processing**: They support real-time querying and analytics, allowing fast responses in dynamic applications.

5. **Integration with AI Models**: They easily integrate with machine learning models and frameworks, streamlining workflows involving AI and data processing.

6. **Advanced Indexing Techniques**: Vector databases utilize specialized indexing structures (e.g., HNSW, IVF) that optimize search performance.

7. **Multi-modal Data Support**: They can handle various types of data (text, images, audio) by converting them into vector form.

8. **Enhanced Search Capabilities**: They offer capabilities such as semantic search, enabling more natural language understanding and retrieval.

Overall, vector databases are crucial for handling modern AI applications requiring fast and efficient processing of complex, high-dimensional data.
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
🚀 ekoDB Python CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   active: 7 users
   inactive: 3 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average score by role:
   {'avg_score': 20.0, 'count': 3, 'role': 'admin'}
   {'avg_score': 70.0, 'count': 7, 'role': 'user'}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB Python Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Database Design Principles (Database)
   2. Natural Language Processing (AI)
   3. Introduction to Machine Learning (AI)
   4. Getting Started with ekoDB (Database)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
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
✓ Created session: DxcKnNVdBVFO5renQijv8LByj3cDksxCjDxl7-r5_dvAptNWkD2LgUn-bgrVymKRhkRW2wL_vJ1x5VFVI6VPHw

=== Sending Chat Message ===
Message ID: MYOZDJKKlLbyuXdYfaosdiuL0m-aI9sIOhuViO7Wxp69NInxqt7icgkn3FRYd4BtJgKF23P4nlOiHRFxAuoBdw

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features.

2. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities.

3. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product.

Execution Time: 4636ms

=== Token Usage ===
Prompt tokens: 1298
Completion tokens: 91
Total tokens: 1389

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: U4UfrPvg6kMQFOc39M23bLBkHCJtnHLEcyVRsPngo8VdE9bGhnEhQ8imZ2HOpWh90w_YUujJtp3-ULSIuuGatA

=== Sending Initial Message ===
✓ Message sent
  Response: Currently, the available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more details or have any further questions, feel free to ask!

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['id', 'llm_model', 'token_usage', 'content', 'context_snippets', 'role', 'chat_id', 'created_at', 'llm_provider', 'updated_at'])
Debug: First message role: {'value': 'assistant', 'type': 'String'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any more questions or need additional information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: YP5qswPHkbIDIVGAfdU0RmffBHKwrpZl6ZFKCk3xKPLWYOAHHg_fQI5ac-GqelJrYv1Wo87jFj0o7a-KiObgVg
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
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Lgj7ao7hej-PLDxXRWbB-gJVVaO_BmS1m4YQMX35qgSJrEXyZNFjQFzsDIw6bWYPKPhfAxANp1-b36KjoJu-yQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more information or have other questions, feel free to ask!

✓ Message 2 sent
  Response: The price of the product ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: lCtRDiXOzJWKnoyqaS0zV59E0XXUe37sIX7HVxZjOBO6bBkiK4BrREsV0TIIiu2f0E6KSABPkZjCo0xeDLaayQ
  Parent: Lgj7ao7hej-PLDxXRWbB-gJVVaO_BmS1m4YQMX35qgSJrEXyZNFjQFzsDIw6bWYPKPhfAxANp1-b36KjoJu-yQ

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: lCtRDiXOzJWKnoyqaS0zV59E0XXUe37sIX7HVxZjOBO6bBkiK4BrREsV0TIIiu2f0E6KSABPkZjCo0xeDLaayQ (Untitled)
  Session 2: Lgj7ao7hej-PLDxXRWbB-gJVVaO_BmS1m4YQMX35qgSJrEXyZNFjQFzsDIw6bWYPKPhfAxANp1-b36KjoJu-yQ (Untitled)
  Session 3: YP5qswPHkbIDIVGAfdU0RmffBHKwrpZl6ZFKCk3xKPLWYOAHHg_fQI5ac-GqelJrYv1Wo87jFj0o7a-KiObgVg (Untitled)
  Session 4: DxcKnNVdBVFO5renQijv8LByj3cDksxCjDxl7-r5_dvAptNWkD2LgUn-bgrVymKRhkRW2wL_vJ1x5VFVI6VPHw (Untitled)
  Session 5: 5UXzj18M6q9HTsNd9G6KNym3g3rukd_sJJzGSvP0zCDTIPcZAL3z2EzKjldUqeILYAuQzKhCP3SA99CndHkKIA (Untitled)
  Session 6: DhA8DyhZtR0FuoyBL1w-RIlzIpVPoqizfcdl22D8cB31svLhft8zl4DkRsY6AgPDTTr2LuqVa1Oz4p5MeuX2cw (Untitled)
  Session 7: TfKmmOMR_2POqQKuPE-ULopkgjXNfSUIAYaLdZXTvnxQXfHoloZdTGSH9hr6eSm1IwL5Mn52hzhdMgkxpKxTWA (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: lCtRDiXOzJWKnoyqaS0zV59E0XXUe37sIX7HVxZjOBO6bBkiK4BrREsV0TIIiu2f0E6KSABPkZjCo0xeDLaayQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'Ft6nmKbJb4n8qbwJJqLrAFzju-62R366iKf8w1IL-FegAsjUQ-TKmkzU5NQrfhYpfCc5yby3TfAVod8vKRE85w'}

=== Upsert Operation ===
✓ Upsert (update existing record): Ft6nmKbJb4n8qbwJJqLrAFzju-62R366iKf8w1IL-FegAsjUQ-TKmkzU5NQrfhYpfCc5yby3TfAVod8vKRE85w
✓ Inserted second record: lBYNrc9p7g6tDCyvSFvZGR9eXcMARvP6Cng5mE-uAL8L3hM_N0BC_S98O6oi74JiV79mirCO7VKleIPrXC5Xxw
✓ Upsert (update second record): lBYNrc9p7g6tDCyvSFvZGR9eXcMARvP6Cng5mE-uAL8L3hM_N0BC_S98O6oi74JiV79mirCO7VKleIPrXC5Xxw

=== Find One Operation ===
✓ Found user by email: {'active': {'type': 'Boolean', 'value': True}, 'name': {'value': 'Alice Johnson', 'type': 'String'}, 'age': {'type': 'Integer', 'value': 29}, 'id': 'Ft6nmKbJb4n8qbwJJqLrAFzju-62R366iKf8w1IL-FegAsjUQ-TKmkzU5NQrfhYpfCc5yby3TfAVod8vKRE85w', 'email': {'value': 'alice.j@newdomain.com', 'type': 'String'}}
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
   Inserted with ripple: {'id': 'TYeMLnPy7JRI5-Sog96gbLEM_wWWB25YeDK3RYGV6Ta4jUgP1lX--rQZriBF6xffrJM_m3qER058J76S_fVwng'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'sMyULDwR3GH2gb-srowODvYi4Y3-u2auPghuy11lfZrqUU47GAnoq4J7umjY4nzHCDJ77XyaihBIkJGwBxQwoQ'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'name': {'type': 'String', 'value': 'Product 1'}, 'id': 'TYeMLnPy7JRI5-Sog96gbLEM_wWWB25YeDK3RYGV6Ta4jUgP1lX--rQZriBF6xffrJM_m3qER058J76S_fVwng', 'price': {'value': 150, 'type': 'Integer'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'price': {'value': 500, 'type': 'Integer'}, 'name': {'value': 'Upsert Product', 'type': 'String'}, 'id': 'TYeMLnPy7JRI5-Sog96gbLEM_wWWB25YeDK3RYGV6Ta4jUgP1lX--rQZriBF6xffrJM_m3qER058J76S_fVwng'}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['id', 'email', 'name']
  First user: {'type': 'String', 'value': 'Bob Smith'} <{'value': 'bob@example.com', 'type': 'String'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['user_role', 'avatar_url', 'email', 'bio', 'name', 'status', 'created_at', 'age', 'id']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'value': 'Dave Brown', 'type': 'String'} (age {'value': 45, 'type': 'Integer'})
    - {'value': 'Alice Johnson', 'type': 'String'} (age {'type': 'Integer', 'value': 30})
    - {'type': 'String', 'value': 'Bob Smith'} (age {'value': 25, 'type': 'Integer'})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'value': 'Carol White', 'type': 'String'}: {'type': 'String', 'value': 'Manager'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['email', 'bio', 'age', 'name', 'id', 'user_role', 'created_at', 'avatar_url', 'status', 'api_key', 'password', 'secret_token']
  Projected query:
    - 3 fields per record
    - Fields: ['email', 'name', 'id']
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
=== KV Precision: Float vs Decimal ===

=== Test 1: Using Python Floats (LOSES PRECISION) ===
✓ Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 2: Using field_decimal() (PRESERVES PRECISION) ===
✓ Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.30000000000000004 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Cleanup ===
✓ Cleaned up test keys

=== Summary ===
✅ Use field_decimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ field_decimal() stores values as strings internally,
   preserving exact precision across all operations.
✓ Client created

=== Get All Chat Models ===
OpenAI models: ['gpt-4-0613', 'gpt-4', 'gpt-3.5-turbo', 'gpt-5.4-mini', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'dall-e-3', 'dall-e-2', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'gpt-4o-audio-preview', 'gpt-4o-realtime-preview', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'gpt-4o-realtime-preview-2024-12-17', 'gpt-4o-audio-preview-2024-12-17', 'gpt-4o-mini-realtime-preview-2024-12-17', 'gpt-4o-mini-audio-preview-2024-12-17', 'o1-2024-12-17', 'o1', 'gpt-4o-mini-realtime-preview', 'gpt-4o-mini-audio-preview', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'gpt-4o-realtime-preview-2025-06-03', 'gpt-4o-audio-preview-2025-06-03', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'gpt-realtime-mini-2025-10-06', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-3.5-turbo-16k', 'tts-1', 'whisper-1', 'text-embedding-ada-002']
Anthropic models: ['claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929', 'claude-opus-4-1-20250805', 'claude-opus-4-20250514', 'claude-sonnet-4-20250514', 'claude-3-haiku-20240307']
Perplexity models: ['sonar', 'sonar-pro', 'sonar-deep-research', 'sonar-reasoning', 'sonar-reasoning-pro']

=== Get OpenAI Models ===
OpenAI models: ['gpt-4-0613', 'gpt-4', 'gpt-3.5-turbo', 'gpt-5.4-mini', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'dall-e-3', 'dall-e-2', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'gpt-4o-audio-preview', 'gpt-4o-realtime-preview', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'gpt-4o-realtime-preview-2024-12-17', 'gpt-4o-audio-preview-2024-12-17', 'gpt-4o-mini-realtime-preview-2024-12-17', 'gpt-4o-mini-audio-preview-2024-12-17', 'o1-2024-12-17', 'o1', 'gpt-4o-mini-realtime-preview', 'gpt-4o-mini-audio-preview', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'gpt-4o-realtime-preview-2025-06-03', 'gpt-4o-audio-preview-2025-06-03', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'gpt-realtime-mini-2025-10-06', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-3.5-turbo-16k', 'tts-1', 'whisper-1', 'text-embedding-ada-002']

=== Get Anthropic Models ===
Anthropic models: ['claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929', 'claude-opus-4-1-20250805', 'claude-opus-4-20250514', 'claude-sonnet-4-20250514', 'claude-3-haiku-20240307']

✓ Chat Models API example complete
✓ Client created

=== Create User Function ===
Created user function with ID: dnBQOxdm49vIGsa6CEzZ13ADTuUnRZtaK_gc8jCFEZ-BoXHBlsYL3wEI89yZCM-0JUVcepAEyagD9nTlhGcggQ

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 25 user functions:
  - fetch_user: Fetch user by code
  - get_users_by_status: Get Users By Status
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_active_users_py: Get Active Users
  - get_user_wrapper: Wrapper that calls fetch_user
  - swr_user: SWR pattern for user data (KV-based)
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_paginated: Get Active Users (Paginated)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_users_by_status: Get Users By Status
  - validate_user: Check if user exists
  - get_verified_user: Get verified and validated user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - cache_api_call_py: Cache External API Call
  - get_high_scoring_active_users: Get High Scoring Active Users
  - validate_user: Check if user exists
  - get_active_users: Get Active Users (Updated)
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_user: Fetch user by code
  - get_active_users: Get Active Users (Updated)
  - get_verified_user: Get verified and validated user
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_slim_user: Validate and slim down user
  - fetch_slim_user: Validate and slim down user

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test_py' exists: False

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test_py' exists: True

=== Count Documents ===
Document count in 'collection_utils_test_py': 5

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: False

=== Cleanup ===
Deleted collection 'collection_utils_test_py'

✓ Collection Utilities example complete
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
Inserted: map[id:ScyWDovw-h1x3Tcub7RtBjfO2BzO-sgtCOWw3ymjVFgPiUFlQl7EzYFZMlptsqcJQyAC51fUgZPuVXO5lreK4A]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:ScyWDovw-h1x3Tcub7RtBjfO2BzO-sgtCOWw3ymjVFgPiUFlQl7EzYFZMlptsqcJQyAC51fUgZPuVXO5lreK4A name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: [map[active:map[type:Boolean value:true] id:ScyWDovw-h1x3Tcub7RtBjfO2BzO-sgtCOWw3ymjVFgPiUFlQl7EzYFZMlptsqcJQyAC51fUgZPuVXO5lreK4A name:map[type:String value:Test Record] value:map[type:Integer value:42]]]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:ScyWDovw-h1x3Tcub7RtBjfO2BzO-sgtCOWw3ymjVFgPiUFlQl7EzYFZMlptsqcJQyAC51fUgZPuVXO5lreK4A name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: bHP1ni5oaE0HgsNvnFzZnpbpJK6MKlPwmNaxhEk4ZOLvCXyckqpXu5c6bKDkDawxQ6Fp8hrQ-iqhLJcf8dZ1lA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1775447479392428000",
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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "bHP1ni5oaE0HgsNvnFzZnpbpJK6MKlPwmNaxhEk4ZOLvCXyckqpXu5c6bKDkDawxQ6Fp8hrQ-iqhLJcf8dZ1lA",
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
        "id": "YqPd6d25_D4tl6fKjbje3AExz8arcSNbcmeUwMRvFm_j2tAzd2q0DMFW65DwWrroSYcvnXyFUI5AqnR8pXWDOg",
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
🚀 ekoDB Functions Example (Go/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: zAEgaoIeyCzPu0ySBpyIkmqFIIC0rkl0hWsOlny7rP9ybtPi6nhEW073KH2adEaYuSIevUvVDmB4idL1L7BT2w
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: JxgiyniQwx4Wl22v82PVASr5MASykbO2lSbWGHjRlM3HrUaVXfp28L9jGGyQPk63de4mXeeFoJCRej1wQcjrVw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: amY5DVGf_6JAVY5tUjOi_IQ5swGGGNEx3z0Ljjl6f6ohb7opq9cmIN8RDUU4fbRWOOEVJRH5lsnErf9iYZvTjA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}
   {"avg_score":50,"count":10,"max_score":90,"status":"inactive"}

📝 Example 4: Function Management

📋 Total scripts: 27
🔍 Retrieved script: Get Active Users
✏️  function updated
🗑️  function deleted

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
Collection created with first record: IwZscuIO7h4UfdkSO6YB4Dro0f3CcGDCgRZmzXd1e3NHBN8ijmK3hHvR9Vd0VNKORoM3G3HaqMToYqBF-Ks9hw

=== List Collections ===
Total collections: 42
Sample collections: [audit__ek0_testing cached_database_metrics enriched_users schema_employees_client_py chat_configurations__ek0_testing]

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
Created Alice: $1000 - ID: K4BEf1pCOlLYX2wyDJA4xoTc_vK0If5fTs522uHUWaO3jjhrz8QzdwIDEjn1_CLSX4ldMKEoJdLAJLs5D0eiaA
Created Bob: $500 - ID: xuYi6dRIzPsYialjPoKMUUuB_g9STSusOvX-wV4NzA_rksdD0feKkM_qhjZHHU0bHjCBG4RoeVD4Opb5wF86eA

=== Example 1: Begin Transaction ===
Transaction ID: 539d80a7-02cb-429b-8243-67629f505d8d

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: map[type:Integer value:800]
Bob: map[type:Integer value:700]

=== Example 5: Rollback ===
New transaction: e2221e2a-7eb7-44d5-bb5c-e02e60dcf148
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: map[type:Integer value:700]

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
[32m✓ transactions.go completed successfully[0m
[34m
=== Running crud_functions.go ===[0m
🚀 ekoDB Complete CRUD Functions Example
============================================================
Demonstrates:
  • Insert + Verify (using Query)
  • Query + Update Status + Verify
  • Query + Update Credits + Verify
  • Query Before Delete + Delete + Verify Gone

Each function shows Functions chaining with proper verification
============================================================

============================================================
📝 function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: OLpINwRh1k41RVJJoiyt6dPUxHmysxK1GtRkxTe-9m8xFsBi02Fzu1EzOZzmshz-THemfvNJvw0h4z_3C-sUhg

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: 4JOzD_11yTREAl-r45mWG1tKYCk33TV-sRsfn-bIsgXLhGcm75rR_qiuUgowr_FfWYLktLD8sNudE9IMlC7JAA
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: xg-nr8wjPpxtxy9MXp8IG1P2fULlQHRKDRqKFWhU3aGb73KNce8iFADkNIA-P7PJGJqgT8Ng7-UVZlUzeqZPSQ

2️⃣ Calling function (Query + Update + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: map[type:String value:active]
   📋 Name: map[type:String value:Alice Smith]

============================================================
📝 function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: Q0_-rHqpVplMuRF4ljg4RK3HQIdUWjHokW5vMRgPTtpfQca6mvywib4d0ODZ2tM4lo9fIa83FGPeoqA8P2Ls3g

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: map[type:Integer value:0]
   📋 Status: map[type:String value:active]
   📋 Name: map[type:String value:Alice Smith]

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: dijPu-NS4kacLjtRyt4agJhEIV1MHEEGvHiup4iq2A8LEL8ewbuUfllooQhHbRtbSmNpfTfB7J6bD-73uphX6A

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: OLpINwRh1k41RVJJoiyt...
   ✅ Deleted script: xg-nr8wjPpxtxy9MXp8I...
   ✅ Deleted script: Q0_-rHqpVplMuRF4ljg4...
   ✅ Deleted script: dijPu-NS4kacLjtRyt4a...
   ✅ Deleted collection: users

============================================================
✅ Complete CRUD Functions Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Functions chain steps together
   ✅ Each function demonstrates operation + verification
   ✅ Parameters make functions reusable
   ✅ Verification is built into the function itself
   ✅ Complete CRUD lifecycle in 4 focused functions
[32m✓ crud_functions.go completed successfully[0m
[34m
=== Running document_ttl.go ===[0m
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
  Output: Document ID = U6a2_aQCbJXgTGYLjH1VdOeWrjg1Yr7V744SYk7ftkbe-iSH1HbxZhToLUszTZ_C4U21uMD5FYLrOkgh2SJvgg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(U6a2_aQCbJXgTGYLjH1VdOeWrjg1Yr7V744SYk7ftkbe-iSH1HbxZhToLUszTZ_C4U21uMD5FYLrOkgh2SJvgg)
  Output: Found document with name = map[type:String value:TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(U6a2_aQCbJXgTGYLjH1VdOeWrjg1Yr7V744SYk7ftkbe-iSH1HbxZhToLUszTZ_C4U21uMD5FYLrOkgh2SJvgg)
  Output: Error (expected) - request failed with status 404: ��Record not found (expired)
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

✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = UkppPTmdxtg9PoNClqzEIyHbDB8KZ7IHHk2OqW114yY4SQmXCBZe1USPG60HbdY8YxrBZ9q1Dn7mkkPhRaWWhA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(UkppPTmdxtg9PoNClqzEIyHbDB8KZ7IHHk2OqW114yY4SQmXCBZe1USPG60HbdY8YxrBZ9q1Dn7mkkPhRaWWhA)
  Output: Found document with name = map[type:String value:WS TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(UkppPTmdxtg9PoNClqzEIyHbDB8KZ7IHHk2OqW114yY4SQmXCBZe1USPG60HbdY8YxrBZ9q1Dn7mkkPhRaWWhA)
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
Inserted: map[id:-XLbdyhQG-l38yBgck3Po6vbnQhrxQP0wn09w_thxG6zPssd7xKyEUB2gmsZdLs0ZiIxiltbtRBA_cYg5NvN7A]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-04-06T03:51:49Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:-XLbdyhQG-l38yBgck3Po6vbnQhrxQP0wn09w_thxG6zPssd7xKyEUB2gmsZdLs0ZiIxiltbtRBA_cYg5NvN7A metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-04-06 03:51:49 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-04-06T03:51:49Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:-XLbdyhQG-l38yBgck3Po6vbnQhrxQP0wn09w_thxG6zPssd7xKyEUB2gmsZdLs0ZiIxiltbtRBA_cYg5NvN7A metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-04-06T03:51:49Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:-XLbdyhQG-l38yBgck3Po6vbnQhrxQP0wn09w_thxG6zPssd7xKyEUB2gmsZdLs0ZiIxiltbtRBA_cYg5NvN7A metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: wfAUxd3-0g7-CpRAl2L-S1RigEA67YjRQ7QvNxWpeEvCOAYD7zA6P-XBJd5ERJRMGjBPyUPgy2MDlkcgFIRIWQ

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
Collection created with first record: Pq6WOLUeHZiH-BhpM8vBHmbeh_HxwrshnXCHjlK7vuil8qoKgEuP0YM-rqOkZvu7dhg7h0exHU69F9Ag88CYLg

=== List Collections ===
Total collections: 41
Sample collections: [audit__ek0_testing cached_database_metrics enriched_users schema_employees_client_py chat_configurations__ek0_testing]

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
  cache:product:1: map[name:Product 1 price:29.99]
  cache:product:2: map[name:Product 2 price:39.99]
  cache:product:3: map[name:Product 3 price:49.99]

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 16

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
Created Alice: $1000 - ID: -YQIv-vax-zRMJ9wOiZdC6YlREECTBAy67DyH7WfaFhzg1gdgqq2Xx3FiBa6bcC5vVVfBNdGnDT66deqF5YVFg
Created Bob: $500 - ID: eUo7I9VP5vUR3d0EzGJV4xvdUhsWBVWJxx5hHpq1g2Jp7DN9052eCkiO0KaNKbYhRmjNpnTA-9EG15EIvnoi_Q

=== Example 1: Begin Transaction ===
Transaction ID: bb98c4aa-2c4d-42f8-9833-577de2716242

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 6ba7b7af-8e0e-4f93-8740-813cd74b86e9
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
  1. Score: 0.770
  2. Score: 0.764
  3. Score: 0.752

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.705
  2. Score: 1.501
  3. Score: 0.308

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
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: EkEQuIa-wuGNT66ibCnMdFiHsqVoAyOp-a-mhHkn1vLLZ8-milKNgCZqpzhtXEEQsAuoAa6BxtDttBKRLsLC0A

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: OGt8xtP7H9JRo2XH8-kDnUQtLfBoMQo2OQkIYjLi5bYrIPZP8A7YhFoR0Fkg4QbZjCbgRR8H7fWpxVu1Q52F6g

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
✓ Inserted document with TTL: I14Acc3ptsCcH58FD9P9TX-9QsuwlRJVTjyN4BgwiEH_4OBgXQ9eraTG5GF-KzxpcodUCCnRmbkamVU3GIAeyA

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
✓ Edge cache script created: V96fE05to9vzFkiil3lhooDN37B5bKKyWwGD-UDI9Udor6lMUIkdVBcw4DVterMuGUs4n8rZYf2nzDumvM5bsw

Call 1: Cache lookup
Response time: 15ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 15ms
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
🚀 ekoDB Functions Example (Go Client)

✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: DIosnDpn6J8hVQB4YHnFpASofFplp-A7ZFqDN4jqe49HPDPuu8e1tjo8Eh9g5n5zlSqUup6qnPeTuYIMaW-APg
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   map[avg_score:60 count:5 status:active]
   map[avg_score:50 count:5 status:inactive]
⏱️  Execution time: 0ms

📝 Example 4: function Management

📋 Total scripts: 29
🔍 Retrieved script: Get Active Users
✏️  function updated
🗑️  function deleted

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
   ⏱️  Duration: 39.747208ms
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
   ⏱️  Duration: 41.152792ms
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
🚀 ekoDB Go Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: 4mMgoKgCMjVxpMvTuC7xTNbeRKsjzUkEvMVylTj43g5eo46wvogfvJMumlHQARXKp41c0eK7IZ7-MaRxYD2ibQ
📊 Found 2 category stats

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories

📝 Example 4: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Function management (save, call, delete)
🚀 ekoDB Go KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: EfYgCU_FqznatQEpLyfGMy5FgEeKoilpk1ATKPnGoOBIyo_F7-Zc-778OUyKuthAHWL2_xXr_WjLSjZQLrOnvQ
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: lSkxsWHQQDQZqF-biAZbFAVwkIXSJnNqt4aRsbc_a4hOTWJSwrIXGim6SxmihEbjPTpWOW9cUGPGusf8AHfiGg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Functions

✅ Function saved: rjwaxX4aWAgIBtnRVOZWiLFg8go6q0FDbVZVmNxe8JIEjVSTtk1n6fgWwG4Yjal2_nanZxX6XmCnvdE4O2HKaw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: vHgbNpPPtuZXsVig00x_FgCUOlpWrOSXUVzRnHqn81SdYP-kkddJeYxFduXOJsek5PYCbFJCS8PRyinpD5-iPw
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
✓ Created SWR script: fetch_api_user_go (e3mWdv7MajE5xQGwX6SUI05WDgmAmrPLlMlya5jUJYLgrvPBDy1gpNxEuZlZG9UqHutE_Rf_1z7su_60JPOepw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": "2026-04-05T23:51:52-04:00",
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
      "id": "1"
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
Response time: 41ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🚀 ekoDB Go Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native (6w7kTrQfFgmn9VitnzJvY_22F26p8pnGuwzMCLh3emKphedRHzsO80mkPukkB-XqbzD5qEIZQGKnC7WlLrLatw)

First call (cache miss - will fetch from GitHub API):
  Response time: 15ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 15ms
  Speedup: 1.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (VZQkQFicFWO9mLS8_LyNrggt-mwW-ld5cRYP2AkURq1IQh3qhqqpqwnlR7apUDBxdSJ0_Rt4APOj3_h569UAew)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (ouoAqQa8EHKGxlheLS8PN-NMVVjpmItQY72J-_cxrrT79K6-AVZIYrTiBiIr9nNIsejfWT6tioRsKLqWKUN3bQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (ke6EMKZm96hSWl6n33qIAmQfMhR_fK0URpSPLkdGhU681WMQRYFr6nDlqWwBJNzxVxufs2zorqjt29n3Zod8nw)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

================================================================================
✅ Key Benefits of Native SWR:
✅ Single function: Replaces 4-function cache-aside pattern
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)
Native SWR:     SWR → Insert (2 functions)
Result:         60% fewer functions, cleaner code, same behavior 🎯

🧹 Cleaning up...
✓ Deleted 4 test scripts

✅ All examples completed!
🚀 ekoDB Go Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   map[avg_price:365.6666666666667 category:Furniture count:3]
   map[avg_price:367 category:Electronics count:5]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB Go AI Functions Example

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
🚀 ekoDB Go CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   map[count:7 status:active]
   map[count:3 status:inactive]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB Go Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. map[type:String value:Database Design Principles] (map[type:String value:Database])
   2. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   3. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
   4. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   5. map[type:String value:Natural Language Processing] (map[type:String value:AI])
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
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
✓ Created session: j35FSFi7T5MNfApTG_tS_0nktaY5bNDTaUm9_Oh1NZq0crz26C65MZINFB_tBRD96KcT1EvDsvfv9PqQglIKyg

=== Sending Chat Message ===
Message ID: arM9iNnQc4t6QdGRRR2gAQVCtvID2ZCzDP0DulkR56SM7uz1S2MJeb690OQ19O_MEzTxIEZHLQD1wOU-WowClg

=== AI Response ===
Here are the available products and their prices:

1. **ekoDB Cloud**  
   - **Price:** $499  
   - **Description:** Fully managed cloud database service product

2. **ekoDB**  
   - **Price:** $99  
   - **Description:** A high-performance database product with AI capabilities

3. **ekoDB Pro**  
   - **Price:** $299  
   - **Description:** Enterprise edition product with advanced features

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:EJdQ7Q31A4PIqstQB1Wm4kLXe-XY09rkGi4rynRABwapF-kJQUcLAkhX7r1Ac-OmX0AebbgiZ5_S4WLRfcO3xA name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:UohNm2rBo3WpNfXBYWIydcH5AAncNg2b7rMVo2N1_rRQyBFrY1PEiSpkHNWgZue4Z2e9w5Axt22E2BQYKy8o0w name:ekoDB price:99] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:4LHdNJtUWoPYLpVBxwKyctqdp7lMpBM3baebNId0jemV3aHsEJPHaF2C9g2rxw3TSCO1xiB-C5VJqxw9ZTak7w name:ekoDB Pro price:299] score:0.1111111111111111]

Execution Time: 6084ms

=== Token Usage ===
Prompt tokens: 1298
Completion tokens: 98
Total tokens: 1396

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 9Eij2nrFnnyU4dMG1oMYMDTNWwilDNsTWsTjfApiy2powrPiMwz02Q2ecRh7GNrxzGZmsWuhI9Ir09Fg1FJQyg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or wish to see additional products, please let me know!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any more questions or need further information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: _DgimDs70IFjtBMQiEuIgkb4BNyDwHu4ZzjUhOnlzAxAeCdSKVpiyLbaB8_hA-OsMxz_HPNaHeYBLDnawpQd2w
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
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: NDeOBVKsMCqOpcQHsNmV96Nji7V4MJEHYbKVcf_YfpvNMO5mcJIO19Nb_3ljx4BrrIZG572g5PizSy-vllMSSA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need more information or have any other questions, feel free to ask!

✓ Message 2 sent
  Response: The price of the product ekoDB is **$99**. If you have any more questions, feel free to ask!

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: ncxck5GtZ29ZoyGLWWUksZJNSjxJIUaLbEiXtcMNbp7ruai_q3ucHetVAZD8FUzaR44LwMLis6xxMQL-8M46IQ
  Parent: NDeOBVKsMCqOpcQHsNmV96Nji7V4MJEHYbKVcf_YfpvNMO5mcJIO19Nb_3ljx4BrrIZG572g5PizSy-vllMSSA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: ncxck5GtZ29ZoyGLWWUksZJNSjxJIUaLbEiXtcMNbp7ruai_q3ucHetVAZD8FUzaR44LwMLis6xxMQL-8M46IQ (Untitled)
  Session 2: NDeOBVKsMCqOpcQHsNmV96Nji7V4MJEHYbKVcf_YfpvNMO5mcJIO19Nb_3ljx4BrrIZG572g5PizSy-vllMSSA (Untitled)
  Session 3: _DgimDs70IFjtBMQiEuIgkb4BNyDwHu4ZzjUhOnlzAxAeCdSKVpiyLbaB8_hA-OsMxz_HPNaHeYBLDnawpQd2w (Untitled)
  Session 4: j35FSFi7T5MNfApTG_tS_0nktaY5bNDTaUm9_Oh1NZq0crz26C65MZINFB_tBRD96KcT1EvDsvfv9PqQglIKyg (Untitled)
  Session 5: Lgj7ao7hej-PLDxXRWbB-gJVVaO_BmS1m4YQMX35qgSJrEXyZNFjQFzsDIw6bWYPKPhfAxANp1-b36KjoJu-yQ (Untitled)
  Session 6: YP5qswPHkbIDIVGAfdU0RmffBHKwrpZl6ZFKCk3xKPLWYOAHHg_fQI5ac-GqelJrYv1Wo87jFj0o7a-KiObgVg (Untitled)
  Session 7: DxcKnNVdBVFO5renQijv8LByj3cDksxCjDxl7-r5_dvAptNWkD2LgUn-bgrVymKRhkRW2wL_vJ1x5VFVI6VPHw (Untitled)
  Session 8: 5UXzj18M6q9HTsNd9G6KNym3g3rukd_sJJzGSvP0zCDTIPcZAL3z2EzKjldUqeILYAuQzKhCP3SA99CndHkKIA (Untitled)
  Session 9: DhA8DyhZtR0FuoyBL1w-RIlzIpVPoqizfcdl22D8cB31svLhft8zl4DkRsY6AgPDTTr2LuqVa1Oz4p5MeuX2cw (Untitled)
  Session 10: TfKmmOMR_2POqQKuPE-ULopkgjXNfSUIAYaLdZXTvnxQXfHoloZdTGSH9hr6eSm1IwL5Mn52hzhdMgkxpKxTWA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: ncxck5GtZ29ZoyGLWWUksZJNSjxJIUaLbEiXtcMNbp7ruai_q3ucHetVAZD8FUzaR44LwMLis6xxMQL-8M46IQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:k5BuEE6vqG4DcWWPcx-26xsYHbqqkzrs_Y508tgQluMLFv_WqWGprBjcrZbhlEph2gNd1OvqSPOjzliAPHzK_Q]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:k5BuEE6vqG4DcWWPcx-26xsYHbqqkzrs_Y508tgQluMLFv_WqWGprBjcrZbhlEph2gNd1OvqSPOjzliAPHzK_Q name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:new-user-id]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:k5BuEE6vqG4DcWWPcx-26xsYHbqqkzrs_Y508tgQluMLFv_WqWGprBjcrZbhlEph2gNd1OvqSPOjzliAPHzK_Q name:map[type:String value:Alice Johnson]]
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
Inserted with ripple: map[id:LE5HQq4SOGkFL1uaeQ3-zkkJYacW2g0EWChkZOuJo9reryQlsjnp__QUku8YLbC-9BzxnO7sPZ2vFigDtZl03Q]
Inserted with bypass_ripple: map[id:9JROMqhrR5nZpWfi4qIMMOkL82efjtf6JBFdMfOOEi2wS8r6aTDFzSvl92cX4BZBoSxFnI5eVQVQ2IJOc3jHAQ]
Inserted with TTL and bypass_ripple: map[id:ckmZV7FImvfN78FAbYLG2WdQofx_ApBmXzJ-KjVad3bbjeRlveOdzNcQLB2E1FpziznBVnbsGUM8H0W0muOp-A]
Updated with bypass_ripple: map[id:LE5HQq4SOGkFL1uaeQ3-zkkJYacW2g0EWChkZOuJo9reryQlsjnp__QUku8YLbC-9BzxnO7sPZ2vFigDtZl03Q name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:custom-id]
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: [email id name]
  First user: Alice Johnson <alice@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: [age avatar_url bio created_at email id name status user_role]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - Dave Brown (age 45)
    - Alice Johnson (age 30)
    - Bob Smith (age 25)

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - Carol White: Manager

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: [age api_key avatar_url bio created_at email id name password secret_token status user_role]
  Projected query:
    - 3 fields per record
    - Fields: [email id name]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
=== KV Precision: Float vs Decimal ===

=== Test 1: Using Go Floats (LOSES PRECISION) ===
✓ Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 2: Using FieldDecimal() (PRESERVES PRECISION) ===
✓ Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.29999999999999999 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Cleanup ===
✓ Cleaned up test keys

=== Summary ===
✅ Use FieldDecimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ FieldDecimal() stores values as strings internally,
   preserving exact precision across all operations.
✓ Client created

=== Get All Chat Models ===
OpenAI models: [gpt-4-0613 gpt-4 gpt-3.5-turbo gpt-5.4-mini gpt-5.4 gpt-5.4-nano-2026-03-17 gpt-5.4-nano gpt-5.4-mini-2026-03-17 davinci-002 babbage-002 gpt-3.5-turbo-instruct gpt-3.5-turbo-instruct-0914 dall-e-3 dall-e-2 gpt-3.5-turbo-1106 tts-1-hd tts-1-1106 tts-1-hd-1106 text-embedding-3-small text-embedding-3-large gpt-3.5-turbo-0125 gpt-4-turbo gpt-4-turbo-2024-04-09 gpt-4o gpt-4o-2024-05-13 gpt-4o-mini-2024-07-18 gpt-4o-mini gpt-4o-2024-08-06 gpt-4o-audio-preview gpt-4o-realtime-preview omni-moderation-latest omni-moderation-2024-09-26 gpt-4o-realtime-preview-2024-12-17 gpt-4o-audio-preview-2024-12-17 gpt-4o-mini-realtime-preview-2024-12-17 gpt-4o-mini-audio-preview-2024-12-17 o1-2024-12-17 o1 gpt-4o-mini-realtime-preview gpt-4o-mini-audio-preview o3-mini o3-mini-2025-01-31 gpt-4o-2024-11-20 gpt-4o-mini-search-preview-2025-03-11 gpt-4o-mini-search-preview gpt-4o-transcribe gpt-4o-mini-transcribe o1-pro-2025-03-19 o1-pro gpt-4o-mini-tts o3-2025-04-16 o4-mini-2025-04-16 o3 o4-mini gpt-4.1-2025-04-14 gpt-4.1 gpt-4.1-mini-2025-04-14 gpt-4.1-mini gpt-4.1-nano-2025-04-14 gpt-4.1-nano gpt-image-1 gpt-4o-realtime-preview-2025-06-03 gpt-4o-audio-preview-2025-06-03 o4-mini-deep-research gpt-4o-transcribe-diarize o4-mini-deep-research-2025-06-26 gpt-5-chat-latest gpt-5-2025-08-07 gpt-5 gpt-5-mini-2025-08-07 gpt-5-mini gpt-5-nano-2025-08-07 gpt-5-nano gpt-audio-2025-08-28 gpt-realtime gpt-realtime-2025-08-28 gpt-audio gpt-5-codex gpt-image-1-mini gpt-5-pro-2025-10-06 gpt-5-pro gpt-audio-mini gpt-audio-mini-2025-10-06 gpt-5-search-api gpt-realtime-mini gpt-realtime-mini-2025-10-06 sora-2 sora-2-pro gpt-5-search-api-2025-10-14 gpt-5.1-chat-latest gpt-5.1-2025-11-13 gpt-5.1 gpt-5.1-codex gpt-5.1-codex-mini gpt-5.1-codex-max gpt-image-1.5 gpt-5.2-2025-12-11 gpt-5.2 gpt-5.2-pro-2025-12-11 gpt-5.2-pro gpt-5.2-chat-latest gpt-4o-mini-transcribe-2025-12-15 gpt-4o-mini-transcribe-2025-03-20 gpt-4o-mini-tts-2025-03-20 gpt-4o-mini-tts-2025-12-15 gpt-realtime-mini-2025-12-15 gpt-audio-mini-2025-12-15 chatgpt-image-latest gpt-5.2-codex gpt-5.3-codex gpt-realtime-1.5 gpt-audio-1.5 gpt-4o-search-preview gpt-4o-search-preview-2025-03-11 gpt-5.3-chat-latest gpt-5.4-2026-03-05 gpt-5.4-pro gpt-5.4-pro-2026-03-05 gpt-3.5-turbo-16k tts-1 whisper-1 text-embedding-ada-002]
Anthropic models: [claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929 claude-opus-4-1-20250805 claude-opus-4-20250514 claude-sonnet-4-20250514 claude-3-haiku-20240307]
Perplexity models: [sonar sonar-pro sonar-deep-research sonar-reasoning sonar-reasoning-pro]

=== Get OpenAI Models ===
OpenAI models: [gpt-4-0613 gpt-4 gpt-3.5-turbo gpt-5.4-mini gpt-5.4 gpt-5.4-nano-2026-03-17 gpt-5.4-nano gpt-5.4-mini-2026-03-17 davinci-002 babbage-002 gpt-3.5-turbo-instruct gpt-3.5-turbo-instruct-0914 dall-e-3 dall-e-2 gpt-3.5-turbo-1106 tts-1-hd tts-1-1106 tts-1-hd-1106 text-embedding-3-small text-embedding-3-large gpt-3.5-turbo-0125 gpt-4-turbo gpt-4-turbo-2024-04-09 gpt-4o gpt-4o-2024-05-13 gpt-4o-mini-2024-07-18 gpt-4o-mini gpt-4o-2024-08-06 gpt-4o-audio-preview gpt-4o-realtime-preview omni-moderation-latest omni-moderation-2024-09-26 gpt-4o-realtime-preview-2024-12-17 gpt-4o-audio-preview-2024-12-17 gpt-4o-mini-realtime-preview-2024-12-17 gpt-4o-mini-audio-preview-2024-12-17 o1-2024-12-17 o1 gpt-4o-mini-realtime-preview gpt-4o-mini-audio-preview o3-mini o3-mini-2025-01-31 gpt-4o-2024-11-20 gpt-4o-mini-search-preview-2025-03-11 gpt-4o-mini-search-preview gpt-4o-transcribe gpt-4o-mini-transcribe o1-pro-2025-03-19 o1-pro gpt-4o-mini-tts o3-2025-04-16 o4-mini-2025-04-16 o3 o4-mini gpt-4.1-2025-04-14 gpt-4.1 gpt-4.1-mini-2025-04-14 gpt-4.1-mini gpt-4.1-nano-2025-04-14 gpt-4.1-nano gpt-image-1 gpt-4o-realtime-preview-2025-06-03 gpt-4o-audio-preview-2025-06-03 o4-mini-deep-research gpt-4o-transcribe-diarize o4-mini-deep-research-2025-06-26 gpt-5-chat-latest gpt-5-2025-08-07 gpt-5 gpt-5-mini-2025-08-07 gpt-5-mini gpt-5-nano-2025-08-07 gpt-5-nano gpt-audio-2025-08-28 gpt-realtime gpt-realtime-2025-08-28 gpt-audio gpt-5-codex gpt-image-1-mini gpt-5-pro-2025-10-06 gpt-5-pro gpt-audio-mini gpt-audio-mini-2025-10-06 gpt-5-search-api gpt-realtime-mini gpt-realtime-mini-2025-10-06 sora-2 sora-2-pro gpt-5-search-api-2025-10-14 gpt-5.1-chat-latest gpt-5.1-2025-11-13 gpt-5.1 gpt-5.1-codex gpt-5.1-codex-mini gpt-5.1-codex-max gpt-image-1.5 gpt-5.2-2025-12-11 gpt-5.2 gpt-5.2-pro-2025-12-11 gpt-5.2-pro gpt-5.2-chat-latest gpt-4o-mini-transcribe-2025-12-15 gpt-4o-mini-transcribe-2025-03-20 gpt-4o-mini-tts-2025-03-20 gpt-4o-mini-tts-2025-12-15 gpt-realtime-mini-2025-12-15 gpt-audio-mini-2025-12-15 chatgpt-image-latest gpt-5.2-codex gpt-5.3-codex gpt-realtime-1.5 gpt-audio-1.5 gpt-4o-search-preview gpt-4o-search-preview-2025-03-11 gpt-5.3-chat-latest gpt-5.4-2026-03-05 gpt-5.4-pro gpt-5.4-pro-2026-03-05 gpt-3.5-turbo-16k tts-1 whisper-1 text-embedding-ada-002]

=== Get Anthropic Models ===
Anthropic models: [claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929 claude-opus-4-1-20250805 claude-opus-4-20250514 claude-sonnet-4-20250514 claude-3-haiku-20240307]

✓ Chat Models API example complete
✓ Client created

=== Create User Function ===
Created user function with ID: bHGs7h3t0LFb7VzGCO2dLuZOBA2liYthI1Uktga13tyO6lVFPRo6OQK_F44COb7BH2zmDJO5moMIhkOqE_vzGw

=== Get User Function ===
Retrieved: get_active_users - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 19 user functions:
  - fetch_user: Fetch user by code
  - get_verified_user: Get verified and validated user
  - fetch_slim_user: Validate and slim down user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - swr_user: SWR pattern for user data (KV-based)
  - swr_user: SWR pattern for user data (KV-based)
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - cache_api_call_py: Cache External API Call
  - validate_user: Check if user exists
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_user: Fetch user by code
  - fetch_user: Fetch user by code
  - validate_user: Check if user exists
  - get_active_users: Get Active Users
  - fetch_slim_user: Validate and slim down user
  - fetch_slim_user: Validate and slim down user

=== List User Functions by Tag ===
Found 19 user functions with 'users' tag:
  - fetch_user
  - get_verified_user
  - fetch_slim_user
  - fetch_and_store_user
  - swr_user
  - swr_user
  - get_user_wrapper
  - fetch_and_store_user
  - validate_user
  - fetch_and_store_user
  - cache_api_call_py
  - validate_user
  - swr_user
  - fetch_user
  - fetch_user
  - validate_user
  - get_active_users
  - fetch_slim_user
  - fetch_slim_user

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test' exists: false

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test' exists: true

=== Count Documents ===
Document count in 'collection_utils_test': 5

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test'

✓ Collection Utilities example complete
✅ [32mGo client examples complete![0m
✅ [32mAll Go integration tests complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.15.2 prepare
> npm run build


> @ekodb/ekodb-client@0.15.2 build
> tsc


up to date, audited 52 packages in 2s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.15.2 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 15 packages in 1s

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '4QU7TCx44hovZHDtPMXmUwYY8M21i_rl4xyn4wtoyb3Sw8NNQgHeT3kZoNzjUDnJ0WpyieTibCwILMmba__0CQ'
}

=== Find by ID ===
Found: {
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  value: { value: 42, type: 'Integer' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  name: { type: 'String', value: 'Test Record' },
  active: { value: true, type: 'Boolean' },
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  id: '4QU7TCx44hovZHDtPMXmUwYY8M21i_rl4xyn4wtoyb3Sw8NNQgHeT3kZoNzjUDnJ0WpyieTibCwILMmba__0CQ',
  price: { value: 99.99, type: 'Float' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  created_at: { type: 'DateTime', value: '2026-04-06T03:52:46.178Z' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-04-06T03:52:46.178Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  data: 'aGVsbG8gd29ybGQ=',
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  value: 42,
  categories: [ 'electronics', 'computers' ],
  name: 'Test Record',
  active: true,
  metadata: { key: 'value', nested: { deep: true } },
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  id: '4QU7TCx44hovZHDtPMXmUwYY8M21i_rl4xyn4wtoyb3Sw8NNQgHeT3kZoNzjUDnJ0WpyieTibCwILMmba__0CQ',
  price: 99.99,
  tags: [ 'tag1', 'tag2', 'tag3' ],
  created_at: '2026-04-06T03:52:46.178Z'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  active: { type: 'Boolean', value: true },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  price: { type: 'Float', value: 99.99 },
  value: { value: 100, type: 'Integer' },
  id: '4QU7TCx44hovZHDtPMXmUwYY8M21i_rl4xyn4wtoyb3Sw8NNQgHeT3kZoNzjUDnJ0WpyieTibCwILMmba__0CQ',
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  created_at: { type: 'DateTime', value: '2026-04-06T03:52:46.178Z' },
  name: { value: 'Updated Record', type: 'String' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 4tBqGs3Q9P0WnmIMFO3zbXDAUbvS1R-crIJHhpQzfIoPh0YRV4O7VxgUQfollWv9toJPNP45HAvq_9y3k28hBQ

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
Collection created with first record: rMhromTnb8yJuC_45LIvqVpEDt94EbLPyD4gQGjI-SXnXbGzhxnqY5uuMhnyyLgmwiUV3rzV8qaEdr71BP-1Ew

=== List Collections ===
Total collections: 47
Sample collections: audit__ek0_testing,cached_database_metrics,schema_products_client_go,enriched_users,schema_employees_client_py

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
  cache:product:1: { price: 29.99, name: 'Product 1' }
  cache:product:2: { price: 39.99, name: 'Product 2' }
  cache:product:3: { name: 'Product 3', price: 49.99 }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 16

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
Created Alice: $1000 - ID: YbxBc26KgKObajyFaKpJ4lhse-iz0ymr6VR0AaT2FeYubDvkd0VvYcTlVSa0rCeRM9tZji-jFe11HlafEb-jxA
Created Bob: $500 - ID: 8j7NvhHUs5x_2e3j5GbP1NpDlz6AJyRlCEvZ8KzFB3whcNkSz5kJXHdkT75RgUfEXH-NGmKcZUtQhW0Wcd676w

=== Example 1: Begin Transaction ===
Transaction ID: 285681d6-a30b-4970-b2d0-a14ba58b8069

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: f5d23281-9d21-4388-a759-24ab5db854b9
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
  1. Score: 25.740, Matched: email.value, email, name, name.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title.value, title, bio
  2. Score: 26.400, Matched: title, title.value, bio.value, bio
  3. Score: 26.400, Matched: title, bio, title.value, bio.value
  4. Score: 26.400, Matched: title, title.value, bio.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, title.value, bio, bio.value
  2. Score: 39.600, Matched: bio.value, title.value, bio, title
  3. Score: 39.600, Matched: title, title.value, bio, bio.value
  4. Score: 39.600, Matched: bio.value, bio, title, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.753, Matched: 
  2. Score: 0.749, Matched: 
  3. Score: 0.746, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.699, Matched: title, content, content.value, title.value
  2. Score: 1.501, Matched: content, title, title.value, content.value
  3. Score: 0.300, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills.value, bio.value, skills, bio

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
✓ Inserted document: 2T734UIfqgGql83KkbPuppfYc1ISat12ZDJexQlERg0DHnVtvlT4G6GGDSvBEjrBNwyKhGrNNUT4xmx_C4Ttsw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: EA2EWBKEGwJ839BN_JvW9wmG797UOh4vil6ZTM4mBa4YyJ-AAF4jd-StTQyFPWqT1k2s_5MsK2RyH6VYqnXulA

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
✓ Inserted document with TTL: nhOR0zcjhnJgCqfLrV85onDP8IH8j3cqmWt0w8K7-p3ozFhMkQd7lkTNAH1zFS27O35Y4ruk8Re-AZWdwdfS-w

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
✓ Edge cache script created: dRrjLP_W22J7hwvwcxjX3JR_1C7o52Mg216hOdtK9RzFh93zISsi9aRlRCWHMj8UXHb6hZdlS-K5YE1s7CKx0Q

Call 1: Cache miss (fetches from API)
Response time: 149ms
Result: {
  "records": [
    {
      "value": {
        "address": {
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "city": "Gwenborough"
        },
        "website": "hildegard.org",
        "email": "Sincere@april.biz",
        "company": {
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "name": "Leanne Graham",
        "username": "Bret",
        "id": 1,
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
Response time: 42ms (3.5x faster!)
Result: {
  "records": [
    {
      "value": {
        "address": {
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "city": "Gwenborough"
        },
        "website": "hildegard.org",
        "email": "Sincere@april.biz",
        "company": {
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "name": "Leanne Graham",
        "username": "Bret",
        "id": 1,
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

🚀 ekoDB Functions Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: dTbiz0gYeHSDVZbBP96Fa11TXD3t29j53ObozT_VBvbLR3LFxwHUkMdSiUW2rW_Itq1rUAaNbx9VAR4MKLozNA
📊 Found 15 active users

📝 Example 2: Parameterized Function

✅ Function saved: EmQgv4LwyFTIdCFx-qkwa_X4I7sBpm6EYRy7sjNNtRL1cWq7m66qlxvm9r9Rd2mZIZIAF-vVNyR_ATPhqkt6oQ
📊 Found 15 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: LbTDpjPzvP3-lhuYrt5s2V5xB_dGiKfogSl91VqETWRB-05AoDT-O0qB0GEFXFcSTXr0T8orA8NBg26G-928Bw
📊 Statistics: 3 groups
   {"status":"active","count":5,"avg_score":60}
   {"status":"null","count":5,"avg_score":60}
   {"avg_score":50,"status":"inactive","count":5}

📝 Example 4: UserFunction Management

📋 Total scripts: 22
🔍 Retrieved script: Get Active Users
✏️  function updated
🗑️  function deleted

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
   ⏱️  Duration: 41ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "website": "hildegard.org",
    "phone": "1-770-736-8031 x56442",
    "email": "Sincere@april.biz",
    "id": 1,
    "name": "Leanne Graham",
    "address": {
      "geo": {
       ...

Second call (cache hit - from cache):
   ⏱️  Duration: 42ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "website": "hildegard.org",
    "phone": "1-770-736-8031 x56442",
    "email": "Sincere@april.biz",
    "id": 1,
    "name": "Leanne Graham",
    "address": {
      "geo": {
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
🚀 ekoDB TypeScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: 8PuhnZZwqaV9E5dQYBQRaNDUDWuBz3GG4FAg6lbd_qUUlB0RRtPEKttVYD4rIarSf2L1RWfZm2k8SrJoSzb0hA
📊 Found 2 product groups
   {"category":"Electronics","count":3,"avg_price":575.6666666666666}
   {"category":"Furniture","count":2,"avg_price":474}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":2,"category":"Furniture"}
   {"count":3,"category":"Electronics"}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: UserFunction with Parameter Definition

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
🚀 ekoDB TypeScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: 6EzTRMvpSMKDmPwWAvSyaS7RMMqyeSPO5kz8-zraAhv8UImMs9piBqee0vnPCivbIFCdL8uWXfsJ1WFulf7O3w
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: wlaUPvAxZftBqEEb-slwcBQDQZCbQ5LHlIS1PvCv1HdQtgJCQz_dA3bV2pZI3DLS4b5FYMTmcUIGB_b3LhQ2vw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: 2HYdK_PpA1YdZBKsC0gN1lubTx0VsqBUod3HYFe4u2B9OfmAp2BNbAl0j7bJez8RA-oN4DFi3TfjPMa0XbJOtQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: nRCDRQyow8s8V-qhzi24mQ-yiHd4cvaKeI0kpUtwMnrlk2wPw2dF_SIfmZQ92uqg6l-EcC861x4GUHQRG-8J_Q
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
✓ Created SWR script: fetch_api_user (vWY53V5nfh8IqKgDXN52H5rPDC2VIgA8y2EraopflYyMMNNMjkvHDCEu1rK3zOHIIACk6K1CCMLYflHOt7gn4g)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "name": "Leanne Graham",
        "website": "hildegard.org",
        "username": "Bret",
        "address": {
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "city": "Gwenborough"
        },
        "id": 1,
        "phone": "1-770-736-8031 x56442",
        "email": "Sincere@april.biz",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 43ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "name": "Leanne Graham",
        "website": "hildegard.org",
        "username": "Bret",
        "address": {
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "city": "Gwenborough"
        },
        "id": 1,
        "phone": "1-770-736-8031 x56442",
        "email": "Sincere@april.biz",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (knK0G8Qj3D9wxM2fHBeBQsFSknTTcf4HqjumQMYsQeqF07NCWge8R3mPlU3T3sCxdiJ3VawjsDc_1P7a4FMA2w)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "shippingInformation": "Ships in 3-5 business days",
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "tags": [
          "beauty",
          "mascara"
        ],
        "weight": 4,
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "title": "Essence Mascara Lash Princess",
        "stock": 99,
        "sku": "BEA-ESS-ESS-001",
        "discountPercentage": 10.48,
        "rating": 2.56,
        "id": 1,
        "brand": "Essence",
        "availabilityStatus": "In Stock",
        "reviews": [
          {
            "reviewerName": "Eleanor Collins",
            "rating": 3,
            "comment": "Would not recommend!",
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com"
          },
          {
            "date": "2025-04-30T09:41:02.053Z",
            "comment": "Very satisfied!",
            "rating": 4,
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "reviewerName": "Lucas Gordon"
          },
          {
            "comment": "Highly impressed!",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins",
            "rating": 5,
            "date": "2025-04-30T09:41:02.053Z"
          }
        ],
        "returnPolicy": "No return policy",
        "meta": {
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
          "createdAt": "2025-04-30T09:41:02.053Z",
          "updatedAt": "2025-04-30T09:41:02.053Z",
          "barcode": "5784719087687"
        },
        "minimumOrderQuantity": 48,
        "dimensions": {
          "height": 13.08,
          "width": 15.14,
          "depth": 22.99
        },
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "price": 9.99,
        "category": "beauty",
        "warrantyInformation": "1 week warranty"
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

🚀 ekoDB TypeScript Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native (aHKJzT6T8qWK8a8g3Lcbo_G02RudXoOCoaAh1Rt36gnX6s1oF3OirWFoM4nQNFAnyv7l2A4qXoWlY5WOP_fIRQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 17ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 18ms
  Speedup: 0.9x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (j5I8GhIOV4NoTt4AoQANXTFeAraT4BBmMQZwajFva1Ypk07JmDjLB4sIDd1u5rThxbQghJEba86T8kKXcN8d7w)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (qXbso1pauPbjr4Gq27joLo-wsvA0S3KNc8lGhlrCt4EjiEsPshxqk8h7EHmG3BPFwQXYiM_8fLb23DXRodNAWA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (OEj1xqf6BE-MVmcWDm-QkEbRQFej7zYOKKoTxVN9nspDi2RhY4zrOuK0YXsu1XdIXO9qDnNovJ1obYJsBWNL3Q)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

================================================================================
✅ Key Benefits of Native SWR:
✅ Single function: Replaces 4-function cache-aside pattern
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)
Native SWR:     SWR → Insert (2 functions)
Result:         60% fewer functions, cleaner code, same behavior 🎯

🧹 Cleaning up...
✓ Deleted 4 test scripts

✅ All examples completed!
🚀 ekoDB TypeScript Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   {"avg_price":367,"count":5,"category":"Electronics"}
   {"count":3,"avg_price":365.6666666666667,"category":"Furniture"}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB TypeScript AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits, including:

1. **Efficient Similarity Search**: They excel in querying high-dimensional data, allowing for fast nearest neighbor searches.

2. **Handling Unstructured Data**: Vector databases can store unstructured data like images, audio, and text in a format suitable for machine learning.

3. **Scalability**: They can efficiently manage large datasets and scale as data grows.

4. **Real-Time Querying**: Support for real-time analytics and updates, making them suitable for applications requiring immediate results.

5. **Support for Machine Learning Models**: They facilitate the integration of embeddings from machine learning models for better data representation.

6. **Enhanced Flexibility**: Ability to combine different data types and formats, enabling complex and multifaceted queries.

7. **Improved Performance**: Optimized for vector operations, providing faster performance than traditional databases for specific tasks like recommendation systems.

8. **Advanced Analytics**: Facilitate deep learning applications and other advanced analytical techniques.

These advantages make vector databases particularly useful in AI, machine learning, and data-driven applications.
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
🚀 ekoDB TypeScript CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   active: 7 users
   inactive: 3 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average score by role:
   {"count":7,"role":"user","avg_score":70}
   {"avg_score":20,"count":3,"role":"admin"}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB TypeScript Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Natural Language Processing (AI)
   2. Vector Databases Explained (Database)
   3. Introduction to Machine Learning (AI)
   4. Database Design Principles (Database)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":"Database","count":3}
   {"count":2,"category":"AI"}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: NywLVYKnaHEA4WfOYsmM9dY1oavQ8hnEq8Dodp6wkPp3PUoHYtYQTghQjokz331UmUU3qkT2VrZ7j6Wtw6o1qg

=== Sending Chat Message ===
Message ID: Bbyu3xlsyt0VU1Wg-dtfsuusdG0oHBawP8p8K1UpNfSrtqtZndYjtuPLUXHAwfjbIsm0eUVak3f64MGu4y-6mw

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB Pro**
   - Description: Enterprise edition product with advanced features
   - Price: $299

2. **ekoDB Cloud**
   - Description: Fully managed cloud database service product
   - Price: $499

3. **ekoDB**
   - Description: A high-performance database product with AI capabilities
   - Price: $99

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'r0arQC9gqPhgLpp-XNfI3-YH8IY9jl6IHhn45WdOJ3Yz8ULeG7y2W4gDlrlVDB0LQNg4eoTC6-u8XyddAQ1teg',
    description: 'Enterprise edition product with advanced features',
    price: 299,
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'mqpGv-mT2EN-mucQD1rJbs0PfjEQGTI2NswnWXjVITq1c289DsAvbbUikTdCT77Ea9X2Cprq0tmdb_yYzIgHig',
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'jowBaOHxPzdCPWg4mQNTO078MkMIK-KHQ6beYpeDIy_FpX0CKTKKkVdYXaSNuK4ZXOyCcJuSTFWJEhuudYPHOA',
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2116ms

=== Token Usage ===
Prompt tokens: 1298
Completion tokens: 91
Total tokens: 1389

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: zma8OuCYPgYiDltVwdyLwErUr96wKzJZuD9Z_dbjXGX7EqEdXk-kaNT-B5CRlAEKhVGlOZ-VozoD4SU2CU4Fyw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or details about other products, just let me know!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any more questions or need further information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: p8s1qPSKPwGUe4GBfg6clOWiJdpIODgQAG5XkF0F0iSmDTfY_Omh2ocmr_ZcLYDggYpFsqMAy6LmFs5Qi-Zv3w
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
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: FR9F_qSfuVyQ4TdxyeyUH62vek2rS32PNm4TEei_tYVSi2403Te85WeB2e8oMlCh0cZQe4SqPTeUlpiKvzJTvg

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more information or additional products, just let me know!

✓ Message 2 sent
  Response: The price of **ekoDB** is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: NJjql1Ff67C9ThiAtEMzrVEHfou-YaKjYZnEk7WhecrX3JvsDS-rHTOPYsnBClMjkkrvylHFrBrLZ4ZxLneivg
  Parent: FR9F_qSfuVyQ4TdxyeyUH62vek2rS32PNm4TEei_tYVSi2403Te85WeB2e8oMlCh0cZQe4SqPTeUlpiKvzJTvg

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: NJjql1Ff67C9ThiAtEMzrVEHfou-YaKjYZnEk7WhecrX3JvsDS-rHTOPYsnBClMjkkrvylHFrBrLZ4ZxLneivg (Untitled)
  Session 2: FR9F_qSfuVyQ4TdxyeyUH62vek2rS32PNm4TEei_tYVSi2403Te85WeB2e8oMlCh0cZQe4SqPTeUlpiKvzJTvg (Untitled)
  Session 3: p8s1qPSKPwGUe4GBfg6clOWiJdpIODgQAG5XkF0F0iSmDTfY_Omh2ocmr_ZcLYDggYpFsqMAy6LmFs5Qi-Zv3w (Untitled)
  Session 4: NywLVYKnaHEA4WfOYsmM9dY1oavQ8hnEq8Dodp6wkPp3PUoHYtYQTghQjokz331UmUU3qkT2VrZ7j6Wtw6o1qg (Untitled)
  Session 5: NDeOBVKsMCqOpcQHsNmV96Nji7V4MJEHYbKVcf_YfpvNMO5mcJIO19Nb_3ljx4BrrIZG572g5PizSy-vllMSSA (Untitled)
  Session 6: _DgimDs70IFjtBMQiEuIgkb4BNyDwHu4ZzjUhOnlzAxAeCdSKVpiyLbaB8_hA-OsMxz_HPNaHeYBLDnawpQd2w (Untitled)
  Session 7: j35FSFi7T5MNfApTG_tS_0nktaY5bNDTaUm9_Oh1NZq0crz26C65MZINFB_tBRD96KcT1EvDsvfv9PqQglIKyg (Untitled)
  Session 8: Lgj7ao7hej-PLDxXRWbB-gJVVaO_BmS1m4YQMX35qgSJrEXyZNFjQFzsDIw6bWYPKPhfAxANp1-b36KjoJu-yQ (Untitled)
  Session 9: YP5qswPHkbIDIVGAfdU0RmffBHKwrpZl6ZFKCk3xKPLWYOAHHg_fQI5ac-GqelJrYv1Wo87jFj0o7a-KiObgVg (Untitled)
  Session 10: DxcKnNVdBVFO5renQijv8LByj3cDksxCjDxl7-r5_dvAptNWkD2LgUn-bgrVymKRhkRW2wL_vJ1x5VFVI6VPHw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: NJjql1Ff67C9ThiAtEMzrVEHfou-YaKjYZnEk7WhecrX3JvsDS-rHTOPYsnBClMjkkrvylHFrBrLZ4ZxLneivg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'pgWuRKsch94VuJSn02EZNHGwNGSFSF8E5aSqLCYu0e29X_BtdlDhb5cd3fIBMf_pcEaCpFuMsMpjV-T-RJ62Wg'
}

=== Upsert Operation ===
✓ First upsert (update): {
  age: { type: 'Integer', value: 29 },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  active: { type: 'Boolean', value: true },
  id: 'pgWuRKsch94VuJSn02EZNHGwNGSFSF8E5aSqLCYu0e29X_BtdlDhb5cd3fIBMf_pcEaCpFuMsMpjV-T-RJ62Wg',
  name: { value: 'Alice Johnson', type: 'String' }
}
✓ Second upsert (insert): {
  id: 'Jfra5ReiSc6gxLmfIT_v-oMf0Rkjsz4Vpt6Gx8JjPdlBU-WrqySi0puk3RuoFz2J3dpUcgb0CU9X1G4mVy2iqA'
}

=== Find One Operation ===
✓ Found user by email: {
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  age: { type: 'Integer', value: 29 },
  active: { type: 'Boolean', value: true },
  name: { value: 'Alice Johnson', type: 'String' },
  id: 'pgWuRKsch94VuJSn02EZNHGwNGSFSF8E5aSqLCYu0e29X_BtdlDhb5cd3fIBMf_pcEaCpFuMsMpjV-T-RJ62Wg'
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
   Inserted with ripple: {"id":"I4_OIe-7hh8nRnC7z8D_Am5pAiENpvFJmkSM9YjJD1C4vdCRpEZdmTefs-nbRds1VgyB8XrFqxpkPI-qeBVu_A"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"0lAK0_dnb9laQ8bFBhb7oqufQHNVjxMt3HczSuNiGzUWw8Rx-FB6i0YQQjzatxKqtUlrIaC1hS84_bcNTAsWxw"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"price":{"value":150,"type":"Integer"},"name":{"value":"Product 1","type":"String"},"id":"I4_OIe-7hh8nRnC7z8D_Am5pAiENpvFJmkSM9YjJD1C4vdCRpEZdmTefs-nbRds1VgyB8XrFqxpkPI-qeBVu_A"}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"custom-id","name":{"value":"Upsert Product","type":"String"},"price":{"type":"Integer","value":500}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["name","email","id"]
  First user: [object Object] <[object Object]>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["bio","id","email","created_at","name","user_role","age","status","avatar_url"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - [object Object] (age [object Object])
    - [object Object] (age [object Object])
    - [object Object] (age [object Object])

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - [object Object]: [object Object]

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["email","password","bio","status","api_key","user_role","secret_token","created_at","id","name","age","avatar_url"]
  Projected query:
    - 3 fields per record
    - Fields: ["name","id","email"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
=== KV Precision: Float vs Decimal ===

=== Test 1: Using JavaScript Floats (LOSES PRECISION) ===
✓ Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 2: Using Field.decimal() (PRESERVES PRECISION) ===
✓ Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.30000000000000004 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Cleanup ===
✓ Cleaned up test keys

=== Summary ===
✅ Use Field.decimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ Field.decimal() stores values as strings internally,
   preserving exact precision across all operations.
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
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
  anthropic:
    - claude-sonnet-4-6
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
    - claude-opus-4-1-20250805
    - claude-opus-4-20250514
    - claude-sonnet-4-20250514
    - claude-3-haiku-20240307
  perplexity:
    - sonar
    - sonar-pro
    - sonar-deep-research
    - sonar-reasoning
    - sonar-reasoning-pro

=== Get Specific Provider Models ===
OpenAI models: gpt-4-0613, gpt-4, gpt-3.5-turbo, gpt-5.4-mini, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, dall-e-3, dall-e-2, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, gpt-4o-audio-preview, gpt-4o-realtime-preview, omni-moderation-latest, omni-moderation-2024-09-26, gpt-4o-realtime-preview-2024-12-17, gpt-4o-audio-preview-2024-12-17, gpt-4o-mini-realtime-preview-2024-12-17, gpt-4o-mini-audio-preview-2024-12-17, o1-2024-12-17, o1, gpt-4o-mini-realtime-preview, gpt-4o-mini-audio-preview, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, gpt-4o-realtime-preview-2025-06-03, gpt-4o-audio-preview-2025-06-03, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, gpt-realtime-mini-2025-10-06, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-3.5-turbo-16k, tts-1, whisper-1, text-embedding-ada-002

=== Get Anthropic Models ===
Anthropic models: claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929, claude-opus-4-1-20250805, claude-opus-4-20250514, claude-sonnet-4-20250514, claude-3-haiku-20240307

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 405: {"code":405,"message":"Method Not Allowed"}

✓ Chat Models example complete
✓ Client created

=== Create User Function ===
Created user function with ID: JuUMUj1owtQUWe_2rFWZLZm1L6CPZbeHnyQQp838UDL3ffY0djPLEsbzfOrwtEaJHzstUmQmmBF70GLCT-dOOw

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 31 user functions:
  - fetch_user: Fetch user by code
  - get_verified_user: Get verified and validated user
  - fetch_slim_user: Validate and slim down user
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_users_by_status: Get Users By Status
  - fetch_slim_user: Validate and slim down user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_api_user: Fetch User with Cache
  - swr_user: SWR pattern for user data (KV-based)
  - swr_user: SWR pattern for user data (KV-based)
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - cache_api_call_py: Cache External API Call
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - validate_user: Check if user exists
  - validate_user: Check if user exists
  - get_verified_user: Get verified and validated user
  - get_user_wrapper: Wrapper that calls fetch_user
  - cache_api_call: Cache External API Call
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_user: Fetch user by code
  - fetch_user: Fetch user by code
  - validate_user: Check if user exists
  - get_active_users_ts: Get Active Users
  - fetch_slim_user: Validate and slim down user
  - fetch_slim_user: Validate and slim down user

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test_ts' exists: false

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test_ts' exists: true

=== Count Documents ===
Document count in 'collection_utils_test_ts': 5

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_ts'

✓ Collection Utilities example complete
✅ [32mTypeScript client examples complete![0m
✅ [32mAll TypeScript integration tests complete![0m
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 1s

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
  id: 'zH_6umpXI5ZjuLMPEts7dKa7Hpx2aCaU7eV4SAoQKv48Tu7tA8qUS9MdBp1pnA5S7uRfq_HJJ2xpmreMFpEekA'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  active: { value: true, type: 'Boolean' },
  id: 'zH_6umpXI5ZjuLMPEts7dKa7Hpx2aCaU7eV4SAoQKv48Tu7tA8qUS9MdBp1pnA5S7uRfq_HJJ2xpmreMFpEekA',
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: [
  {
    active: { type: 'Boolean', value: true },
    name: { type: 'String', value: 'Test Record' },
    value: { value: 42, type: 'Integer' },
    id: 'zH_6umpXI5ZjuLMPEts7dKa7Hpx2aCaU7eV4SAoQKv48Tu7tA8qUS9MdBp1pnA5S7uRfq_HJJ2xpmreMFpEekA'
  }
]

=== Update Document ===
Updated: {
  active: { value: true, type: 'Boolean' },
  id: 'zH_6umpXI5ZjuLMPEts7dKa7Hpx2aCaU7eV4SAoQKv48Tu7tA8qUS9MdBp1pnA5S7uRfq_HJJ2xpmreMFpEekA',
  name: { type: 'String', value: 'Updated Record' },
  value: { type: 'Integer', value: 100 }
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
✓ Inserted test record: 2DOD8LoFM1YcP6TpYzByDNQGvsYlgWHo0ilW5RnPTJqXGI0dcv82AWI7r7_3BRdZkx7OAc_PIiAEtjWFUAZoKw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1775447629564",
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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "bHP1ni5oaE0HgsNvnFzZnpbpJK6MKlPwmNaxhEk4ZOLvCXyckqpXu5c6bKDkDawxQ6Fp8hrQ-iqhLJcf8dZ1lA",
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
        "id": "2DOD8LoFM1YcP6TpYzByDNQGvsYlgWHo0ilW5RnPTJqXGI0dcv82AWI7r7_3BRdZkx7OAc_PIiAEtjWFUAZoKw",
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
        "id": "YqPd6d25_D4tl6fKjbje3AExz8arcSNbcmeUwMRvFm_j2tAzd2q0DMFW65DwWrroSYcvnXyFUI5AqnR8pXWDOg",
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
🚀 ekoDB Functions Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: 7NuK91hGg7vyIlzUSurvMrl2SCBHRdynTVjfZlBdx-p4i2RTzxocoo8NAOVw_h5iZRqBr1C8BzdPqnsaCawbBg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: 5jdEmB_OU0Nv0vnMFlVTKKOt-8GlDzRB6S6sMUmIJlgx-kVqzaFW5DeqavjStLSg8agbR2anlzY094vaQhmmbw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: wajVvZXlzN7jFlVSxibiv1PX0QzZyEcrBK_KVk0t0msTa0Xj2pVomnWDUnO5mThAXQEUdZzpzmwZXmVqUffIXw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":50,"count":10,"max_score":90,"status":"inactive"}
   {"status":"active","max_score":100,"count":10,"avg_score":60}

📝 Example 4: Function Management

📋 Total functions: 33
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

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
Retrieved value: { username: 'john_doe', userId: 123 }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
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
Collection created with first record: iwUoV0UolZeJKnzbR-hRzcQjtOKiPqRWawwX3-pBdIoHfCtjbgHJDOLxaeeZ3BfSzQA-smMNAAyMxKP2p2We2g

=== List Collections ===
Total collections: 50
Sample collections: [
  'audit__ek0_testing',
  'cached_database_metrics',
  'schema_products_client_go',
  'enriched_users',
  'schema_employees_client_py'
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
Created Alice: $1000 - ID: fZ2IaVvUGtqfi8JnrpNwaYFAqvOITCd6mXPmVy1d1gCLc5DM8aj5GFlhN95ZRNbsLqJmN8iW4pOfrWIgS56KLg
Created Bob: $500 - ID: jggeJ00B9M7gNt1ZZpHrpqHyXQz6pkM4s0YLzzjlYvZuqWnPwbjGZQjILAOfBsBA9XEnQH1AfspVlfV3XyebtA

=== Example 1: Begin Transaction ===
Transaction ID: 3fa76e62-45c3-437d-9176-7177de68df1c

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: {"type":"Integer","value":800}
Bob: {"type":"Integer","value":700}

=== Example 5: Rollback ===
New transaction: ef34157e-0ac1-44e3-8dc5-fb2784f8aa38
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"value":700,"type":"Integer"}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
[32m✓ transactions.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/crud_functions.js ===[0m
🚀 ekoDB Complete CRUD Functions Example
============================================================
Demonstrates:
  • Insert + Verify (using Query)
  • Query + Update Status + Verify
  • Query + Update Credits + Verify
  • Query Before Delete + Delete + Verify Gone

Each function shows Functions chaining with proper verification
============================================================

============================================================
📝 Function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: GryGx93TzJwgKvmb31_nUOwOFUe7rViK3VelyPfXYgPcbNVE52o_rjtifARqFydfm7HXGWQwOVVf09CMvYnGwA

2️⃣ Calling Function (Insert + Verify)...
   ✅ Function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Name: [object Object]
   📋 Email: {"value":"alice@example.com","type":"String"}
   📋 Status: [object Object]
   📋 Credits: [object Object]

============================================================
📝 Function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: k5bk6N0noOhaPIito0y6LZYfY3MEfFKHmNwJYf_jnjbL99eRnHwXeHt9kiKqnaNRSswnRr0m6GwF3uYqPmhLNg

2️⃣ Calling Function (Query + Update + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"value":"active","type":"String"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: l2SOKpPtNCqDwmfwQF_ilZTh3LNJTIMm7jnFjgqUN6S0Hi2oPRssN7ZIpddCfB51lC-LcwkXg-MZYZHPB_aivA

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":0}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: thUJFOSq0Xq8kDuoppJTxQ3TqZ4hwWfvJmNxbg4zL24e-n7WF1KHMH_kAQci3lNHcbl4WLplT08D48kOpx7g6Q

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: GryGx93TzJwgKvmb31_n...
   ✅ Deleted function: k5bk6N0noOhaPIito0y6...
   ✅ Deleted function: l2SOKpPtNCqDwmfwQF_i...
   ✅ Deleted function: thUJFOSq0Xq8kDuoppJT...
   ✅ Deleted collection: users

============================================================
✅ Complete CRUD Functions Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Functions chain Functions together
   ✅ Each function demonstrates operation + verification
   ✅ Parameters make functions reusable
   ✅ Verification is built into the function itself
   ✅ Complete CRUD lifecycle in 4 focused functions
[32m✓ crud_functions.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/document_ttl.js ===[0m
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
  Output: Document ID = Y5IQRTg5ObEemXfcQ15jYxrXi4q7iLzerSozpv7LZbPGIB5dl8zl9kKmP0xjfcZ9_h0qhQPTUCcXCazcBokHDw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(Y5IQRTg5ObEemXfcQ15jYxrXi4q7iLzerSozpv7LZbPGIB5dl8zl9kKmP0xjfcZ9_h0qhQPTUCcXCazcBokHDw)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(Y5IQRTg5ObEemXfcQ15jYxrXi4q7iLzerSozpv7LZbPGIB5dl8zl9kKmP0xjfcZ9_h0qhQPTUCcXCazcBokHDw)
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/websocket_ttl.js ===[0m
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
  Output: Document ID = YXTBwBg1SU4dwVT0fl-3u-0N2GVlzPROFcdhVgKiqLkArraVPBt-kwkJlWvQitPMUIwm5Futit5EeMU2mQssgQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(YXTBwBg1SU4dwVT0fl-3u-0N2GVlzPROFcdhVgKiqLkArraVPBt-kwkJlWvQitPMUIwm5Futit5EeMU2mQssgQ)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(YXTBwBg1SU4dwVT0fl-3u-0N2GVlzPROFcdhVgKiqLkArraVPBt-kwkJlWvQitPMUIwm5Futit5EeMU2mQssgQ)
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

> @ekodb/ekodb-client@0.15.2 prepare
> npm run build


> @ekodb/ekodb-client@0.15.2 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.15.2 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 1s

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'XoOOEPNYhpYs6sBODqAGXy4HAkZXT2ZFDo1Qrws54ega8FWVeJdAW2o3owgxhqE8G5WFr3pPjSOj0OnQ0dlNzg'
}

=== Find by ID ===
Found: {
  id: 'XoOOEPNYhpYs6sBODqAGXy4HAkZXT2ZFDo1Qrws54ega8FWVeJdAW2o3owgxhqE8G5WFr3pPjSOj0OnQ0dlNzg',
  active: { value: true, type: 'Boolean' },
  name: { value: 'Test Record', type: 'String' },
  value: { type: 'Integer', value: 42 }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  active: { type: 'Boolean', value: true },
  name: { value: 'Updated Record', type: 'String' },
  value: { type: 'Integer', value: 100 },
  id: 'XoOOEPNYhpYs6sBODqAGXy4HAkZXT2ZFDo1Qrws54ega8FWVeJdAW2o3owgxhqE8G5WFr3pPjSOj0OnQ0dlNzg'
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: Eoyjb5_9w5J7Z-STXWTWVff3P4_oqI7ZiBWpldWHomuLB9b2k6xmn__XEID68NV727b6kHJ2LGXI2rnxlIIqpA

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
Collection created with first record: QPJY4fvzk5L1pJ2vHWsS1WY8jB5r7GvufsLHFp2mydBogJMOHGi2OjLvFNB7ixCoDxqtduG_nzoBDO6fjqM0Mw

=== List Collections ===
Total collections: 49
Sample collections: audit__ek0_testing,cached_database_metrics,schema_products_client_go,enriched_users,schema_employees_client_py

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

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 19

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: LfDPZSQwQUNpnYRrhui2B6aeYkmkIoQI8zq9hmF77YA5KpXf_HxL2SNgNnWBY9jtbBzPOaqeRud1w7CtmSGBSA
Created Bob: $500 - ID: vSEB1uYY6uwziLMNvQNcsQ6xniFBTt16Blul3xjG6MeQr3XSB450YJfUgbUZp980r5uBodeEFq3l0DGp-cUAgA

=== Example 1: Begin Transaction ===
Transaction ID: eddda2b3-0ffa-4f89-a08b-f7d01aa93510

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: e73321b4-6af3-4aa2-ae2f-2edf0095928d
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
  1. Score: 26.400, Matched: bio, title.value, title, bio.value
  2. Score: 26.400, Matched: title, bio, title.value, bio.value
  3. Score: 26.400, Matched: bio.value, bio, title, title.value
  4. Score: 26.400, Matched: title.value, title, bio.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio.value, bio, title
  2. Score: 39.600, Matched: title, title.value, bio.value, bio
  3. Score: 39.600, Matched: title, bio, title.value, bio.value
  4. Score: 39.600, Matched: bio, bio.value, title, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.752, Matched: 
  2. Score: 0.746, Matched: 
  3. Score: 0.741, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.698, Matched: content.value, content, title, title.value
  2. Score: 1.496, Matched: title.value, content, content.value, title
  3. Score: 0.301, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio.value, skills.value, bio, skills

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
✓ Inserted document: RBKShIYF6oYbu2yjyZJ7j4prypz7K3dY4MGigxkbOajIxFg1c5X3Q1gqwyi0WTcJkTkYTU6tjlfSWmHCAb-FBg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: OZ2inoxC8R-RzKkv6YFcDefmOUqWBv8XKxqEElMwXHNG-JAK056CGSBsgVSVMGB_TGhklv07c-fwT0GUy1LDnQ

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
✓ Inserted document with TTL: y_-oJ8eUVPPbD3t3JPbOabW5TqSok6POT1W4mnWJwUCVONVwWFtrqJK53i5qWq7OefA2Cmu04ew0sVPmEt9XsA

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
✓ Edge cache script created: DCet2_HUUfNNQGyYuWQUjNgHbUuBYTTt9twwlmhvuES1MCkSAN_qRSWW8Exjn-WTbsb4QW9-AmE9crZWjaMgeQ

Call 1: Cache miss (fetches from API)
Response time: 44ms
Result: {
  "records": [
    {
      "value": {
        "current": {
          "temperature_2m": 7.8,
          "time": "2026-04-06T03:45",
          "interval": 900
        },
        "generationtime_ms": 0.01347064971923828,
        "elevation": 32,
        "longitude": -73.99308,
        "timezone": "GMT",
        "current_units": {
          "interval": "seconds",
          "time": "iso8601",
          "temperature_2m": "°C"
        },
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0,
        "latitude": 40.710335
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
Response time: 43ms (1x faster!)
Result: {
  "records": [
    {
      "value": {
        "current": {
          "temperature_2m": 7.8,
          "time": "2026-04-06T03:45",
          "interval": 900
        },
        "generationtime_ms": 0.01347064971923828,
        "elevation": 32,
        "longitude": -73.99308,
        "timezone": "GMT",
        "current_units": {
          "interval": "seconds",
          "time": "iso8601",
          "temperature_2m": "°C"
        },
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0,
        "latitude": 40.710335
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

🚀 ekoDB Functions Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: gMZnrXAnm1g12tl07w9BSj38-4DQcnyovOyuRw9vO5GMFPceIDKnDPPYZ_QjoHQwzGI7YfIth30wjwTekh7kTg
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"avg_score":60,"status":"active","count":5}
   {"count":5,"avg_score":50,"status":"inactive"}
⏱️  Execution time: 0ms

📝 Example 4: Function Management

📋 Total functions: 36
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage function saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count function saved
📊 Total user count: 10
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test functions

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
   Name: {"value":"User 1","type":"String"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 174ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "id": 1,
    "username": "Bret",
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romaguera-Crona",
      "bs": "harness real-time e-ma...

Second call (cache hit - from cache):
   ⏱️  Duration: 41ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "id": 1,
    "username": "Bret",
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romaguera-Crona",
      "bs": "harness real-time e-ma...
   🚀 Cache speedup: 4.2x faster!

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
🚀 ekoDB JavaScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: jOkz8tB_GRtjoYiuaNXYm5ghTFztzooF8iwMYnI23D2Wubxg2VUwLfbSSYAiNRMNd_-y-p8zpb_SJEr7IWMW-A
📊 Found 2 product groups
   {"category":"Furniture","count":2,"avg_price":474}
   {"category":"Electronics","count":3,"avg_price":575.6666666666666}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":"Furniture","count":2}
   {"category":"Electronics","count":3}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: Function with Parameter Definition

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete function examples finished!

💡 This example demonstrates ekoDB's Function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
🚀 ekoDB JavaScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: LsUdAv14JMQOcbMHVQCLZa_MdS72eaADJzQmASa6aNp-n08DcGI3EDlIFAmFqMgAoQ9UF7V6TMxfScrNUY3vbg
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: qjih_hs2Ygh_zgNRdvmfmGgmsC3haqLs1ZB38uMx0TWoPcujxWBdKu2_CLhiGw5tQu8P5iklWbT509wnhvSvkQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: g--WtMdIQHQz1g0QhLRIX-40nyxqYQBBmIOCVXxIJS4D57Xa5SJT9v2A3FBwWwK1uFeFaascuYvE1fcWTwQGpw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: i1-tddoaHYRtFMMTqg5OHA2DdbOEzy0dsn82GW4rvqCt8JhwKhBPYN3XuaZqTz82QU5h8VOK-GteUIElueWG4A
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
🚀 ekoDB CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average scores by role:
   admin: 20.0 (3 users)
   user: 70.0 (7 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Function saved
📊 Users (showing first 5 of 10):
   1. User 1 - Score: 10
   2. User 4 - Score: 40
   3. User 8 - Score: 80
   4. User 5 - Score: 50
   5. User 7 - Score: 70
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Function saved
📊 User summary (2 groups):
   inactive users: 3 users, Total Score: 180
   active users: 7 users, Total Score: 370
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD function examples completed!
🚀 ekoDB Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Getting Started with ekoDB (Database)
   2. Vector Databases Explained (Database)
   3. Natural Language Processing (AI)
   4. Database Design Principles (Database)
   5. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   AI: 2 documents
   Database: 3 documents
⏱️  Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Function saved
📊 Document titles (5 docs):
   1. Getting Started with ekoDB
   2. Vector Databases Explained
   3. Natural Language Processing
   4. Database Design Principles
   5. Introduction to Machine Learning
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Getting Started with ekoDB
   2. Vector Databases Explained
   3. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Function saved
📊 All documents (5 total, showing first 2):
   1. Getting Started with ekoDB (Database)
   2. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search function examples completed!
🚀 ekoDB Advanced Functions Example

📋 Setting up test data...
✅ Created 10 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 10 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Found 2 categories
   Electronics: 6 items (avg $325.67)
   Furniture: 4 items (avg $294.00)
⏱️  Execution time: 0ms

📝 Example 3: Count All Products

✅ Function saved
📊 Total products: 10
⏱️  Execution time: 0ms

📝 Example 4: Multi-Stage Aggregation

✅ Function saved
📊 Category analysis (2 categories):
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Function saved
📊 Product summaries (10 items, showing first 3):
   1. Monitor 27" - $399 (⭐4.6)
   2. Standing Desk - $599 (⭐4.7)
   3. Wireless Mouse - $29 (⭐4.5)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced function examples completed!
🚀 ekoDB AI Functions Example

⚠️  Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data...
✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat function saved
🤖 AI Response:
   1. Precision: Vector databases are able to store data with high precision, making them suitable for applications such as CAD and GIS where accuracy is crucial.

2. Scalability: Vector data can be easily scaled without losing quality. This means that you can zoom in and out without the image becoming pixelated or blurred.

3. Less Storage: Vector data is usually less memory-intensive than raster data. This is because it only needs to store the coordinates of points and the attributes of lines and polygons, rather than individual pixel values.

4. Topology: Most vector data models incorporate topological information. This means they accurately store spatial relationships between features such as connectivity, adjacency, and containment.

5. Attribute Data: Vector databases allow each entity to have associated attribute data, which can be useful for analysis and decision-making processes.

6. Quality of Data: Vector databases typically have a higher quality of data due to less redundancy and better structure compared to other databases.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding function saved
📊 Generated embeddings for 3 articles
   1. "Advanced Query Patterns" - 1536D vector
   2. "Getting Started with ekoDB" - 1536D vector
   3. "Draft Article" - 1536D vector
⏱️  Execution time: 0ms

📝 Example 3: List All Articles

✅ Function saved
📊 Found 3 articles
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_js (tZm7hh-cMg70LXI_-iwbt0fNKM7fEuJKZOobSevPt-ruBX-y-kWQuDC76q-1JM-cJ7_tjl-2exsXKExoQo0QHQ)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "name": "Leanne Graham",
        "website": "hildegard.org",
        "username": "Bret",
        "address": {
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "city": "Gwenborough"
        },
        "id": 1,
        "phone": "1-770-736-8031 x56442",
        "email": "Sincere@april.biz",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 43ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🚀 ekoDB JavaScript Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native (rxDY1TokPG9Hh1_6Zrrn2NDlp4B8CLyLSPRE2H_ix_mjmbWDd8VfrAja6zCG2wgxOk4elRZ7fyQvYcmJ8Iykxw)

First call (cache miss - will fetch from GitHub API):
  Response time: 16ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 17ms
  Speedup: 0.9x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (sI9aZ-AmzuvK0Vr6F6l0qYoJtU63GLt3YRfTHbrjVNlnB2-9qWzLGuahIpqtxTOAKFR3guR09d7D5P6AbBJMPg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (s012Wg-0-rzy2pwAnklURD6LLt4CEXs9cJTyXhzCewDIDN9mZOwi_yIaXQnH9yYGccBCTHlmp8acgyWb6ZoVKg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (nNLfkBSHG3PsEdYNCN6DiaHqImkkb2SpD2KxfxHDyAoqq48dYF-R1_iSNldQNbMV7zZ2QDLfSiT_CfmB1UwgEQ)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

================================================================================
✅ Key Benefits of Native SWR:
✅ Single function: Replaces 4-function cache-aside pattern
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)
Native SWR:     SWR → Insert (2 functions)
Result:         60% fewer functions, cleaner code, same behavior 🎯

🧹 Cleaning up...
✓ Deleted 4 test functions

✅ All examples completed!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: pC9ZgDag9UdMhCCFGdintrN2bnIO_gW46_Nttw0a_juTVmQF5CEaOuhfQEDArRFsaYmGhDipDc_iNMRZNF6FRg

=== Sending Chat Message ===
Message ID: -BU-3PYkvQPjGr45P4ImnVwEXhekygsqFC0k1leoA3EL650YazrDbwputwlGlBuc-jp9bWQR1L9oTjOae80Ngw

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB Pro**
   - Description: Enterprise edition product with advanced features
   - Price: $299

2. **ekoDB Cloud**
   - Description: Fully managed cloud database service product
   - Price: $499

3. **ekoDB**
   - Description: A high-performance database product with AI capabilities
   - Price: $99

If you need more information about any of these products, feel free to ask!

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'T2_MeD6UEko5iBMRo57n3c4G_xuGPVdz4QEiR5IW4UuRw9mVRKx3z9-18DQDp-nit7TpRFi25BWuLTkN_j-lJQ',
    price: 299,
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'HcYS8VP5jLZCNm4yhk3cMqskVhJPKJY06Q5Xyc_wHkc8cXYqz93GgDzP0iKg7oIh9-D0sX2EtUUDsyt_d3U7pA',
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    price: 99,
    id: 'MPBar4MqTvaetGr3O9NlCNRhcDobatUyFBFaZoGM7aXOWt2u5TfIcg-DjnJF38Mf26MpJM-8TTUStyqldkRrjA'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3151ms

=== Token Usage ===
Prompt tokens: 1298
Completion tokens: 110
Total tokens: 1408

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: MOE12b2du5c0PP4iUi8oc9qvpApOjpzYQjvYjTKAZgyhHIKQe9y-QYXr_hNEAfKbOsC8wBIma92BjXM5ZAeN9w

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or would like to see additional products, let me know!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any more questions or need further assistance, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: aOI93L8pyMfccWvlebaCdtv4RQTjsTioblCS8i-LVCgjWDcpwAayoz6F0mgIsdKO2SrZrLOCpP1MwE-YyBR9IA
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
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 3eLXgFvR1U8w_gYtTTlwRI4XUpNX8EqKVXDhT6OEOgq4HZe3yy0uQ_gp4CiV9prGTHjY1HzXXIxB4CGaKHrGtQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Price:** $99

If you'd like more information or to explore additional products, please let me know!

✓ Message 2 sent
  Response: The price of ekoDB is **$99**. If you need any more information, feel free to ask!

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: DpExGxMi3Nt31b6hHoGt6-6mP5Sgw_chdhsUuvJgOZdV3RZAJV2ZkNGJ2SWetdLLkVHBAxfBhOsxruVOFamCtg
  Parent: 3eLXgFvR1U8w_gYtTTlwRI4XUpNX8EqKVXDhT6OEOgq4HZe3yy0uQ_gp4CiV9prGTHjY1HzXXIxB4CGaKHrGtQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: DpExGxMi3Nt31b6hHoGt6-6mP5Sgw_chdhsUuvJgOZdV3RZAJV2ZkNGJ2SWetdLLkVHBAxfBhOsxruVOFamCtg (Untitled)
  Session 2: 3eLXgFvR1U8w_gYtTTlwRI4XUpNX8EqKVXDhT6OEOgq4HZe3yy0uQ_gp4CiV9prGTHjY1HzXXIxB4CGaKHrGtQ (Untitled)
  Session 3: aOI93L8pyMfccWvlebaCdtv4RQTjsTioblCS8i-LVCgjWDcpwAayoz6F0mgIsdKO2SrZrLOCpP1MwE-YyBR9IA (Untitled)
  Session 4: pC9ZgDag9UdMhCCFGdintrN2bnIO_gW46_Nttw0a_juTVmQF5CEaOuhfQEDArRFsaYmGhDipDc_iNMRZNF6FRg (Untitled)
  Session 5: FR9F_qSfuVyQ4TdxyeyUH62vek2rS32PNm4TEei_tYVSi2403Te85WeB2e8oMlCh0cZQe4SqPTeUlpiKvzJTvg (Untitled)
  Session 6: p8s1qPSKPwGUe4GBfg6clOWiJdpIODgQAG5XkF0F0iSmDTfY_Omh2ocmr_ZcLYDggYpFsqMAy6LmFs5Qi-Zv3w (Untitled)
  Session 7: NywLVYKnaHEA4WfOYsmM9dY1oavQ8hnEq8Dodp6wkPp3PUoHYtYQTghQjokz331UmUU3qkT2VrZ7j6Wtw6o1qg (Untitled)
  Session 8: NDeOBVKsMCqOpcQHsNmV96Nji7V4MJEHYbKVcf_YfpvNMO5mcJIO19Nb_3ljx4BrrIZG572g5PizSy-vllMSSA (Untitled)
  Session 9: _DgimDs70IFjtBMQiEuIgkb4BNyDwHu4ZzjUhOnlzAxAeCdSKVpiyLbaB8_hA-OsMxz_HPNaHeYBLDnawpQd2w (Untitled)
  Session 10: j35FSFi7T5MNfApTG_tS_0nktaY5bNDTaUm9_Oh1NZq0crz26C65MZINFB_tBRD96KcT1EvDsvfv9PqQglIKyg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: DpExGxMi3Nt31b6hHoGt6-6mP5Sgw_chdhsUuvJgOZdV3RZAJV2ZkNGJ2SWetdLLkVHBAxfBhOsxruVOFamCtg

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

BUILD SUCCESSFUL in 4s
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
✓ Inserted user: EkoRecord(fields={id=StringValue(value=cJeqGsyApFSKPvIR9z4GZ7zSj7Y1UzAwUSds6ssvqOr8yl38QGCQp98K0ekZuLNZt_miHtCMxLUJa0xmzLHCBw)})
  User ID: cJeqGsyApFSKPvIR9z4GZ7zSj7Y1UzAwUSds6ssvqOr8yl38QGCQp98K0ekZuLNZt_miHtCMxLUJa0xmzLHCBw

=== Read ===
✓ Found user by ID: EkoRecord(fields={categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), created_at=ObjectValue(value={value=StringValue(value=Sun Apr 05 23:55:21 EDT 2026), type=StringValue(value=String)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)})}), age=ObjectValue(value={value=IntegerValue(value=28), type=StringValue(value=Integer)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), id=StringValue(value=cJeqGsyApFSKPvIR9z4GZ7zSj7Y1UzAwUSds6ssvqOr8yl38QGCQp98K0ekZuLNZt_miHtCMxLUJa0xmzLHCBw)})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Sun Apr 05 23:55:21 EDT 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {nested={deep=true}, key=value}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: categories, name, created_at, tags, price, email, data, active, embedding, metadata, age, user_id, id

=== Update ===
✓ Updated user: EkoRecord(fields={data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Sun Apr 05 23:55:21 EDT 2026)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)})}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), id=StringValue(value=cJeqGsyApFSKPvIR9z4GZ7zSj7Y1UzAwUSds6ssvqOr8yl38QGCQp98K0ekZuLNZt_miHtCMxLUJa0xmzLHCBw), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])})})

=== Query ===
✓ Found 1 users matching query
  - EkoRecord(fields={data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)})}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), id=StringValue(value=cJeqGsyApFSKPvIR9z4GZ7zSj7Y1UzAwUSds6ssvqOr8yl38QGCQp98K0ekZuLNZt_miHtCMxLUJa0xmzLHCBw), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), created_at=ObjectValue(value={value=StringValue(value=Sun Apr 05 23:55:21 EDT 2026), type=StringValue(value=String)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])})})

=== Delete ===
✓ Deleted user with ID: cJeqGsyApFSKPvIR9z4GZ7zSj7Y1UzAwUSds6ssvqOr8yl38QGCQp98K0ekZuLNZt_miHtCMxLUJa0xmzLHCBw

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

=== Example Complete ===

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
=== ekoDB Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client ===
✓ WebSocket client created

=== Connect to WebSocket ===
✓ Connected to WebSocket

=== Insert Test Record ===
✓ Inserted test record

=== Find All via WebSocket ===
✓ WebSocket findAll result:
  {"data":[{"id":"pgJ6Sn3NEtK7rcQEIDs9ru6XEAtYPCO74bcHq7B74Jmh1hmoc4k7YnkjupKPb6bzUmSNvwiNX_9KUoC5dKY7qA","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]}

=== Close WebSocket ===
✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_example

=== Example Complete ===

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
=== ekoDB Kotlin Client - Batch Operations Example ===

=== Batch Insert ===
✓ Inserted 5 records
  IDs: 5rc3hUQ9607XIPb-Ci1wVr8cL39l73ZjURrx4MZDlZo0UeqhAHMOzwLgzxYUoL1osNlEJZ65c6HcPBWnGsz0QQ, 2I-Alysel9vsaDVzz82muW-iLplBE-o-UdJf2Ky936cMGYu6bPms18zm1gEXf4KNWDJkId-bXwIcm4RIp59CUw, iCS8RPucwb32iUjWrgJTrYYfGpNECqHDLI-Zsu61chDewet-c8zwkN274HwbOPgPP0AD0beOBM_FfDbScfsYJw...

=== Batch Update ===
✓ Updated 3 records

=== Batch Delete ===
✓ Deleted 2 records

=== Cleanup ===
✓ Deleted collection: kotlin_batch_example

=== Example Complete ===

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
=== ekoDB Kotlin Client - Collection Management Example ===

=== List Collections ===
✓ Found 53 collections
  - audit__ek0_testing
  - cached_database_metrics
  - schema_products_client_go
  - enriched_users
  - schema_employees_client_py

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-04-06T03:55:40.059416Z","last_modified":"2026-04-06T03:55:40.059416Z","bypass_ripple":false,"primary_key_alias":"id"}

=== Cleanup ===
✓ Deleted collection: kotlin_collection_example

=== Example Complete ===

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
=== ekoDB Kotlin Client - KV Operations Example ===

=== KV Set ===
✓ Set key: user:123

=== KV Get ===
✓ Retrieved value: {"email":"alice@example.com","name":"Alice","role":"admin"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"user_id":"123","created_at":1775447745172}
  (Will expire in 10 seconds)

=== KV Batch Set ===
✓ Batch set 3 keys
  config:db: success
  config:cache: success
  config:api: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  config:db: {"value":{"port":5432,"host":"localhost"}}
  config:cache: {"value":{"enabled":true,"ttl":3600}}
  config:api: {"value":{"timeout":30,"retries":3}}

=== KV Exists ===
✓ Key exists: true

=== KV Find (Pattern Query) ===
✓ Found 3 keys matching 'config:.*'

=== KV Query (Alias for Find) ===
✓ Total keys in store: 20

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
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 2Da_wGf65sekoSUXUpHSVJF-3aZnU7c_Oa4G66TulX-TB8xgCm4MROoLe97Ya34_6yHTQhsFhSKxb3v-QsJ6Hw
Created Bob: $500 - ID: ecuHfwH8aUSMsgv6nMaRwbPXKSM5cz9nFIIC7eqDmykqqVsE5Dtpq3QjEDMJCJvmG4IWMEetgzi2h0Zzvj7G4w

=== Example 1: Begin Transaction ===
Transaction ID: bcc9c33d-83e7-4cdc-a481-377063a35370

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: e17d02db-25c7-456d-8242-d8fd56ebff06
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed

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
  - Score: ObjectValue(value={value=IntegerValue(value=88), type=StringValue(value=Integer)})

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
=== ekoDB Kotlin Client - Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Search results for 'programming':
  {"results":[{"record":{"tags":{"type":"String","value":"programming,rust,tutorial"},"id":"Y9lqQJhDbSkh0CgPoWRFoChDv6Q3mySQFcFh_-j0z-YC93cUBA5ZD9KB61-teAW02vzY0f7q9i1TVEyHTp5-zg","title":{"type":"String","value":"Rust Programming"},"views":{"type":"Integer","value":978},"description":{"type":"String","value":"Learn Rust programming language with hands-on examples and best practices."}},"score":19.8,"matched_fields":["tags","description","title"]},{"record":{"id":"ry_ffZhKMWaI8daloO0RauWALtHnPRz-guswkdoLUYQ2E6o5eks3hK488TyTj26VBMJarC5FE7kgqV04RkVF6A","tags":{"value":"programming,python,data-science","type":"String"},"title":{"value":"Python for Data Science","type":"String"},"description":{"type":"String","value":"Master Python for data analysis, machine learning, and visualization."},"views":{"type":"Integer","value":735}},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"title":{"type":"String","value":"JavaScript Web Development"},"description":{"type":"String","value":"Build modern web applications using JavaScript, React, and Node.js."},"views":{"value":958,"type":"Integer"},"tags":{"value":"programming,javascript,web","type":"String"},"id":"xhUp9vq7UyzvJZSIg4VOIY-65_1XEYmtQR9-tDHleA_YMgJANoWKxpAfWvXTfGECKdKyj71Iju87wYCZTOmnvA"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"id":"YjoczoqZHKmx3yKoMjWaN-UsK47u963QJ8oCV_DXpxHynvIWZpw-elDYeTBCSV9KcsFzuluJtoJ7-DaeUUzIww","title":{"value":"Machine Learning Basics","type":"String"},"views":{"type":"Integer","value":69},"tags":{"value":"ai,machine-learning,python","type":"String"},"description":{"type":"String","value":"Introduction to machine learning algorithms and neural networks."}},"score":2.7,"matched_fields":["title","tags","description"]},{"record":{"tags":{"value":"programming,python,data-science","type":"String"},"description":{"type":"String","value":"Master Python for data analysis, machine learning, and visualization."},"views":{"type":"Integer","value":735},"title":{"value":"Python for Data Science","type":"String"},"id":"ry_ffZhKMWaI8daloO0RauWALtHnPRz-guswkdoLUYQ2E6o5eks3hK488TyTj26VBMJarC5FE7kgqV04RkVF6A"},"score":1.0,"matched_fields":["description"]},{"record":{"id":"04X1eq55ITkzVGPHj2BMW9GEA-xvmjwxXnr0sCZ-F2SodS0_g_U89P5gzlUrYFpFpZAL-N39z8QXwCsN0mgY7Q","description":{"type":"String","value":"Learn database design principles, normalization, and query optimization."},"title":{"type":"String","value":"Database Design"},"views":{"type":"Integer","value":824},"tags":{"value":"database,design,sql","type":"String"}},"score":0.5,"matched_fields":["description"]},{"record":{"views":{"type":"Integer","value":978},"tags":{"type":"String","value":"programming,rust,tutorial"},"id":"Y9lqQJhDbSkh0CgPoWRFoChDv6Q3mySQFcFh_-j0z-YC93cUBA5ZD9KB61-teAW02vzY0f7q9i1TVEyHTp5-zg","description":{"type":"String","value":"Learn Rust programming language with hands-on examples and best practices."},"title":{"type":"String","value":"Rust Programming"}},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":0}

=== Cleanup ===
✓ Deleted collection: kotlin_search_example

✓ All search operations completed successfully

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
=== ekoDB Kotlin Client - Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'kotlin_schema_example' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: StringValue(value=jrnUxqlXdPr6fvcF7bhBYsC0PuKf2jK4Y5ftG1TzWrzpx8cc49cFkLIcAVAlPrLpgkVwcjANx-uag-iKrQVX-g)
✓ Inserted user 2: StringValue(value=-eg5xnqFw1y4GtznCnSYcQsC6oLdv0vj4o5AiSOMq-ByG6FLRBQKCNo_dFwrlRtXraDgrmT8eEwriqk5wHn78w)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 54
  Sample: [audit__ek0_testing, cached_database_metrics, schema_products_client_go, enriched_users, schema_employees_client_py]

=== Cleanup ===
✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

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
  - Mouse ($25) by Alice Johnson
  - Laptop ($1200) by Alice Johnson

=== Example 5: Complex join with multiple conditions ===
✓ Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✓ Deleted test collections

✓ Join operations example completed successfully

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
=== ekoDB Kotlin Client - Document TTL Example ===

=== Insert with TTL ===
✓ Inserted document with 10s TTL
  Document ID: 0TlTCdC_PPF5SppD9sBcLFCJoLlUD4VSCeHOcvdQWCJmXcsbAeFtZ_bvXcSOWPTgCqShb3hiIkuqxEarEHFCiQ

=== Verify Document Exists ===
✓ Document found: user_id, created_at, session_id, id, ttl

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: 4RbXjXT9RM3W2Mr3q2dZYkpRhwrxjBueErPx5QWAuLds93JIs0nHBLDIQIgDsq0v1VF2B5PsSSD3dIgVO-_KeQ

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: cache_key, value, id, ttl

=== Delete Document ===
✓ Deleted document

=== Cleanup ===
✓ Deleted collection: kotlin_ttl_example

=== Example Complete ===

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
=== ekoDB Kotlin Client - WebSocket TTL Example ===

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: StringValue(value=k3cX6y5dHKIxv2drC-j-3E3a3WgUYhvly7kYS-CO0ydJt8BzsvQTT9CWS4YNiD3l6p1MF3svE6v_RBWie9J0lQ)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"data":[{"created_at":{"type":"Integer","value":1775447784616},"id":"k3cX6y5dHKIxv2drC-j-3E3a3WgUYhvly7kYS-CO0ydJt8BzsvQTT9CWS4YNiD3l6p1MF3svE6v_RBWie9J0lQ","name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-04-06T04:56:24.751306Z","value":{"type":"Integer","value":42}}]}

✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_ttl_example

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration

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
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup function...
✓ Edge cache function created: Js9Zh5LRj4JaTX5j6rrkotZ8rky8P4TDO7lFVkceajt9DQ3uPSLxb_bDjVQdxrfbydAqJO98hKmeY5hziVb7uQ

Call 1: Cache lookup
Found 1 cached entries
Response time: 30ms

Call 2: Cache lookup (connection warm)
Found 1 cached entries
Response time: 19ms

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
🚀 ekoDB Functions Example (Kotlin Client)

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: 3xmx4MnOtcytmjs_bPj2z43e4YobvOFeIBxRHIR3TZy3ulxZ2RWVnKTjs6bEIVxgQpTMIrSJ4AVPxlcvluuRmQ
📊 Found 15 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 15 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 3 groups
   {"avg_score":60.0,"status":"null","count":5}
   {"status":"inactive","count":5,"avg_score":50.0}
   {"status":"active","avg_score":60.0,"count":5}
⏱️  Execution time: 0ms

📝 Example 4: function Management

📋 Total functions: 36
🔍 Retrieved function: Get Active Users
✏️  function updated
🗑️  function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage function saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count function saved
📊 Total user count: 15
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test functions

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • Built-in error handling

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
🚀 ekoDB Kotlin Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: sbCPP6Y10jqkORP5k4fdAFpkodK_P7gBRzX-XCvk6jVJDpHX_VFcHIQxDftirLe5ddAKUBlZs09L7mfy3Eg2iw
📊 Found 2 category groups

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories

📝 Example 4: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed with 1 results

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete function examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Function management (save, call, delete)

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
🚀 ekoDB Kotlin KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within functions
   • Combined wrapped types + KV workflows

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: StringValue(value=lg4tsMKvba6yJkydzBIUcbQHtegDYIprj9AJKfxpYoSDxwqO1pmZIPeTscFJdaZVyFoc-uFPZiPZa0fuG8BACw)
✅ Inserted 2 products

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: qUVhIrVwVdEOIAKQHAlvZa0T18yzOJPX24F4JWWP7qdWXS7mQ8eyPNgLA6CubiwBlf-rXhK7b9xrCv614vjXkA
📊 Created order via function
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: qsmCNVOGUxFGuljyocsfqb8StYG0RUwhGoP3JN9ieY2nh1Ceo_XSc0rPKTyAPVW2YT3GqQJ6ekKZwSGyY1qdGw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: ujzFZRgLzhzmcO69I-WbSs81q2BulzFYhP6i91XM2lvWZHTAH4bzQBJ6QcQEYpV0maPnOSzycFrPNG2LIpVq8Q
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
   ✅ FunctionStageConfig.Kv* classes work within functions

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
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Setting up cache collection...
✓ Cache entry created

Step 2: Create SWR cache lookup function
✓ Created SWR function: swr_cache_lookup_kt (cA_TJohUwyMVhrG07R09oEhLaPwRqOnaPx8YPLTpZE3fVv09jLrK7jNLXqjlBfsaLRNDyG2BmR_gpfeIpZGdOA)

Step 3: First call - Cache lookup
Found 1 cached entries
✓ Cache lookup complete

Step 4: Second call - Fast cache hit
Response time: 21ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation

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
🚀 ekoDB Kotlin Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.

Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR function: github_user_native (HfT5tsOe96y8TuJlG-w6Ii3fmFWACnABf9kDm85oWMsLOxWkOgv1lCDpLyLi0eeL6R2MfDnBEzSOsdPmOFjuMA)

First call (cache miss - will fetch from GitHub API):
  Response time: 33ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 20ms
  Speedup: 1.7x faster 🚀


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR function with audit trail: product_swr_audit (S9-grk0rKwXdI2kQcXOTfNOZfPpDLtFsjRahKTqphzvwDkPBATkJp-P0usIDpzvGOra7KC7yuTJF_M1-zB3XWg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (lfr1YatMQDDbJzvlEitjSkAu1AF73ubeEP_0-QdC3B0njv0TFMq3365T4YKnCDJVxFNYv-t9WCzvfwzUXGwmPg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL function: flexible_cache (PZyF2m2tECl4JuGIP5HQu5BoTJvp4G8pTF1c6scLNBazH3uUi2PUZYgRT-7hxeS_bPcmQHyDMx0KLn8xyynSEw)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

================================================================================
✅ Key Benefits of Native SWR:
✅ Single function: Replaces 4-function cache-aside pattern
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)
Native SWR:     SWR → Insert (2 functions)
Result:         60% fewer functions, cleaner code, same behavior 🎯

🧹 Cleaning up...
✓ Deleted 4 test functions

✅ All examples completed!

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
🚀 ekoDB Kotlin Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   {"category":"Furniture","avg_price":365.6666666666667,"count":3}
   {"avg_price":367.0,"count":5,"category":"Electronics"}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced function examples finished!

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
🚀 ekoDB Kotlin AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat function saved
🤖 AI Response:
   {"tokens_used":260,"response":"Vector databases offer several benefits, including:\n\n1. **Efficient Similarity Search**: They excel at finding similar items based on vector representations, improving search accuracy in large datasets.\n\n2. **Scalability**: Designed to handle large volumes of high-dimensional data, making them suitable for big data applications.\n\n3. **Real-time Queries**: Support for real-time processing allows for quick responses to user queries and updates.\n\n4. **Integration with AI/ML**: They seamlessly integrate with machine learning models, facilitating tasks like recommendation systems and natural language processing.\n\n5. **Enhanced Retrieval**: Enable advanced retrieval techniques like nearest neighbor searches, crucial for applications in image and voice recognition.\n\n6. **Flexibility**: Can handle various data types, including text, images, and audio, represented as vectors.\n\n7. **Lower Latency**: Optimized for low-latency access, improving user experience in applications requiring quick responses. \n\n8. **Rich Data Representations**: Support for embedding techniques, capturing semantic relationships and similarities in complex datasets. \n\nThese benefits make vector databases increasingly popular in modern applications requiring advanced data handling capabilities."}
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed function saved
📊 Embedding generated
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI function examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text

BUILD SUCCESSFUL in 9s
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
🚀 ekoDB Kotlin CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   {"status":"inactive","count":3}
   {"status":"active","count":7}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD function examples finished!

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
🚀 ekoDB Kotlin Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. {"type":"String","value":"Database Design Principles"} ({"value":"Database","type":"String"})
   2. {"type":"String","value":"Natural Language Processing"} ({"value":"AI","type":"String"})
   3. {"value":"Introduction to Machine Learning","type":"String"} ({"type":"String","value":"AI"})
   4. {"type":"String","value":"Vector Databases Explained"} ({"type":"String","value":"Database"})
   5. {"value":"Getting Started with ekoDB","type":"String"} ({"type":"String","value":"Database"})
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":"Database","count":3}
   {"count":2,"category":"AI"}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search function examples finished!

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
=== ekoDB Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 1BnjMP513034w5cMTSZNivU2Gxe3Njao4GrzWOJH3Ol297XwKfN50jjNIWz0VSHjdZ6Io13tVOsN-OB9EKqoSw

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "yAqAWCjPGORbf-kQTwQPGEmc_bNxmYxDjT-FWLYJlog4Emv1kS-9wzOsxAarXNar42ulcHB3LZnWMenGPCNaQg"
  Responses: ["ekoDB is a high-performance database that features intelligent caching, real-time capabilities, and AI integration. \n\n### Features of ekoDB:\n1. **Chat Feature**: Allows querying the database using natural language, providing AI-powered responses with relevant context.\n2. **Search Capabilities**: Supports full-text search, vector search, and hybrid search, enabling advanced retrieval of information and context.\n\nThese features ensure that ekoDB is efficient and capable of handling various data retrieval needs effectively."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 11s
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
✓ Created session: L6XHLVKonEzRtTVWYOLRWLsI-QxVyg5G4FVjtKCFpg3VGJvsZZb1YN-GGP765Q-3RUmwkrmC6aKV8eGkPvDx7A

=== Sending Initial Message ===
✓ Message sent
  Responses: ["The available product is:\n\n- **Name:** ekoDB\n- **Description:** High-performance database product\n- **Price:** $99\n\nIf you need more information or additional products, just let me know!"]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["We currently have the following product available:\n\n- **Name**: ekoDB\n- **Description**: High-performance database product\n- **Price**: $99\n\nIf you have any further questions or need more information, feel free to ask!"]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: dyzGk4yiVwdXaWvAQXA36Hepeyxz1u09NcWEmTNCvh0ZSLJ8bk3CvdVB7a_jal0WzZ7Pdcm64cifcU_0TWTJOA

=== Merging Sessions ===
✓ Merged sessions
  Total messages in merged session: 2

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 10s
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
✓ Created session: EpA3-JltXVCwfnBb9K39GdPrdxiGnP5oCnk3LsOl1H0bpSmKrPeYftJUBf4uDZVHaNGBQkda20Pg1rpujbkmSg

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The available product is:\n\n- **Product Name:** ekoDB\n- **Description:** A high-performance database product with AI capabilities\n- **Price:** $99\n\nIf you need more information or details about other products, feel free to ask!"]

✓ Message 2 sent
  Responses: ["The price of the product ekoDB is **$99**."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"EpA3-JltXVCwfnBb9K39GdPrdxiGnP5oCnk3LsOl1H0bpSmKrPeYftJUBf4uDZVHaNGBQkda20Pg1rpujbkmSg"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"HALM8i3SMEy-W0tF7EZef-ZVVru7wZbXJ0ns3BXlsEEB12NQqqIzyW69smbobsLFGFw7k1VjeudugskYN_7OiA","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-04-06T03:57:55.195986Z"},"id":"fXi-pTq7JdfHqkNcdrixZQHVL0eTApy2qnTzyv4wB8bgDigmOi360e-yrMgzpp-1laVoSEr5Z2cK5gG9gF4h5g","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":82,"prompt_tokens":2529,"total_tokens":2611}},"updated_at":{"type":"DateTime","value":"2026-04-06T03:57:55.195986Z"}},{"chat_id":{"type":"String","value":"EpA3-JltXVCwfnBb9K39GdPrdxiGnP5oCnk3LsOl1H0bpSmKrPeYftJUBf4uDZVHaNGBQkda20Pg1rpujbkmSg"},"content":{"type":"String","value":"The available product is:\n\n- **Product Name:** ekoDB\n- **Description:** A high-performance database product with AI capabilities\n- **Price:** $99\n\nIf you need more information or details about other products, feel free to ask!"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"HALM8i3SMEy-W0tF7EZef-ZVVru7wZbXJ0ns3BXlsEEB12NQqqIzyW69smbobsLFGFw7k1VjeudugskYN_7OiA","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-04-06T03:57:55.196869Z"},"id":"LdxjTHGTKOJ0u7_FhJgh4SFfSPcMrqI-jdbybzAbphgEuiN4lwFETEGAPg7EavGvqBkVnD9BP9pE-CmQHqFuAQ","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":82,"prompt_tokens":2529,"total_tokens":2611}},"tool_call_count":{"type":"Number","value":2},"tool_call_history":{"type":"Object","value":{"iterations":2,"tool_calls":[{"arguments":{"collection":"kotlin_chat_sessions_example","limit":10,"select_fields":["product","price","description"]},"id":"call_KL2GGZQLv9HH5q6ZjeT6rBQU","name":"query_collection"}],"tool_results":[{"error":null,"result":{"count":1,"records":[{"description":"A high-performance database product with AI capabilities","id":"HALM8i3SMEy-W0tF7EZef-ZVVru7wZbXJ0ns3BXlsEEB12NQqqIzyW69smbobsLFGFw7k1VjeudugskYN_7OiA","price":99,"product":"ekoDB"}]},"success":true,"tool_call_id":"call_KL2GGZQLv9HH5q6ZjeT6rBQU","tool_name":"query_collection"}]}},"updated_at":{"type":"DateTime","value":"2026-04-06T03:57:55.196869Z"}},{"chat_id":{"type":"String","value":"EpA3-JltXVCwfnBb9K39GdPrdxiGnP5oCnk3LsOl1H0bpSmKrPeYftJUBf4uDZVHaNGBQkda20Pg1rpujbkmSg"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"HALM8i3SMEy-W0tF7EZef-ZVVru7wZbXJ0ns3BXlsEEB12NQqqIzyW69smbobsLFGFw7k1VjeudugskYN_7OiA","price":99,"product":"ekoDB"},"score":2.1047619047619044}]},"created_at":{"type":"DateTime","value":"2026-04-06T03:57:56.397730Z"},"id":"r4m90Mi0t9LWhFggltNqgn0XBdgwIOy1Xy_qT-VRupGJ9BWPRVA0XO2KV6qvUb4Yfu1EY88T_7824JKgLyB7XA","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":10,"prompt_tokens":1285,"total_tokens":1295}},"updated_at":{"type":"DateTime","value":"2026-04-06T03:57:56.397730Z"}},{"chat_id":{"type":"String","value":"EpA3-JltXVCwfnBb9K39GdPrdxiGnP5oCnk3LsOl1H0bpSmKrPeYftJUBf4uDZVHaNGBQkda20Pg1rpujbkmSg"},"content":{"type":"String","value":"The price of the product ekoDB is **$99**."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"HALM8i3SMEy-W0tF7EZef-ZVVru7wZbXJ0ns3BXlsEEB12NQqqIzyW69smbobsLFGFw7k1VjeudugskYN_7OiA","price":99,"product":"ekoDB"},"score":2.1047619047619044}]},"created_at":{"type":"DateTime","value":"2026-04-06T03:57:56.400694Z"},"id":"3hfce8yIPMtXv8mGOXuqi4dRbXB-mOAit5M7w7XBLms2kxkt48blwKcbBhXfS1WxlN9SFzgTPqCk6TakFLkONw","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":10,"prompt_tokens":1285,"total_tokens":1295}},"updated_at":{"type":"DateTime","value":"2026-04-06T03:57:56.400694Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 16

=== Branching Session ===
✓ Created branched session: WZZ3Axga0_6V8ghNuEVEvV2rSYzs_2G-EmPQ92V_Y-fzNNBVlSxd8y9yhMuOm7mT4BFhk59hAMz6qzsyAVmgWA

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

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
=== ekoDB Kotlin Client - Chat Models Example ===

=== List Chat Models ===
Available chat models by provider:
  openai:
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
  anthropic:
    - claude-sonnet-4-6
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
    - claude-opus-4-1-20250805
    - claude-opus-4-20250514
    - claude-sonnet-4-20250514
    - claude-3-haiku-20240307
  perplexity:
    - sonar
    - sonar-pro
    - sonar-deep-research
    - sonar-reasoning
    - sonar-reasoning-pro

=== Get OpenAI Models ===
OpenAI models: gpt-4-0613, gpt-4, gpt-3.5-turbo, gpt-5.4-mini, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, dall-e-3, dall-e-2, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, gpt-4o-audio-preview, gpt-4o-realtime-preview, omni-moderation-latest, omni-moderation-2024-09-26, gpt-4o-realtime-preview-2024-12-17, gpt-4o-audio-preview-2024-12-17, gpt-4o-mini-realtime-preview-2024-12-17, gpt-4o-mini-audio-preview-2024-12-17, o1-2024-12-17, o1, gpt-4o-mini-realtime-preview, gpt-4o-mini-audio-preview, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, gpt-4o-realtime-preview-2025-06-03, gpt-4o-audio-preview-2025-06-03, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, gpt-realtime-mini-2025-10-06, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-3.5-turbo-16k, tts-1, whisper-1, text-embedding-ada-002

=== Get Anthropic Models ===
Anthropic models: claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929, claude-opus-4-1-20250805, claude-opus-4-20250514, claude-sonnet-4-20250514, claude-3-haiku-20240307

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Request failed with status 405: {"code":405,"message":"Method Not Allowed"}

=== Chat Models Example Complete ===

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
=== ekoDB Kotlin Client - User Functions Example ===

=== Create User Function ===
Created user function with ID: 2c9IRCRAOWZLJH51o_ddhaZq_K8-CbkHNpiCGozPfETO8uFyXbRxGCWo15nOKAAfxWcmehxuNrbkzZSb5aXc_A

=== Get User Function ===
Retrieved: "get_active_users_kt" - "Get Active Users (Kotlin)"
Description: "Fetches all users and filters by active status"

=== List All User Functions ===
Found 31 user functions:
  - "fetch_user": "Fetch user by code"
  - "fetch_slim_user": "Validate and slim down user"
  - "fetch_slim_user": "Validate and slim down user"
  - "fetch_and_store_user": "Fetch user from API and cache in KV"
  - "swr_user": "SWR pattern for user data (KV-based)"

=== List User Functions by Tag ===
Found 31 user functions with 'kotlin' tag:
  - "fetch_user"
  - "fetch_slim_user"
  - "fetch_slim_user"
  - "fetch_and_store_user"
  - "swr_user"
  - "fetch_slim_user"
  - "fetch_and_store_user"
  - "fetch_api_user"
  - "swr_user"
  - "swr_user"
  - "get_active_users_kt"
  - "fetch_and_store_user"
  - "fetch_and_store_user"
  - "swr_user"
  - "validate_user"
  - "fetch_and_store_user"
  - "cache_api_call_py"
  - "fetch_product_with_reviews"
  - "validate_user"
  - "validate_user"
  - "cache_api_call"
  - "fetch_user"
  - "swr_user"
  - "fetch_user"
  - "fetch_user"
  - "fetch_user"
  - "validate_user"
  - "cache_api_call_js"
  - "validate_user"
  - "fetch_slim_user"
  - "fetch_slim_user"

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

=== User Functions Example Complete ===

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
=== ekoDB Convenience Methods Example ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== Native Object Creation ===
✓ Created record: EkoRecord(fields={id=StringValue(value=eQq2-tEUqgavbmQfSOEb90FfFy5TEiFzhjZY7ar-ycVEwqJwWYydEZK6atXwkmCD4wL-0MHAuctG4zEkw-xRCw)})

=== Upsert Operation ===
✓ First upsert (update): EkoRecord(fields={id=StringValue(value=eQq2-tEUqgavbmQfSOEb90FfFy5TEiFzhjZY7ar-ycVEwqJwWYydEZK6atXwkmCD4wL-0MHAuctG4zEkw-xRCw), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), email=ObjectValue(value={value=StringValue(value=alice.j@newdomain.com), type=StringValue(value=String)})})
✓ Second upsert (insert): EkoRecord(fields={id=StringValue(value=qCbTUJvR0FLGuqm7LPjgEYIsKJ72qvQNMZboeq12FMYF71go6YIGyYaK0PM5MmxHA7KHCEP_REbbdJPzmJF1JA)})

=== Find One Operation ===
✓ Found user by email: EkoRecord(fields={id=StringValue(value=eQq2-tEUqgavbmQfSOEb90FfFy5TEiFzhjZY7ar-ycVEwqJwWYydEZK6atXwkmCD4wL-0MHAuctG4zEkw-xRCw), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice.j@newdomain.com)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)})})
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

BUILD SUCCESSFUL in 10s
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
   Inserted with ripple: EkoRecord(fields={id=StringValue(value=GFcl_HbVGNRCsTxuYInvY6xgkPxCREKsgv-emt9G3i2oozxG8K37mtjLcY8WJJtrOQxoOjRe6ENLJ8OCX7enpA)})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=fu5s0RZ9tflyhWz9ff5c49dOqrA95xIRIBzdZgMt_BY85ssdrqW8eF0qoukHHU80XG0OERGZ36bPjUEdap4Egg)})

3. Update with bypass_ripple:
   Updated with bypass_ripple: EkoRecord(fields={price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=150)}), id=StringValue(value=GFcl_HbVGNRCsTxuYInvY6xgkPxCREKsgv-emt9G3i2oozxG8K37mtjLcY8WJJtrOQxoOjRe6ENLJ8OCX7enpA), name=ObjectValue(value={value=StringValue(value=Product 1), type=StringValue(value=String)})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: EkoRecord(fields={price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=500)}), id=StringValue(value=GFcl_HbVGNRCsTxuYInvY6xgkPxCREKsgv-emt9G3i2oozxG8K37mtjLcY8WJJtrOQxoOjRe6ENLJ8OCX7enpA), name=ObjectValue(value={value=StringValue(value=Upsert Product), type=StringValue(value=String)})})

✅ All bypass_ripple operations completed successfully!

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
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields
Fetched 3 users with only 3 fields each

Example 2: Exclude sensitive fields
Fetched 2 admins without sensitive data
  Password field excluded: true

Example 3: Complex query with projection
Fetched 3 active users with profile fields

Example 4: Find by ID with projection
Fetched user profile: Alice Johnson

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 3 fields per user

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!

BUILD SUCCESSFUL in 4s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin client examples complete![0m
🟣 [33mKotlin Transactions...[0m
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Xk0Q3jDV9BvB6B__XFvKqLZLD1YrCIifNXmcluA9qDEROsoiHKb0P-4COKRjosifyIE_nMsgTn1UTfiwrY0p1Q
Created Bob: $500 - ID: sxjRWjVnz4H9eUD107vRHvuRRIU0Vj7dOgdO72CjYIi8wvxP4d2pndTalEZlh5JvmIszh3Y0Fup0b4ol0zN-EQ

=== Example 1: Begin Transaction ===
Transaction ID: f90a3774-ea90-4574-8db3-99dd6f91ac2e

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: da4c1bdf-3aef-45c5-b890-70a5428ef937
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
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.15.2-cp38-abi3-macosx_11_0_arm64.whl
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
    ✓ Generated embedding: 1536 dimensions in 580.220375ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.036571667s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 454.740833ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 477.5225ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 840.724292ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 597.341125ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.100219542s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.228279625s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 592.826083ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 513.894334ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 372.284875ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 507.299208ms
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
    ✓ Generated embedding: 1536 dimensions in 597.090375ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 296.277584ms

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

Writing memory-safe, high-performance database code involves several best practices and principles across database design, programming languages, and access patterns. Here are some tips to help you achieve this:

### 1. Choose the Right Database
- **Understand the Use Case**: Select between SQL (for structured data, complex queries) and NoSQL (for flexible schemas, high write throughput) based on your requirements.
- **Performance**: Opt for databases that match your performance requirements (e.g., in-memory databases for high-speed access).

### 2. Database Design
- **Normalization**: Normalize your relational database to eliminate redundancy and improve data integrity. Use appropriate normalization forms but balance with performance, as over-normalization can lead to complex joins.
- **Use Indexes**: Create indexes on frequently queried fields to speed up read operations. Be mindful that while indexes improve read performance, they can slow down write operations.

### 3. Memory Management
- **Connection Pooling**: Use connection pooling to reduce the overhead of establishing connections to the database. This keeps a small number of open connections and reuses them for multiple requests.
- **Batch Operations**: Use batch inserts, updates, and deletes to reduce the number of round trips to the database, which can significantly improve performance.
  
### 4. Optimize Queries
- **Prepared Statements**: Use prepared statements to prevent SQL injection and allow the database to optimize execution plans.
- **Limit Data Retrieval**: Always retrieve only the necessary columns and rows (use pagination) to minimize data transfer and processing time.

### 5. Error Handling
- **Graceful Degradation**: Implement robust error handling to manage exceptions or failures gracefully. This includes retry mechanisms for transient errors like timeouts.
- **Validation**: Validate input data before performing database operations to prevent issues such as injection attacks or data type mismatches.

### 6. Language Features
- **Memory-Safe Languages**: Use languages that provide memory safety (like Rust, Go, or Java) or utilize frameworks that enforce such safety. Memory management features (like garbage collection) can help avoid memory leaks.
- **Concurrency Control**: Use mechanisms provided by the language or database to handle concurrent access safely, such as transactions and locking.

### 7. Performance Monitoring and Profiling
- **Profiling Tools**: Utilize profiling tools to monitor database queries and application performance. Identify slow queries that can benefit from optimization or indexing.
- **Logging**: Log database operations to keep track of performance and errors. Analyze logs to identify bottlenecks.

### 8. Caching Strategies
- **Use Caching**: Implement caching mechanisms to alleviate the load on the database for frequently accessed data. Options include in-memory caching (like Redis) or application-level caching.

### 9. Transaction Management
- **ACID Transactions**: Utilize transactions to ensure that your database operations adhere to ACID principles, providing correctness and reliability.

### Conclusion
By adopting these principles, you can create memory-safe, high-performance database code that minimizes errors and maximizes efficiency. Regularly review and refactor your codebase as your application grows and changes to maintain performance and safety.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 519.744708ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3419 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 447.693709ms
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
  ✓ Text search completed in 49.038958ms

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
    ✓ Generated embedding: 1536 dimensions in 0.512s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.486s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.522s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.481s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.669s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.529s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.468s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.376s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.409s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.500s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.357s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.740s
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
    ✓ Generated embedding: 1536 dimensions in 1.222s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.058s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.331] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.300] From conv_database_design
     What is database normalization?

  3. [Score: 0.300] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  4. [Score: 0.256] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.188] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves several best practices across various aspects of programming and database management. Here's how you can achieve that:

### 1. **Choose the Right Database Technology**
   - **SQL vs. NoSQL**: Use SQL for structured data with complex queries and ACID transactions, and NoSQL for flexible schemas, horizontal scaling, or unstructured data.
   - **Database System**: Consider the database's performance capabilities, such as indexing, query optimization, and concurrency handling.

### 2. **Understand Database Normalization and Denormalization**
   - **Normalization**: Organize your database to reduce redundancy and improve data integrity by using foreign keys and breaking large tables into smaller ones.
   - **Denormalization**: In certain cases, especially for read-heavy applications, denormalization can be beneficial for performance by reducing the number of joins.

### 3. **Optimize Database Schema**
   - **Indexes**: Create indexes on frequently queried fields to enhance lookup speeds. Use indexing strategies (BTree for range queries, Hash for equality checks) based on query patterns.
   - **Data Types**: Use appropriate and efficient data types for fields to minimize storage space and improve performance.

### 4. **Use Efficient Querying Techniques**
   - **Minimize Overhead**: Fetch only the data you need (projection) by selecting specific fields instead of using `SELECT *`.
   - **Batch Operations**: Use batch insert or update operations to reduce round-trips to the database.
   - **Parameterized Queries**: Always use parameterized queries to prevent SQL injection attacks and improve performance.

### 5. **Handle Concurrency Safely**
   - **Transactions**: Use transactions to ensure atomic operations, maintaining data integrity across multiple operations.
   - **Isolation Levels**: Choose the right isolation level based on your application's needs (e.g., Read Committed, Serializable).

### 6. **Monitoring and Profiling**
   - **Query Optimization**: Regularly profile and monitor database queries. Use tools to visualize query performance and execution plans (`EXPLAIN` in SQL).
   - **Resource Usage**: Monitor memory, CPU, and disk I/O usage of the database to identify bottlenecks.

### 7. **Connection Management**
   - **Connection Pooling**: Use connection pooling to minimize the overhead associated with establishing connections.
   - **Close Connections**: Ensure connections are closed properly to prevent memory leaks and resource exhaustion.

### 8. **Error Handling**
   - Implement comprehensive error handling to gracefully manage and log database errors without crashing your application.

### 9. **Utilize ORM and Abstraction**
   - If using an Object-Relational Mapping (ORM) tool, ensure it is optimized and efficiently queries the database.

### 10. **Caching Strategies**
   - Use caching for frequently accessed data to reduce load on the database and improve response times. Consider in-memory caches like Redis or Memcached where applicable.

### Conclusion
Combining these strategies can significantly enhance the memory safety and performance of your database interactions. Always remember to test your application in conditions that simulate production to ensure that optimization strategies work as intended.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.563s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3331 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.600s
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
    ✓ Generated embedding: 1536 dimensions in 0.423s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.587s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.603s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.534s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.512s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.546s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.396s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.421s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.565s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.404s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.475s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.831s
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
    ✓ Generated embedding: 1536 dimensions in 0.518s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.050s
✓ Found 5 related messages across all conversations:
  1. [Score: 0.199] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.180] From conv_database_design
     What is database normalization?

  3. [Score: 0.180] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  4. [Score: 0.154] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.112] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

To write memory-safe and high-performance database code, consider the following best practices and strategies:

### 1. **Use Prepared Statements**
   - **Benefits**: Prevent SQL injection attacks and optimize query execution.
   - **Implementation**: Always use parameterized queries to separate SQL logic from data.

### 2. **Manage Connections Efficiently**
   - **Connection Pooling**: Use connection pools to minimize the overhead of creating and closing connections for each request.
   - **Best Practices**: Ensure connections are reused and released properly to prevent leaks.

### 3. **Leverage Transactions**
   - **ACID Properties**: Group multiple operations within a transaction to maintain data integrity.
   - **Implementation**: Use transactions for batch operations to reduce the number of commits and improve performance.

### 4. **Indexing**
   - **Performance Improvement**: Create indexes on frequently queried fields to speed up lookups.
   - **Monitoring**: Regularly analyze query performance and add/remove indexes as necessary.

### 5. **Batch Operations**
   - **Bulk Inserts/Updates**: Use batch operations for inserting or updating multiple records simultaneously to minimize round trips to the database.
   - **Implementation**: Group operations in a single transaction when possible.

### 6. **Efficient Query Design**
   - **Select Only What You Need**: Avoid using SELECT *; instead, specify only required fields.
   - **Optimize Joins**: Design queries to reduce the number of joins and ensure they are efficient.

### 7. **Use ORM Responsibly**
   - **Object-Relational Mapping (ORM)**: While ORMs can simplify database interactions, they may introduce performance overhead. Use them judiciously, and consider raw SQL for performance-critical parts.
   - **Profiling**: Monitor and analyze the performance impact of ORM-generated queries.

### 8. **Concurrency Control**
   - **Locking Mechanisms**: Use appropriate locking strategies (optimistic vs pessimistic) based on your application needs.
   - **Isolation Levels**: Understand and configure the appropriate transaction isolation levels to balance performance and consistency.

### 9. **Memory Management in Code**
   - **Automated Memory Management**: Use languages with garbage collection (like Java, C#) or ensure proper memory management (in C/C++) to avoid leaks.
   - **Avoid Excessive Load**: Keep data retrieved in memory to a minimum by paginating results when necessary.

### 10. **Monitoring and Logging**
   - **Query Profiling**: Use monitoring tools to identify slow queries and optimize them accordingly.
   - **Error Handling**: Implement robust error handling and logging mechanisms to diagnose issues without sacrificing performance.

### 11. **Review and Refactor Code Regularly**
   - **Code Reviews**: Regularly review and optimize the code for performance and security vulnerabilities.
   - **Benchmarking**: Test and benchmark critical database interactions to ensure they meet performance requirements.

### 12. **Use Suitable Database Technologies**
   - **SQL vs NoSQL**: Choose the right database technology based on your data model and access patterns. Use SQL for structured data with complex queries, and NoSQL for unstructured data and horizontal scalability.

### Conclusion
Combining these practices will help you write safe, efficient, and high-performance database code. Always stay updated with the latest database technologies and practices, as this area continues to evolve.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.387s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3502 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.501s
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
  ✓ Text search completed in 0.042s
✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

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
    ✓ Generated embedding: 1536 dimensions in 0.363s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.617s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.309s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.488s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.646s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.472s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.587s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.515s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.557s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.550s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.451s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.569s
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
    ✓ Generated embedding: 1536 dimensions in 0.635s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.050s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.199] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.180] From conv_database_design
     What is database normalization?

  3. [Score: 0.180] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  4. [Score: 0.154] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.112] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves several key principles and best practices. Here’s a comprehensive guide to help you achieve that:

### 1. **Use Prepared Statements**
   - **Avoid SQL Injection**: Prepared statements separate SQL logic from data input. This not only prevents SQL injection attacks but also improves performance as the database can cache the execution plan.
   - **Example**: Instead of concatenating SQL strings, use parameterized queries.

### 2. **Connection Pooling**
   - **Reduce Connection Overhead**: Instead of creating a new database connection for each request, use a connection pool to reuse existing connections. This minimizes the overhead associated with repeatedly opening and closing connections.
   - **Implementation**: Most database libraries provide connection pooling mechanisms.

### 3. **Efficient Query Design**
   - **Optimize Queries**: Avoid unnecessary complexity in queries. Use proper indexing, and ensure that your queries are as selective as possible.
   - **Limit Data Retrieval**: Only retrieve the fields you need. Using `SELECT *` can lead to retrieving more data than necessary.

### 4. **Batch Processing**
   - **Reduce Round Trips**: When inserting or updating data, use batch operations to send multiple records to the database at once rather than one at a time.
   - **Performance Improvement**: This reduces the number of round trips to the database, thus improving performance.

### 5. **Indexing Strategies**
   - **Use Indexes Wisely**: Indexes can greatly improve read performance but can slow down writes. Analyze your read and write patterns and create indexes accordingly, ensuring not to over-index.
   - **Monitor Query Performance**: Utilize `EXPLAIN` plans to understand how queries are executed and whether indexes are being used.

### 6. **Memory Management**
   - **Watch for Memory Leaks**: Ensure that all database resources (like connections, cursors, and transactions) are properly closed after use.
   - **Use Transactions**: Group multiple operations that should be atomic using transactions to ensure consistency and to optimize resource usage.

### 7. **Error Handling**
   - **Graceful Handling**: Implement robust error handling to manage database exceptions gracefully, avoiding uncaught exceptions that can lead to resource leaks.

### 8. **Use ORM Wisely**
   - **Object-Relational Mapping (ORM)**: If using an ORM, be aware of how it handles SQL generation and performance optimization. While ORMs can make it easier to interact with dbs, they may introduce overhead, so understanding their inner workings is crucial.

### 9. **Asynchronous Operations**
   - **Non-Blocking I/O**: If applicable, use asynchronous database operations to avoid blocking your application's main thread, improving overall application responsiveness and performance.

### 10. **Profiling and Performance Monitoring**
   - **Continuous Monitoring**: Use profiling tools to monitor the performance of your database interactions and identify bottlenecks.
   - **Database Logs**: Analyze logs for slow queries and optimize them as needed.

### 11. **Concurrency Control**
   - **Handle Concurrent Access**: Implement concurrency control mechanisms to prevent issues like deadlocks and race conditions. This can include optimistic concurrency control or using appropriate isolation levels.

### 12. **Database Normalization**
   - **Organize Data**: Normalize your database schema to reduce redundancy and improve integrity, but balance this with performance, as excessive normalization can lead to complex queries.

By integrating these strategies, you can write code that is not only safe in terms of memory management but also performs efficiently in database interactions.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.350s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3767 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.408s
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
  ✓ Text search completed in 0.042s

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
    ✓ Generated embedding: 1536 dimensions in 0.438s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.564s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.392s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.394s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.313s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.439s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.519s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.358s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.364s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.568s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.375s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.281s
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
    ✓ Generated embedding: 1536 dimensions in 0.399s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.065s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.199] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.180] From conv_database_design
     What is database normalization?

  3. [Score: 0.180] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  4. [Score: 0.154] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.112] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ Context prepared from search results
✓ AI would use this context to generate comprehensive response

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.301s
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
  ✓ Text search completed in 0.039s

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

> @ekodb/ekodb-client@0.15.2 prepare
> npm run build


> @ekodb/ekodb-client@0.15.2 build
> tsc


up to date, audited 52 packages in 2s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.15.2 build
> tsc

✅ [32mTypeScript client built![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning TypeScript SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user (VNi0oxJsq1MksVbHI6d5qAxbTkaevBiFcL0krvC7S9QvKRBWVmZFspjcO6XBgVwhvzauEgco2_Pd3m4PaX6_Qw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
        },
        "address": {
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "city": "Gwenborough",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light"
        },
        "name": "Leanne Graham",
        "website": "hildegard.org",
        "phone": "1-770-736-8031 x56442",
        "username": "Bret",
        "email": "Sincere@april.biz",
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
Response time: 42ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
        },
        "address": {
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "city": "Gwenborough",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light"
        },
        "name": "Leanne Graham",
        "website": "hildegard.org",
        "phone": "1-770-736-8031 x56442",
        "username": "Bret",
        "email": "Sincere@april.biz",
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
✓ Created enrichment script: fetch_product_with_reviews (jfxO0OMPJPzlfyg-jXvl1a6-saSZs1uz-wIkOktz-GWiyVkTAm0yRNNi6oUMAQbzMMgKTnL5wqOyAXNQPCqB9w)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "shippingInformation": "Ships in 3-5 business days",
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "tags": [
          "beauty",
          "mascara"
        ],
        "weight": 4,
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "title": "Essence Mascara Lash Princess",
        "stock": 99,
        "sku": "BEA-ESS-ESS-001",
        "discountPercentage": 10.48,
        "rating": 2.56,
        "id": 1,
        "brand": "Essence",
        "availabilityStatus": "In Stock",
        "reviews": [
          {
            "reviewerName": "Eleanor Collins",
            "rating": 3,
            "comment": "Would not recommend!",
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com"
          },
          {
            "date": "2025-04-30T09:41:02.053Z",
            "comment": "Very satisfied!",
            "rating": 4,
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "reviewerName": "Lucas Gordon"
          },
          {
            "comment": "Highly impressed!",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins",
            "rating": 5,
            "date": "2025-04-30T09:41:02.053Z"
          }
        ],
        "returnPolicy": "No return policy",
        "meta": {
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
          "createdAt": "2025-04-30T09:41:02.053Z",
          "updatedAt": "2025-04-30T09:41:02.053Z",
          "barcode": "5784719087687"
        },
        "minimumOrderQuantity": 48,
        "dimensions": {
          "height": 13.08,
          "width": 15.14,
          "depth": 22.99
        },
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "price": 9.99,
        "category": "beauty",
        "warrantyInformation": "1 week warranty"
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
✓ Edge cache script created: hj0i1-Te0qli9yGafUAomcD1rvDJ_ZhTtBvAmsSxgsoD6_75q6_JXk0uonuQ8f2ceZU3WsT5f52Wr0TP8PypMg

Call 1: Cache miss (fetches from API)
Response time: 170ms
Result: {
  "records": [
    {
      "value": {
        "company": {
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "address": {
          "suite": "Apt. 556",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light",
          "zipcode": "92998-3874",
          "city": "Gwenborough"
        },
        "website": "hildegard.org",
        "username": "Bret",
        "email": "Sincere@april.biz",
        "name": "Leanne Graham",
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
Response time: 44ms (3.9x faster!)
Result: {
  "records": [
    {
      "value": {
        "company": {
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "address": {
          "suite": "Apt. 556",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light",
          "zipcode": "92998-3874",
          "city": "Gwenborough"
        },
        "website": "hildegard.org",
        "username": "Bret",
        "email": "Sincere@april.biz",
        "name": "Leanne Graham",
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
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.16s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.15.2-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.15.2-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
Successfully installed ekodb-client-0.15.2
✅ [32mPython client package built and installed![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (skTBEOeJmqPkK89_QOqP4aWdSR21tiOm7twDnNRtdh_gMvVrAMfC7ztOYWPNs-88GrlfghcJuY6NsBnRXavyGA)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 237ms
Result: [
  {
    "cached_at": "1775448028",
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
        "followers": 295020,
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
    "id": "torvalds"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 46ms (5.1x faster!)
Result: [
  {
    "cached_at": "1775448028",
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
        "followers": 295020,
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
    }
  }
]
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (kCIPIKik0c6-cajdVuX1G25RsxT6fpKAKItASqSueS9Hivz0y-iLcr0hM1d3Qg7aRjpwzXlyZfUdnS7EtjQyOw)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "1775448028",
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
    "id": "1"
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
✓ Created SWR script: fetch_github_user (h67Z7ASHSsalSv9IhfCrLLlZ3SLzg4PbaBtkdoczyXZoKytFbv5oA82pE_ve4zDSd0pmYqoYoKRO4RZ7XoJpow)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 42.762ms
Result: [
  {
    "cached_at": "1775448028",
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
        "followers": 295020,
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
    }
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 43.334833ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (esNDnGMXCldkKYhzLdmFqdjci0_yP4nuKXxyKVFctxlEmMZOO72Qo9sVeetgYfZl11wqBHCWrWerS8Elv0FKbA)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
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
    }
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (l2GGG-ppsqBdIseKNj0GUeoMvLOzhVzV9qKrK8wd35h2diCm2saAZ8XYgUKPlOg51ORS1RnNQ2LWyueXLFKARQ)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 44ms
Result: {
  "data": {
    "type": "Object",
    "value": {
      "following": 0,
      "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
      "updated_at": "2026-01-13T07:10:05Z",
      "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
      "public_gists": 1,
      "type": "User",
      "followers_url": "https://api.github.com/users/torvalds/followers",
      "organizations_url": "https://api.github.com/users/torvalds/orgs",
      "gravatar_id": "",
      "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
      "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
      "hireable": null,
      "url": "https://api.github.com/users/torvalds",
      "html_url": "https://github.com/torvalds",
      "user_view_type": "public",
      "node_id": "MDQ6VXNlcjEwMjQwMjU=",
      "repos_url": "https://api.github.com/users/torvalds/repos",
      "twitter_username": null,
      "bio": null,
      "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
      "email": null,
      "company": "Linux Foundation",
      "name": "Linus Torvalds",
      "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
      "location": "Portland, OR",
      "site_admin": false,
      "created_at": "2011-09-03T15:26:22Z",
      "public_repos": 11,
      "login": "torvalds",
      "blog": "",
      "followers": 295020,
      "received_events_url": "https://api.github.com/users/torvalds/received_events",
      "id": 1024025
    }
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 44ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (-nQnSASZWjarRFLCf6XzTqIUUaQhcdIp_Br3mzO9ci0mPCqdD-d_FENzfXMkiurANZ9KObXJxrHdVidhVdGLDA)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "enriched_data": {
    "value": {
      "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
      "dimensions": {
        "height": 13.08,
        "depth": 22.99,
        "width": 15.14
      },
      "warrantyInformation": "1 week warranty",
      "shippingInformation": "Ships in 3-5 business days",
      "images": [
        "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
      ],
      "rating": 2.56,
      "minimumOrderQuantity": 48,
      "availabilityStatus": "In Stock",
      "title": "Essence Mascara Lash Princess",
      "brand": "Essence",
      "price": 9.99,
      "weight": 4,
      "tags": [
        "beauty",
        "mascara"
      ],
      "sku": "BEA-ESS-ESS-001",
      "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
      "category": "beauty",
      "returnPolicy": "No return policy",
      "stock": 99,
      "reviews": [
        {
          "rating": 3,
          "comment": "Would not recommend!",
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerName": "Eleanor Collins"
        },
        {
          "comment": "Very satisfied!",
          "reviewerName": "Lucas Gordon",
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerEmail": "lucas.gordon@x.dummyjson.com",
          "rating": 4
        },
        {
          "comment": "Highly impressed!",
          "date": "2025-04-30T09:41:02.053Z",
          "rating": 5,
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "reviewerName": "Eleanor Collins"
        }
      ],
      "meta": {
        "updatedAt": "2025-04-30T09:41:02.053Z",
        "createdAt": "2025-04-30T09:41:02.053Z",
        "barcode": "5784719087687",
        "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
      },
      "discountPercentage": 10.48,
      "id": 1
    },
    "type": "Object"
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
✓ Created SWR function: fetch_github_user (JPMj0kpMZkpzsEo4y4MSY7SSnjUzvRfQGzkl7OgZqphZ71MTqAuP3Kb71BfOu_XUkO61_KH_yMzGaoiNAgsLig)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 56ms
Result: [{"data":{"value":{"blog":"","url":"https://api.github.com/users/torvalds","node_id":"MDQ6VXNlcjEwMjQwMjU=","login":"torvalds","subscriptions_url":"https://api.github.com/users/torvalds/subscriptions","events_url":"https://api.github.com/users/torvalds/events{/privacy}","following":0,"location":"Portland, OR","site_admin":false,"twitter_username":null,"bio":null,"id":1024025,"organizations_url":"https://api.github.com/users/torvalds/orgs","hireable":null,"starred_url":"https://api.github.com/users/torvalds/starred{/owner}{/repo}","html_url":"https://github.com/torvalds","type":"User","created_at":"2011-09-03T15:26:22Z","avatar_url":"https://avatars.githubusercontent.com/u/1024025?v=4","public_gists":1,"followers":295020,"user_view_type":"public","gists_url":"https://api.github.com/users/torvalds/gists{/gist_id}","email":null,"updated_at":"2026-01-13T07:10:05Z","company":"Linux Foundation","following_url":"https://api.github.com/users/torvalds/following{/other_user}","repos_url":"https://api.github.com/users/torvalds/repos","received_events_url":"https://api.github.com/users/torvalds/received_events","name":"Linus Torvalds","followers_url":"https://api.github.com/users/torvalds/followers","gravatar_id":"","public_repos":11},"type":"Object"},"cached_at":"1775448028"}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 46ms
Cache hit was 1.2x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment function: fetch_product_enriched (BBKX7c4NIJz-RWHAMuOxvsBdCaQ3LnAF7VAzKrRLMMvoRqUCOJZDVRqagsz-33xe2tDwOQ8adv-hl5-UR1jKjA)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"enriched_data":{"type":"Object","value":{"id":1,"price":9.99,"shippingInformation":"Ships in 3-5 business days","title":"Essence Mascara Lash Princess","meta":{"createdAt":"2025-04-30T09:41:02.053Z","updatedAt":"2025-04-30T09:41:02.053Z","barcode":"5784719087687","qrCode":"https://cdn.dummyjson.com/public/qr-code.png"},"tags":["beauty","mascara"],"warrantyInformation":"1 week warranty","images":["https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"],"discountPercentage":10.48,"dimensions":{"height":13.08,"depth":22.99,"width":15.14},"returnPolicy":"No return policy","minimumOrderQuantity":48,"sku":"BEA-ESS-ESS-001","stock":99,"rating":2.56,"weight":4,"description":"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.","reviews":[{"date":"2025-04-30T09:41:02.053Z","reviewerEmail":"eleanor.collins@x.dummyjson.com","rating":3,"reviewerName":"Eleanor Collins","comment":"Would not recommend!"},{"reviewerName":"Lucas Gordon","comment":"Very satisfied!","rating":4,"date":"2025-04-30T09:41:02.053Z","reviewerEmail":"lucas.gordon@x.dummyjson.com"},{"reviewerName":"Eleanor Collins","date":"2025-04-30T09:41:02.053Z","rating":5,"comment":"Highly impressed!","reviewerEmail":"eleanor.collins@x.dummyjson.com"}],"category":"beauty","availabilityStatus":"In Stock","brand":"Essence","thumbnail":"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp"}}}]
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


BUILD SUCCESSFUL in 5s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
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
   ⏱️  Duration: 161.743334ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "id": 1,
    "company": {
      "bs": "harness real-time e-markets",
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romaguera-Crona"
    },
    "websi...

Second call (cache hit - from cache):
   ⏱️  Duration: 42.162167ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "address": {
      "zipcode": "92998-3874",
      "city": "Gwenborough",
      "geo": {
        "lat": "-37.3159",
        "lng": "81.1496"
      },
      "street": "Kulas Light",
 ...
   🚀 Cache speedup: 3.8x faster!

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

> @ekodb/ekodb-client@0.15.2 prepare
> npm run build


> @ekodb/ekodb-client@0.15.2 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.15.2 build
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
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 42ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "bs": "harness real-time e-markets",
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romaguera-Crona"
    },
    "username": "Bret",...

Second call (cache hit - from cache):
   ⏱️  Duration: 43ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "bs": "harness real-time e-markets",
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romaguera-Crona"
    },
    "username": "Bret",...
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
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.16s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.15.2-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.15.2-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.15.2
    Uninstalling ekodb_client-0.15.2:
      Successfully uninstalled ekodb_client-0.15.2
Successfully installed ekodb-client-0.15.2
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
   ⏱️  Duration: 42.5ms
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
   ⏱️  Duration: 42.4ms
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
   ⏱️  Duration: 43.520958ms
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
   ⏱️  Duration: 42.410042ms
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
   ⏱️  Duration: 44ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "bs": "harness real-time e-markets",
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romaguera-Crona"
    },
    "username": "Bret",...

Second call (cache hit - from cache):
   ⏱️  Duration: 44ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "bs": "harness real-time e-markets",
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romaguera-Crona"
    },
    "username": "Bret",...
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
  ✓ Reusable Function building blocks
  ✓ Functions calling other Functions
  ✓ Clean SWR patterns via composition
  ✓ Multi-level nesting (arbitrary depth)
  ✓ No code duplication
  ✓ Single source of truth

[36mBuild complex workflows from simple pieces! 🚀[0m


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
📡 [36mWebSocket Subscription Tests[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[33mThese tests verify real-time WebSocket subscriptions by:[0m
  1. Authenticating and connecting via WebSocket
  2. Subscribing to a collection
  3. Inserting records via REST to trigger notifications
  4. Verifying MutationNotification push messages arrive
  5. Unsubscribing and cleaning up

🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
✅ [32mClient build complete![0m

🦀 [33mRust WebSocket Subscription Test...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_websocket_subscribe`
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_rs' ===
✓ Subscribed (subscription_id: sub_5b28a114d1144a7e8da89f98711b932e)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "_C6_CLSFvYKReGPgol43dA5y4wnGOCZ_OiZAZm_GBKQ7tjxQTVCxmClk-6Mo8doKtAsMaZuQgoSqFO7FWD3G0g"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_rs"
     Record IDs: ["_C6_CLSFvYKReGPgol43dA5y4wnGOCZ_OiZAZm_GBKQ7tjxQTVCxmClk-6Mo8doKtAsMaZuQgoSqFO7FWD3G0g"]
     Timestamp:  "2026-04-06T04:00:53.885485+00:00"

Inserting record 2...
✓ Inserted: "DOvEAvDiEihWbulHlwK7v3Ljw2jVFEKhQe03DWnag5dvMegnATY_HZPQa5nwcqh9q6wLZAuCzCuSDi8IuSGstg"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["DOvEAvDiEihWbulHlwK7v3Ljw2jVFEKhQe03DWnag5dvMegnATY_HZPQa5nwcqh9q6wLZAuCzCuSDi8IuSGstg"]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
✅ [32mRust subscription test complete![0m

🔷 [33mGo WebSocket Subscription Test...[0m
=== WebSocket Subscription Example (Go) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_go' ===
✓ Subscribed (subscription_id: sub_c8e586775c1645ccb2d6ecfc2bc3d569)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: dWqJeE0ZUieFzreCAT2XtajFc_fwAzDZlj6i7mud2f1dbRByxZWsLMi7-1GTbgwzonfmFVHENd5bk93OPXXTqA

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_go
     Record IDs: [dWqJeE0ZUieFzreCAT2XtajFc_fwAzDZlj6i7mud2f1dbRByxZWsLMi7-1GTbgwzonfmFVHENd5bk93OPXXTqA]
     Timestamp:  2026-04-06T04:00:54.036391+00:00

Inserting record 2...
✓ Inserted: nZKHndKh9mm9sQx6YffDmwB4YP15TN4FBQlFJzliQ2sFjDxM-DEDtTW2MJlEOfkRvrKZzRC1TLkh4GnbtcHLsQ

  📡 Notification received:
     Event:      insert
     Record IDs: [nZKHndKh9mm9sQx6YffDmwB4YP15TN4FBQlFJzliQ2sFjDxM-DEDtTW2MJlEOfkRvrKZzRC1TLkh4GnbtcHLsQ]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
✅ [32mGo subscription test complete![0m

🐍 [33mPython WebSocket Subscription Test...[0m
=== WebSocket Subscription Example (Python) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_py' ===
✓ Subscribed (subscription_id: sub_894e1f9cd40d4ffd86d303b79ab55466)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: Ygus-iWpzaqM684Ig61G7shoIhxosKmI5NFdLsLEgZvQqdDvcfBcc2rt9cB6ZEiLfSRtK9qTdU127vMoooV3XQ

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: Ygus-iWpzaqM684Ig61G7shoIhxosKmI5NFdLsLEgZvQqdDvcfBcc2rt9cB6ZEiLfSRtK9qTdU127vMoooV3XQ
     Timestamp:  2026-04-06T04:00:54.215526+00:00

Inserting record 2...
✓ Inserted: tMpWJdlbJW_pwU315gRIg4JyYiuGVECNyQ6d4-2gtbE8glsZHTPas0BkqjBil_j8yTeeeEHyvSKx6cnl3fa9TQ

  📡 Notification received:
     Event:      insert
     Record IDs: tMpWJdlbJW_pwU315gRIg4JyYiuGVECNyQ6d4-2gtbE8glsZHTPas0BkqjBil_j8yTeeeEHyvSKx6cnl3fa9TQ

=== Unsubscribing ===
✓ Unsubscribed: {'collection': 'ws_subscribe_example_py', 'found': True, 'unsubscribed': True}

✓ WebSocket subscription example completed successfully
✅ [32mPython subscription test complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.15.2 prepare
> npm run build


> @ekodb/ekodb-client@0.15.2 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.15.2 build
> tsc

✅ [32mTypeScript client built![0m

📘 [33mTypeScript WebSocket Subscription Test...[0m
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_a1ce94e9895d493eae41f83da7541051)

=== Listening for mutation notifications ===

Inserting a record...

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_ts
     Record IDs: qhpaNTk2wSR0J5ZminodYpd5iY0gwMJl28wyMsTYH-JUA5amkZi8EMcxJUoaJOKfQI-0d_mVQJozxu_JgHLWHg
     Timestamp:  2026-04-06T04:00:56.925408+00:00
     Records:    {"active":true,"id":"qhpaNTk2wSR0J5ZminodYpd5iY0gwMJl28wyMsTYH-JUA5amkZi8EMcxJUoaJOKfQI-0d_mVQJozxu_...
✓ Inserted record: qhpaNTk2wSR0J5ZminodYpd5iY0gwMJl28wyMsTYH-JUA5amkZi8EMcxJUoaJOKfQI-0d_mVQJozxu_JgHLWHg

Inserting another record...

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_ts
     Record IDs: tQ4dbagSKQeXXzHNItY5Gwfz9FI7h5fl3mXpWQgisBEHfUUvC2WQdlCGWOgwiuFKQ4LA3lxpcDXfi1ptSQODog
     Timestamp:  2026-04-06T04:00:57.934566+00:00
     Records:    {"active":true,"id":"tQ4dbagSKQeXXzHNItY5Gwfz9FI7h5fl3mXpWQgisBEHfUUvC2WQdlCGWOgwiuFKQ4LA3lxpcDXfi1p...

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✅ [32mTypeScript subscription test complete![0m

🟣 [33mKotlin WebSocket Subscription Test...[0m
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
=== WebSocket Subscription Example (Kotlin) ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_kt' ===
✓ Subscribed (subscription_id: sub_5662c88b24cc4cf1935c18626b09f6b1)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "TXe2v1U9_bJo3sLjaJfB-GeId9l9CTtnnE_Gm_K10RkPaTZ4abG8b30YtWpADxZPHs57pLxWXj90toSNNT2m4Q"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_kt"
     Record IDs: ["TXe2v1U9_bJo3sLjaJfB-GeId9l9CTtnnE_Gm_K10RkPaTZ4abG8b30YtWpADxZPHs57pLxWXj90toSNNT2m4Q"]
     Timestamp:  "2026-04-06T04:01:04.009683+00:00"

Inserting record 2...
✓ Inserted: "2WInblQDhQTCdhqoA5Vv0kXCVd6rR2Z4uQrspiSo-2UaLCI8IWUiBIwp35JiMGrijrdJ0UMKEsfdqQGMsqIDCA"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["2WInblQDhQTCdhqoA5Vv0kXCVd6rR2Z4uQrspiSo-2UaLCI8IWUiBIwp35JiMGrijrdJ0UMKEsfdqQGMsqIDCA"]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully

BUILD SUCCESSFUL in 5s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin subscription test complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
✅ [32mAll WebSocket Subscription Tests Passed![0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
