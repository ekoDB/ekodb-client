make test-examples
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
     Locking 1 package to latest compatible version
    Updating ekodb_client v0.6.1 (/Users/tek/Development/ekoDB/ekodb-client/ekodb_client) -> v0.7.0
   Compiling ekodb_client v0.7.0 (/Users/tek/Development/ekoDB/ekodb-client/ekodb_client)
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.72s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("nBDCw92VP1W-mWciZTzWsvV4blgv9PSUT4grzew28I9ZpP6XUIJIGERyvcnuE3CPvgBavhhKE6rS8rZ7ZjeF-Q")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("nBDCw92VP1W-mWciZTzWsvV4blgv9PSUT4grzew28I9ZpP6XUIJIGERyvcnuE3CPvgBavhhKE6rS8rZ7ZjeF-Q"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("nBDCw92VP1W-mWciZTzWsvV4blgv9PSUT4grzew28I9ZpP6XUIJIGERyvcnuE3CPvgBavhhKE6rS8rZ7ZjeF-Q"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("nBDCw92VP1W-mWciZTzWsvV4blgv9PSUT4grzew28I9ZpP6XUIJIGERyvcnuE3CPvgBavhhKE6rS8rZ7ZjeF-Q"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.81s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "IjdISjptT6bysMqTDvn82myude-F1EHII_2b_ly8-sMxVL2QcMVlBbQPhso4ZGmgB5bQBHKla7FbAnTqScFsJw"

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
        "id": "IjdISjptT6bysMqTDvn82myude-F1EHII_2b_ly8-sMxVL2QcMVlBbQPhso4ZGmgB5bQBHKla7FbAnTqScFsJw",
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.57s
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.54s
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.54s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "tMxPwjPxjAbdPRJPLgMnM3Zj1adee61B8v2RzkS10mTshg884HNWthC5Wf0oMMP132-UNVPhZSHqnhBvn2f1-A"

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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.52s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "xmanNAKrAQQXRz6QziWlZ4cSLW33jO6McNAwmdgHajRjLB4if3hKcvKCrG_XFl7QsnUoGGtc79-1ncBZ9oY5ug"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "zNKPQfdrV1-nXLoKSbgXIRDSAVWzvK-sgBuNJlH7GEmEJqzONUlj4PUPafJtFTZbhxS457E_37r8tahMu8aNSg"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "WgXjSAfD6xjjFU5XYVkYvLo5qzmSmG3hxcEavphwmMd-92HpGP-GrgtlPqaWoGq5m9lEolE2y-vySC_b_UuwCQ"

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.85s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("lt9BQWuymp0q9cFgy7IRuhrCeOmmSUhrkbpdpzzrqtBgrtJkktfY-IiVymLqLGK4prpu3r2lfxELJnoehMFOFA")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.63s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: RSOGn1hEse-7vkScXRn4uOznNhsgVmQ2Ql6EGPnoNIX_GdVXfU4anSVWK9VaWjb97dj2ipWJHsLPUkJUPR0aHQ
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "zVqPRmi1HWVeoRfL9_rszBK9oudmaOjJiM7mNedyLcxRCoRnmEXwhJDektWDkHD3eva1N3Fql6dFbl6FgK26Jg"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: 8fvSGlo6c0QqqgOvEa1dbQXM8gz5ApzeQx5j2X4mku3RFnaTna4BNU3HV4n2O2kXkwfbucbn7yEWDeAX-7zq3g
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 3RUCbv5So2hAqB2PqwismC7mzi9MpoHy1hlb_qLkcEC7jZNcrJcOjzeu-A8t_2cZtPjaJDDiNf8gslSDgLhC1Q
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.55s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: rSO-V4UG8BNUEwLow9YWGGLvEYkHKBp0dxoN4J02hNjpZ2TAC3iHXV1LE4TWV8sRGRXqUmXR7RoF8UvsfLFAwg
Created Bob: $500 - ID: USDjZs7EAgxe1CT-Fm-qdqjkoZZ1ni_GTtyOJWMZqeoa-eJncU-GdFS6hrQzbtZBQjE-ps4gnroYwdNSVG1Xcg

=== Example 1: Begin Transaction ===
Transaction ID: 5de04620-6080-4525-92e9-577927b185b9

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
New transaction: cf8c9567-c39b-48e6-89e7-906e53ee0ac3
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.09s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record { fields: {"id": String("IkYnZ7qEAjprBp8rSetrpBgTmLgJNe6roh5My-W8v4qH-s64NjzEi-nWGnpieJWOjCExE7BAVAvZNZD7D56mSQ")} }

=== Find by ID ===
Found: Record { fields: {"value": Object({"value": Integer(42), "type": String("Integer")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "created_at": Object({"value": String("2026-01-03T09:49:31.846210Z"), "type": String("DateTime")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "id": String("IkYnZ7qEAjprBp8rSetrpBgTmLgJNe6roh5My-W8v4qH-s64NjzEi-nWGnpieJWOjCExE7BAVAvZNZD7D56mSQ"), "name": Object({"value": String("Test Record"), "type": String("String")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])})} }

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-01-03T09:49:31.846210Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record { fields: {"name": Object({"value": String("Test Record"), "type": String("String")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "value": Object({"value": Integer(42), "type": String("Integer")}), "id": String("IkYnZ7qEAjprBp8rSetrpBgTmLgJNe6roh5My-W8v4qH-s64NjzEi-nWGnpieJWOjCExE7BAVAvZNZD7D56mSQ"), "price": Object({"value": Float(99.99), "type": String("Float")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "created_at": Object({"value": String("2026-01-03T09:49:31.846210Z"), "type": String("DateTime")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])})} }]

=== Update Document ===
Updated: Record { fields: {"tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "id": String("IkYnZ7qEAjprBp8rSetrpBgTmLgJNe6roh5My-W8v4qH-s64NjzEi-nWGnpieJWOjCExE7BAVAvZNZD7D56mSQ"), "value": Object({"type": String("Integer"), "value": Integer(100)}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "name": Object({"type": String("String"), "value": String("Updated Record")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-01-03T09:49:31.846210Z")})} }

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.99s
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.69s
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.98s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "TY233xqxxRNPBi9IoHXrltZTFQLaF_8_7rTwe6LJ2SBYEXhsGe9GkWUUIWwUdl5RER909kDGGG0EzG_N2qCu1A"

=== List Collections ===
Total collections: 9
Sample collections: ["test_accounts", "users", "ttl_cache", "ws_ttl_test", "websocket_test"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.96s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "W78pJjXDv07IDFkPw3u-gYgs8k2L8yChmlVFjEhvHkogqcJsA8oWELY6o1xTtuzIrCUzt6gTNhbVPPGpNUQGuQ"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("PmgeXwTAetWRuxf7iiVmpnjNwuxvxF2ietNptuZJQs7AiTRRVC767HqgdC6aYeS5DW5LepFwhQ7elKbDl-eB3w"))

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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.27s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: tdL4Z-xrce6vVHZ3nXkqn4Uqnx74s_rj3-YogCUeVAnleVsXvi74wYCI_iVBc2CEG9Ga3pbF4MSNX8t83M6mWw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.26s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("ThDvY7gKT2FumnFaGHh5NcAqBUIhIMbFAbMm_YvOtLFJgt95RuzyHlWfGEvHyy-TM_Aj-VHMJBByIwqvPHYlSQ"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.98s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))
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
  - Some(Object({"type": String("String"), "value": String("David")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"value": String("Charlie"), "type": String("String")}))

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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.97s
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
     Matched: ["description.value", "description", "title.value", "title"]
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
  2. Score: 6.6000 - Some(String("Python for Data Science"))

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 1ms
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.01s
     Running `target/debug/examples/client_schema_management`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("BxEyJEcDt8ZiF6E-lFT7OdV3EL4r8LGUJotFboPb-AyFJ3qVcvuKHDgus3zMeWfVMuzxv40E3jzoJDrP4_lddA"))
✓ Inserted user 2: Some(String("pcOUL0hzP4ZWyod1dg6G5UXh0fh0nb0O4JXJEyCZjt8gtwc4vmSmmnq7fZY9HiQepHodWWX1BSOUnx61AaL7Mw"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - email: String
    (required)
  - age: Integer
  - status: String
  - title: String
    (required)

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["test_accounts", "schema_client_rust", "users", "ttl_cache", "ws_ttl_test"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.04s
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.93s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: _7tHWwvnTeyUz_55D47bdvKv-LR17o5N-3frkweqyiY75TE85p882gSX8NaWWrOIRzliqYJXFtiK6qpW5i5ydQ

=== Sending Chat Message ===
Message ID: F-0PSVFWZJnnqtkZqG-XLg6v24vsee3dRZMhtUL7tEox2WKfw0_38vD4-oRY50U1yzydTbhQUPnqbmZfU1Bvzg

=== AI Response ===
Response 1: ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration. It has several key features, including an AI chat integration and advanced search capabilities. The AI chat feature allows you to query your database using natural language and get AI-powered responses with relevant context. In addition, ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["content", "category", "title"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("gloYimgw8qYZ_kszRXZ-LY_Nb2RlXng0efkdgNlM5gde3tEYNhOC5Z9XHRGvfElS1vIthHXigBwJdCkyNRO_Mg"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("mVfEDi8aW7DV1qnlSrHunDu6wPRzV4PvRFwxVsKW6XmT0iUyAK3cbyEk3rPAhTv1e-DC__HXUmQJY_858LhgqA"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "category", "title"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("LWiS5EOtWAKOhj5dF264bDsEu0y4UlKeoKCpL76bHNxNKP4lp8bMgOQ_4o2UdKwOMtIATYTbzBZ5XjClgxmPKQ"), "title": String("Search Features")}

Execution Time: 2805ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.03s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: SI0IVjSitRK4WC0Rq1i2_ftWMw6wc_a-F_5jxxLXdDFcbDYkAVkRLYe-vu7rx-cnpa6CbU66anXhCjolvyc_ZQ
=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is "ekoDB". It's a high-performance database product with AI capabilities. The price for this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: vn2TdqcBSrZYZ3GEkauujHZvtihmQTlv0LP9Lp5I_XbIIJMaI0bIgCmC0QXPEhI8LJsKG6wmUS35Nr4W2vV7Dw
  Parent: SI0IVjSitRK4WC0Rq1i2_ftWMw6wc_a-F_5jxxLXdDFcbDYkAVkRLYe-vu7rx-cnpa6CbU66anXhCjolvyc_ZQ

=== Listing Sessions ===
✓ Found 3 sessions
  Session 1: vn2TdqcBSrZYZ3GEkauujHZvtihmQTlv0LP9Lp5I_XbIIJMaI0bIgCmC0QXPEhI8LJsKG6wmUS35Nr4W2vV7Dw (Untitled)
  Session 2: SI0IVjSitRK4WC0Rq1i2_ftWMw6wc_a-F_5jxxLXdDFcbDYkAVkRLYe-vu7rx-cnpa6CbU66anXhCjolvyc_ZQ (Untitled)
  Session 3: _7tHWwvnTeyUz_55D47bdvKv-LR17o5N-3frkweqyiY75TE85p882gSX8NaWWrOIRzliqYJXFtiK6qpW5i5ydQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: vn2TdqcBSrZYZ3GEkauujHZvtihmQTlv0LP9Lp5I_XbIIJMaI0bIgCmC0QXPEhI8LJsKG6wmUS35Nr4W2vV7Dw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.01s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: om7Yg5pv9-5_eagTHicdXA37K8R6tS9Wj3F4TFW3dvzMO5rFdupHqUbXls4iiRPsBXhMa4dV3QD1GvPomL-FHg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, the available product is ekoDB. It is a high-performance database product priced at $99.

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
✓ Created second session: kjC5gFDXqUMkJtTG9oQxoJ4GZsqn4-JnYCiIcGGKMR0243BGdCUoT_VC4DQP5SHwUY2nER_UmhkyajKM6mJdfQ
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.28s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: TQUSOH7PjElYhKhhxxW4KGA0AlI44NzajXpA1Xw4V_UI8bPBxzlb55IALp5sKmegQnjS1-DuTPgQmeMe0prtrA
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: joKf03z3VmZuSjq7WqINyaxywXUMlpePjOOKRpIfO1QkXrBZI6FqyAClKdNnr5bMMb6sqxf8ODMPCsCFL2JqNA
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: FShzg8i3Xf-hX-NbFRoNK9TKb79djUSstsq7zQK60DxDVmZ2du9fKimLMXxbHkVtPp7BnI8o-6ZTnn1Piy3M0A
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 6
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.97s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 1qoDWWsmLsUlwBQvt4D-qWMCWIKOuZiuIG0nbGPzMvqVC6Rv2MPkFdPVto7X0g7F_NETMFNivkZT9DGvGLOCjw
Created Bob: $500 - ID: UGPaJL7eBDAAEVkPklm6R-_sCWr8fgUtBg5BhITlyKa6bRQlrB575a_5WZ8yOlhhHiLzQrmYySKeoTlQ69b4og

=== Example 1: Begin Transaction ===
Transaction ID: 9fd13ef7-9660-466b-9bc8-d3b3e092a21d

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: aa8913cd-557c-478b-810e-68d6e5c88ca3
Status before rollback: "Active"
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mRust client examples complete![0m
✅ [32mAll Rust integration tests complete![0m
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'W8vbx_g5wVRjgJx8djaqBkCT1N8m79fPUCkDfHLAKwsNB8kVHpC9524CqJOcdBmhzKD4DW8u3ln7TaI6qZ7KfQ'}

=== Find by ID ===
Found: {'value': {'value': 42, 'type': 'Integer'}, 'id': 'W8vbx_g5wVRjgJx8djaqBkCT1N8m79fPUCkDfHLAKwsNB8kVHpC9524CqJOcdBmhzKD4DW8u3ln7TaI6qZ7KfQ', 'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Test Record'}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'value': {'type': 'Integer', 'value': 100}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'id': 'W8vbx_g5wVRjgJx8djaqBkCT1N8m79fPUCkDfHLAKwsNB8kVHpC9524CqJOcdBmhzKD4DW8u3ln7TaI6qZ7KfQ', 'active': {'value': True, 'type': 'Boolean'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: kL8DVSUh7_LbJlGQ8Jp8rpmf3HJUvw0sR1K61EIbOfhPQNbCdjciPwd8xTHlb6AY0AiPX1Gmo0kplRygdd5vow

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
        "id": "kL8DVSUh7_LbJlGQ8Jp8rpmf3HJUvw0sR1K61EIbOfhPQNbCdjciPwd8xTHlb6AY0AiPX1Gmo0kplRygdd5vow",
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
        "id": "IjdISjptT6bysMqTDvn82myude-F1EHII_2b_ly8-sMxVL2QcMVlBbQPhso4ZGmgB5bQBHKla7FbAnTqScFsJw",
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
✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 10 total records in collection

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records
✓ Verified: Record updated with status="{'value': 'active', 'type': 'String'}"

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {'userId': 123, 'username': 'john_doe'}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: {'name': 'Product 1', 'price': 29.99}
cache:product:2: {'price': 39.989999999999995, 'name': 'Product 2'}
cache:product:3: {'price': 49.989999999999995, 'name': 'Product 3'}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: dN6BoJNNKmFJiEaKrxxfr5zZL0TJBPtG7oyVdL5rZvI02KpJ4iCUdcVWo6ZRa6LWLsSon_eg_J6xPJHM6sBDpw

=== List Collections ===
Total collections: 11
Sample collections: ['test_accounts', 'users', 'ttl_cache', 'ws_ttl_test', 'chat_configurations__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 7YGPrcYM7oXi7jzJnIYz3Z737d484neLxCRWn_qtVMeivUGroBR5ZYYSEfQPN4pkQrjS7kBvwEjpRBQ_wt7Lig

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: created

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: cXHxuiuw2XmxmvY-_QoWpVsqMGKqudSHEP5TRUWR4q90KE5YkZ-SlgnOg-RpyHQin--1uGo9HKJEVXEdmMA2nQ

=== Query Documents ===
✓ Found 5 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: _sP-bqSQBPb9ShFJ4kgu_mqiYqNlgXYkLx5lwL9nVhYM7pdPhguL4u7ElUnW7MRPd0K5q8bL3N0Q8XtCP4ARfw

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: p5Rgtd69AhJoxpmufXf_dE5frfxl6z-Q5zzhqg3WsyOYIk00rSTVDH1D7IAL4Dwp6iLR_a2fDCFxHE84R8i73w
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: dlG2Jut91eGvyk_MySi0M0YgKX3umNfysG8IysZhjST-_QO7CuPncRL_PNjUhWdtZs0v4ApTNGv-urdjiZONdg
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: FAgVzqrMuYLmozAMTjXbHpvOx4OtbiiWJ87AQPkCpAbgF4RHwAP-9_LrFW-dZwRakAkYX6Tz2buSCVZTSn05PA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'status': 'inactive', 'max_score': 90, 'count': 15, 'avg_score': 50.0}
   {'status': 'active', 'count': 15, 'max_score': 100, 'avg_score': 60.0}

📝 Example 4: Function Management

📋 Total scripts: 8
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
✓ Authentication successful

📋 Getting original configuration...
   Original durable_operations: False


============================================================
🔥 TEST 1: Original Config (durable=False)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: rlST9Hj8yVQ8OS-SD__hgmdr5KqkF02_Yy_OvFSbBOSrIENmzNB37z7x41udjBEzS-Q9E4LtcQPZ_s6NevTVfA
Created Bob: $500 - ID: yAKGSr95gNbzumn-WfkSssf9ofqS0QHXc9Hdq3On5enGPgsXrFuvTuUST922z67WamUdFI8OPk7ykW-L0MY2uA

=== Example 1: Begin Transaction ===
Transaction ID: 7450175e-5356-4d76-a42e-b4c7296985ad

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
New transaction: e6e6e0d6-264b-45ae-87bd-dbf3b71bad80
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: E-LkehClOGh_Bl2e-4Tb03dgxLslJYYAa_-h1-ncSJTE_ibmzgBzBag34K7HSm1xACfnbrX3xGVy8WIBKLUqmA
Created Bob: $500 - ID: yz3wniFHT6F5soPaDXcThxzvxCkdWkIAxutcl89hNDa_Sf1OcNhjKcokyvbFgm-XZg2qgQPZSh1z6t1B6xApAg

=== Example 1: Begin Transaction ===
Transaction ID: 8ee12960-c888-4372-8cf1-2558995e70dd

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
Created Alice: $1000 - ID: ut3G523iKyowpbt_tGgcvAiF4x6k2Ly_Ly1vGKeU4USDp580TvCL5l72Hop7Bk-RxuqGwmKEALm6qxUSyzYScQ
Created Bob: $500 - ID: wrL8Xode5pWyFt19KJi7qNLo4ZYdMa7ezMPvXpjjrmU1SKluR6SZSGi2gXHuzGyXgGQdLlOG_41QJLfvxZ_yLg

=== Example 1: Begin Transaction ===
Transaction ID: 37c38f35-65d5-4174-b4bb-1a5829434afe

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
New transaction: f10c40e3-4f15-4460-8acc-4da11a2c06a7
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: gb9OZLdWF0p8yD0O8JXLHzOWGgAEG1HmPTUMN9lLxkeIWNDDvP6GOki5ed2cUOpDT65lMMFAkJS0xevD9w2yEA
Created Bob: $500 - ID: Pp5H5KsQsd6XwpcZ_7C97beTqSuWehaMfmzZPp1WIcUjvDfEm-UeDoMQK1hCMhMRnzSPLSiDCK9vMtBhyP1jzA

=== Example 1: Begin Transaction ===
Transaction ID: 0db1b769-3f6b-47a3-a69f-5990f70c3eea

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
Created Alice: $1000 - ID: s_F-ZG1Mkve5LzOvXCxpFFERcYYsLC_ECjeE6We7VA9kLqgrOuIHmcTf_r3cTLbmkgQM2xYmW6J5A8a6W4-e8w
Created Bob: $500 - ID: _Y4B5lK9Jb_7zMpCnX7eY-EByVNZG7biuvUWIrJLEBUwt_4CH8dad_7S2N-At7vb4HM4428mEZQdiG5c3O-TTA

=== Example 1: Begin Transaction ===
Transaction ID: f7d3378d-5119-44b8-a5c1-3e777e833994

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
New transaction: d81ec17a-0773-4d51-959c-1c3a271a6b89
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: e7pMfuW3hK9gaoWoMtXi5sDKhjKuXzu_JSswbUF0W1VNUVirIOdU-2rY6Y-ldSxNjAuHLpixbA-XodMbjG5lbA
Created Bob: $500 - ID: Ofa5QzfKT7OttEGjhwFdts6INWrOZLEcdtV-Ij2cHnPx-ap7CCmELIw5W9mpqt0XurA-3id0-6uhrRZvn_k3Gw

=== Example 1: Begin Transaction ===
Transaction ID: 548ba4b1-6a4d-4522-ab14-e79bc0b48def

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
   ✅ Script saved: 4_moP4qWwVujv8Ck-ZuXiZbPiVMIc9JIDFToQ2MvS_bOBYSGtmJldrsiqIX0U95f2k2TK3rBLBY8MNq-B3lOiw

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: UDbNRXy6WEAgGsFuFTrqZv8SuQ-92DkR4_ZWKcGEEUNEBkM3wBAkcn8BDGSEzqKKA2Mcnv2mz_lC3nIqlN61Rg
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: bHVcPK7HO1xHXnjUw2doD2e6Ww7XiMvzCOFhja9LBWG_h8TPeeop9bJXgo22EChF4xGVSpoXDW8mOTgW4W13bQ

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'value': 'active', 'type': 'String'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: bsj-RCFOTzcecHrkXgpG8UBwQNlUsy7BC22vbXdL8TUEenNpGMMzoszobjynxKJDDL0itCGalTWLao8ttA2p8A

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
   ✅ Script saved: IqGSwJwAxKhHejdZtSHfuXhGJDKg4cdiiTloNqN3X5LmvPd5eJtAjtX3kC5_gcnnvRHPElCMoNLT5zvFFAEJVg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 4_moP4qWwVujv8Ck-ZuX...
   ✅ Deleted script: bHVcPK7HO1xHXnjUw2do...
   ✅ Deleted script: bsj-RCFOTzcecHrkXgpG...
   ✅ Deleted script: IqGSwJwAxKhHejdZtSHf...
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
=== Running batch_operations.py ===[0m
[32m✓ batch_operations.py completed successfully[0m
[34m
=== Running kv_operations.py ===[0m
[32m✓ kv_operations.py completed successfully[0m
[34m
=== Running collection_management.py ===[0m
[32m✓ collection_management.py completed successfully[0m
[34m
=== Running document_ttl.py ===[0m
[32m✓ document_ttl.py completed successfully[0m
[34m
=== Running websocket_ttl.py ===[0m
[32m✓ websocket_ttl.py completed successfully[0m
[34m
=== Running http_functions.py ===[0m
[32m✓ http_functions.py completed successfully[0m
[34m
=== Running transactions.py ===[0m
[32m✓ transactions.py completed successfully[0m
[34m
=== Running crud_scripts.py ===[0m
[32m✓ crud_scripts.py completed successfully[0m
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
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.7.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.7.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.7.0
    Uninstalling ekodb_client-0.7.0:
      Successfully uninstalled ekodb_client-0.7.0
Successfully installed ekodb-client-0.7.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'YT15MOmk9O49hyHQ3MtgvbDIE9ySeJM2F5ExMLrhmFl7nqYSXKVBDmOxlwrP9K2Jqve0GkQ2g1ZcNb2Zi_x3Uw'}

=== Find by ID ===
Found: {'value': {'type': 'Integer', 'value': 42}, 'name': {'type': 'String', 'value': 'Test Record'}, 'price': {'type': 'Float', 'value': 99.99}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'id': 'YT15MOmk9O49hyHQ3MtgvbDIE9ySeJM2F5ExMLrhmFl7nqYSXKVBDmOxlwrP9K2Jqve0GkQ2g1ZcNb2Zi_x3Uw', 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'active': {'type': 'Boolean', 'value': True}, 'created_at': {'value': '2026-01-03T04:50:35.017567', 'type': 'String'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-03 04:50:35.017567
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'value': 42, 'name': 'Test Record', 'price': 99.99, 'data': 'aGVsbG8gd29ybGQ=', 'id': 'YT15MOmk9O49hyHQ3MtgvbDIE9ySeJM2F5ExMLrhmFl7nqYSXKVBDmOxlwrP9K2Jqve0GkQ2g1ZcNb2Zi_x3Uw', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'active': True, 'created_at': '2026-01-03T04:50:35.017567', 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'tags': ['tag1', 'tag2', 'tag3'], 'categories': ['electronics', 'computers'], 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5]}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'id': 'YT15MOmk9O49hyHQ3MtgvbDIE9ySeJM2F5ExMLrhmFl7nqYSXKVBDmOxlwrP9K2Jqve0GkQ2g1ZcNb2Zi_x3Uw', 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'value': {'type': 'Integer', 'value': 100}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'created_at': {'value': '2026-01-03T04:50:35.017567', 'type': 'String'}, 'active': {'value': True, 'type': 'Boolean'}, 'price': {'type': 'Float', 'value': 99.99}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}}

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
Collection created with first record: "IWkbr53nt_eHtqqTjJFKzu9L_Fm0uAPeBHLL7XhdydGsX6vJD2SQ4e-VMxE19jb4cKqFey_PBCE_HtHriSeuMg"

=== List Collections ===
Total collections: 10
Sample collections: ['test_accounts', 'ttl_cache', 'ws_ttl_test', 'chat_configurations__ek0_testing', 'websocket_test']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: O1IPNZZbGn0a1ng2A-MlrlsY1lrcDuy06aSSbJL2k2Q2yhQcZU3lZ2d8obK2dEom6RK1OpZgstij8t-jyzUIAA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: iBIGulWCK6iCCbuKL5E2d4CxgXD84GkpqA1XvOpC1FxPNku6FQ_Tf4px_YxY_zN5xNHj0MntKlwBiZp_Cc1TWg

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
✓ Inserted test record: lD0I63jZPMBRJwXlxulK_7z6sKGvTvUixzEnHeT-ZNJIG_ZwRW-0JsiUChqOQsvSrAMs9hXM0NeyaxFJJHtJMg

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
✓ Inserted document with TTL: 7cxgxiFDCfE-1fmqfHBrYywg8g3KbZlA3_vPKp0geNUQE3fD7LdgLJAwQTIhd5kq8NAXshXTBehBMmz-CQV_cw

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
  1. Score: 25.740, Matched: name, email.value, email, name.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title, title.value, bio
  2. Score: 26.400, Matched: title.value, bio.value, title, bio
  3. Score: 26.400, Matched: bio.value, title, title.value, bio
  4. Score: 26.400, Matched: bio.value, title.value, title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, bio.value, title.value, title
  2. Score: 39.600, Matched: title.value, bio.value, title, bio
  3. Score: 39.600, Matched: title, title.value, bio.value, bio
  4. Score: 39.600, Matched: bio, title.value, bio.value, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.783
  2. Score: 0.753
  3. Score: 0.751

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.727, Matched: title.value, content.value, title, content
  2. Score: 1.126, Matched: content, content.value, title, title.value
  3. Score: 0.548, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: title, title.value

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
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

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
✓ Created session: YQzzCfZjPdw5kP1aBN6THyY1bRvLMtCvolkzyPoRuQRuLMHMWeb6xYWJbJXPi-iZTzqBLMxeJJFJzPXJlfwLbQ

=== Sending Chat Message ===
Message ID: zF3RAkXpeSLhWD16K_OeA79Q8AvaGZeXTh0YiwuUIBkEWpey0bDZj85ReifvLJPrwAGOqyl_1ZBTRIDFQ8TnUA

=== AI Response ===
The available products are:

1. ekoDB Cloud: This is a fully managed cloud database service product. It is priced at $499.

2. ekoDB: This is a high-performance database product with AI capabilities. The cost for this product is $99.

3. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

Execution Time: 2720ms

=== Token Usage ===
Prompt tokens: 599
Completion tokens: 81
Total tokens: 680

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: r8-q1NPgr8SOnlJxjWfntm6zuChhix2h6xtYe5ervcSBCOE-h8CFI_i7m9pJxLVzK_gop-dpdEHNtpNLVEH2Zg

=== Sending Messages ===
✓ Message 1 sent
  Response: Answer: One of the available products is ekoDB, which is a high-performance database product. The price for this product is $99.

✓ Message 2 sent
  Response: The price of the product "ekoDB" is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: CilX15i-UC2kwOpaRifVfHnnduebob47Ypk9S3zki1WChBCYdbJRKxVfOVmLExlQHmO6gDbJxHYpnce1zT4cXw
  Parent: r8-q1NPgr8SOnlJxjWfntm6zuChhix2h6xtYe5ervcSBCOE-h8CFI_i7m9pJxLVzK_gop-dpdEHNtpNLVEH2Zg

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: CilX15i-UC2kwOpaRifVfHnnduebob47Ypk9S3zki1WChBCYdbJRKxVfOVmLExlQHmO6gDbJxHYpnce1zT4cXw (Untitled)
  Session 2: r8-q1NPgr8SOnlJxjWfntm6zuChhix2h6xtYe5ervcSBCOE-h8CFI_i7m9pJxLVzK_gop-dpdEHNtpNLVEH2Zg (Untitled)
  Session 3: YQzzCfZjPdw5kP1aBN6THyY1bRvLMtCvolkzyPoRuQRuLMHMWeb6xYWJbJXPi-iZTzqBLMxeJJFJzPXJlfwLbQ (Untitled)
  Session 4: kjC5gFDXqUMkJtTG9oQxoJ4GZsqn4-JnYCiIcGGKMR0243BGdCUoT_VC4DQP5SHwUY2nER_UmhkyajKM6mJdfQ (Untitled)
  Session 5: SI0IVjSitRK4WC0Rq1i2_ftWMw6wc_a-F_5jxxLXdDFcbDYkAVkRLYe-vu7rx-cnpa6CbU66anXhCjolvyc_ZQ (Untitled)
  Session 6: _7tHWwvnTeyUz_55D47bdvKv-LR17o5N-3frkweqyiY75TE85p882gSX8NaWWrOIRzliqYJXFtiK6qpW5i5ydQ (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: CilX15i-UC2kwOpaRifVfHnnduebob47Ypk9S3zki1WChBCYdbJRKxVfOVmLExlQHmO6gDbJxHYpnce1zT4cXw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: jXNcdOZuMBn1XW-A-bOVBhQD--2S4HRWKD4P6jTz6krHXqoBRIoC3yXBLc8_6zRQTPvUCluI5FEhX6QMT5GBwQ

=== Sending Initial Message ===
✓ Message sent
  Response: The product available based on the provided context is ekoDB. It's a high-performance database product.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['updated_at', 'role', 'chat_id', 'created_at', 'token_usage', 'content', 'context_snippets', 'id'])
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
✓ Created second session: kjK1pDTNLOKHPH9OvZuvvQLFzTMCSQt-byLD2bA78E2U9VUsJHSz_I5zfuQo1BXHoSW53CpQnsFBhK4Q530HVw
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

✅ Script saved: 9Idosq_OCUDgUk1SBFRPvHqJF0EmUnyqeZVqFJBRj8iCz533wMTvfQhAb1dCOLsZysSuTiS1laYhTfD5bxXdqQ
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: uMHmWwya9IrBqa2Jro9yl8AER9Rn1D6t3FVHcl3NheXnS24ZYESGJj2sgUk4InX5KCNtzGOC8HALJA5YAYLh9g
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 0iMPU3Zy9NHo2XLWz0uaNTiqq7WBADc3IGkGClEwuBpktvLFFvo1zHl7f7pXnWpJQWR6NtcA8E_84nmQ73eXKg
📊 Statistics: 2 groups
   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

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

✅ Script saved: c3GaK-dz2fGVRUxgZD6-yXx-RYaq00ds-SQoJ0ESEwq1axJb3f9yeP6l9nzZwT03xxQ0LBSgx-PJ9uclY4v4eA
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
Created Alice: $1000 - ID: 7jc0fqV-Z3Yhe3Li7uylMaInByvhaWmWCbdv1wec8aBOev2pKDpVNNDQWR6TjYqL49zkwpzuVqy0kLFP3y1JkQ
Created Bob: $500 - ID: 7yfyWZn4M631frINfZT62K-cTTc_XFvpv6sI7jXCe_46GgwX7pk6uAbYKRbBtVh0e1J_TOz_mpldGXYVhye6mw

=== Example 1: Begin Transaction ===
Transaction ID: c2eae7c1-2ffe-4c22-9cc6-c11adb24e73d

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: c18f88a6-c56c-45e8-9cad-b7ecd9778cd3
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
✓ Authentication successful

=== Insert Document ===
Inserted: map[id:Ub49L1WXGY9wW0hHHOXwTXWviUMlUkIMqf6wNgy3M2TNUN66jEQl0QnEOMYXOQynzN0z4-QiBHlgAMxZ8-U6CQ]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:Ub49L1WXGY9wW0hHHOXwTXWviUMlUkIMqf6wNgy3M2TNUN66jEQl0QnEOMYXOQynzN0z4-QiBHlgAMxZ8-U6CQ name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:Ub49L1WXGY9wW0hHHOXwTXWviUMlUkIMqf6wNgy3M2TNUN66jEQl0QnEOMYXOQynzN0z4-QiBHlgAMxZ8-U6CQ name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: 1JNp5xbnQN5NRBa8fEEUHRYA9jRzodnChyV6YjAfYfef-s0OYAwTz4IUNZObWzDqbBNaYEZYexQ6TF4zhalGVQ

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
        "id": "1JNp5xbnQN5NRBa8fEEUHRYA9jRzodnChyV6YjAfYfef-s0OYAwTz4IUNZObWzDqbBNaYEZYexQ6TF4zhalGVQ",
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
        "id": "kL8DVSUh7_LbJlGQ8Jp8rpmf3HJUvw0sR1K61EIbOfhPQNbCdjciPwd8xTHlb6AY0AiPX1Gmo0kplRygdd5vow",
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
        "id": "IjdISjptT6bysMqTDvn82myude-F1EHII_2b_ly8-sMxVL2QcMVlBbQPhso4ZGmgB5bQBHKla7FbAnTqScFsJw",
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
=== Running batch_operations.go ===[0m
✓ Authentication successful

=== Batch Insert ===
Batch insert response: map[failed:[] successful:[QdPO4lQCTPr6atcSdpS3eRexi7HhG4CJ-LhmsGw0ECy2opCm34DS-V1kMsF_15LPmr2aP6le73pZPPAz0IgpUQ JYydxlDZVx8DAizKancU7PgXqzjmrlywZ_OQa-mYtc3dZClh8EwZEqB8fWJBZ5cVDsNmaH6conM4kqfogwlZoQ JyZoEdUYOSBAzYtvihRO2Qls4dDcj_pwWouzxEquN_ePvXyidXs6hpHNgfxfFHUIOULZDYcxSwflisOsoyRu8Q bjw1QwUxwru5beaXcrJCWuBrVQ3Sx_2H_jOzAYkZ-aFYYiYobuRcQG94QdpZ2tQTRywX3-BmUwihZ1HqEXeapg W6Ez00HJKblWr5oemKFhfPsaQw3RmX_krWHdv9nPecQV1AF2WtrVcejSHA-MGcPhY8Y6Bth-KToZsr134c_PYw]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[oYvh5loGgYXfhzHUGCDaqCOLGvKccDNi1yPEG23xXySHRwHFfntsz2VsDs3eiRHJiUhElBoWncZXEjg9d_0hmg tRz99h9CkeomIZUPbUx5HomBqdVoiG8l-aDjM-KD6Ut5JaCDAwlurgnshSmGRlArEYfePxegxozFHyUZ0DOmcA RqCJMyLRHvjJwdIN6KA0MWG2x65mNOK7-9CrvdV9a5RwthkKbpP9uVsqBJlrI5rpZpxIdR2xtUHVW08HErWRMQ]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[oYvh5loGgYXfhzHUGCDaqCOLGvKccDNi1yPEG23xXySHRwHFfntsz2VsDs3eiRHJiUhElBoWncZXEjg9d_0hmg tRz99h9CkeomIZUPbUx5HomBqdVoiG8l-aDjM-KD6Ut5JaCDAwlurgnshSmGRlArEYfePxegxozFHyUZ0DOmcA RqCJMyLRHvjJwdIN6KA0MWG2x65mNOK7-9CrvdV9a5RwthkKbpP9uVsqBJlrI5rpZpxIdR2xtUHVW08HErWRMQ]]
✓ Batch delete completed

✓ All batch operations completed successfully
[32m✓ batch_operations.go completed successfully[0m
[34m
=== Running kv_operations.go ===[0m
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
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: mlXk5Mn0fFZUgpA8--mMsBQ_ULHh8kgSoahpdPAOa1538au4INApfGc98tkIwfYbR7cG__t23e_Z4DEb45hYWA

=== List Collections ===
Total collections: 15
Sample collections: [test_accounts users ttl_cache ws_ttl_test chat_configurations__ek0_testing websocket_test test_collection schema_users_client_py chat_messages__ek0_testing demo_collection schema_products_client_py scripts__ek0_testing batch_users schema_employees_client_py schema_documents_client_py]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
[32m✓ collection_management.go completed successfully[0m
[34m
=== Running document_ttl.go ===[0m
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: KD5gTt_zcZrwhDq_kUzTLO5oyFAvgCiTwMn68lNT2RAcm__7UrNuY-mcDgbIgBuwyjAF1l4w7BJttx9AeN9mZA

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: <nil>

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: FGWqrCgrPZN6ZYp4rMd9Khm-_aCsEPnWjuOu-f3-Gc6i13pe6ELVGc7eH_qkN8_zJr8nY601JJiG9fJ4wggWUg

=== Query Documents ===
✓ Found 6 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
[32m✓ document_ttl.go completed successfully[0m
[34m
=== Running websocket_ttl.go ===[0m
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: O3lTGA8EyNtSwFtDQ6zvCaah22VF9giWRtCf4_WFOIQ-T1dpPnuklrAMyP0zWgEIzWE5RZXkttV0ZS5YsbCFOw

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 3 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
[32m✓ websocket_ttl.go completed successfully[0m
[34m
=== Running http_functions.go ===[0m
🚀 ekoDB Scripts Example (Go/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: _gHKtBlzlBEb2DpdtQ-ir5cJdNJi7KE7VY7GPRE7vCNc1L_2JwbQ6TRRH_-VJaRE04JDqJQiEc8TwmWFvQ8GWA
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: 2V4jhN34bH7fCVR80TcWlU1csE9s6cdDWQksqfp_x2Tr60yg3V6UW1O9mzDHycFy2XuM00tnUGk9llMpY4tSSQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: Oe8ArE_Cw458SI1BP51KFKft1Q9phiKUDnAXChyRQIsq8Ya7dNbOgOclwNUG5beE4qpW30JQe_3ErmFN1RYRiQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}
   {"avg_score":50,"count":10,"max_score":90,"status":"inactive"}

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
=== Running transactions.go ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: k5QhYPbJPTIxR5t7KOL5T_-QNZVejOjr3S8SMt_JCo1FYC0xwnH43UT3hxO1B81khraRrrssIYwaY4ib2Zp_Tw
Created Bob: $500 - ID: G9RllK69GTVz4uZTRqpiUob1VQueWNacvhQBp_w5Xe06cSVJj1i9uYFPy6umJapfRxeKam-iMl4inF4XPiE-SA

=== Example 1: Begin Transaction ===
Transaction ID: 5ddaa8b7-40c3-4d2d-92e5-d0fe8c488bac

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
New transaction: 5943ea02-ebdb-4e74-8935-bd49f6970c06
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
   ✅ Script saved: ZvGvhoJRgnSMyNx-qPvQFnccaIwpl9ZmBDAff9mIw5DwCquxiBq1C381SWkq4vJAJIFpG3vq6nzkvadP120tuA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: pifxT9tZabIKcSUk4vqAdCT_6EuBzKVz7fx5jnh_u_ZFNG9yPNgKp9wYz4fQ5-EMEP0KfopdJ16qy8cClLB1pg
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: Wet2uJxYv234-Rb-e2nK9a1OcIhA54xEymNCYh3NR9rf-GFUynW4qllJJf27qUTatvaOzmxPUEwr8KW1gclmdw

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
   ✅ Script saved: aUdV7ZdHQliI8ck17kGHiYys11ujOf73SVzhes3ou2ljCR7a_wMa7qbJ-EAS4dSTCPmTCnv2N8u5if_uYKztzQ

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
   ✅ Script saved: RQhEzNGjS5x4M8uf0LG6VoSCGGGcMg71o4YhYgalFbDIOy4PGEagA7IAyvOF9TIwCleWnwMa652nUTeANJmXnA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: ZvGvhoJRgnSMyNx-qPvQ...
   ✅ Deleted script: Wet2uJxYv234-Rb-e2nK...
   ✅ Deleted script: aUdV7ZdHQliI8ck17kGH...
   ✅ Deleted script: RQhEzNGjS5x4M8uf0LG6...
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
Inserted: map[id:hDErmXATtxWzYNabBUPsW3vhrDPqyy_GN6vsAJLacuv2WweJWGq2wkn5c_yXu3gowQTTgh3FnWN4wjkDTOtBiw]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-03T09:51:08Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:hDErmXATtxWzYNabBUPsW3vhrDPqyy_GN6vsAJLacuv2WweJWGq2wkn5c_yXu3gowQTTgh3FnWN4wjkDTOtBiw metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-03 09:51:08 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-03T09:51:08Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:hDErmXATtxWzYNabBUPsW3vhrDPqyy_GN6vsAJLacuv2WweJWGq2wkn5c_yXu3gowQTTgh3FnWN4wjkDTOtBiw metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-03T09:51:08Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:hDErmXATtxWzYNabBUPsW3vhrDPqyy_GN6vsAJLacuv2WweJWGq2wkn5c_yXu3gowQTTgh3FnWN4wjkDTOtBiw metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

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
Collection created with first record: Hi_a3G4zCN8te5bH0m47pVvOT0q404ibIipxGLjePjNL5QBOxzUgAiy2eREmkxpyzY8kF5gONq3W7UmOHNrZfw

=== List Collections ===
Total collections: 14
Sample collections: [client_collection_management_go test_accounts ttl_cache ws_ttl_test chat_configurations__ek0_testing]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: Whd0jZiDFISxdkp9WH5hUB6ItgcLNQYpNGPcn5d93-DhATUZ5nC25tL-JayOzSVYaybKcr6pjIqGqzsRaKCdRA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: xLitIGPsHCjNpzlBhZ7qo_HcAwC4HYTuBLdlsRiBC6DA_x6ENx_Ovjzb77p84ir4y1IpvNS3F0p6aDAQt4Hxbw

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
✓ Inserted test record: wDXf6aWUmBWm4zSUF_I5rSkoerTHFLrmORGxlGJaYFK4vpkHKOHkFdQhl6s6da4osy42li8BFGXipQ2EeH2MEA

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
✓ Inserted document with TTL: W9K2zRgJ8p4_oWAcUfixp1F8DTyi8EIXC42M47uXPqNg-oV-Llr1mPovph4xonA7oll5mH8Fm8r5LyV52D2DqQ

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
✓ Created session: IA5-rWYt2lejYRO9aNTZg4W-5TeMAvxB42a9TSCPeQw1CwR_yBAuwn2TANGMOssgAlVBKIqvHCr9wPW1BHcn5g

=== Sending Chat Message ===
Message ID: Ex8TPEExV9UDcaDpootf-ivJXauHIuA0dHMlosdUERW9E6SfUfkABYo2cfGETgGYiKN6tyMA0-orncHpwFLtiQ

=== AI Response ===
There are three products available:

1. ekoDB Pro: This is an enterprise edition product with advanced features. The price is $299.

2. ekoDB: This is a high-performance database product with AI capabilities. The price is $99.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price is $499.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:j1dwIwow-6bPVUjI11s0MJZh6d-fRLE1Y4PEyUmOaeoZ52GzfV3k31fc6ZadsxGq3L5L5-0-_BvPLCv0Bbksyw name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:CLhHlMULpkCCK8OxlWZ1YpmMzH93WO9T4qI7qH5d9Kz3r5modKFuZCYGMsdVEa7aY_COQb4DUtT89eha41-Shg name:ekoDB price:99] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:b0eSG5BHs4XZB3YFa6gol2RLRXbZNYdqi7OdHJrhfBxHmV_rG3U-usd64G7HvlnfJ6QgGcyUbvBG2VqgTBj_xA name:ekoDB Cloud price:499] score:0.1111111111111111]

Execution Time: 2922ms

=== Token Usage ===
Prompt tokens: 616
Completion tokens: 75
Total tokens: 691

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: cW1cPq-jltIWG_-9ktoHU7rmTVOUdCHtyHfIf3qKNv2xf70zgTMgqEs5j74CEdrtwEPO8J8FdZPBC7ZzrPe3EA

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is a high-performance database product named "ekoDB". It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 3vmzfLB-WmvB8idZJB7jLZqj0YQQ49rOv6FirnUVrQeZrdJ1DLq76y_li-PXJCbZtnxU8LGX_0daiNa5PI3Z_A
  Parent: cW1cPq-jltIWG_-9ktoHU7rmTVOUdCHtyHfIf3qKNv2xf70zgTMgqEs5j74CEdrtwEPO8J8FdZPBC7ZzrPe3EA

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: 3vmzfLB-WmvB8idZJB7jLZqj0YQQ49rOv6FirnUVrQeZrdJ1DLq76y_li-PXJCbZtnxU8LGX_0daiNa5PI3Z_A (Untitled)
  Session 2: cW1cPq-jltIWG_-9ktoHU7rmTVOUdCHtyHfIf3qKNv2xf70zgTMgqEs5j74CEdrtwEPO8J8FdZPBC7ZzrPe3EA (Untitled)
  Session 3: IA5-rWYt2lejYRO9aNTZg4W-5TeMAvxB42a9TSCPeQw1CwR_yBAuwn2TANGMOssgAlVBKIqvHCr9wPW1BHcn5g (Untitled)
  Session 4: kjK1pDTNLOKHPH9OvZuvvQLFzTMCSQt-byLD2bA78E2U9VUsJHSz_I5zfuQo1BXHoSW53CpQnsFBhK4Q530HVw (Untitled)
  Session 5: r8-q1NPgr8SOnlJxjWfntm6zuChhix2h6xtYe5ervcSBCOE-h8CFI_i7m9pJxLVzK_gop-dpdEHNtpNLVEH2Zg (Untitled)
  Session 6: YQzzCfZjPdw5kP1aBN6THyY1bRvLMtCvolkzyPoRuQRuLMHMWeb6xYWJbJXPi-iZTzqBLMxeJJFJzPXJlfwLbQ (Untitled)
  Session 7: kjC5gFDXqUMkJtTG9oQxoJ4GZsqn4-JnYCiIcGGKMR0243BGdCUoT_VC4DQP5SHwUY2nER_UmhkyajKM6mJdfQ (Untitled)
  Session 8: SI0IVjSitRK4WC0Rq1i2_ftWMw6wc_a-F_5jxxLXdDFcbDYkAVkRLYe-vu7rx-cnpa6CbU66anXhCjolvyc_ZQ (Untitled)
  Session 9: _7tHWwvnTeyUz_55D47bdvKv-LR17o5N-3frkweqyiY75TE85p882gSX8NaWWrOIRzliqYJXFtiK6qpW5i5ydQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 3vmzfLB-WmvB8idZJB7jLZqj0YQQ49rOv6FirnUVrQeZrdJ1DLq76y_li-PXJCbZtnxU8LGX_0daiNa5PI3Z_A

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: PV6OHVsfD57CjKcOBKZcXPrKUhYEB3cGU7rr2xylkI6kd1J4LaZeg2937gfgLya0Yy9tWQOhJSNSGAk9kniZjA

=== Sending Initial Message ===
✓ Message sent
  Response: The available product from the context is ekoDB, which is a high-performance database product.

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
✓ Created second session: aRl2LkGcRd7T25p5PRat3KIdAs16ofTpaDxFNR29iOaKKEM6fI71deiXeF0clC7DGdm3GL9KoHsZbKVfRlwKSg
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
  1. Score: 0.785
  2. Score: 0.776
  3. Score: 0.759

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.714
  2. Score: 1.503
  3. Score: 0.311

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
🚀 ekoDB Scripts Example (Go Client)

✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: CtMvQnI37q0ZxQT1Rtzvs3k2sDeqMHJ9lstW-mNMjzyWO_aa_TC5-Od7PX7d6DERHg41fifpbp5Nu6tk0QGV0A
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
Created Alice: $1000 - ID: Vmrf6ef8h3aeibUHq1V_30fzK3qTW2HMeIiGYO1f4l-U-N6YoxHPlw4KA-keeNiGsQ3iMQcthltyNTX6Q-KzVA
Created Bob: $500 - ID: RTWIiJf7IqqIoH6-JDgColpjsMfAe70bYo_TSr8WB2ravow0TZzTo7hYuUmetSPwYn3Qx1Q1HdCTEWAU2zbsnA

=== Example 1: Begin Transaction ===
2026/01/03 04:51:26 invalid isolation level: ReadCommitted (must be one of: READ_UNCOMMITTED, READ_COMMITTED, REPEATABLE_READ, SERIALIZABLE)
exit status 1
make[2]: *** [test-examples-go-client] Error 1
✅ [32mAll Go integration tests complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.7.0 prepare
> npm run build


> @ekodb/ekodb-client@0.7.0 build
> tsc


up to date, audited 7 packages in 845ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 661ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'hZ2ERKSh2N_9d85MXl37TWx2L6221p4kenhShpKqYvrslKXpGdRF92ZH4_YX8CRDwV6pPHfNC-xOXgqbk3J28g'
}

=== Find by ID ===
Found: {
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  price: { type: 'Float', value: 99.99 },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  active: { type: 'Boolean', value: true },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  id: 'hZ2ERKSh2N_9d85MXl37TWx2L6221p4kenhShpKqYvrslKXpGdRF92ZH4_YX8CRDwV6pPHfNC-xOXgqbk3J28g',
  name: { type: 'String', value: 'Test Record' },
  created_at: { type: 'DateTime', value: '2026-01-03T09:51:28.852Z' },
  value: { type: 'Integer', value: 42 }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-03T09:51:28.852Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  metadata: { nested: { deep: true }, key: 'value' },
  price: 99.99,
  categories: [ 'electronics', 'computers' ],
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  active: true,
  data: 'aGVsbG8gd29ybGQ=',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  tags: [ 'tag1', 'tag2', 'tag3' ],
  id: 'hZ2ERKSh2N_9d85MXl37TWx2L6221p4kenhShpKqYvrslKXpGdRF92ZH4_YX8CRDwV6pPHfNC-xOXgqbk3J28g',
  name: 'Test Record',
  created_at: '2026-01-03T09:51:28.852Z',
  value: 42
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  value: { value: 100, type: 'Integer' },
  id: 'hZ2ERKSh2N_9d85MXl37TWx2L6221p4kenhShpKqYvrslKXpGdRF92ZH4_YX8CRDwV6pPHfNC-xOXgqbk3J28g',
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  created_at: { type: 'DateTime', value: '2026-01-03T09:51:28.852Z' },
  active: { type: 'Boolean', value: true },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  name: { type: 'String', value: 'Updated Record' },
  price: { type: 'Float', value: 99.99 }
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
cache:product:1: { price: 29.99, name: 'Product 1' }
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
Collection created with first record: r06iZtsqwfiY3HvxHFY02yENMaESJznJl8eTS0U8CR1VQIYmj6SbSPqcx4W8CTFbLn7Xx9b6VdcXeGkYwmmBjA

=== List Collections ===
Total collections: 18
Sample collections: test_accounts,schema_users_client_go,client_collection_management_ts,schema_employees_client_go,ttl_cache

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: v1fOVp5jhBN8dYwrmlhYEibG7kC3TYT8YXKvVdRlsALI3twTdFxtJpS0W-pdxVf2DJnlDsm5k-1dfUqw1xrrfQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: LvkVQwMievbOovAjq-aYB2JBGvkAdVltKeLSsf_vHxPlcExtJ0nj_EPfn5L-LfWoHezdthLfxwzBLS2YPErMbg

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
✓ Inserted test record: ws367Y0WVPj7aaVdBR1bLPXesRqKbLYesaipwLVTZzb3CKg6TYX0XEE-Wu3PrC7w_KsX21Worc06sROsw2Q7hQ

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
✓ Inserted document with TTL: J9-vEWJGZMeAhn5ui9uMWr7szDCutLHPHrTkFVKlcVTJwOv16bpfn_mFbKD5GA2ebO3XZ6RFMHnKKbnpx9klbA

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
✓ Created session: THUFBEFfdkmCMHn2hdYXm4n0jJJ9YnsomL5MpahhsHwun4UkrANjuobwskFCQzkTmcJPnE9KBefdHotFCGlrBQ

=== Sending Chat Message ===
Message ID: 4-9lnihgjt8USsPnGXsFcUm8VC7BXNwKQUuFHKZjJRdOx0wapWahWEBFctthZBRsWWM8VWXaI9biFJCP7-1Yvw

=== AI Response ===
There are three products available:

1. ekoDB - A high-performance database product with AI capabilities, priced at $99.
2. ekoDB Pro - An enterprise edition product with advanced features, priced at $299.
3. ekoDB Cloud - A fully managed cloud database service product, priced at $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 299,
    name: 'ekoDB Pro',
    id: 'cRxzon1HZjl6InRD0VHP4nMSvDRzDm5mwM80jmqn-ViMURHk0PRqWjI9YF-nuBfKJwDpVNoHo8Rsg88IuausTw',
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    id: 'uiCcYZq4KFdMJ7XNqVXqK8EFFUEP8SGc93XZEVkpqBIvMkYIeVUvNnPB-EsAY23_YYWthigDA0ZtBqryPMkhfQ',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Cloud',
    id: 'noHSSG6sSdfnkzEHWylRLG_rNmUQPfkapFbDWqnHP7GQRXBi2gb41I8sdOUVhtPFTdKO2W8NDORBrWn7MhIZVA',
    price: 499,
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 1809ms

=== Token Usage ===
Prompt tokens: 603
Completion tokens: 66
Total tokens: 669

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: g-_bT_eNpoXLhbWtsQaFoZBaZ1XThT42geudrg3gWt0bMgB0VETxtXs4hKsIQmF4MmT_GAvKMB-QqQC3FIzFhA

=== Sending Messages ===
✓ Message 1 sent
  Response: Answer: The product available according to the provided context is "ekoDB". It is a high-performance database product.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: fo0Jvez4Z0G95m88_BG8MxDasRd8Dk-jR2kbMF-lzgCLAiHbCvuCYkNCWOBwK392uYLyozoatkkyGdGF5wHuEQ
  Parent: g-_bT_eNpoXLhbWtsQaFoZBaZ1XThT42geudrg3gWt0bMgB0VETxtXs4hKsIQmF4MmT_GAvKMB-QqQC3FIzFhA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: fo0Jvez4Z0G95m88_BG8MxDasRd8Dk-jR2kbMF-lzgCLAiHbCvuCYkNCWOBwK392uYLyozoatkkyGdGF5wHuEQ (Untitled)
  Session 2: g-_bT_eNpoXLhbWtsQaFoZBaZ1XThT42geudrg3gWt0bMgB0VETxtXs4hKsIQmF4MmT_GAvKMB-QqQC3FIzFhA (Untitled)
  Session 3: THUFBEFfdkmCMHn2hdYXm4n0jJJ9YnsomL5MpahhsHwun4UkrANjuobwskFCQzkTmcJPnE9KBefdHotFCGlrBQ (Untitled)
  Session 4: aRl2LkGcRd7T25p5PRat3KIdAs16ofTpaDxFNR29iOaKKEM6fI71deiXeF0clC7DGdm3GL9KoHsZbKVfRlwKSg (Untitled)
  Session 5: cW1cPq-jltIWG_-9ktoHU7rmTVOUdCHtyHfIf3qKNv2xf70zgTMgqEs5j74CEdrtwEPO8J8FdZPBC7ZzrPe3EA (Untitled)
  Session 6: IA5-rWYt2lejYRO9aNTZg4W-5TeMAvxB42a9TSCPeQw1CwR_yBAuwn2TANGMOssgAlVBKIqvHCr9wPW1BHcn5g (Untitled)
  Session 7: kjK1pDTNLOKHPH9OvZuvvQLFzTMCSQt-byLD2bA78E2U9VUsJHSz_I5zfuQo1BXHoSW53CpQnsFBhK4Q530HVw (Untitled)
  Session 8: r8-q1NPgr8SOnlJxjWfntm6zuChhix2h6xtYe5ervcSBCOE-h8CFI_i7m9pJxLVzK_gop-dpdEHNtpNLVEH2Zg (Untitled)
  Session 9: YQzzCfZjPdw5kP1aBN6THyY1bRvLMtCvolkzyPoRuQRuLMHMWeb6xYWJbJXPi-iZTzqBLMxeJJFJzPXJlfwLbQ (Untitled)
  Session 10: kjC5gFDXqUMkJtTG9oQxoJ4GZsqn4-JnYCiIcGGKMR0243BGdCUoT_VC4DQP5SHwUY2nER_UmhkyajKM6mJdfQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: fo0Jvez4Z0G95m88_BG8MxDasRd8Dk-jR2kbMF-lzgCLAiHbCvuCYkNCWOBwK392uYLyozoatkkyGdGF5wHuEQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: RqHn5Rv7510ycwxyLP4Dj_qeNVOsdk6NIIWCuMIXSOkgEChTPudSesTWL_EKha3vq9T88BeWLkWYravcpNhsag

=== Sending Initial Message ===
✓ Message sent
  Response: Currently, the available product is ekoDB, which is a high-performance database product. It is priced at $99.

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
✓ Created second session: JGeXXCkOoElfPsWHUpLTVXdLCBr0umT43ZDLGkWabp_fOmMK_meJTdznkscieQlsmFufkGttBgS4SYVAO_pNqw
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
  1. Score: 25.740, Matched: email, email.value, name.value, name
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio, bio.value, title
  2. Score: 26.400, Matched: bio.value, title.value, bio, title
  3. Score: 26.400, Matched: title.value, title, bio.value, bio
  4. Score: 26.400, Matched: bio.value, bio, title, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, bio.value, title, title.value
  2. Score: 39.600, Matched: title, bio, title.value, bio.value
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
  1. Score: 0.756, Matched: 
  2. Score: 0.741, Matched: 
  3. Score: 0.736, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.694, Matched: title.value, content.value, title, content
  2. Score: 1.502, Matched: content, title, title.value, content.value
  3. Score: 0.296, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio.value, skills, bio, skills.value

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

✅ Script saved: DXu3oI_nCag5eyD79RDmV4yZf-YjfTVxOpitZclxZBMXNZgu3nX-I_B6v-5_VqSul7LRTqEQzj8I92QtbDGI3Q
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: 1q2EpYU64cVU02PEtiR9HSLtU2HbchcaEPGXmGaxLDtIZkkQFW5j0jfCAIIN1etUlf-dth8CVTU_s3X3p73_Hg
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: ry8oKS4zZwAKjURsZBYGAuQvsIXBgzEmMvAUxEJs2wOsdbLZjoI5sYEY6M6Uu23g4eTVK0TyILH9VUOzr6wJyQ
📊 Statistics: 2 groups
   {"status":"active","count":5,"avg_score":60}
   {"count":5,"avg_score":50,"status":"inactive"}

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

✅ Script saved: 6U7oKn5ZfV7NpXgLVyScd5J3UhHXgFxwZSri502EW6cX1TyD6DO-pgbesQl4CFhzExUSb40W8NicwmxiqX_4Gw
📊 Found 2 product groups
   {"category":"Furniture","avg_price":474,"count":2}
   {"category":"Electronics","count":3,"avg_price":575.6666666666666}
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
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: C9ePxBiKlVMTH9eg13HpgPXBzXBOVnu91LH8naA9zrZCkB6jlRTWnZPB2TIUN5bB9an9zwqUCOtp6gGtlC3_Yw
Created Bob: $500 - ID: Bu4z6PxyhdOHzMhURYsn-W5NiC6sjCMzbPbtfxfyrzp890kNTACRHdm8Emy3O5AfDEQXYLhPOMrMZD8Zh9p3dQ

=== Example 1: Begin Transaction ===
Transaction ID: 4c5a68b6-45c6-4bc9-a9b0-dc7a20a53db9

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: a94aedca-0a70-4791-8a6f-8c47c9de2518
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mTypeScript client examples complete![0m
✅ [32mAll TypeScript integration tests complete![0m
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 642ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
(node:26761) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
[32m✓ Server is ready[0m
[34m
=== Getting Authentication Token ===[0m
[32m✓ Authentication successful[0m
[33m
=== Running 10 Examples ===[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js ===[0m
✓ Authentication successful

=== Insert Document ===
(node:26764) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Inserted: {
  id: 'LoWZ4G86hvi1KDVES8pu1ypces-_KaR2gIxx0deYDF7aweIwj0oV0VyvT0767ChYDlmnf6agrPCsZzfP8GhWqA'
}

=== Find by ID ===
Found: {
  value: { value: 42, type: 'Integer' },
  active: { value: true, type: 'Boolean' },
  id: 'LoWZ4G86hvi1KDVES8pu1ypces-_KaR2gIxx0deYDF7aweIwj0oV0VyvT0767ChYDlmnf6agrPCsZzfP8GhWqA',
  name: { type: 'String', value: 'Test Record' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'LoWZ4G86hvi1KDVES8pu1ypces-_KaR2gIxx0deYDF7aweIwj0oV0VyvT0767ChYDlmnf6agrPCsZzfP8GhWqA',
  name: { type: 'String', value: 'Updated Record' },
  value: { type: 'Integer', value: 100 },
  active: { type: 'Boolean', value: true }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js ===[0m
✓ Authentication successful

=== Inserting Test Data ===
(node:26773) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted test record: yV7smnFmR21AhWS983k-h4ILOnt_mQAxDgGfjBb08tBDEY379Rd6pjtd353vDAOb_3E0EKkcp7PKVPVWXNyKAA

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
        "id": "1JNp5xbnQN5NRBa8fEEUHRYA9jRzodnChyV6YjAfYfef-s0OYAwTz4IUNZObWzDqbBNaYEZYexQ6TF4zhalGVQ",
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
        "id": "kL8DVSUh7_LbJlGQ8Jp8rpmf3HJUvw0sR1K61EIbOfhPQNbCdjciPwd8xTHlb6AY0AiPX1Gmo0kplRygdd5vow",
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
        "id": "IjdISjptT6bysMqTDvn82myude-F1EHII_2b_ly8-sMxVL2QcMVlBbQPhso4ZGmgB5bQBHKla7FbAnTqScFsJw",
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
        "id": "yV7smnFmR21AhWS983k-h4ILOnt_mQAxDgGfjBb08tBDEY379Rd6pjtd353vDAOb_3E0EKkcp7PKVPVWXNyKAA",
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
[32m✓ simple_websocket.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/batch_operations.js ===[0m
✓ Authentication successful

=== Batch Insert ===
(node:26790) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/kv_operations.js ===[0m
✓ Authentication successful

=== KV Set ===
(node:26793) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {
  loginTime: '2026-01-03T09:51:51.452Z',
  userId: 123,
  username: 'john_doe'
}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
[32m✓ kv_operations.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/collection_management.js ===[0m
✓ Authentication successful

=== Create Collection (via insert) ===
(node:26810) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Collection created with first record: PZfR3ak4HRS4Bqzy3JZyhZgpSB6JmckVmrfSOvLVTL8J6xDrnA1SxD_yvJe_Asd-yH0Jlo50xKdTEVqX-Ek9cg

=== List Collections ===
Total collections: 23
Sample collections: [
  'schema_users_client_ts',
  'test_accounts',
  'schema_documents_client_ts',
  'schema_users_client_go',
  'users'
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/document_ttl.js ===[0m
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
(node:26813) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document: 8WHS4K_8LsHm-TKsCmVXgj-d5qsiJFUv7QDL8gAmWKv_1utfpBmq72s7tkF-pO6B6y8bdbW4vZFCZt5GNoecsQ

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: created

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: Bb20PKJzxscxtDPvsPz1VXswT0S9r9qdDLjnv-5FrdZFDilV953P6kIWJj9l6_hIwUKwCvF_5APkPe8ajQMYmA

=== Query Documents ===
✓ Found 7 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
[32m✓ document_ttl.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js ===[0m
✓ Authentication successful

=== Insert Test Data with TTL ===
(node:26816) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document with TTL: ourLizFRMJS7jc8lCK7jiluNp9r-k581vhTNFsUtLbVwPo6IPxQCZhZ26pItcTuECR5H8tGIVOBuw_2Au2EJCQ

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 4 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
[32m✓ websocket_ttl.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/http_functions.js ===[0m
🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: nD8CvlKQ8jsvVb_Bhpv5oi3dtG-TcXvRV7qtJlM2oXCgf0aFx6ZiUFWsd0nlMUZfzNHPzEY0fQPz-VqeVwjv5g
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: onV_-nbElDaRg-fZ4SlqLG9yZVgM-wYCmpC4sVmVamdbgLwz93UT0pnRED-_iwvttDLyWcvOGIO4PTdEDCG1tA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: RynfYkHLf9C1rpXFzx0y2z7p66cadFF-Cqk8tnnf7AAV0Y_HLqvLPlFVjfhOrRJEgx1USlmxx1NH6NL-q02tig
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":"inactive","count":10,"avg_score":50,"max_score":90}
   {"avg_score":60,"max_score":100,"count":10,"status":"active"}

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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/transactions.js ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: QWGl0Z2z9ri4M73_ykJatrWvURgCM8WN0pc5ECWHOGXCSFG__NCwsJmj1lvsX4lcutEELzj_JEvflk7Q2Sk6BQ
Created Bob: $500 - ID: JEQvLOw7Z7oLGozZl1dljY-oe4ZO1XB63O8D4B150WY3HAyUvSPODa-2euUZoCidIz412HQu8wcaXvmeoRm7_w

=== Example 1: Begin Transaction ===
Transaction ID: ae0ecbf8-2ea9-4597-9fb8-b386ee6c10f2

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
New transaction: 108c48a8-ae8e-4ba7-b417-8839b945df63
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
   ✅ Script saved: s4rQAkKS7Zfty82MZHTCtKX9435yxdLPiiYa--VwxbIcC_zSFQrr4VJxTIenBEFX12GHoAFgd7zVmPiYjHVGbQ

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
   ✅ Script saved: oXo83bzpAcKH-bWs9YtupDjOhChjkAFtH1HE7ND19WVahRyD7ZpkUygZm4UkPaRfshVTscwZvn5Sz7SRlHkVzQ

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
   ✅ Script saved: VGd4W8xpHGjjQTDEiHX0MFT9LxnG3zxx885sgVBwC9VhUZJ4kAEb6MAvoOLsIMYdu9nu1MKMINd1nU8SXe55wQ

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":0}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: RIEmi_z_4PXhwtbR_lKYBiro8bNGfFsv_haiHgBS3A0f8euOd_PQ5oCMHpfwVXZuvyityzaD1zC0Cbs2Se_kvg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: s4rQAkKS7Zfty82MZHTC...
   ✅ Deleted script: oXo83bzpAcKH-bWs9Ytu...
   ✅ Deleted script: VGd4W8xpHGjjQTDEiHX0...
   ✅ Deleted script: RIEmi_z_4PXhwtbR_lKY...
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
╔════════════════════════════════════════╗[0m
[34m║           Test Summary                 ║[0m
[34m╚════════════════════════════════════════╝[0m
[34mTotal: 10[0m
[32mPassed: 10[0m
[32mFailed: 0[0m
✅ [32mJavaScript direct examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.7.0 prepare
> npm run build


> @ekodb/ekodb-client@0.7.0 build
> tsc


up to date, audited 7 packages in 835ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 651ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'a-IGcCgD0lsNfkQDaSHvOZkYFv196kIwUzh459vFHYY9wtDZH9WoS5sKHc1OmCLqMILoYSmYQZw6Pgimzz6ahw'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  id: 'a-IGcCgD0lsNfkQDaSHvOZkYFv196kIwUzh459vFHYY9wtDZH9WoS5sKHc1OmCLqMILoYSmYQZw6Pgimzz6ahw',
  value: { type: 'Integer', value: 42 },
  active: { type: 'Boolean', value: true }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { value: 100, type: 'Integer' },
  id: 'a-IGcCgD0lsNfkQDaSHvOZkYFv196kIwUzh459vFHYY9wtDZH9WoS5sKHc1OmCLqMILoYSmYQZw6Pgimzz6ahw',
  active: { type: 'Boolean', value: true },
  name: { value: 'Updated Record', type: 'String' }
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
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

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
Collection created with first record: VrGRmvAvWDF6aPRoJVPpEVkG40t7Otxoqk4w0XJVbDbBJv_TWic2VZtGnPNF_tUeKgJpbKNSNm7gvBwJNIHXHw

=== List Collections ===
Total collections: 22
Sample collections: schema_users_client_ts,test_accounts,schema_documents_client_ts,schema_users_client_go,schema_employees_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: iSJqaGwsk7D29f2U09pUMCj7rRFEeuWmLrNTNYFolKKLs5ndVuSiuDrz8R02M1n6Kv4Ru9rFnUgzXTOaROXX6g

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: kEL6ghaF4aWuacFAT4v2CdhXEJ-WrYT0dsze1ih-KLd0tSOq4Ovmyv-lGftfWbMWVS9HuafDgXFyLp0rLhrnOA

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
✓ Inserted test record: RWslWjZExqNPzbeFyTIvovypU4ZTM90cwmaK5b3HqRDf7aTPASotOg1AjICfOD3kywv8aTk_3VtmJ4cF8rmnzw

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
✓ Inserted document with TTL: GzcG9o2FOlIlyAUuG9z9ojbEAnbs7c5wYVm334V-lnUQSCJg8WefnHYeFwPT4XkLkQ0mW1bVoJdXyWP4XAv_uQ

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

✅ Script saved: sobESycYEskQzHGI8Md5nmcaTOCTKBs06mUwARwa-GEVuTcQsBa9vmbLOYjdMOgfi-A6HiKO3TSLKB2PQ13Cbw
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: DE6ioxgMKMdVUx8KALd7H8sYi0n9dwmoIhT2GtUyxJDqw5U5X_8HgECbiA3inlLCzz5pVXXe5LaSgtxTbJEpdQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: Nr8tcyzNVcsGtL4U6O6ykYGC6ZBBZjBA93SkZ3ZXH9XjGvIdWbRB_MM8uVTNpEwPLahPMGA0G00PI6TAiCDFEA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":5,"max_score":90,"avg_score":50,"status":"inactive"}
   {"status":"active","max_score":100,"count":5,"avg_score":60}

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

✅ Script saved: VxcDcHbRxjMUT2vxSluWH6VdJFKoGNg5vnOwQome5SoBaRtqXIEyjdjGGGnnBN0YwFO9D5WH5PNYXWkI7ioX5w
📊 Found 20 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 20 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"count":10,"status":"active","avg_score":60}
   {"count":10,"avg_score":50,"status":"inactive"}
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
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved
📊 Product summaries (10 items, showing first 3):
   1. Standing Desk - $599 (⭐4.7)
   2. USB-C Cable - $19 (⭐4.3)
   3. Bookshelf - $149 (⭐4.1)
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
   active: 7 users
   inactive: 3 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved
📊 Average scores by role:
   user: 70.0 (7 users)
   admin: 20.0 (3 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved
📊 Users (showing first 5 of 0):
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
   2. Getting Started with ekoDB (Database)
   3. Vector Databases Explained (Database)
   4. Introduction to Machine Learning (AI)
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
   1. Natural Language Processing
   2. Getting Started with ekoDB
   3. Vector Databases Explained
   4. Introduction to Machine Learning
   5. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Natural Language Processing
   2. Getting Started with ekoDB
   3. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Natural Language Processing (AI)
   2. Getting Started with ekoDB (Database)
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
   1. Efficient Spatial Analysis: Vector databases allow efficient spatial analysis and complex manipulations.
2. Detailed Representation: They provide more detailed representation of geographic features.
3. Scalability: They can be easily scaled from small to large without loss of detail or accuracy.
4. Storage Efficiency: They require less storage space compared to raster databases.
5. Flexibility: They have the ability to attach attributes to individual elements, making them more flexible. 
6. Precision: They offer high precision as they use points and their exact coordinates.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Draft Article" - 1536D vector
   2. "Getting Started with ekoDB" - 1536D vector
   3. "Advanced Query Patterns" - 1536D vector
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
Created Alice: $1000 - ID: MHu5IPQOosjPoAxqZWVoYAQx71b1gE96r6BqjFUXiNARqNTDku9l-1X7SvUfibVufZjj_1_7quXcGw6MMlCeBw
Created Bob: $500 - ID: dSbEz0dEMFxNQ5rXSf4ivtYyNNmb0s69-1nfEKStqZXpXOu59J1yFl05n94RrgdkDYtWSlCkDBS5w3PFCaMEew

=== Example 1: Begin Transaction ===
Transaction ID: be8a73fd-356c-4b3f-819e-48b228d5342c

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 8f128318-4be7-4090-a8d5-f50e92bcbfe6
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
> Task :compileTestKotlin NO-SOURCE
> Task :compileTestJava NO-SOURCE
> Task :processTestResources NO-SOURCE
> Task :testClasses UP-TO-DATE
> Task :test NO-SOURCE
> Task :check UP-TO-DATE
> Task :build UP-TO-DATE

BUILD SUCCESSFUL in 3s
2 actionable tasks: 2 up-to-date
✅ [32mKotlin client built![0m
🧪 [36mRunning Kotlin client library examples...[0m
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Simple CRUD Example ===

=== Create ===
✓ Inserted user: Record(fields={id=StringValue(value=8b1mTQ1yIK8YF-CtIKdNnrkG8uEBj3YMtUq8A6DKdRLvKSmpPaq26C1j7zIFtSwg2q0NC8OM1JqWaG7VhsUE4Q)})
  User ID: 8b1mTQ1yIK8YF-CtIKdNnrkG8uEBj3YMtUq8A6DKdRLvKSmpPaq26C1j7zIFtSwg2q0NC8OM1JqWaG7VhsUE4Q

=== Read ===
✓ Found user by ID: Record(fields={data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)})}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), id=StringValue(value=8b1mTQ1yIK8YF-CtIKdNnrkG8uEBj3YMtUq8A6DKdRLvKSmpPaq26C1j7zIFtSwg2q0NC8OM1JqWaG7VhsUE4Q), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), created_at=ObjectValue(value={value=StringValue(value=Sat Jan 03 04:52:23 EST 2026), type=StringValue(value=String)}), age=ObjectValue(value={value=IntegerValue(value=28), type=StringValue(value=Integer)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Sat Jan 03 04:52:23 EST 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {nested={deep=true}, key=value}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: [data, email, embedding, metadata, user_id, price, tags, categories, id, name, active, created_at, age]

=== Update ===
✓ Updated user: Record(fields={id=StringValue(value=8b1mTQ1yIK8YF-CtIKdNnrkG8uEBj3YMtUq8A6DKdRLvKSmpPaq26C1j7zIFtSwg2q0NC8OM1JqWaG7VhsUE4Q), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)})}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), created_at=ObjectValue(value={value=StringValue(value=Sat Jan 03 04:52:23 EST 2026), type=StringValue(value=String)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])})})

=== Query ===
✓ Found 1 users matching query
  - Record(fields={tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), id=StringValue(value=8b1mTQ1yIK8YF-CtIKdNnrkG8uEBj3YMtUq8A6DKdRLvKSmpPaq26C1j7zIFtSwg2q0NC8OM1JqWaG7VhsUE4Q), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)})}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), created_at=ObjectValue(value={value=StringValue(value=Sat Jan 03 04:52:23 EST 2026), type=StringValue(value=String)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)})})

=== Delete ===
✓ Deleted user with ID: 8b1mTQ1yIK8YF-CtIKdNnrkG8uEBj3YMtUq8A6DKdRLvKSmpPaq26C1j7zIFtSwg2q0NC8OM1JqWaG7VhsUE4Q

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

=== Example Complete ===

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
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
  IDs: _Ab6KGmiNj1wIFXeaiadA2HUOl8p8aYZIqP6L71iJ9Xa-1baWgwLU85d4qMZiTVeGSyei4em1lJ1EKffNy5nQw, orM3vlQrm6c8HFe6vSU5aMrhizH0XvMNtnJRwbD2dCZUwz0-6H0lJwYHnuqUju9taZsOwuJnNkCedEwegeXf3Q, yhPd0dkX2eKIl4vmA036Au5BR4-u1Z5sSbMPAjqPjMUTzLgAd5AIeO-05cmRa5OiXsuauGtRmizBHl7pkdulfA...

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
=== ekoDB Kotlin Client - KV Operations Example ===

=== KV Set ===
✓ Set key: user:123

=== KV Get ===
✓ Retrieved value: {"name":"Alice","role":"admin","email":"alice@example.com"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"created_at":1767433957019,"user_id":"123"}
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
  - schema_users_client_ts
  - test_accounts
  - schema_documents_client_ts
  - schema_users_client_go
  - schema_employees_client_go

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-01-03T09:52:41.298225Z","last_modified":"2026-01-03T09:52:41.298225Z","bypass_ripple":false,"primary_key_alias":"id"}

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
  Document ID: Y2zRA-cFVqzryWtl6PlRH8X8zmg3FpI8747Jb4ut4rYBFqO9PGxipqKsrYAlXVcusUVFRGmfUUBJcQUJjvXDAw

=== Verify Document Exists ===
✓ Document found: [id, session_id, user_id, created_at, ttl]

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: PchkVdu3xMuRg5ytTmjBxUH2_WqK-JsaL0S2VdymYjwKIJEbmAVGYbSvxwv38xaYEskftyQPUBM5WEC0n4H2kQ

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: [id, value, cache_key, ttl]

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
  {"payload":{"data":[{"id":"kfOC-N8kG8oiL3BLnjO1FSFOGnCaN_mWUdQFiKUcFjW_Jf0ba_Ig7-KG1Xbj7N4Orxo3Q9S_wLD1uKojiR041g","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

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
✓ Inserted user 1: StringValue(value=gXSBJisugc1mMTCoTCeIFWxPCsoD7T60At2h17QxY91Ugx_rzHUB3eyVTXe8tuOURUMebBl1uAfVwIO3VWHiPw)
✓ Inserted user 2: StringValue(value=rtrTi5nwaYRTh7mFWT8GipDYEpgEWeyPZMeilHR7otwSQXyAnZRXoXJJGo1SlsHFz11O9LI9Uwd2HzFYZwkrwg)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 22
  Sample: [schema_users_client_ts, test_accounts, schema_documents_client_ts, schema_users_client_go, schema_employees_client_go]

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
  {"results":[{"record":{"id":"6Hhcw61Yy0kUWN6TW98TemFMc79fzOD_eHgij3Byo3n5lmI309PEx0ZuesxQ8-O7Fc8jWyGVwvVRxwNuENieNA","title":"Rust Programming","views":25,"tags":"programming,rust,tutorial","description":"Learn Rust programming language with hands-on examples and best practices."},"score":19.8,"matched_fields":["tags","title","description"]},{"record":{"views":979,"tags":"programming,javascript,web","title":"JavaScript Web Development","description":"Build modern web applications using JavaScript, React, and Node.js.","id":"OCUWTgbklN4lVc1s2Xu6OvdOf0X0IizBSN4aQPEHi6TL9q_6zjwV8Yydn3qBTwWHgVWUAm8gdGwzvP5rV2wofA"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"views":610,"tags":"programming,python,data-science","description":"Master Python for data analysis, machine learning, and visualization.","title":"Python for Data Science","id":"TnL9Qw7UhWoviBV6Mj53n5zQIaQd4HNSj7NLsPxDrUrEWAoL3HgjcmeC456JP0r6aL77lOYjgpz9SE6M9rVqoA"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"id":"HXYd4b9w3duLnFFBd2vlXrmxQuFXei_B0aVLMysOkG-VgulHw7M816c8TJCQZXYeo2E2P7Z-CgM0os9NBRX-Uw","description":"Introduction to machine learning algorithms and neural networks.","views":152,"title":"Machine Learning Basics","tags":"ai,machine-learning,python"},"score":2.7,"matched_fields":["tags","description","title"]},{"record":{"tags":"programming,python,data-science","id":"TnL9Qw7UhWoviBV6Mj53n5zQIaQd4HNSj7NLsPxDrUrEWAoL3HgjcmeC456JP0r6aL77lOYjgpz9SE6M9rVqoA","description":"Master Python for data analysis, machine learning, and visualization.","views":610,"title":"Python for Data Science"},"score":1.0,"matched_fields":["description"]},{"record":{"id":"6Hhcw61Yy0kUWN6TW98TemFMc79fzOD_eHgij3Byo3n5lmI309PEx0ZuesxQ8-O7Fc8jWyGVwvVRxwNuENieNA","tags":"programming,rust,tutorial","title":"Rust Programming","views":25,"description":"Learn Rust programming language with hands-on examples and best practices."},"score":0.5,"matched_fields":["description"]},{"record":{"title":"Database Design","description":"Learn database design principles, normalization, and query optimization.","views":108,"id":"zk32qLziUvPF5ApiOcVKekFjp7hlWqW4nyLTkzlOuS8E4tIM2xfBB1G10UNX0WwWdxZvFbLxwzOZmEgE6nR70A","tags":"database,design,sql"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
✓ Inserted document with TTL: StringValue(value=C7kUr8Z4mvXtivKXA132B_S1aIIqBuBUx4Tfcu0mBPD2MD75oa0zyvUBjXRIBBcUSMW6k8TYu4n8sumTmkiWiA)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1767433982446},"id":"C7kUr8Z4mvXtivKXA132B_S1aIIqBuBUx4Tfcu0mBPD2MD75oa0zyvUBjXRIBBcUSMW6k8TYu4n8sumTmkiWiA","name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-01-03T10:53:01.581360Z","value":{"type":"Integer","value":42}}]},"type":"Success"}

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
✓ Created session: ogHFF9JHm-yi9wGYyRkWhv13bogGjsbqUD-778WeVKsH-D2BZJCP9iFMJf49RTmczD1iBKPp880R2iGnxW4-8g

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "7-7VLKaGz1OPk_vIf6DJUQhvxkNO_LlH6dOXCR3zGwqhrIGoaieAhsQZ-uub-ArJSTlucnOpNqtajpqwniSgxg"
  Responses: ["ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration. It comes with several features such as AI Chat Integration and advanced Search Features. The AI Chat Integration feature allows you to query your database using natural language and get AI-powered responses with relevant context. On the other hand, ekoDB's Search Features supports full-text search, vector search, and hybrid search with automatic context retrieval."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

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
✓ Created session: tgYfN9Gcu866oncPr15ikCp5WKE4kkLzNY8KwrRXkh91RCq5MUrXeuP5QuUszAQAYYwEnk-a2IgBCZdmBJmhYQ

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The product available is called ekoDB. It is a high-performance database product with AI capabilities. The price for this product is $99."]

✓ Message 2 sent
  Responses: ["The price of the ekoDB product is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"tgYfN9Gcu866oncPr15ikCp5WKE4kkLzNY8KwrRXkh91RCq5MUrXeuP5QuUszAQAYYwEnk-a2IgBCZdmBJmhYQ"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"8Wy_6lcV_LcKFg0jSrFkNyvfFjpFypYPW-YS-eYB106fWRWusqQqXcYW0jwD_T6BDMnOqpy6DayULPePt-ZLUg","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-03T09:53:19.940896Z"},"id":"0Pi7Ahk8sw7yTqOondANeEc0WNUqylTax5LSh6H2KBhEnjSW1yt7hw62Npu9orAG9Gm1oM_yCd57_ml7b4KIjg","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":29,"prompt_tokens":237,"total_tokens":266}},"updated_at":{"type":"DateTime","value":"2026-01-03T09:53:19.940896Z"}},{"chat_id":{"type":"String","value":"tgYfN9Gcu866oncPr15ikCp5WKE4kkLzNY8KwrRXkh91RCq5MUrXeuP5QuUszAQAYYwEnk-a2IgBCZdmBJmhYQ"},"content":{"type":"String","value":"The product available is called ekoDB. It is a high-performance database product with AI capabilities. The price for this product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"8Wy_6lcV_LcKFg0jSrFkNyvfFjpFypYPW-YS-eYB106fWRWusqQqXcYW0jwD_T6BDMnOqpy6DayULPePt-ZLUg","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-03T09:53:19.945522Z"},"id":"zVfWWs50QKmM9uZg3qzoFLJ6LxyPRRH_Fg2yAttGONOS5wLK366_808VIxuFUg8pRS9Mi5dnfMPVCWZmFJCrxA","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":29,"prompt_tokens":237,"total_tokens":266}},"updated_at":{"type":"DateTime","value":"2026-01-03T09:53:19.945522Z"}},{"chat_id":{"type":"String","value":"tgYfN9Gcu866oncPr15ikCp5WKE4kkLzNY8KwrRXkh91RCq5MUrXeuP5QuUszAQAYYwEnk-a2IgBCZdmBJmhYQ"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"8Wy_6lcV_LcKFg0jSrFkNyvfFjpFypYPW-YS-eYB106fWRWusqQqXcYW0jwD_T6BDMnOqpy6DayULPePt-ZLUg","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-03T09:53:20.654876Z"},"id":"iSlESA1RWkROlP4dasVk-x_svU28ayCpN8gVyqsgTJZ0Aowiq0-iM4IHDr25nlJjzJnYiDwogIRHiwyaNs8cqw","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":237,"total_tokens":249}},"updated_at":{"type":"DateTime","value":"2026-01-03T09:53:20.654876Z"}},{"chat_id":{"type":"String","value":"tgYfN9Gcu866oncPr15ikCp5WKE4kkLzNY8KwrRXkh91RCq5MUrXeuP5QuUszAQAYYwEnk-a2IgBCZdmBJmhYQ"},"content":{"type":"String","value":"The price of the ekoDB product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"8Wy_6lcV_LcKFg0jSrFkNyvfFjpFypYPW-YS-eYB106fWRWusqQqXcYW0jwD_T6BDMnOqpy6DayULPePt-ZLUg","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-03T09:53:20.659030Z"},"id":"LLPmprjDECn6Gbs48Hl-F8OaxycVUIabivRkMcyw05W_dy8eIAuCFosmqHE3JBBIuAQQxP7oKf98XlFfBAoVgg","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":237,"total_tokens":249}},"updated_at":{"type":"DateTime","value":"2026-01-03T09:53:20.659030Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 13

=== Branching Session ===
✓ Created branched session: Y9rK_ZFy8T5HbxrdhqisqyL7cr_VYJiGb9dwz7dbV90PoTbwnVRan5KCEbS5bwylPdhzhr00fKD1zJ2-Hl4QFQ

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

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
=== ekoDB Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: bzB7ap8HzybyNBR0Q-LSn21kNLEuL8bLKDTf1jZgHHLmWl9H2p8VWGfcF7XZ-xFyGLPpMF0VQem9SFvpt6x0Wg

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the information available, one product is the ekoDB, a high-performance database product. It is priced at $99."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Based on the information available, the product ekoDB is available. It's a high-performance database product and it is priced at $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: e-xAwgfd71Xy8XGOcV0U2m-MwYWt-Ad-gOjxJcMXCFgad7og-CtcaDwVBUmmLFtHXAKjknqDcTW_hHjuoCIMPQ

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
🚀 ekoDB Scripts Example (Kotlin Client)

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: v-KP3guwySmz4FiF7HA1PlzLCmS_xC4-jHjXzWYuqu7ZRepxmj4DlWDmCLBDO2z4vD9v7wpDIVrjhDUwNyI8Pw
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"count":5,"status":"active","avg_score":60.0}
   {"count":5,"avg_score":50.0,"status":"inactive"}
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
Created Alice: $1000 - ID: Nma3IHdYjv2SBDS7l-0R75T_gb215KHOd-m71yImBbZo9-GDJKt3vRDeykuLlielOqC0YvjeXlc4yaSBzXJQTQ
Created Bob: $500 - ID: zW_i4p6t8tGRj42iNRj42TEBqqUxPKgEyveeEBOI___iXuC_m6-6k_pSoOROrzECPfE7FGk83bi04lrToagqHw

=== Example 1: Begin Transaction ===
Transaction ID: 85043a02-a60c-4fcc-be63-a3d892a310cb

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 57ec401c-fd03-43f0-b0b0-c52e23855db6
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
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.7.0-cp38-abi3-macosx_11_0_arm64.whl
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
    ✓ Generated embedding: 1536 dimensions in 1.117242791s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.118960541s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 382.89325ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 324.071167ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 355.813125ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.019615041s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 345.001875ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 731.811167ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 540.472708ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.011862292s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 301.673459ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 323.910459ms
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
    ✓ Generated embedding: 1536 dimensions in 1.083127208s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 109.37775ms

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

Writing memory-safe, high-performance database code involves several principles and best practices. Here are some general guidelines:

1. **Optimize Queries**: Make sure your queries are as efficient as possible. Use indexes wisely, avoid unnecessary columns in SELECT statements, and minimize the use of joins where possible. 

2. **Use Prepared Statements**: Prepared statements not only help prevent SQL injection attacks but can also improve performance as they're pre-compiled and stored by the database. 

3. **Normalization**: Normalize your database to eliminate data redundancy and ensure data integrity as discussed in Context 3. However, remember that excessive normalization can lead to many small, complex joins which can degrade performance. The optimal level of normalization often requires a balancing act between performance and data integrity. 

4. **Use Appropriate Database Type**: As mentioned in Context 1 and 4, choosing the right type of database (NoSQL vs SQL) can have a significant impact on performance. SQL databases are typically used for structured data with complex queries and ACID transactions, while NoSQL databases excel with unstructured data, high write throughput, and horizontal scaling.

5. **Manage Connections**: Open and close connections judiciously, and use connection pools to manage them efficiently. Keeping a connection open for longer than necessary can tie up resources, reducing the number of concurrent operations your database can handle.

6. **Caching**: Use caching to store frequently accessed data, reducing the need to execute expensive queries and freeing up your database to handle other tasks.

7. **Memory Management**: Depending on the language you're using, you may need to manage memory allocation and deallocation yourself. Make sure to free up any resources you no longer need to prevent memory leaks.

8. **Batch Operations**: Whenever possible, batch your database operations. Performing many small operations can be less efficient than performing one large operation.

9. **Monitor and Tune Performance**: Use tools to monitor your database performance, identify bottlenecks, and adjust your strategies as necessary.

Remember, performance doesn't just depend on the code itself, but also on the underlying system's capabilities, including the CPU, memory, disk I/O, and network bandwidth.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 279.835833ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2361 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 228.5535ms
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
  ✓ Text search completed in 53.8805ms

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
    ✓ Generated embedding: 1536 dimensions in 1.034s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.265s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.701s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.501s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.296s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.325s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.272s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.221s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.977s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.240s
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
    ✓ Generated embedding: 1536 dimensions in 0.211s
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
    ✓ Generated embedding: 1536 dimensions in 0.312s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.092s

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

Writing memory-safe high-performance database code involves a number of best practices:

1. Understand your database: Whether you're using SQL or NoSQL, understand how your database handles data, transactions, and queries. This includes understanding the underlying principles such as ACID properties in SQL or CAP theorem in NoSQL.

2. Use Database Normalization: As mentioned in Context 1, database normalization can help improve data integrity and reduce redundancy. This can result in more efficient use of memory and faster queries.

3. Optimize your queries: Make sure to use efficient queries to reduce the amount of data that needs to be processed. This includes using indexes, avoiding full table scans, and minimizing the use of joins where possible. 

4. Use appropriate data types: Use the correct data types for your fields to reduce memory usage. For example, use integers instead of strings for numerical data.

5. Manage your connections: Establishing and closing database connections can be costly. Use connection pooling to manage and reuse database connections.

6. Use caching: Cache frequently accessed data to reduce database load and improve performance. However, be cautious as caching can lead to stale data if not managed properly.

7. Handle errors carefully: Make sure to handle database errors properly to avoid memory leaks. This includes closing connections and freeing up resources in case of errors.

8. Test, Monitor and Tune: Continuously monitor your database performance, and tune it based on the insights you get. Use load testing to see how your database performs under heavy load.

Remember, the choice between SQL and NoSQL should be based on your specific use case (as mentioned in Context 3 and Context 4). Both have their strengths and weaknesses, and understanding these can help you write more efficient, memory-safe code.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.371s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 1868 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.274s
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
    ✓ Generated embedding: 1536 dimensions in 0.266s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.258s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.509s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.332s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.251s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.415s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.234s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.263s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.297s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.275s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.228s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.434s
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
    ✓ Generated embedding: 1536 dimensions in 0.413s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.100s
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

Writing memory-safe, high-performance database code requires a combination of best practices from both database management and programming. Here are some recommendations:

1. Optimize Database Queries: Try to optimize your database queries for better performance. This can be achieved by using indexes effectively, avoiding full table scans, limiting the data retrieved with SELECT statements, and avoiding complex joins where possible. 

2. Use Appropriate Database Type: Depending on your specific needs, you might want to choose between SQL or NoSQL. SQL is typically better for complex queries, ACID transactions, and structured data with well-defined relationships, while NoSQL is typically better for flexible schemas, horizontal scaling, high write throughput, and unstructured data.

3. Normalization: Use database normalization to reduce redundancy and improve data integrity. This involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

4. Efficient Memory Management: In terms of programming, it's important to manage memory effectively to ensure safety and performance. This involves properly allocating and freeing memory, avoiding memory leaks, and handling exceptions carefully to prevent crashes.

5. Use Prepared Statements: Prepared statements not only help prevent SQL injection attacks, but they can also improve performance by allowing the database to cache the query plan and reuse it.

6. Connection Pooling: Instead of opening and closing database connections for every single operation, use connection pooling to manage database connections efficiently.

7. Profiling and Logging: Always use tools to profile your database queries. This will help you understand where the bottlenecks are. Logging can help trace any memory related issues.

Remember, the specific implementations of these practices might vary depending on your programming language and database system.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.344s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 1947 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.281s
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
  ✓ Text search completed in 0.057s
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
    ✓ Generated embedding: 1536 dimensions in 0.269s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.318s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.394s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.264s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.267s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.247s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.261s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.350s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.218s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.289s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.306s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.280s
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
    ✓ Generated embedding: 1536 dimensions in 0.201s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.096s

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

To write memory-safe, high-performance database code, follow these guidelines:

1. **Effective Indexing**: Indexing can significantly improve the performance of your database queries by reducing the number of disk I/O operations. However, over-indexing can lead to unnecessary memory usage. Thus, ensure you only index the necessary columns.

2. **Use Prepared Statements**: Prepared statements help prevent SQL injection attacks and improve performance by reducing parsing time as the database can reuse the same execution plan for identical queries.

3. **Normalize Your Database**: As per Context 1, database normalization can help enhance data integrity and reduce redundancy. This process can also lead to performance benefits because it can minimize the need for complex joins and updates.

4. **Optimize Your Queries**: As per Context 5, optimizing your queries can help improve performance. This can include avoiding SELECT *, reducing the use of complex JOINS, and limiting the data returned by your queries.

5. **Choose the Appropriate Database Type**: As per Context 3 and 4, the choice between SQL and NoSQL can affect performance. SQL databases are better for complex queries and structured data with well-defined relationships, while NoSQL databases are better for unstructured data, high write throughput, and horizontal scaling.

6. **Memory Management**: In terms of memory safety, avoid memory leaks by properly closing database connections after use. In languages like C++, be careful with pointers and memory allocation.

7. **Concurrency Control**: Use proper locking mechanisms to avoid data inconsistencies and conflicts in a multi-user environment. Use transactions where necessary, especially in SQL databases (as per Context 3), which support ACID properties.

Remember, the specific optimizations can depend on the specific database system, the programming language, and the nature of your application.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.032s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 1930 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.515s
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
  ✓ Text search completed in 0.060s

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_new_question: To write memory-safe, high-performance database code, follow these guidelines:

1. **Effective Indexing**: Indexing can significantly improve the performance of your database queries by reducing the number of disk I/O operations. However, over-indexing can lead to unnecessary memory usage. Thus, ensure you only index the necessary columns.

2. **Use Prepared Statements**: Prepared statements help prevent SQL injection attacks and improve performance by reducing parsing time as the database can reuse the same execution plan for identical queries.

3. **Normalize Your Database**: As per Context 1, database normalization can help enhance data integrity and reduce redundancy. This process can also lead to performance benefits because it can minimize the need for complex joins and updates.

4. **Optimize Your Queries**: As per Context 5, optimizing your queries can help improve performance. This can include avoiding SELECT *, reducing the use of complex JOINS, and limiting the data returned by your queries.

5. **Choose the Appropriate Database Type**: As per Context 3 and 4, the choice between SQL and NoSQL can affect performance. SQL databases are better for complex queries and structured data with well-defined relationships, while NoSQL databases are better for unstructured data, high write throughput, and horizontal scaling.

6. **Memory Management**: In terms of memory safety, avoid memory leaks by properly closing database connections after use. In languages like C++, be careful with pointers and memory allocation.

7. **Concurrency Control**: Use proper locking mechanisms to avoid data inconsistencies and conflicts in a multi-user environment. Use transactions where necessary, especially in SQL databases (as per Context 3), which support ACID properties.

Remember, the specific optimizations can depend on the specific database system, the programming language, and the nature of your application.

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
    ✓ Generated embedding: 1536 dimensions in 0.389s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.33s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.256s
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
    ✓ Generated embedding: 1536 dimensions in 0.32s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.218s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.321s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.261s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.256s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.318s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.25s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.375s
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
    ✓ Generated embedding: 1536 dimensions in 0.616s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
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
✓ Context prepared from search results
✓ AI would use this context to generate comprehensive response

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.34s
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
  ✓ Text search completed in 0.05s

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

> @ekodb/ekodb-client@0.7.0 prepare
> npm run build


> @ekodb/ekodb-client@0.7.0 build
> tsc


up to date, audited 7 packages in 842ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.0 build
> tsc

✅ [32mTypeScript client built![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning TypeScript SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (X9UXTt2UtxTOojNvviFA3ogm3AKUVyzk7XznpHBvOy3R2vwU_yTH1g_Bg8ufS83j3LUys9cD-K2WPuLAvYlrsg)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [
    {
      "cached_at": "2026-01-03T09:54:56.585Z",
      "id": "gmZq05yPdsLsX2lPOmSh3gJZ1iEeHIKg9MMXy0XMasXWyiWvZ2Jro5Ay8AOG-w9YQ9KecNvMOO92jxc4SCiSZg",
      "data": "{\"login\":\"torvalds\",\"id\":1024025,\"node_id\":\"MDQ6VXNlcjEwMjQwMjU=\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/1024025?v=4\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/torvalds\",\"html_url\":\"https://github.com/torvalds\",\"followers_url\":\"https://api.github.com/users/torvalds/followers\",\"following_url\":\"https://api.github.com/users/torvalds/following{/other_user}\",\"gists_url\":\"https://api.github.com/users/torvalds/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/torvalds/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/torvalds/subscriptions\",\"organizations_url\":\"https://api.github.com/users/torvalds/orgs\",\"repos_url\":\"https://api.github.com/users/torvalds/repos\",\"events_url\":\"https://api.github.com/users/torvalds/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/torvalds/received_events\",\"type\":\"User\",\"user_view_type\":\"public\",\"site_admin\":false,\"name\":\"Linus Torvalds\",\"company\":\"Linux Foundation\",\"blog\":\"\",\"location\":\"Portland, OR\",\"email\":null,\"hireable\":null,\"bio\":null,\"twitter_username\":null,\"public_repos\":9,\"public_gists\":1,\"followers\":269549,\"following\":0,\"created_at\":\"2011-09-03T15:26:22Z\",\"updated_at\":\"2025-11-24T04:16:14Z\"}"
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
Response time: 85ms (served from cache)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (nJMD8FTrf2OsMiOGlD6HGuQuQJzxLYpsT6AI-R4V_JEktSXTdsGNpYhbvSdPVi7Rc7A-zP4fLrQXOW0I1nF2Jw)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "enriched_at": "2026-01-03T09:54:56.854Z",
      "enriched_data": "{\"id\":1,\"title\":\"Essence Mascara Lash Princess\",\"description\":\"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.\",\"category\":\"beauty\",\"price\":9.99,\"discountPercentage\":10.48,\"rating\":2.56,\"stock\":99,\"tags\":[\"beauty\",\"mascara\"],\"brand\":\"Essence\",\"sku\":\"BEA-ESS-ESS-001\",\"weight\":4,\"dimensions\":{\"width\":15.14,\"height\":13.08,\"depth\":22.99},\"warrantyInformation\":\"1 week warranty\",\"shippingInformation\":\"Ships in 3-5 business days\",\"availabilityStatus\":\"In Stock\",\"reviews\":[{\"rating\":3,\"comment\":\"Would not recommend!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"},{\"rating\":4,\"comment\":\"Very satisfied!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Lucas Gordon\",\"reviewerEmail\":\"lucas.gordon@x.dummyjson.com\"},{\"rating\":5,\"comment\":\"Highly impressed!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"}],\"returnPolicy\":\"No return policy\",\"minimumOrderQuantity\":48,\"meta\":{\"createdAt\":\"2025-04-30T09:41:02.053Z\",\"updatedAt\":\"2025-04-30T09:41:02.053Z\",\"barcode\":\"5784719087687\",\"qrCode\":\"https://cdn.dummyjson.com/public/qr-code.png\"},\"images\":[\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp\"],\"thumbnail\":\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp\"}",
      "id": "unzOPUv5WGBSMjKyFegLv3TOuzDo2TBbsNjo0vqmC8t3jlXhUB8gr8__5FvftoYDBwW5qdJiA7eHyi1Q78BoiA"
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
✓ Edge cache script created: FiAIEpM6Afp_Z36jxAiqlmBsBygNvwdOmJHWW4tO1uPIItJVg-EYnBaIJenHi_ItJ3IICIBhjJknyIc8N37Elw

Call 1: Cache miss (fetches from API)
Response time: 580ms
Result: {
  "records": [
    {
      "id": "GeGPwJCfy3hEg-gJJp6mmbq4ZrbtymT16mZCKpgU5Aqa2nCuf2ntVDOgF0kPh3Jsx76Ajg5tm6w0ivTvjx6ejQ",
      "data": "{\"latitude\":40.710335,\"longitude\":-73.99309,\"generationtime_ms\":0.026226043701171875,\"utc_offset_seconds\":0,\"timezone\":\"GMT\",\"timezone_abbreviation\":\"GMT\",\"elevation\":32.0,\"current_units\":{\"time\":\"iso8601\",\"interval\":\"seconds\",\"temperature_2m\":\"°C\"},\"current\":{\"time\":\"2026-01-03T09:45\",\"interval\":900,\"temperature_2m\":-5.0}}",
      "cached_at": "2026-01-03T09:54:57.332Z"
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
Response time: 558ms (1x faster!)
Result: {
  "records": [
    {
      "id": "pvi2U6UUQUsfdXTkaC_z9y73HuOFEvCu7XSLlFmgKheyGTGOe2H84OEns1QxpMQhOMsDHFGZ0doJznD4Ly8T9w",
      "data": "{\"latitude\":40.710335,\"longitude\":-73.99309,\"generationtime_ms\":0.020265579223632812,\"utc_offset_seconds\":0,\"timezone\":\"GMT\",\"timezone_abbreviation\":\"GMT\",\"elevation\":32.0,\"current_units\":{\"time\":\"iso8601\",\"interval\":\"seconds\",\"temperature_2m\":\"°C\"},\"current\":{\"time\":\"2026-01-03T09:45\",\"interval\":900,\"temperature_2m\":-5.0}}",
      "cached_at": "2026-01-03T09:54:57.332Z"
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
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.7.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.7.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
Successfully installed ekodb-client-0.7.0
✅ [32mPython client package built and installed![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (_idyU2JpuqQTfrjHxXTrgWwmhLDKDedCGVebDtlMV4WxSfuL97iLdC0SEfkcWkQj_ro5tGzC1c4CQQul0yEmdg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 83ms
Result: [
  {
    "cached_at": "1767434100",
    "data": "{\"login\":\"torvalds\",\"id\":1024025,\"node_id\":\"MDQ6VXNlcjEwMjQwMjU=\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/1024025?v=4\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/torvalds\",\"html_url\":\"https://github.com/torvalds\",\"followers_url\":\"https://api.github.com/users/torvalds/followers\",\"following_url\":\"https://api.github.com/users/torvalds/following{/other_user}\",\"gists_url\":\"https://api.github.com/users/torvalds/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/torvalds/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/torvalds/subscriptions\",\"organizations_url\":\"https://api.github.com/users/torvalds/orgs\",\"repos_url\":\"https://api.github.com/users/torvalds/repos\",\"events_url\":\"https://api.github.com/users/torvalds/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/torvalds/received_events\",\"type\":\"User\",\"user_view_type\":\"public\",\"site_admin\":false,\"name\":\"Linus Torvalds\",\"company\":\"Linux Foundation\",\"blog\":\"\",\"location\":\"Portland, OR\",\"email\":null,\"hireable\":null,\"bio\":null,\"twitter_username\":null,\"public_repos\":9,\"public_gists\":1,\"followers\":269549,\"following\":0,\"created_at\":\"2011-09-03T15:26:22Z\",\"updated_at\":\"2025-11-24T04:16:14Z\"}",
    "id": "754TijnW6YcqxoxRUYbqNGM3qU-ST9dfpUCfQc4kSW6mrR96cfU-163PyyqjPYET_NgUePUi3qTLDY9sGTF7BA"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 103ms (0.8x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (Ale-ksO6Jp4BCPNl803VARkBN0kJiF_H-q1gCjk6QTdxI6dwbOhTAIcSeWriE5ekUFTbAnEjlWZnregkwwU9UQ)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "1767434100",
    "enriched_data": "{\"id\":1,\"title\":\"Essence Mascara Lash Princess\",\"description\":\"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.\",\"category\":\"beauty\",\"price\":9.99,\"discountPercentage\":10.48,\"rating\":2.56,\"stock\":99,\"tags\":[\"beauty\",\"mascara\"],\"brand\":\"Essence\",\"sku\":\"BEA-ESS-ESS-001\",\"weight\":4,\"dimensions\":{\"width\":15.14,\"height\":13.08,\"depth\":22.99},\"warrantyInformation\":\"1 week warranty\",\"shippingInformation\":\"Ships in 3-5 business days\",\"availabilityStatus\":\"In Stock\",\"reviews\":[{\"rating\":3,\"comment\":\"Would not recommend!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"},{\"rating\":4,\"comment\":\"Very satisfied!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Lucas Gordon\",\"reviewerEmail\":\"lucas.gordon@x.dummyjson.com\"},{\"rating\":5,\"comment\":\"Highly impressed!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"}],\"returnPolicy\":\"No return policy\",\"minimumOrderQuantity\":48,\"meta\":{\"createdAt\":\"2025-04-30T09:41:02.053Z\",\"updatedAt\":\"2025-04-30T09:41:02.053Z\",\"barcode\":\"5784719087687\",\"qrCode\":\"https://cdn.dummyjson.com/public/qr-code.png\"},\"images\":[\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp\"],\"thumbnail\":\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp\"}",
    "id": "vke8kTt5LAdQdeY8c1m9fZ_1EMBJs4exdKSZ2-6FHjoq3Ak8n1JIa7n0TcKmPpheH1d76JQi6svYaLzKFUYydQ"
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
✓ Created SWR script: fetch_github_user (LBb-I52T7NBnPQ6rkgZdcjiTCekmr9mOxt9S2GfI6RFKy5nfnd_OxjFn8gRWp2wSjFXnaYK3x7onrpf1oz4VgQ)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 79.807458ms
Result: [
  {
    "cached_at": "1767434100",
    "data": "{\"login\":\"torvalds\",\"id\":1024025,\"node_id\":\"MDQ6VXNlcjEwMjQwMjU=\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/1024025?v=4\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/torvalds\",\"html_url\":\"https://github.com/torvalds\",\"followers_url\":\"https://api.github.com/users/torvalds/followers\",\"following_url\":\"https://api.github.com/users/torvalds/following{/other_user}\",\"gists_url\":\"https://api.github.com/users/torvalds/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/torvalds/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/torvalds/subscriptions\",\"organizations_url\":\"https://api.github.com/users/torvalds/orgs\",\"repos_url\":\"https://api.github.com/users/torvalds/repos\",\"events_url\":\"https://api.github.com/users/torvalds/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/torvalds/received_events\",\"type\":\"User\",\"user_view_type\":\"public\",\"site_admin\":false,\"name\":\"Linus Torvalds\",\"company\":\"Linux Foundation\",\"blog\":\"\",\"location\":\"Portland, OR\",\"email\":null,\"hireable\":null,\"bio\":null,\"twitter_username\":null,\"public_repos\":9,\"public_gists\":1,\"followers\":269549,\"following\":0,\"created_at\":\"2011-09-03T15:26:22Z\",\"updated_at\":\"2025-11-24T04:16:14Z\"}",
    "id": "7MFmGlCdG-S0ea6zU6_5QKvYcRMRkRQMzIUtvhzpMM2Zdx7UdVbr_-94rSsF2EzWccqBwnY_gTBrXVADNGGrcA"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 87.184ms (0.9x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (9ifEsticLYCGjGkOmLUBGtg4lduvQIYjRMCRAMxZu70PgxYC5qot_EkvEoSnnXOoFxwedCgvsZtT66f88fltQg)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "2026-01-03T04:55:01-05:00",
    "enriched_data": "{\"id\":1,\"title\":\"Essence Mascara Lash Princess\",\"description\":\"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.\",\"category\":\"beauty\",\"price\":9.99,\"discountPercentage\":10.48,\"rating\":2.56,\"stock\":99,\"tags\":[\"beauty\",\"mascara\"],\"brand\":\"Essence\",\"sku\":\"BEA-ESS-ESS-001\",\"weight\":4,\"dimensions\":{\"width\":15.14,\"height\":13.08,\"depth\":22.99},\"warrantyInformation\":\"1 week warranty\",\"shippingInformation\":\"Ships in 3-5 business days\",\"availabilityStatus\":\"In Stock\",\"reviews\":[{\"rating\":3,\"comment\":\"Would not recommend!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"},{\"rating\":4,\"comment\":\"Very satisfied!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Lucas Gordon\",\"reviewerEmail\":\"lucas.gordon@x.dummyjson.com\"},{\"rating\":5,\"comment\":\"Highly impressed!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"}],\"returnPolicy\":\"No return policy\",\"minimumOrderQuantity\":48,\"meta\":{\"createdAt\":\"2025-04-30T09:41:02.053Z\",\"updatedAt\":\"2025-04-30T09:41:02.053Z\",\"barcode\":\"5784719087687\",\"qrCode\":\"https://cdn.dummyjson.com/public/qr-code.png\"},\"images\":[\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp\"],\"thumbnail\":\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp\"}",
    "id": "KbGQ4-bLH-846a8wwwVpO8JzNDqejflG6qShobtonrOXVva17XibM_01u38nnn8WuJS9dJcdcSAnRIh7B9c0mw"
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.81s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (500a0lPeYoGgTfYYgOPouqj1zo5XefobhC6ZwrxhezUcdKRXNTkesvXGybQjqPxRElaVIh0pAkqCuQ1Tz7u2Mw)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 87ms
Result: {
  "cached_at": "1767434100",
  "id": "fRKcARC_obVDip-WNWGwGAVez_AFb9g5SEGwtDfLOTctdeFIGNJB4s7iLmXeTRXny0yDvLxLmv6kZD2P8B9v7Q",
  "data": "{\"login\":\"torvalds\",\"id\":1024025,\"node_id\":\"MDQ6VXNlcjEwMjQwMjU=\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/1024025?v=4\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/torvalds\",\"html_url\":\"https://github.com/torvalds\",\"followers_url\":\"https://api.github.com/users/torvalds/followers\",\"following_url\":\"https://api.github.com/users/torvalds/following{/other_user}\",\"gists_url\":\"https://api.github.com/users/torvalds/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/torvalds/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/torvalds/subscriptions\",\"organizations_url\":\"https://api.github.com/users/torvalds/orgs\",\"repos_url\":\"https://api.github.com/users/torvalds/repos\",\"events_url\":\"https://api.github.com/users/torvalds/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/torvalds/received_events\",\"type\":\"User\",\"user_view_type\":\"public\",\"site_admin\":false,\"name\":\"Linus Torvalds\",\"company\":\"Linux Foundation\",\"blog\":\"\",\"location\":\"Portland, OR\",\"email\":null,\"hireable\":null,\"bio\":null,\"twitter_username\":null,\"public_repos\":9,\"public_gists\":1,\"followers\":269549,\"following\":0,\"created_at\":\"2011-09-03T15:26:22Z\",\"updated_at\":\"2025-11-24T04:16:14Z\"}"
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 99ms (0.9x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (CAsRU60tCjVUuGzwJPWi_MB09QDdi05CYzFgYFiutH70nZGorJ6UH8a7EBO4cOTM9a2zTmrdQLnIGFssy7RBhA)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "enriched_at": "2026-01-03T04:55:01-05:00",
  "id": "v9xIKT1k0Yq0x3USeyLJQEX7TLUOxHMiNhu96j2_iO8C1ZxD-Jnx_eRslI7ezq6u22nefNMmJ9Ma_RomzYq-5w",
  "enriched_data": "{\"id\":1,\"title\":\"Essence Mascara Lash Princess\",\"description\":\"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.\",\"category\":\"beauty\",\"price\":9.99,\"discountPercentage\":10.48,\"rating\":2.56,\"stock\":99,\"tags\":[\"beauty\",\"mascara\"],\"brand\":\"Essence\",\"sku\":\"BEA-ESS-ESS-001\",\"weight\":4,\"dimensions\":{\"width\":15.14,\"height\":13.08,\"depth\":22.99},\"warrantyInformation\":\"1 week warranty\",\"shippingInformation\":\"Ships in 3-5 business days\",\"availabilityStatus\":\"In Stock\",\"reviews\":[{\"rating\":3,\"comment\":\"Would not recommend!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"},{\"rating\":4,\"comment\":\"Very satisfied!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Lucas Gordon\",\"reviewerEmail\":\"lucas.gordon@x.dummyjson.com\"},{\"rating\":5,\"comment\":\"Highly impressed!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"}],\"returnPolicy\":\"No return policy\",\"minimumOrderQuantity\":48,\"meta\":{\"createdAt\":\"2025-04-30T09:41:02.053Z\",\"updatedAt\":\"2025-04-30T09:41:02.053Z\",\"barcode\":\"5784719087687\",\"qrCode\":\"https://cdn.dummyjson.com/public/qr-code.png\"},\"images\":[\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp\"],\"thumbnail\":\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp\"}"
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
✓ Created SWR script: fetch_github_user (as3u0ZPTZxgtJMBNtaa8D11TuPu5ONz7oMRJgW28NNBABNK8879HvneETvbCMsD6jYHktq-ddlm8KbIZSC4GKg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 86ms
Result: [{"cached_at":"1767434100","id":"Ch2gw-ciPiLzPM3jqeYXXccnUBCaRVGf68nEGyGyPwimocpSEW_TPB7dYjLQOCINWWhVg0-0nXmEXD534n5x7g","data":"{\"login\":\"torvalds\",\"id\":1024025,\"node_id\":\"MDQ6VXNlcjEwMjQwMjU=\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/1024025?v=4\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/torvalds\",\"html_url\":\"https://github.com/torvalds\",\"followers_url\":\"https://api.github.com/users/torvalds/followers\",\"following_url\":\"https://api.github.com/users/torvalds/following{/other_user}\",\"gists_url\":\"https://api.github.com/users/torvalds/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/torvalds/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/torvalds/subscriptions\",\"organizations_url\":\"https://api.github.com/users/torvalds/orgs\",\"repos_url\":\"https://api.github.com/users/torvalds/repos\",\"events_url\":\"https://api.github.com/users/torvalds/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/torvalds/received_events\",\"type\":\"User\",\"user_view_type\":\"public\",\"site_admin\":false,\"name\":\"Linus Torvalds\",\"company\":\"Linux Foundation\",\"blog\":\"\",\"location\":\"Portland, OR\",\"email\":null,\"hireable\":null,\"bio\":null,\"twitter_username\":null,\"public_repos\":9,\"public_gists\":1,\"followers\":269549,\"following\":0,\"created_at\":\"2011-09-03T15:26:22Z\",\"updated_at\":\"2025-11-24T04:16:14Z\"}"}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 79ms
Cache hit was 1.1x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (2OsQBcP4aEEVMTDtar_oEJ0A4mM9Yjb8xJ6KETjkM67rQWmED4FAZj_DmkVnT7iQ0Qi_3lO3V0x27jdFdOxLnQ)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"id":"QxAZiQCgZquN3tdtb5mSGbLjIy7NPR9B7ZfTGLp34rHnFlxWt1I9R2IztAz-glNEuJqX75Yzfo3n982akQqZ7A","enriched_at":"2026-01-03T04:55:01-05:00","enriched_data":"{\"id\":1,\"title\":\"Essence Mascara Lash Princess\",\"description\":\"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.\",\"category\":\"beauty\",\"price\":9.99,\"discountPercentage\":10.48,\"rating\":2.56,\"stock\":99,\"tags\":[\"beauty\",\"mascara\"],\"brand\":\"Essence\",\"sku\":\"BEA-ESS-ESS-001\",\"weight\":4,\"dimensions\":{\"width\":15.14,\"height\":13.08,\"depth\":22.99},\"warrantyInformation\":\"1 week warranty\",\"shippingInformation\":\"Ships in 3-5 business days\",\"availabilityStatus\":\"In Stock\",\"reviews\":[{\"rating\":3,\"comment\":\"Would not recommend!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"},{\"rating\":4,\"comment\":\"Very satisfied!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Lucas Gordon\",\"reviewerEmail\":\"lucas.gordon@x.dummyjson.com\"},{\"rating\":5,\"comment\":\"Highly impressed!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"}],\"returnPolicy\":\"No return policy\",\"minimumOrderQuantity\":48,\"meta\":{\"createdAt\":\"2025-04-30T09:41:02.053Z\",\"updatedAt\":\"2025-04-30T09:41:02.053Z\",\"barcode\":\"5784719087687\",\"qrCode\":\"https://cdn.dummyjson.com/public/qr-code.png\"},\"images\":[\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp\"],\"thumbnail\":\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp\"}"}]
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.94s
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
   Department: Some(Object({"value": String("engineering"), "type": String("String")}))

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 87.3055ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 94.3715ms
   📊 Records: 1
   🚀 Cache speedup: 0.9x faster!

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

> @ekodb/ekodb-client@0.7.0 prepare
> npm run build


> @ekodb/ekodb-client@0.7.0 build
> tsc


up to date, audited 7 packages in 852ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.0 build
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
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 77ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 86ms
   📊 Records: 1
   🚀 Cache speedup: 0.9x faster!

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
    Finished `release` profile [optimized] target(s) in 0.10s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.7.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.7.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.7.0
    Uninstalling ekodb_client-0.7.0:
      Successfully uninstalled ekodb_client-0.7.0
Successfully installed ekodb-client-0.7.0
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
   ⏱️  Duration: 81.2ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 89.3ms
   📊 Records: 1
   🚀 Cache speedup: 0.9x faster!

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
   ⏱️  Duration: 81.505875ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 85.423125ms
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
   Name: {"type":"String","value":"User 1"}
   Department: {"value":"engineering","type":"String"}

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
   ⏱️  Duration: 79ms
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

