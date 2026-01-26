make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("lKNhmkv4ygou5o0eN6Y6L-v32sbEn2ILo6aT-eiwVqGeJG4hk2_gvVschxB-qbytukznAMsLs3oQ2jJ4fMC8YQ")}

=== Find by ID ===
Found: Object {"active": Bool(true), "id": String("lKNhmkv4ygou5o0eN6Y6L-v32sbEn2ILo6aT-eiwVqGeJG4hk2_gvVschxB-qbytukznAMsLs3oQ2jJ4fMC8YQ"), "name": String("Test Record"), "value": Number(42)}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("lKNhmkv4ygou5o0eN6Y6L-v32sbEn2ILo6aT-eiwVqGeJG4hk2_gvVschxB-qbytukznAMsLs3oQ2jJ4fMC8YQ"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("lKNhmkv4ygou5o0eN6Y6L-v32sbEn2ILo6aT-eiwVqGeJG4hk2_gvVschxB-qbytukznAMsLs3oQ2jJ4fMC8YQ"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "hqv3ZeE80SGWiiGHjNPjmodyzWaTYy5FkLK3O98nFYXHTzeIusk1MyTtfMT4roJOF9NhJp4yMqsxgGZOgbaAiA"

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
        "id": "hqv3ZeE80SGWiiGHjNPjmodyzWaTYy5FkLK3O98nFYXHTzeIusk1MyTtfMT4roJOF9NhJp4yMqsxgGZOgbaAiA",
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
Collection created with first record: "bBXsPeqrftC8MJby9UTqBYMWe64Kj9Os6IB1ipl3i3c5-rQ0U0Kn_iAU3PJ9pRH3iE9Rxr6IEPNJyQiA4xm3BQ"

=== List Collections ===
Total collections: 4
Sample collections: ["batch_users", "websocket_test", "demo_collection", "test_collection"]

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
✓ Inserted document: "pfA8CmMKUj8r3NeQsBVMv6vaC6i2ieIwLZxnfLhgwZydhVH_yO4oX-pN_g5L7S_RAs1tBMYWIWpwqCQrSIeB0w"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "oWAe6IyQb2LivRSteS5kP7SgipZln9A_TKtFz5xA70IqaQU2UOkuwvWBUbJrc90lxohMahTSXACvljv2udd6SA"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "NG9VGqXWmZnRkIK6Y7C3v9avVveF65IiPBAt2vtZ-7YyURj1zNOhpt7Hr84GXxxuMpZHU5ULMgzwz81d33raow"

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
✓ Inserted document with TTL: String("pFSitQzSnLb3q-ZmxRn6gI9wpfKwFiOh9_f5R1ECreh7K8DCEE6z2SDBSpFBbrRNX9ukK7nNxu584vhW02QOIw")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "pFSitQzSnLb3q-ZmxRn6gI9wpfKwFiOh9_f5R1ECreh7K8DCEE6z2SDBSpFBbrRNX9ukK7nNxu584vhW02QOIw"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-01-26T16:03:47.716923Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: SE423kz72h4aX2n59TGYcmpmSkm4nxPZj7vcD_Du4eq3imT9mJ3j2tTA2OhCqIjbfXtbbKe28DjSl6k86QAdcA
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "tIThz-5txhEjqPMfvbh_iWVp0KYBu4J6AREAVdxFSIxzD6lBisQsuIVUwFuR65JbVASpVGPqMDRjHFjd0Tam_g"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: VVgeaut1gtPmCz0NTQYKN8cwiKGMfTrU8lta88VQgjVL6cce5RBZZrQYKWhdg5fFq0Tn6y-uTW6o3DqoEhUs-g
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: ab3TZ9eY3ciW9G0_c62ij2rpsqcue2Tq5Aqf9gryqrzyXduVtyc8X3VwX5R2tIWD6t1eQ9pNnBJJbXAw4tdScw
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
Created Alice: $1000 - ID: SnPaF9Y_OC5kXroRUj6onT-VAUAx90tVVY9A-90qK6sz9xaGhQx9i1K8dYfz2AALF3CiQPHpiFwQK9lT8SR3dA
Created Bob: $500 - ID: yJgV17L-MptlMv9x3ekntSMb7jngNXgUm-AierdWlfF0-cqbUarAgc-PoLwYpregFECIlp_qSi-Z3_ICP9VQxA

=== Example 1: Begin Transaction ===
Transaction ID: 72efa2bd-d55f-4765-8b5f-96555981541c

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
New transaction: 071f2eb3-5441-4c4c-9e79-2a41f31c5e15
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
Inserted: Record({"id": String("zx1AWvy3kcjjI1_yHn9MefmmZXixLxykQbYmphiegVYmr10EyG44J4r_ecFgzWzlwJI-n_XO1UEb84yCkfDNMg")})

=== Find by ID ===
Found: Record({"value": Integer(42), "created_at": String("2026-01-26T15:03:52.830542+00:00"), "data": String("aGVsbG8gd29ybGQ="), "id": String("zx1AWvy3kcjjI1_yHn9MefmmZXixLxykQbYmphiegVYmr10EyG44J4r_ecFgzWzlwJI-n_XO1UEb84yCkfDNMg"), "categories": Array([String("electronics"), String("computers")]), "name": String("Test Record"), "price": Float(99.99), "active": Boolean(true), "user_id": String("550e8400-e29b-41d4-a716-446655440000"), "embedding": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "tags": Array([String("tag1"), String("tag2"), String("tag3")]), "metadata": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-01-26T15:03:52.830542+00:00")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 0 dims
  categories (Set): 0 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"created_at": Object({"type": String("DateTime"), "value": String("2026-01-26T15:03:52.830542Z")}), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "name": Object({"value": String("Test Record"), "type": String("String")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "id": String("zx1AWvy3kcjjI1_yHn9MefmmZXixLxykQbYmphiegVYmr10EyG44J4r_ecFgzWzlwJI-n_XO1UEb84yCkfDNMg"), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")})})]

=== Update Document ===
Updated: Record({"name": Object({"value": String("Updated Record"), "type": String("String")}), "created_at": Object({"value": String("2026-01-26T15:03:52.830542Z"), "type": String("DateTime")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "value": Object({"value": Integer(100), "type": String("Integer")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "id": String("zx1AWvy3kcjjI1_yHn9MefmmZXixLxykQbYmphiegVYmr10EyG44J4r_ecFgzWzlwJI-n_XO1UEb84yCkfDNMg"), "price": Object({"type": String("Float"), "value": Float(99.99)}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 2dEZ65ZRpbJfWgH8Zmmm-BRuX1dYnuoTMjn3koNgpsPAw0KbiCngbtDL5d-0DaSj42zFpCdupV_WgDD5pySVGA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  name: "WebSocket Test Record"
  active: true
  value: 42
  id: "2dEZ65ZRpbJfWgH8Zmmm-BRuX1dYnuoTMjn3koNgpsPAw0KbiCngbtDL5d-0DaSj42zFpCdupV_WgDD5pySVGA"

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
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
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "Z4UZz0kxLlvTzDBl2JQ1pKDg4de1zoaCAKx9iEr23vExQxJK6jQHwRnFiBVrzcaWtZ-HAcbuaB_UWkYQJpmyFw"

=== List Collections ===
Total collections: 9
Sample collections: ["batch_users", "ttl_cache", "websocket_test", "client_collection_management_rust", "scripts__ek0_testing"]

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
Created Alice: $1000 - ID: 1pt-xRub7VZElWvMyF6_DsS5iGaw8z8zNEwE7GQQMjb0HdsNtpFPirh4FGXa9iOKcdq8VNrtL-5668zXkMKwcQ
Created Bob: $500 - ID: W_QQbcgNW-KXZrpFyB5ZyDJ0gdFFTv1F7V_4RxACgU-zDwlYCNpzrYn_CpvNIYZf6v1Qf2NHXK-EeX0X0RSjng

=== Example 1: Begin Transaction ===
Transaction ID: 300ebb93-8a53-4764-b930-85a8a75bb980

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 90962ad5-7855-4daf-b74d-21b22021b9fd
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
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Regex Query ===
✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"value": String("Charlie"), "type": String("String")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
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
     Matched: ["description.value", "description", "title.value", "title"]
  2. Score: 2.0000
     Title: Some(String("Python for Data Science"))
     Matched: ["description", "description.value"]
  3. Score: 1.0000
     Title: Some(String("Database Design"))
     Matched: ["description.value", "description"]
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
  2. Score: 6.6000 - Some(String("JavaScript Web Development"))

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
✓ Inserted user 1: Some(String("2_1YfdFRH848SIpT7l2bF3iiCG715Y1l7wJfKU-ZT08bDfAlJFpqrNqBBCcDPusptQV9BOzEmRiC_joLzIJlnw"))
✓ Inserted user 2: Some(String("TKyeGNb5qPowEP_UkHdo8uAfwz6t23DUyMN9gE_sac-ckvqLELwigXRxH2eLmmxpYjnjYQHICTLVeg5-GUqcYg"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - age: Integer
  - status: String
  - title: String
    (required)
  - email: String
    (required)

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["batch_users", "ttl_cache", "websocket_test", "scripts__ek0_testing", "ws_ttl_test"]

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
✓ Inserted document: "QZ4UC-KVbwKdP5YHtk2SfJs6VWS7oLfPSJrU4m6gpDhMrH15yeeNrxrigp0b8FQTO7sUhpnwDSuPuenkLkfDUQ"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("4efBgVrNRee7MYfCcErrKmbGAp2gLkhkeEBoA9M_i7HCH8bTvDfG-Gjt_Ya1d2qmDRvvF-oVia8Y69Up6gSYSA"))

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
✓ Inserted document with TTL: Some(String("oihscwI0FMUU78RYPpGoBSp-O4LfJgE7sblQ4cf4bsrln-dwXk5erna5kwTkieN0v_T1lehpg5sf3FofzbQQMg"))

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
✓ Edge cache script created: 7Pz0j51u6StuSxxT6d3ZS0PWPJGBu32Vg5DGKbzoxhvcq5vn9jXesL7nTrO_Wm2JYG6Mb4bV59wdTkwahaiymQ

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: jW2wbVM0NJoboApoH_r_YEl8pwYz_AC3TKuPMsG2VkrkS5aI_LXS4c9L10JZXhKK7GugJHuV3178FpLZJXhzqg
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: wOkL6OYbAPqH8zWmz00DBhdw5dT4_bJ72IyD--LBcuLki9YywjGsR88O9d8_cd8KG8EIXDN8Aqlb4wO6vUgcwQ
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: edr3bAAmc_w3DtO0BPKtwspUh3K248o4MysYTwjCbp46Mv049Hq3UXYwl0x14Op6OW9aubKJSnZdPx69av3E0w
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

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 130.856292ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 63.3605ms
   📊 Records: 1
   🚀 Cache speedup: 2.1x faster!

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

✅ Script saved: HQOBQbv-w1KwzBEnwXz1-3lspfNcsaho3H0_nlbri541BHZ01hFas7ywcE6vKVtKGVxIJ3qDcWYthCMqlTYImg
📊 Found 2 product groups
   Record({"category": String("Electronics"), "avg_price": Float(575.6666666666666), "count": Integer(3)})
   Record({"count": Integer(2), "category": String("Furniture"), "avg_price": Float(474.0)})
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

✅ Inserted order: Some(String("Wd7gSdNsbBQedSMz1sbpk740Pp9xkXXXTX4dQwiaViSRSFHHCX_4FdwdT3COyQywnKl-im_Ls51KaFAbQS5muQ"))
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: VMwVhGqIO08z4cIEeQyeZXh_o2WWvTJOAbP0k4h3KdW8Mnvvzz35_yJM-f47Mu3xXGuJ_f674XU7u4sJIhDXeQ
📊 Script executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: gNstH-dnNbsmth1zL8GFnw-DX6Lkklco3dF7oXI76ez4vYLM-yFk77b8lLAh5Rf7RxgrB5687gu-5cCjDXufVw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: B-uJt5ArzJxmIsYWCi14D3Ga_xI9oz3ijcCRfHxpX2540bbiUpJt8RjwrcRIobGP9y_eHXO9xc-zKfHuncR5kA
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
✓ Created SWR script: fetch_api_user_rs (faOvTsGkh5gWZyQ4Vt5yR-NOd2YA2fsLoWXqYwO3zjS6EOFn1cc3Hk-P6k5JZfgJjHY4NrjIEmCYzslgr5m0PA)

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native (JDk2xurPcPHwWs_jSsvGLL4o1NPdjI5un0VFqT3lnxN0iEWs61f03mXqNVsU-UbUPd4STDuPxQU1wmWZL56x3Q)

First call (cache miss - will fetch from GitHub API):
  Response time: 124ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 5ms
  Speedup: 24.8x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (dpezzTh9dvHgR8oREghEOdKF93QsFiH2Q4AtMJYGcgEnMMHsmLrcemLFMinBZRkMIeo9WoafriDpBGA8-xAVVA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (pjtfve741gMopw1u-5srAIyPZ0waWfHoJznQi78G--AlIkJEucqpYHYfjotfIUjgcsX-QygKLqtFVaX9YRpwRg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (iD38RMi2vWlygYcasQ1b119syy-J21wPzJY4q94ONdCRteMRzQzkVvIEmWbj6nE-46Hkk36xkGg0gqin0ASbJw)
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
   Record({"count": Integer(3), "avg_price": Float(365.6666666666667), "category": String("Furniture")})
   Record({"avg_price": Float(367.0), "count": Integer(5), "category": String("Electronics")})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer benefits such as efficient storage and querying, support for complex geometric operations, high precision, and the ability to handle spatial relationships like proximity and containment. They also allow representation of real world features like roads, buildings, and areas accurately.
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
   Record({"status": String("inactive"), "count": Integer(3)})
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
   1. Database Design Principles (Database)
   2. Natural Language Processing (AI)
   3. Getting Started with ekoDB (Database)
   4. Introduction to Machine Learning (AI)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   Record({"category": String("Database"), "count": Integer(3)})
   Record({"category": String("AI"), "count": Integer(2)})
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
✓ Created session: bZ-7B3w-TbSQt3kqjj27W2ZHe4J10z8GsnkV781WYxzhZK0OIgmiWjzyKuoXWNnvtWFIDo8zcO4S6xPvVHkDRA

=== Sending Chat Message ===
Message ID: OC1-5QNfOGXdKmeo2I8H5NiMV1HCvgGz5AvQ5xN2jJgxxojU_I0ze8iWBgCaEiCAbA8NUint8N8y0Wfdzyoqzg

=== AI Response ===
Response 1: ekoDB is a high-performance database that is designed with intelligent caching and real-time capabilities. It also integrates AI to enhance its functionality. Some of its notable features include an AI chat integration which allows you to query your database using natural language, providing AI-powered responses with context. Additionally, ekoDB offers robust search features, supporting full-text search, vector search, and hybrid search with automatic context retrieval.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["content", "category", "title"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("Z7G7JOAKvtKh7nVSMVH0wCSNPDkOrjevkR-Vl7Smyi-J7CQxGPMJ1A9vCM9G5eEbR78Q-0zrdXnKd8rr8T3U2A"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("soMo5_Nb3qZsQFvxxptuTM1VQleUin1dLBJGA8qhNK9wPUJU6fC3JLvjN5zSTlhutSZHMiXqA_zSJ4EbyyOr-w"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content", "category"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("-o9pxhFDXtmit2EQQfxaya9snYPX1Mi7IaeB-mjOOjdLG2bwTJpAUfdnuFVw28ZqxYnNp2nskKRsG8cwG5A9KQ"), "title": String("Search Features")}

Execution Time: 4065ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: sv4S8fh2IZkAqUZU81VeCfyVGN6dV9_dGNmOCurpT_7crDabbcxL8g6virx3Ov0pYvby8s-UwnngodaYhN-okQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, one product available is the "ekoDB". This is a high-performance database product priced at $99.

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
✓ Created second session: We-cGv7b94MoxJd3A7vLwsVvnPb2qnW2A2SPcwXauV2x_6VwPpPz28nGCnhz1NGsCtHfNXyOfLwtFENoEJ2KHg
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
✓ Created session: b3lg9Yy6htKsJGVHXEZH9XVx52hF9v35CMNfcM9ODLsCTUXxJBBw7oPw2uhR0kv2BowTcwuqcUIJOd2LaNzNyQ
=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is "ekoDB". It is a high-performance database product with AI capabilities. The price for this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: wkPuzczGQhsEWymxLejsRZrv5Ufch0b5GmI9zzLt6rC3kAaiwMBAjhn8AzUXVPklIcUwl2AS66nGLjr8IV5DBQ
  Parent: b3lg9Yy6htKsJGVHXEZH9XVx52hF9v35CMNfcM9ODLsCTUXxJBBw7oPw2uhR0kv2BowTcwuqcUIJOd2LaNzNyQ

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: wkPuzczGQhsEWymxLejsRZrv5Ufch0b5GmI9zzLt6rC3kAaiwMBAjhn8AzUXVPklIcUwl2AS66nGLjr8IV5DBQ (Untitled)
  Session 2: b3lg9Yy6htKsJGVHXEZH9XVx52hF9v35CMNfcM9ODLsCTUXxJBBw7oPw2uhR0kv2BowTcwuqcUIJOd2LaNzNyQ (Untitled)
  Session 3: We-cGv7b94MoxJd3A7vLwsVvnPb2qnW2A2SPcwXauV2x_6VwPpPz28nGCnhz1NGsCtHfNXyOfLwtFENoEJ2KHg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: wkPuzczGQhsEWymxLejsRZrv5Ufch0b5GmI9zzLt6rC3kAaiwMBAjhn8AzUXVPklIcUwl2AS66nGLjr8IV5DBQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("3dT9Pi9OeKerrUfpQPauOZn3BkCj0cUDQfmg4ja7xZPA9rybnur30gjEg41POUy1ykaAeUcZJp69JjNvlkyt3w")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"age": Object({"type": String("Integer"), "value": Integer(35)}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "email": Object({"type": String("String"), "value": String("bob@example.com")}), "name": Object({"type": String("String"), "value": String("Bob Smith")}), "id": String("3dT9Pi9OeKerrUfpQPauOZn3BkCj0cUDQfmg4ja7xZPA9rybnur30gjEg41POUy1ykaAeUcZJp69JjNvlkyt3w")})
✓ Second upsert (update): Record({"name": Object({"value": String("Bob Smith"), "type": String("String")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "age": Object({"value": Integer(36), "type": String("Integer")}), "id": String("3dT9Pi9OeKerrUfpQPauOZn3BkCj0cUDQfmg4ja7xZPA9rybnur30gjEg41POUy1ykaAeUcZJp69JjNvlkyt3w"), "email": Object({"value": String("bob.smith@newdomain.com"), "type": String("String")})})

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
   Inserted with ripple: Record({"id": String("l_3QQyFoGM-xacqyL0yisrGvFSOrK0D54vgHQfo-CA2iG87D9CNqjIDJ3YqL29u7KbF1HuvUOhku3uipWDRFpg")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("Vcfb8B9KoJur5Jl9ScqL0dyRLgJhRlUpISkOSJmhlcgXjDsPs3L3QAAX83ikBOlov2A7VbfIdCrT2AIjEXUq8g")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"price": Object({"type": String("Integer"), "value": Integer(150)}), "name": Object({"type": String("String"), "value": String("Product 1")}), "id": String("l_3QQyFoGM-xacqyL0yisrGvFSOrK0D54vgHQfo-CA2iG87D9CNqjIDJ3YqL29u7KbF1HuvUOhku3uipWDRFpg")})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("KZAupb-oMPlSKW2yvuNr2CrykmLHw9ePDHtbVSmdk8MaECIXc-9UjasP2yLCla0JYXBqqndOozoVcYh_nHAOhw")})

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
