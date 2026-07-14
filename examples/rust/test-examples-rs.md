make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("qrTo89GVgAjgxY7lDPJUCw1XtKzRnpCWwiMRDhmCJOi_aBryOZWCROMgGyf5u5fNjyD4KT9iv5ptR4h134LAWA")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("qrTo89GVgAjgxY7lDPJUCw1XtKzRnpCWwiMRDhmCJOi_aBryOZWCROMgGyf5u5fNjyD4KT9iv5ptR4h134LAWA"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("qrTo89GVgAjgxY7lDPJUCw1XtKzRnpCWwiMRDhmCJOi_aBryOZWCROMgGyf5u5fNjyD4KT9iv5ptR4h134LAWA"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("qrTo89GVgAjgxY7lDPJUCw1XtKzRnpCWwiMRDhmCJOi_aBryOZWCROMgGyf5u5fNjyD4KT9iv5ptR4h134LAWA"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "YzqB7vN0r4D-3zhsgSJ8CEDDI_9gRK0zRHJnV7qaDbIYbCjfXHmDa2JMO7MiFFVbRGFPw8PepqVJ69RV3PvSHQ"

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1784036740898172000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "YzqB7vN0r4D-3zhsgSJ8CEDDI_9gRK0zRHJnV7qaDbIYbCjfXHmDa2JMO7MiFFVbRGFPw8PepqVJ69RV3PvSHQ",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/kv_operations`
✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: Object {"type": String("Object"), "value": Object {"userId": Number(123), "username": String("john_doe")}}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: Object {"type": String("Object"), "value": Object {"name": String("Product 1"), "price": Number(29.99)}}
cache:product:2: Object {"type": String("Object"), "value": Object {"name": String("Product 2"), "price": Number(39.989999999999995)}}
cache:product:3: Object {"type": String("Object"), "value": Object {"name": String("Product 3"), "price": Number(49.989999999999995)}}

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
Collection created with first record: "Ll1PVeMVvNNKT4hsdrJh33jDls040xuUavnJR-gYt-amEvkAfnZHurH9F7304wB2DhMSLyqmv1AxA-gyJRdf5g"

=== List Collections ===
Total collections: 5
Sample collections: ["audit__ek0_testing", "test_collection", "websocket_test", "batch_users", "demo_collection"]

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
✓ Inserted document: "Upv053ke_HD46KNglN78qaTTgYlbpG_W3BXJ5j1haYL--qUzFtiYIRG9flyu19Xsmmv1w1Pndil8NsODCNu5IA"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "GooeXV7TU2lJp4S3Xb_eDsr6OxFEn_e8598QXqHdpYXInWrSsHSQqGCQnwqMvhNUjiGEZMXxGNAA6jQ_NmruhA"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "w0aADw3ebt08qpp2pFT5TDt6ICy-0TO6kPyVuKdlzupBzm4J6qy-1c1TWaujqZn8Tlkz-0f4Bst-DP72ZGU70g"

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
✓ Inserted document with TTL: String("p6zrhnYfdWu4OUccgbmJJdcRGVAuJzMW_qEpBjhlc4ixiKUleCqKhdP-jgYe6wi3_V5jwTAPybQtw1qPuqjQ7Q")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "p6zrhnYfdWu4OUccgbmJJdcRGVAuJzMW_qEpBjhlc4ixiKUleCqKhdP-jgYe6wi3_V5jwTAPybQtw1qPuqjQ7Q"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-07-14T14:45:42.001156Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Functions Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: rUDNvN39QPwt_K8gPzfgwZDG2oZJryq0MZ8ffhSAw7tpRE3M90Hw0toAXig-Tmjzztvq3U1SuU9z5QARYve7Zw
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: JmWE1dn-xlav3t2i1_EbUhLVEJquXDFrDkjG_xB9EfY7MhmwVeEbyTGep0opA9A-38Fc7E1uvxZgqOfPDKD_IQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Function saved: j77bzUyDGsJsfCLG34aXu8YdE7Vvk2a2R3hMmy7TZyfcguyrw_LL2YiHEJcsyXWibJ-eYpXbfLQkw018l2ymEw
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: 16ojxwXbTCOePs0wAizty7egl0Gc-ntTsXg8_QTZFCPNQRU_0X-SP9s2uUOnPZyDUI7wsTpSkHZfErKdwQF6Fw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":{"type":"Float","value":60.0},"count":{"type":"Integer","value":5},"max_score":{"type":"Integer","value":100},"status":{"type":"String","value":"active"}}
   {"avg_score":{"type":"Float","value":50.0},"count":{"type":"Integer","value":5},"max_score":{"type":"Integer","value":90},"status":{"type":"String","value":"inactive"}}

📝 Example 4: Function Management

📋 Total scripts: 4
🔍 Retrieved script: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: HsLhcLjOK5SU8dMzKMRSYXdSrBl0kxYSxY_NOo3H3F8x7zl-LecXANpcksepe5JY-YNf5t4VpjFksi5K2QnX-A
Created Bob: $500 - ID: cJwzeG9hprQKxKAa0givWPjXTUYqE3EmlSJRIbvJW5Qjw9JdDKw4LnhnyLZdGZcRgSJVSTWj6vYxDiVDvVxdAA

=== Example 1: Begin Transaction ===
Transaction ID: b84452b6-0938-4538-b0ec-e75e743b93a7

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
New transaction: 5625716b-576d-44e6-90d2-d5ba36b5be22
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("s4wDqLzsw_yLYxLev9jHsKQ8Rkk-T1_w2My1D7Fk907zX-hnjW-GkmwiqltAnKBgIXdQ-Y6fE6BRhew-WQZ8zw")})

=== Find by ID ===
Found: Record({"data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-07-14T13:45:42.982112+00:00")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "name": Object({"type": String("String"), "value": String("Test Record")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "id": String("s4wDqLzsw_yLYxLev9jHsKQ8Rkk-T1_w2My1D7Fk907zX-hnjW-GkmwiqltAnKBgIXdQ-Y6fE6BRhew-WQZ8zw"), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-07-14T13:45:42.982112+00:00")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"created_at": Object({"value": String("2026-07-14T13:45:42.982112+00:00"), "type": String("DateTime")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "name": Object({"type": String("String"), "value": String("Test Record")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "id": String("s4wDqLzsw_yLYxLev9jHsKQ8Rkk-T1_w2My1D7Fk907zX-hnjW-GkmwiqltAnKBgIXdQ-Y6fE6BRhew-WQZ8zw"), "active": Object({"value": Boolean(true), "type": String("Boolean")})})]

=== Update Document ===
Updated: Record({"data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "name": Object({"type": String("String"), "value": String("Updated Record")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "value": Object({"type": String("Integer"), "value": Integer(100)}), "price": Object({"value": Float(99.99), "type": String("Float")}), "id": String("s4wDqLzsw_yLYxLev9jHsKQ8Rkk-T1_w2My1D7Fk907zX-hnjW-GkmwiqltAnKBgIXdQ-Y6fE6BRhew-WQZ8zw"), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "created_at": Object({"value": String("2026-07-14T13:45:42.982112+00:00"), "type": String("DateTime")})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: EQvyqTt-rqsF3VwFtUEUTQ6B5zs3qGhEKt4rFiWnyHDzcGTP2xO7dM-rbM_AQteNB-K88stuoWTIwPR3ZG1pyA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  active: true
  id: "EQvyqTt-rqsF3VwFtUEUTQ6B5zs3qGhEKt4rFiWnyHDzcGTP2xO7dM-rbM_AQteNB-K88stuoWTIwPR3ZG1pyA"
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "gtcf9kGKpmkkTZFpp6OMux-4Rep6DOKnJMhXh0v5NL4PyWa6VGUktNqXKohgMliBxWj-c0z97I9IVOS-6np_2w"

=== List Collections ===
Total collections: 11
Sample collections: ["client_collection_management_rust", "audit__ek0_testing", "functions__ek0_testing", "users", "test_collection"]

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
Retrieved value: Object {"type": String("Object"), "value": Object {"userId": Number(123), "username": String("john_doe")}}

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: Record({"price": Float(29.99), "name": String("Product 1")})
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
Created Alice: $1000 - ID: H6NcHrZYZFb0ufjkcYaDRQywFLK7AqXrzOMHEgIlW06P2BWvlfOs8MJmE0zbk30ZTb1SFnAx0FeM612SIv-4MA
Created Bob: $500 - ID: m1jbQjLZZe9bYkcJ5rOuWJqQVYHxW2yOQWkM9RqZaTNdMEMf7lwB71N8RtBZkwaJ8uTHVTj8kn3RF7BM1OYVkQ

=== Example 1: Begin Transaction ===
Transaction ID: 860890b9-70d9-4b34-a494-54a88d8dc859

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 6113caf7-f4ec-4e48-aaa0-08c8e1e71a02
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
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

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

=== Prefix Query (StartsWith) ===
✓ Found 1 users with names starting with A
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"type": String("String"), "value": String("Charlie")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
  2. Some(Object({"type": String("String"), "value": String("David")}))
  3. Some(Object({"type": String("String"), "value": String("Alice")}))

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
  1. Score: 26.4000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 13.2000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})
  3. Score: 13.2000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})

=== Fuzzy Search ===
✓ Found 3 results for 'progamming' (typo)
  1. Score: 2.2000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 1.1000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})
  3. Score: 1.1000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})

=== Field-Specific Search ===
✓ Found 4 results in title/description
  1. Score: 2.0000
     Title: Some(Object {"type": String("String"), "value": String("Machine Learning Basics")})
     Matched: ["title", "description"]
  2. Score: 1.0000
     Title: Some(Object {"type": String("String"), "value": String("Python for Data Science")})
     Matched: ["description"]
  3. Score: 0.5000
     Title: Some(Object {"type": String("String"), "value": String("Rust Programming")})
     Matched: ["description"]
  4. Score: 0.5000
     Title: Some(Object {"type": String("String"), "value": String("Database Design")})
     Matched: ["description"]

=== Weighted Search ===
✓ Found 2 results with field weights
  1. Score: 23.1000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})
  2. Score: 3.3000 - Some(Object {"type": String("String"), "value": String("Machine Learning Basics")})

=== Advanced Search Options ===
✓ Found 1 results with stemming
  1. Score: 26.4000 - Some(Object {"type": String("String"), "value": String("Database Design")})

=== Search with Limit ===
✓ Limited to 2 results (requested 2)
  1. Score: 26.4000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 13.2000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})

=== Search with a metadata pre-filter (category = programming) ===
✓ Found 2 results in category 'programming' (database/ai excluded)
  1. Some(Object {"type": String("String"), "value": String("Rust Programming")}) (category: Some(Object {"type": String("String"), "value": String("programming")}))
  2. Some(Object {"type": String("String"), "value": String("Python for Data Science")}) (category: Some(Object {"type": String("String"), "value": String("programming")}))

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
✓ Inserted user 1: Some(String("139Ni5gHV1B7HqVsecixULGwzWc7eTBU40rNV1o75OIu82VgjxbRqgmRApIDxJz2_jNJT1KqKNvi4Dmg6YoQuQ"))
✓ Inserted user 2: Some(String("ZGrCamz56Xd2T1-cXcyTr27cvZu-GCf2cGIPBxP6_lEcTWThI7LZ7s_cGbEq60of7s7tgIcaNF4XtjzWX8MzUw"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - status: String
  - age: Integer
  - email: String
    (required)
  - title: String
    (required)

=== Listing Collections ===
✓ Total collections: 11
  Sample: ["audit__ek0_testing", "schema_client_rust", "functions__ek0_testing", "users", "test_collection"]

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
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

=== Example 5: Complex join with multiple conditions ===
✓ Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✓ Deleted test collections

✓ Join operations example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "zybiYWKvLgjtx0XqKOnckVvTAalyNr9iMnGpYojzgu7D0MQFxmUvIY34p9vzcLSaL2YXSvIe6g4e1VXYSxbZdg"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("Zte7e8NdtCO6gugO6FwMC99CqyoB7SGuypbcnQ5WRQW8MSFOYXrN8bsxHWnzi61l-dxDWqcKln46_bQAjzfrzA"))

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
✓ Inserted document with TTL: Some(String("S13h3gKLqMU4WHmvVbWh_VmQtZkegjVDFTOEV0NNDG-u2vcNwzrO4Qkxa8WRksaG0aY0W248fgYDD0FTQefgUw"))

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
✓ Edge cache script created: rmiOJYOTCdHd2BKMI9MEnjhE7FXCXZdEsIDX1uBf5KozGz-ZjOwc-zkYRC5xsvsZI7GAzORq2h_WnbKVk_ESvg

Call 1: Cache lookup
Response time: 2ms
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
=== ekoDB Rust Client - Functions Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

ℹ️  Function 'get_active_users' already existed — updated instead
✅ Function saved: T0TGdhj6fuWangYJ3xPntowiZnBsyR8iKSSHKxaa-Nrp-8Apt6soCv2iKMP4cC-jyt5UtACtP-3ynTT4JUb5WQ
📊 Found 20 active users

📝 Example 2: Parameterized Function

✅ Function saved: bo_lTGoaipjI7lWBtGUgTIUknvnh738P2wXwvy8Y3aHikBKh0k2qewX-6VurktgUjCAbnMhQjM1oMjSoPCC8tQ
📊 Found 20 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: QAQhmSmSRTiOD2jSyOcdliSikRUG64L9f0VtCW_xidOPJe-CjaSmAKOzDUoEOeC30OksZmj3NF20rq46w2px3w
📊 Statistics: 2 groups

📝 Example 4: Function Management

📋 Total functions: 5
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

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
   ⏱️  Duration: 154.20725ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "username": "Bret",
      "address": {
        "street": "Kulas Light",
        "suite": "Apt. 556",
        "geo": {
          "lng": "81.1496",
          "lat": "...

Second call (cache hit - from cache):
   ⏱️  Duration: 2.621917ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "company": {
        "name": "Romaguera-Crona",
        "bs": "harness real-time e-markets",
        "catchPhrase": "Multi-layered client-server neural-net"
      }...
   🚀 Cache speedup: 77.0x faster!

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
🚀 ekoDB Rust Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: U5S5z0aYURWlm3LiwH8Cw8TkozJbMHx0xFwLjoWk4J0yF587y92rJHqPlZ0GeI3NLyDT7hoCO96QN4Syc7kUzw
📊 Found 2 product groups
   Record({"avg_price": Object({"type": String("Float"), "value": Float(474.0)}), "count": Object({"value": Integer(2), "type": String("Integer")}), "category": Object({"value": String("Furniture"), "type": String("String")})})
   Record({"category": Object({"value": String("Electronics"), "type": String("String")}), "avg_price": Object({"type": String("Float"), "value": Float(575.6666666666666)}), "count": Object({"type": String("Integer"), "value": Integer(3)})})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   Record({"category": Object({"type": String("String"), "value": String("Furniture")}), "count": Object({"value": Integer(2), "type": String("Integer")})})
   Record({"count": Object({"type": String("Integer"), "value": Integer(3)}), "category": Object({"value": String("Electronics"), "type": String("String")})})
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("BSpea666s4RArSC24y4foPZVMZsbvMOFFYxAxesPl7YP5nu8Jd9OzYLa78ShuyXpg8au0QE8B4S5_nqEZr9V4w"))
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: EKNfDQ5t8HDy8yRDKpH7wW1TWRtzUX6AO1zHd7viELjTtIga1ps0BcjPZAofsMLP5QICgtxbOqTrJbiB2QKS2w
📊 function executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"type": String("Object"), "value": Object {"role": String("admin"), "userId": String("user_abc")}})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: toXIIhjxxY5mxbBK4ffCy3NaYDC8qn-qFAbdqSSs3w0uwfsrk2tl7lqOPWa4UV9PPY6vDw9fHDSLpN2HoENaoQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: tZfGI746HCFNMkqIB8YNia4_dDaShqc-xa7KwAqxP5BKU_jjtHynSwDL2HWO8Z9IopiGeA0Tbjy7X3eB1rm4kQ
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
✓ Created SWR script: fetch_api_user_rs (hjZnXJiYACRzaBF4B-2BA3Vj5a38Keco2ptZUZYMer9K4MQp1fLZsxV1kWoKRpvNryKEzEJVsuaZkDJW01y8YQ)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native (9LT0MljLoGedtIeIRNm8gWMYUd1L0LWTvuTpXEtyVQW1Cwpp0BQkx4NO1v1OpLceOJT6P7cTpT1_0G1mmIkz2Q)

First call (cache miss - will fetch from GitHub API):
  Response time: 110ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 36.7x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (BpDsk-MXKZz4E20-7wDhRnUPot4MIjq79ZcoIy6i_K_0r23g-4rKiUVD9VqOz1u2OptlO6yejCY_bltLlw9dAw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (48_8t5Z4gx259PIYWP_vqa6yowQlwNzPEoAbV1pZXFUqhBxHl6Ekke9TDBg5-iJEdF8KzruA1aNOLApc7o1eJg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (wiaZeF7awJFkLkvcTivsFRPNk4QTV6i0YBARzbka20Ida-cLGkkbpQgeyjboHP90R7H5nxsPCU5R3Rm9cIbQvQ)
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
   Record({"avg_price": Object({"type": String("Float"), "value": Float(365.6666666666667)}), "category": Object({"type": String("String"), "value": String("Furniture")}), "count": Object({"value": Integer(3), "type": String("Integer")})})
   Record({"count": Object({"type": String("Integer"), "value": Integer(5)}), "avg_price": Object({"value": Float(367.0), "type": String("Float")}), "category": Object({"type": String("String"), "value": String("Electronics")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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
   Record({"count": Object({"value": Integer(3), "type": String("Integer")}), "status": Object({"type": String("String"), "value": String("inactive")})})
   Record({"count": Object({"value": Integer(7), "type": String("Integer")}), "status": Object({"value": String("active"), "type": String("String")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Introduction to Machine Learning (AI)
   2. Vector Databases Explained (Database)
   3. Natural Language Processing (AI)
   4. Getting Started with ekoDB (Database)
   5. Database Design Principles (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   Record({"category": Object({"type": String("String"), "value": String("AI")}), "count": Object({"type": String("Integer"), "value": Integer(2)})})
   Record({"count": Object({"value": Integer(3), "type": String("Integer")}), "category": Object({"type": String("String"), "value": String("Database")})})
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
✓ Created session: 4cAFSBdJJg7EuCQq2Ao6n32foY6PGCN4T2aMFWJYwyex8-q_GdJf3sNqxjJkVUQeU7a3f_cFD2cPPseckCeFmA

=== Sending Chat Message ===
Message ID: d-VbZKUqBjv8sylFbjfoFe894LKpcG3-7g6ZKO_-yQ8v3rOQx5WTay5LAYtN7xHcjAD0-6QhsC4ZHxFAQbVI3g

=== AI Response ===
Response 1: ekoDB is a high-performance database that integrates intelligent caching, real-time capabilities, and AI features. It is designed to provide efficient data management and retrieval, supporting various use cases.

### Features of ekoDB:
1. **AI Chat Integration**: The chat feature enables querying of the database using natural language, facilitating AI-powered responses with relevant context.
2. **Search Capabilities**: ekoDB supports full-text search, vector search, and hybrid search, which includes automatic context retrieval for more relevant search results.
3. **Real-Time Capabilities**: The database is optimized for real-time data processing and management, enhancing user experience and performance.

For more detailed information, you can explore the documentation provided within the ekoDB platform.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["title", "content", "category"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("jxS2VR_C4oHCn2uwwV3uXFXjeifhrThaF83NJQcawVNtkZ62hpruI9ayQPKlxdPbysS-8B9Bx-jCBLdGF1g8Pg"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["category", "title", "content"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("iUCd7I7ETm8on5Uy800XrpFLRar46ncX5TvEropul_YpYx1Wo9mv_GbYfzmzaVvnC9RMjKbQP4DGgL1lfjIkAw"), "title": String("Search Features")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("bv2bGg4AgXMvt2Yu9Joxy-Hcq41t40sqpU_TQg5FTkfd1Kvc43K3_7TehkEx01U6xQmyCXy0TFvdy3erpVOg5Q"), "title": String("Introduction to ekoDB")}

Execution Time: 12945ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.63s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: PMHxcN8bcCXX1qI7nAmpZrgQnSg88KF9POA5BgHrbq0Ahy86L1egIp_74CSo7tMhejSPvN1j5-MrjKbtXWoRWQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you have any further questions or need more information, let me know!

✓ Second message sent
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you need more information or have further questions, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: 5XWDr5Je240IcJaiceVUokJtYGILqDmFRj1JzwUZ7wBcK6AeJfVfM1f8XzRsKua-1h0Ce7W-49WlHtk9TUv1Qw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: lz_AbI9DujI6oJ2KcgwO3Kz7MU-2mDEK20c0uMa_BZgeqvjMxtVGhGkWlfgRN30ecMd8SZ2Q1f61D05626W-mA
=== Sending Messages ===
✓ Message 1 sent
  Response: It looks like there are currently no specific product records available in the database for the query. However, based on the context you provided, one product mentioned is **ekoDB**, which is priced at **$99** and described as a high-performance database product with AI capabilities.

If you need more details or information about other products, please let me know!

✓ Message 2 sent
  Response: The price of **ekoDB** is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: mcLF76DEV7Mx70Nl_U648BC-7WGpi9wtK8ehDrpajVDGZtf_P9GbWpK8vAbsFHsm6od7X9ntYyvJMykNdTbDzg
  Parent: lz_AbI9DujI6oJ2KcgwO3Kz7MU-2mDEK20c0uMa_BZgeqvjMxtVGhGkWlfgRN30ecMd8SZ2Q1f61D05626W-mA

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: mcLF76DEV7Mx70Nl_U648BC-7WGpi9wtK8ehDrpajVDGZtf_P9GbWpK8vAbsFHsm6od7X9ntYyvJMykNdTbDzg (Untitled)
  Session 2: lz_AbI9DujI6oJ2KcgwO3Kz7MU-2mDEK20c0uMa_BZgeqvjMxtVGhGkWlfgRN30ecMd8SZ2Q1f61D05626W-mA (Untitled)
  Session 3: 5XWDr5Je240IcJaiceVUokJtYGILqDmFRj1JzwUZ7wBcK6AeJfVfM1f8XzRsKua-1h0Ce7W-49WlHtk9TUv1Qw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: mcLF76DEV7Mx70Nl_U648BC-7WGpi9wtK8ehDrpajVDGZtf_P9GbWpK8vAbsFHsm6od7X9ntYyvJMykNdTbDzg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_models`
✓ Client created

=== Get All Chat Models ===
Available chat models by provider:

OpenAI models (125):
  - text-embedding-ada-002
  - whisper-1
  - gpt-3.5-turbo
  - tts-1
  - gpt-3.5-turbo-16k
  - gpt-4-0613
  - gpt-4
  - davinci-002
  - babbage-002
  - gpt-3.5-turbo-instruct
  - gpt-3.5-turbo-instruct-0914
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
  - omni-moderation-latest
  - omni-moderation-2024-09-26
  - o1-2024-12-17
  - o1
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
  - gpt-5.4
  - gpt-5.4-nano-2026-03-17
  - gpt-5.4-nano
  - gpt-5.4-mini-2026-03-17
  - gpt-5.4-mini
  - gpt-image-2
  - gpt-image-2-2026-04-21
  - gpt-5.5
  - gpt-5.5-2026-04-23
  - gpt-5.5-pro
  - gpt-5.5-pro-2026-04-23
  - chat-latest
  - gpt-realtime-translate
  - gpt-realtime-2
  - gpt-realtime-whisper
  - gpt-5.6-sol
  - gpt-5.6-terra
  - gpt-5.6-luna
  - gpt-realtime-2.1
  - gpt-realtime-2.1-mini

Anthropic models (10):
  - claude-sonnet-5
  - claude-fable-5
  - claude-opus-4-8
  - claude-opus-4-7
  - claude-sonnet-4-6
  - claude-opus-4-6
  - claude-opus-4-5-20251101
  - claude-haiku-4-5-20251001
  - claude-sonnet-4-5-20250929
  - claude-opus-4-1-20250805

Perplexity models (0):

=== Get Models for Specific Provider ===

openai models (125):
  - text-embedding-ada-002
  - whisper-1
  - gpt-3.5-turbo
  - tts-1
  - gpt-3.5-turbo-16k
  ... and 120 more

anthropic models (10):
  - claude-sonnet-5
  - claude-fable-5
  - claude-opus-4-8
  - claude-opus-4-7
  - claude-sonnet-4-6
  ... and 5 more

perplexity models (0):

✓ Chat Models API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_user_functions`
✓ Client created

=== Create User Function ===
Created user function with ID: mEPKN9FuwsHbZs4oC7Ea5bsv84ZxfP84HPAMRZxMwvxI4LzCr718bo6sMEoCqoy3FYkqfrZ5NMmgfXhyrBV4Pg

=== Get User Function ===
Retrieved: get_active_users_rs - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 12 user functions:
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_high_scoring_active_users: Get High Scoring Active Users
  - get_verified_user: Get verified and validated user
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_active_users_updated: Get Active Users (Updated)
  - get_users_by_status: Get Users By Status
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_slim_user: Validate and slim down user
  - get_active_users_rs: Get Active Users

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("DTOfHcMygaT1mpveq-MN4DzxJYjKWIpKNgI278EUspQvBvh7MKwYnOp2fzJjqKWGlMdiStkSBhRPbtL0xdnXww")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("WzwO8QsByDX3oC79wzjL8UCzjLJSSBKUAV8X4Utv6FL9KINmS9tWphctMTLer1Quq-W8YUpsHjHDFqnnl6BOCw")})
✓ Second upsert (update): Record({"id": String("edtyImtEEH1zK9YXGokDbq0yyd8qNbPLtXujewbGEjh70OdgQz2MNHKExjjZNAMCEZMr_o9zC805tb54ECKJ7A")})

=== Find One Operation ===
✓ Found user by email: Record({"email": Object({"value": String("alice@example.com"), "type": String("String")}), "id": String("DTOfHcMygaT1mpveq-MN4DzxJYjKWIpKNgI278EUspQvBvh7MKwYnOp2fzJjqKWGlMdiStkSBhRPbtL0xdnXww"), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "name": Object({"type": String("String"), "value": String("Alice Johnson")}), "age": Object({"type": String("Integer"), "value": Integer(28)})})
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/bypass_ripple_example`
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: Record({"id": String("7FXo2E9t4n--cZq0_gN-w9rCBbqXVjIrISHWVaIN_mVEUevmLo-Ze16YM5J5O90uf2pYH9_EbSKV1uR0UrnGHg")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("KCDOhf-8doX8HaDIR4lrEB8F-R_bWmV-iflLYRyHoUj9W80gFlxuPX3jkRNaLl2zO_bWZzccykrSDW9fUX-DXw")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"id": String("7FXo2E9t4n--cZq0_gN-w9rCBbqXVjIrISHWVaIN_mVEUevmLo-Ze16YM5J5O90uf2pYH9_EbSKV1uR0UrnGHg"), "price": Object({"type": String("Integer"), "value": Integer(150)}), "name": Object({"type": String("String"), "value": String("Product 1")})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("_0OGY48DoioeIcHe1T74XrNkl4shQGzqbVHXNLLtZfkEPNvl00Z4g7jVKuYHpmaQhX5rNUAhmvqatnfScePCCw")})

✅ All bypass_ripple operations completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/projection_example`
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["id", "email", "name"]

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["bio", "created_at", "user_role", "id", "avatar_url", "email", "name", "status", "age"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["user_role", "avatar_url", "id", "secret_token", "name", "age", "password", "status", "bio", "api_key", "email", "created_at"]
  Projected query:
    - 3 fields per record
    - Fields: ["id", "email", "name"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/examples/client_jwt_auth_flow`
✓ Client created
✓ rs_users_register saved
✓ rs_users_login saved
✓ rs_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/rs_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/rs_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/rs_users_verify_token { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking — the {{env.JWT_SECRET}} placeholder reads from that whitelist, NEVER from the function definition.

✓ Cleaned up demo functions
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/examples/client_crypto_stages`
✓ Client created
✓ crypto_demo_hmac saved
✓ crypto_demo_aes saved
✓ crypto_demo_uuid saved
✓ crypto_demo_totp saved
✓ crypto_demo_encoding saved

All crypto-stage demos defined. Invoke any of them with:
  POST /api/functions/crypto_demo_hmac     { "payload": "hi" }
  POST /api/functions/crypto_demo_aes      { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid
  POST /api/functions/crypto_demo_totp
  POST /api/functions/crypto_demo_encoding { "title": "Héllo World" }

✓ Cleaned up demo functions
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_concurrency_stages`
✓ Client created
✓ conc_demo_pay saved
✓ conc_demo_rl_fail saved
✓ conc_demo_rl_skip saved
✓ conc_demo_lock saved

Invoke them like:
  POST /api/functions/conc_demo_pay        { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail    { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip    { "user_id": 42 }
  POST /api/functions/conc_demo_lock       { "resource": "queue:drain" }

✓ Cleaned up demo functions
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/examples/client_path_routed_function`
✓ Client created
✓ rs_route_admin → GET /api/route/users/admin
✓ rs_route_user_by_id → GET /api/route/users/:id
✓ rs_route_user_posts → GET /api/route/users/:id/posts/:post_id
✓ rs_route_org_create_member → POST /api/route/orgs/:org/members

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions
✅ [32mRust client examples complete![0m
