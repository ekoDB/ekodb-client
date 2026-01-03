make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("znHGJR5Caci3TIo7E6HUY9-wZwzftjQVwaRGkeeFM7HdxTP3H1Ccsrz-nDVHyP2WNjyt3xFb3bmcORFX3vFJXQ")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("znHGJR5Caci3TIo7E6HUY9-wZwzftjQVwaRGkeeFM7HdxTP3H1Ccsrz-nDVHyP2WNjyt3xFb3bmcORFX3vFJXQ"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("znHGJR5Caci3TIo7E6HUY9-wZwzftjQVwaRGkeeFM7HdxTP3H1Ccsrz-nDVHyP2WNjyt3xFb3bmcORFX3vFJXQ"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("znHGJR5Caci3TIo7E6HUY9-wZwzftjQVwaRGkeeFM7HdxTP3H1Ccsrz-nDVHyP2WNjyt3xFb3bmcORFX3vFJXQ"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "hVJrJrqbHjCEisTnEM8A-FztQmjbfEL92UsvIUHFCyH4j45VgMzWEtQyv_b1grPXz9_tC-9hntazuuCZdw86tg"

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
        "id": "hVJrJrqbHjCEisTnEM8A-FztQmjbfEL92UsvIUHFCyH4j45VgMzWEtQyv_b1grPXz9_tC-9hntazuuCZdw86tg",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "Lbl5L5PyVQQD0UI30cTm_OtkOVFq4FdtZn1xZFT8KOIEwasv8OPwxGssKq88jwG2aMO7EUg25fYKci3AqVklUw"

=== List Collections ===
Total collections: 4
Sample collections: ["test_collection", "demo_collection", "websocket_test", "batch_users"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.57s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "ICoBTmrJD4rm-MALFoFZJNisiRuLFt2p0xE_fvMShwvrDboOs3i2PwhNs3d6ZE4clJ6QW7q4ccUOw2-nhtDQ8g"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "MXnJBI_PApWm0Mo9Pn5JH_QpdC_YslRcWLvMaXQLJDKQVqINGE8tyqYRgKaoxPSoSV454VlJO1knist1uBh4bQ"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "9gMG_R8TZsDhgorR83yyfdGjIcEWhubPm01NdUtPBCjYDdFh9Yg7JNqCQ-ctlc2vSFN3bN7jk7OgH2cEiRP-UA"

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("lWFB7feWOjNDgEocAV3BV8rJG4YE_h_8LvjNzQ7gWD3jcdf3B1fJSSCkkpUPMA0Y4Y4zq1iKfVfa7khVfYPldA")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: HQ0RPHw4_AYex_GAbLrAO3KD0jTatzBIsrZNTqaiLMVFRF5LXwzwy5TYbUeNI0-2sygax_zM5ddq3pS96ZQ3-g
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "qkKO7H0JlQbsQ_jTX8JimuDk9F4lwD09LDtYUamt3_o6dAASyw47J8RphC0T4FUPtPrJ_vdz1UNBdJHdDC-E8g"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: J1C9gVuLSNDO4ciBAAy8B5RcQHDi5I2RQq1fMk5inSiLF-OMy8pK_KJPFuTXIo8ggfTjsbDpetGuLtR8_kWtWQ
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 9EnOFUVDwPkTgvhzfq9Ova8m_Ck_K2ef6mUQoCc1nVtu2BHjLZoVy9lZEMYv-OVwHkQOdyR06t8OPuNirPZLzg
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
Created Alice: $1000 - ID: P_gsuwe1zzDFXbtog3F3KhVKjAC0PPH9aw9chC3KVBN6bxGZQ16Piwn81ZMcO_zFot9hfsMufravvfGH30IZbQ
Created Bob: $500 - ID: Y3fIdJdUJWm7g1Z151g_i6Y2lFiQYQmwYtIJPEJdS6VBOZhn1478ww6Z0dy4_t600LnuDtkHo_nabOktYBFXcg

=== Example 1: Begin Transaction ===
Transaction ID: 18964ff6-7265-46ba-8eba-e562573ca269

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
New transaction: c50a0ee1-13c0-4272-a5a6-4c5816cd55fa
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
Inserted: Record { fields: {"id": String("nCpuQExWTtGXQiAKcCH8ASeSp_HtzO0MzIVQoyy6JSRHA3qhxyCS_WAPc0Xu07ga-qARmTbYIJlG4BVlUXm6jQ")} }

=== Find by ID ===
Found: Record { fields: {"embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "id": String("nCpuQExWTtGXQiAKcCH8ASeSp_HtzO0MzIVQoyy6JSRHA3qhxyCS_WAPc0Xu07ga-qARmTbYIJlG4BVlUXm6jQ"), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "created_at": Object({"value": String("2026-01-03T05:25:56.604651Z"), "type": String("DateTime")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "name": Object({"type": String("String"), "value": String("Test Record")})} }

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-01-03T05:25:56.604651Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record { fields: {"created_at": Object({"value": String("2026-01-03T05:25:56.604651Z"), "type": String("DateTime")}), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "value": Object({"value": Integer(42), "type": String("Integer")}), "name": Object({"type": String("String"), "value": String("Test Record")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "id": String("nCpuQExWTtGXQiAKcCH8ASeSp_HtzO0MzIVQoyy6JSRHA3qhxyCS_WAPc0Xu07ga-qARmTbYIJlG4BVlUXm6jQ"), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)})} }]

=== Update Document ===
Updated: Record { fields: {"id": String("nCpuQExWTtGXQiAKcCH8ASeSp_HtzO0MzIVQoyy6JSRHA3qhxyCS_WAPc0Xu07ga-qARmTbYIJlG4BVlUXm6jQ"), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "value": Object({"value": Integer(100), "type": String("Integer")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "created_at": Object({"type": String("DateTime"), "value": String("2026-01-03T05:25:56.604651Z")}), "name": Object({"value": String("Updated Record"), "type": String("String")}), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")})} }

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
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
Collection created with first record: "o7a5FDF5Bm2_7JsUqBD2VPup_8OgrsT3jA0NdbivLlsAnCaJZVLne8EluuoEExwlgSdSB4gdvFCuk2crpseFXg"

=== List Collections ===
Total collections: 9
Sample collections: ["ws_ttl_test", "test_accounts", "scripts__ek0_testing", "test_collection", "users"]

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
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "Y76FJTK5RjZTy8Trrs8L82wSskSJlW_GxupRhvQrbYSvJDVl2m4tjr9kT2zDfiIJQx7udC0FStD75PBHNxrhNA"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("SjaRZmL42lwvww-EWf_MCXIncnPWAYS7hYYxT3hOlBzeEEULHB6gMZ1CPaiLHpWpsz5ZHSfRDOtQsgSc-3buJg"))

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
✓ Inserted test record: g36Peb1hMk4uA9XhALK6VYU27ooPKy8F8owPHpezrOxgGFKK-sSd0UUqrBjdqCkvZQM3MqToXIcasjBXCA3J8w

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
✓ Inserted document with TTL: Some(String("w-bc3hnyjTSXAgQjGczDIlLD2BAH25G_befAZ4a1xsN75t19u5PI60oFHXcmSrQ6SAnn_IrTDqHYw3v_hWBdKQ"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
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
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

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
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
  2. Some(Object({"value": String("David"), "type": String("String")}))
  3. Some(Object({"value": String("Alice"), "type": String("String")}))

=== Pagination (page 2, size 2) ===
✓ Page 2 results:
  - Some(Object({"value": String("Charlie"), "type": String("String")}))
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
     Matched: ["description", "description.value", "title", "title.value"]
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
Execution time: 1ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_schema_management`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("2f80Yos-XXJBj3lRQwE7hfQ8Ek3zhkoxyNwJcmJwr6LbRMCnMnYf2_BQONG6jpBH66PddJ8Al-CjAKNJsNWISQ"))
✓ Inserted user 2: Some(String("acvGhjeZZsGTMhBnHQsMdvGUQS1EMdcUbi0Ld1izSNK58O6WeTnGEZ9_fBR6WNNrRbThlvjajxKikiss1omlIQ"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - email: String
    (required)
  - title: String
    (required)
  - age: Integer
  - status: String

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["ws_ttl_test", "test_accounts", "scripts__ek0_testing", "test_collection", "users"]

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
✓ Created session: Qq23Wydaedyu53YVx8GKmxuBLGgTrRMMJJrULpbZ9S1uB9m90eEgQQja0jX0_QNEgwLr3vzJYz91qxxD6Dh-tQ

=== Sending Chat Message ===
Message ID: Bid_irZa4CaIE31DIxwxyVtNaUx0vSpmYV_5wGlMqeR7fRajdP7Iq11DQ9wfr0-KsF3H_sKen-OdGA7jWa8O3A

=== AI Response ===
Response 1: ekoDB is a high-performance database that offers intelligent caching, real-time capabilities, and AI integration. It's designed to handle a variety of tasks efficiently and effectively. One of the key features of ekoDB is its AI Chat Integration. This feature allows you to query your database using natural language and get AI-powered responses with relevant context. Another notable feature is it supports full-text search, vector search, and hybrid search with automatic context retrieval. This means you can easily locate and retrieve information from your database.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["content", "title", "category"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("oHcO5LscjMkiPWG9TVNJPCL2-Jt6-nizaKEImbAXBh-SWJdGLKNkm62gUcCx5pxxsKGOtROxZzroU4s-q-rCWw"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("Nn2UTfl1F1SDnC0CI6awigE5CY0LNnkFHgffhBsWsyYM9tE-iL-kbyUZ2KqmWyya8kSI9ImfEh6FWH61-CskoA"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "category", "title"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("N-ZJ3NcskvL2lly0SCK8Zi6B2LgII8NwsjnQmSUlYSiB34002sJp8T2rz1XZBIssF6iA_UA67XGhp0yCCbCaMQ"), "title": String("Search Features")}

Execution Time: 3055ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: PNbBYjnZytJfK4lVrRLNRsiBxwy7a78bqkqjn6t5ALLHU37liYPxCrve7K80XGdiFzBiAyyIZnLhiCk0RpcR3Q
=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the product available is "ekoDB", a high-performance database product with AI capabilities. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 4ItX11H4EyvHhc337eW7slFRjUkZY7gPIgDZU6b4HWPj-5AOQZTNn2KH2fGbJpjWCD6Z2xrc9YPtnUT8MmeRSw
  Parent: PNbBYjnZytJfK4lVrRLNRsiBxwy7a78bqkqjn6t5ALLHU37liYPxCrve7K80XGdiFzBiAyyIZnLhiCk0RpcR3Q

=== Listing Sessions ===
✓ Found 3 sessions
  Session 1: 4ItX11H4EyvHhc337eW7slFRjUkZY7gPIgDZU6b4HWPj-5AOQZTNn2KH2fGbJpjWCD6Z2xrc9YPtnUT8MmeRSw (Untitled)
  Session 2: PNbBYjnZytJfK4lVrRLNRsiBxwy7a78bqkqjn6t5ALLHU37liYPxCrve7K80XGdiFzBiAyyIZnLhiCk0RpcR3Q (Untitled)
  Session 3: Qq23Wydaedyu53YVx8GKmxuBLGgTrRMMJJrULpbZ9S1uB9m90eEgQQja0jX0_QNEgwLr3vzJYz91qxxD6Dh-tQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 4ItX11H4EyvHhc337eW7slFRjUkZY7gPIgDZU6b4HWPj-5AOQZTNn2KH2fGbJpjWCD6Z2xrc9YPtnUT8MmeRSw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: ZQMeBQpvkMYPXIPtB04lqTzVy_bldnLzUmEYo2GYWDX9SLqXTgj9hcTnOttq0W3wuyGs7_c7yVdI0k9KYwLijw

=== Sending Initial Message ===
✓ Message sent
  Response: The product available is a high-performance database product named "ekoDB". It is priced at $99.

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
✓ Created second session: ufrTyD4PrTBEQptwzw4r9KilyotTBDWqwmk4QImGnlZfBf6Qx-Xc8uVTAkyswCKMpDmpE7KDxy8Iez2OfoOILQ
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: ThzV1p4FjzxJ-_qS8sEsPmPJQeMrqbqfqIwzsmBkSGNKvX4dC9Xk3zsdXUPmlqS7owZeCJewnyqVwjMrm1bGHg
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: FPVLpEoR71zXE2yIkQ1UvEhfJJJXSxSKv3Xhr5HXh6x9YJZg4WwlfPdI3yxAgxR_ef_aYyS5I4zsaz6xg1hbuQ
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: TjqCiXvnlZYdI_NYa-2buCyPMSDoXVWP3CCer1IpkmDGUzbOZ2Ybx8ZKM8zbS1BVGej76wutzKIZBR1Rj_4GZg
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
Created Alice: $1000 - ID: PnlpKVA3SQL8IzY7MjOG228Je1NhDTFZyJjO1gCZKiufOHp9uzpYYCkxNNb4MWhh3SL0L2p9lbbhUSDsLjxuGA
Created Bob: $500 - ID: Lk8kxzC1BTuNIaS96uJVbQmqcyi2LfuZcg5CZYX7P9PV81-DQlLszfSnT-g0tbj2FsCyU0ISeKWNkMnpgwuomQ

=== Example 1: Begin Transaction ===
Transaction ID: 4beab1f9-1f69-457b-9bbb-4b6ac06cad94

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: aa0c840a-8015-4544-b6b5-cd2c674a6bc3
Status before rollback: "Active"
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mRust client examples complete![0m
