make test-examples
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
✅ [32mAll Rust integration tests complete![0m
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'srp2jVhnwqfjXydVN6usmsKiebh3rtZGM_IMPSFTDH1LEiHMfBqj2rfBsq_dKlj-83D1rFeigvcfw6myyTr3NQ'}

=== Find by ID ===
Found: {'id': 'srp2jVhnwqfjXydVN6usmsKiebh3rtZGM_IMPSFTDH1LEiHMfBqj2rfBsq_dKlj-83D1rFeigvcfw6myyTr3NQ', 'value': {'type': 'Integer', 'value': 42}, 'name': {'value': 'Test Record', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'id': 'srp2jVhnwqfjXydVN6usmsKiebh3rtZGM_IMPSFTDH1LEiHMfBqj2rfBsq_dKlj-83D1rFeigvcfw6myyTr3NQ', 'value': {'type': 'Integer', 'value': 100}, 'active': {'type': 'Boolean', 'value': True}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: pgX3tN0CrrfdwZdfEP7IfhBBi2_OMTlOZKaFUVkClPD4NDbdz0ze2s_DBBIMQ5LTxd-lIMtq_vWFdt_XE62GnQ

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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "pgX3tN0CrrfdwZdfEP7IfhBBi2_OMTlOZKaFUVkClPD4NDbdz0ze2s_DBBIMQ5LTxd-lIMtq_vWFdt_XE62GnQ",
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
cache:product:2: {'name': 'Product 2', 'price': 39.989999999999995}
cache:product:3: {'name': 'Product 3', 'price': 49.989999999999995}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: C-Knh7qvWpKtP898TWo5DCz4Tji3kZTqUSezR3EVUeKlGA88lxgfcO2qA0h1AU1ic1l6pjrYQRA7O1JWPgkXbg

=== List Collections ===
Total collections: 11
Sample collections: ['ws_ttl_test', 'test_accounts', 'scripts__ek0_testing', 'test_collection', 'users']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: ImxmN2hPkQlZUF2D4wotJZH8rmTz8Rhocnks4BHxn3OKjK53qH3qxjC1J8KeG9HMu94BqfVeo0Ry2E4yoVSjRQ

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: created

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: -_X9YAbX_hjIO7vZk1YxHo_RU553sPNWBbUHRHi8stFVLVB7Y8Rs0eEAy-Ntoak-Xj5zTeTYt3KTleDaHbgCTg

=== Query Documents ===
✓ Found 5 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: GvbN3QMJ-nIAwyz_bnQy1y9vphKyw3ojyXpUHbnf8j0NNZCnRdy6t8X_-tv5UgRj2zxb8sjOtBLVIP7PWDSP8g

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: lb_nGCGP3nSMANGUW_ZDifPRJ-xOmzxHovFzSCGUo7Ika_07EjhTgn84UT176lib0FhlopK390RJAzjmkV016w
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: hYC5ETvWHPERsjJzB9ISHfxZI8LuCAtGQvrTcjwKkN8vpv9SxLSSLLNOJVcrlaD7kKnFLgg6n9EJXbb4Y_aZug
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: EhappmHeMV8qHHfitpEIOX760Ql4mPhB8fwP0Zvpm9Nd0uapOemOrTceg8J8u5DSuBMH6RL6DzCN3uisFgTFTA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'count': 15, 'max_score': 100, 'avg_score': 60.0, 'status': 'active'}
   {'avg_score': 50.0, 'status': 'inactive', 'count': 15, 'max_score': 90}

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
Created Alice: $1000 - ID: OMCC9HawJggZKfY08o-c3pIBR7S6dzArjHRtSQbnIKVdQT8XiAyefVaGcsM95pDF5nmRcWlK7lRr2J215-POGQ
Created Bob: $500 - ID: AdFjJtlIvrucF2ONkb4_F1s3plk2e8enLPp2OspmhPhjCdSogTaNgFUC78tYFcdnZaPEW4I0o1gS8JcbVN43sQ

=== Example 1: Begin Transaction ===
Transaction ID: 1e532b63-a4c8-42d9-9331-4a4bf2e6ec40

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
New transaction: 078a0e76-9f39-45f2-bb10-ae7e1b978494
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: nVUxO58G-Y21SMTcT1qrSWvvTDR5OSfC1Upe-UjaytlXzluMhMZ7QavHEyscDAz81eI7wZNrFx7XNrEwsasIVg
Created Bob: $500 - ID: _UrPyZW1uAf2z50I-d4wqHy993VSeBchVWbWSYHmv1KDITkES_oYvvqXsIC96ivo86oXtsg4x_AU-X2fioochg

=== Example 1: Begin Transaction ===
Transaction ID: d38fa636-8f2b-4e6c-a894-0177403d7181

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
Created Alice: $1000 - ID: Z1cRvI40bfPE5XgROqCmQf52yrAwpWzr_Ed4MwnMDraxRIPfnakb3XiGF6Nj67BLKWWjL3dj7LIq9XTF6JyMrw
Created Bob: $500 - ID: VDXBPCLnKVfUiGCM7v_sYrB3sI1YBgYvOBdhpmZxdemEYLPVuHh7lTKcjNao7EbllJuATjbC-SYpE26anuSDhQ

=== Example 1: Begin Transaction ===
Transaction ID: cd1bec71-c7f7-4dd0-9f5f-7b8a5ce59f5a

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
New transaction: 52413ae6-2fb8-4693-9419-d5ee030ff5b4
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: bZ_9lLpg-Svj-jYE7PAcNp9VFQYIWDbkdr0n6Q3AmyRXL3c6v1IJI7A_3o7cRMyTNclcJg3Lr9GLj5O974sOTw
Created Bob: $500 - ID: rkRezE83F-RlZB73CrucTTlruIVyTt4l5cc7bDJBu8pmxOqI2h6jp-4p0ylL2j6iMrifE6-s3JyJ6Zi-FkqCKA

=== Example 1: Begin Transaction ===
Transaction ID: 13853489-b1bb-4cb3-8bc2-5bf806197c3f

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
Created Alice: $1000 - ID: Dz1AGiTY0sMJzCaZP8hzZAtUzObALe1ZZFme-5QtTCaDdXkkcbAvhEyEDh0ErIcvafHCKB4DghvgvJ42mIUFyQ
Created Bob: $500 - ID: xbvZ5U2PzWBJCFKJQdMkQYhHAOj-mKHr8twiQII3uJ9B9aOIO4fEufk4CUde40Ayer_zu-2F0AaTntGdJ3i8qg

=== Example 1: Begin Transaction ===
Transaction ID: d63e81fe-b379-4651-9e91-01aa7fbd1da9

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
New transaction: ff119684-c05d-4650-a5b4-4f19829e1014
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: wHPCQp-P82CR_Pi-H-xl47RZateFI8unueTbZfJYD31HQw56VudWTHkcBXXE09Cm4fSAJotHVPCSLx0u8jI68g
Created Bob: $500 - ID: 9UP3bEAGeW4ny3FKefvIor-cWQvCVGdRegXgoycBdgwzRRnJLD9dWGZiNNlflrsgMv57rNkTkee7Qj4CJaEPOA

=== Example 1: Begin Transaction ===
Transaction ID: 125f761a-4de9-45df-89bc-06fb85e81b45

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
   ✅ Script saved: _bjMZgw3HgakutDxZXS3RaxpzhnhimeAfw8RFtCnfi4PVcdRqzqdUJpJxzazvst8c55HE7TqufVV519rrDWH6w

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: EfymqsGklObH4QW4dC8Hg4QaJMibg0wg4CMRqcLEdivk0y4NEdwDIHHRlObHExJf8WpCefwaYYxjZ6Bk9DjeZg
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: FlBv5x0gE1Q2lqNYoQ0F2WfW8nFChzDfvKj7iJ57th6O5apE2oUZkVLVfqrDp0bQQFqEf66_vKl_7qBlV9SK0w

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'type': 'String', 'value': 'active'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: vhPZs36QwJd-5WESWr5pHcIcDC35Bhhjzr8jATlsqRJsdOR3cWLyxgky2s5pya-fjLlTjtmoe85NYMRKREVzZg

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 8x5XuMrMMB6x6Eu9fhXf9l5VJSxO8vWigCc9ja4X6dIYC378dkI25IdbyS6TdADEiu7HLuqMMt2aItgz0DpKtw

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: _bjMZgw3HgakutDxZXS3...
   ✅ Deleted script: FlBv5x0gE1Q2lqNYoQ0F...
   ✅ Deleted script: vhPZs36QwJd-5WESWr5p...
   ✅ Deleted script: 8x5XuMrMMB6x6Eu9fhXf...
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
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.6.1-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.6.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.6.1
    Uninstalling ekodb_client-0.6.1:
      Successfully uninstalled ekodb_client-0.6.1
Successfully installed ekodb-client-0.6.1
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'iPEEe7_k8MvltVX2mtjQEP4knaJdiMfY8om88f7FMQJz1YGMWtAVROkUURF8mNaEtu3vyN_3EDd6H-TgaWVaaQ'}

=== Find by ID ===
Found: {'active': {'value': True, 'type': 'Boolean'}, 'name': {'value': 'Test Record', 'type': 'String'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'price': {'type': 'Float', 'value': 99.99}, 'value': {'type': 'Integer', 'value': 42}, 'created_at': {'value': '2026-01-03T00:26:43.177847', 'type': 'String'}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'id': 'iPEEe7_k8MvltVX2mtjQEP4knaJdiMfY8om88f7FMQJz1YGMWtAVROkUURF8mNaEtu3vyN_3EDd6H-TgaWVaaQ'}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-03 00:26:43.177847
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'active': True, 'name': 'Test Record', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'data': 'aGVsbG8gd29ybGQ=', 'tags': ['tag1', 'tag2', 'tag3'], 'categories': ['electronics', 'computers'], 'price': 99.99, 'value': 42, 'created_at': '2026-01-03T00:26:43.177847', 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'id': 'iPEEe7_k8MvltVX2mtjQEP4knaJdiMfY8om88f7FMQJz1YGMWtAVROkUURF8mNaEtu3vyN_3EDd6H-TgaWVaaQ'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'price': {'value': 99.99, 'type': 'Float'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'created_at': {'type': 'String', 'value': '2026-01-03T00:26:43.177847'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'active': {'value': True, 'type': 'Boolean'}, 'id': 'iPEEe7_k8MvltVX2mtjQEP4knaJdiMfY8om88f7FMQJz1YGMWtAVROkUURF8mNaEtu3vyN_3EDd6H-TgaWVaaQ', 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'value': {'type': 'Integer', 'value': 100}}

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
Collection created with first record: "kiVTAFXPjgEMx2h-kemDQzl6dh6VFeWayIcSBZAmdeZP6HBHYbHonxl4XkPQ3QYOvKbGmYe4OvnL7esm3QV7Kw"

=== List Collections ===
Total collections: 10
Sample collections: ['ws_ttl_test', 'test_accounts', 'scripts__ek0_testing', 'test_collection', 'websocket_test']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: o0t4Ogtcx344b_kF5Yd8MA47oF001mLnT9U_EH-LgywkmlE22VMOtjGPmoyZBF0kh6s_8Yn1MgEaJ3P_8vBHCw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: W4J--01h8oJhi6YGr2I_OqFLm9fW2OtAjqLNP3--z6bn2Fb4ofpOG2AxufcanYcmBGVJdFJ0eJWSHFPbjwpEdQ

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
✓ Inserted test record: tXT_J49TYn3DdeDrDl9lxmRajVYMl6aHfB4At3XeCThzqhvnJQJElqT_O7QHGTr2D8QsASHJOxCCZPsFNsu9FA

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
✓ Inserted document with TTL: pd6E4qh64nVlX12W0W5DXgy6NNPdWowarRMO4ZeqyWmdvGHa_oKDlu05J5yszfeghPa2vyP3ogVBtH137smS2w

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
  1. Score: 25.740, Matched: email, name, email.value, name.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, title, bio.value, title.value
  2. Score: 26.400, Matched: title.value, bio.value, bio, title
  3. Score: 26.400, Matched: title, bio, bio.value, title.value
  4. Score: 26.400, Matched: title, title.value, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, title.value, bio.value, bio
  2. Score: 39.600, Matched: bio, title.value, title, bio.value
  3. Score: 39.600, Matched: bio, bio.value, title.value, title
  4. Score: 39.600, Matched: bio, bio.value, title.value, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.776
  2. Score: 0.764
  3. Score: 0.764

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.735, Matched: title.value, content, content.value, title
  2. Score: 1.143, Matched: title.value, title, content, content.value
  3. Score: 0.535, Matched: 

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
✓ Created session: zzMnYpnbOdEiHo60NW7v27gF22aW9C9vI17DJPbTOj_dVJ8W5PrS1iXVv3DLxXuW_gp_SHv8N7h3_9yZz7UGpg

=== Sending Chat Message ===
Message ID: u86ztLXecbAIBWAZGOkniLpulTzqJciFFdIcobTwnnbDEUsyAOJfpiNc92DByC3UMnyZMDjkaTzrP0fbvgdrFQ

=== AI Response ===
Based on the information available, there are three products:

1. "ekoDB Pro" is an enterprise edition product with advanced features. Its price is $299.
2. "ekoDB Cloud" is a fully managed cloud database service product. Its price is $499.
3. "ekoDB" is a high-performance database product with AI capabilities. Its price is $99.

Execution Time: 3195ms

=== Token Usage ===
Prompt tokens: 616
Completion tokens: 77
Total tokens: 693

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: qZMycTvr9bUB7YzLX4ZcWbXjNc0DhwM1cTVcywNKzMxIiU7HUYO1RD5mktQvJD1w7qRXgiu87_BhTzLj1_HUKQ

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, the product that is available is "ekoDB", which is a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 5n465zdaC6KlllJUCSAbZ3C7OUyZ9z8OWjuLEKnkn1GuzNHzVTc1HIdYDOwc6uoGRTZ9zcGQvuJYzVPhtiPxcA
  Parent: qZMycTvr9bUB7YzLX4ZcWbXjNc0DhwM1cTVcywNKzMxIiU7HUYO1RD5mktQvJD1w7qRXgiu87_BhTzLj1_HUKQ

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: 5n465zdaC6KlllJUCSAbZ3C7OUyZ9z8OWjuLEKnkn1GuzNHzVTc1HIdYDOwc6uoGRTZ9zcGQvuJYzVPhtiPxcA (Untitled)
  Session 2: qZMycTvr9bUB7YzLX4ZcWbXjNc0DhwM1cTVcywNKzMxIiU7HUYO1RD5mktQvJD1w7qRXgiu87_BhTzLj1_HUKQ (Untitled)
  Session 3: zzMnYpnbOdEiHo60NW7v27gF22aW9C9vI17DJPbTOj_dVJ8W5PrS1iXVv3DLxXuW_gp_SHv8N7h3_9yZz7UGpg (Untitled)
  Session 4: ufrTyD4PrTBEQptwzw4r9KilyotTBDWqwmk4QImGnlZfBf6Qx-Xc8uVTAkyswCKMpDmpE7KDxy8Iez2OfoOILQ (Untitled)
  Session 5: PNbBYjnZytJfK4lVrRLNRsiBxwy7a78bqkqjn6t5ALLHU37liYPxCrve7K80XGdiFzBiAyyIZnLhiCk0RpcR3Q (Untitled)
  Session 6: Qq23Wydaedyu53YVx8GKmxuBLGgTrRMMJJrULpbZ9S1uB9m90eEgQQja0jX0_QNEgwLr3vzJYz91qxxD6Dh-tQ (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: 5n465zdaC6KlllJUCSAbZ3C7OUyZ9z8OWjuLEKnkn1GuzNHzVTc1HIdYDOwc6uoGRTZ9zcGQvuJYzVPhtiPxcA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 4Y6v4MUApBJKT2bv1pUeGfDEENiQ8e3OfWdsi73KUmsh1rhNECkJvadXd0TCeZESatNR8-7DFrMYZ0jDCVguBw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, we have the ekoDB product. It's a high-performance database product.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['chat_id', 'context_snippets', 'content', 'token_usage', 'role', 'id', 'created_at', 'updated_at'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is 99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: GcI_-JS9SW66hdDUtOx8VFC7p1LwmVDKuARk01Ave2Iv5cQ4tA5LKv4tnVsbWwxppCW-Vc4RbAEteKlXSvJ3aQ
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

✅ Script saved: v9VWm-1xZu2VYtjZurZR4p6_kglTUN3_hq6uUMQSXoQn5P07THjujwGzSYH9zZzs74l-xmcSdOFLt9fGkmkRLg
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: INJ3Spv9K_eDrWVl3XXXyi-EjT63mc4IRDsG143BGQg-ha4OS5DpqwPUIlG9hWc8GuLuf1XDEQYnhdCjGdB12g
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: NVU9rQhsAbe9wLSPc2-8eq-JK-W7CVw5rcP7jzehTRUjfSyN7zvvAqkLakg6PVTeOJJ-LTY8MXUyEKa4qpQhYQ
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

✅ Script saved: KbLUpn4yvbpM4BR5sk6-xm_MC4kWh5KaqJ-ORfFwsWnrXSlSzvGQcrnELHbDzYMdarhRjY1j4cMeJIXh4M-4NQ
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
Created Alice: $1000 - ID: ZYgX5cS6yJo1GUdAzqHyW9SkCQFU2HRXDQKMNSL33V_gbrF1i_CSDUqMOsO4lUJYjS61os9aQPevngmfbrjXDQ
Created Bob: $500 - ID: eUcYQ65l91nId1xIGzm-AcLgf3HbnHAG5z4qGzM0famk54KTyqO2oaXvCovAvDVd1j9rB7yOkyaZWCfoZ71cOg

=== Example 1: Begin Transaction ===
Transaction ID: ae8e2a42-dd65-4eea-b889-5506cf4ba342

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 3e3f4e69-e14d-4e28-81bb-285a135c79d7
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
Inserted: map[id:hIn_DzOmZ2Zakn3E8o_i2f-azmPWJGunBy9U-UqT42gt76-mP9xQJv2snT3farkwqiUYmj1fT9ZtTnLRCh48pg]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:hIn_DzOmZ2Zakn3E8o_i2f-azmPWJGunBy9U-UqT42gt76-mP9xQJv2snT3farkwqiUYmj1fT9ZtTnLRCh48pg name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:hIn_DzOmZ2Zakn3E8o_i2f-azmPWJGunBy9U-UqT42gt76-mP9xQJv2snT3farkwqiUYmj1fT9ZtTnLRCh48pg name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: MC6awzEkQm2HxIFz8YHls2Ws7hmMZSn51NM_fz8xZSMVu3ZXvP8tNXq4OgMXARzOhP5i5qmyxItK6259PZlT4Q

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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "pgX3tN0CrrfdwZdfEP7IfhBBi2_OMTlOZKaFUVkClPD4NDbdz0ze2s_DBBIMQ5LTxd-lIMtq_vWFdt_XE62GnQ",
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
        "id": "MC6awzEkQm2HxIFz8YHls2Ws7hmMZSn51NM_fz8xZSMVu3ZXvP8tNXq4OgMXARzOhP5i5qmyxItK6259PZlT4Q",
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
Batch insert response: map[failed:[] successful:[RMaOnh2NCqA5RIjpx_HYQHmwB4jPM0QDI7n003GXErIK-OHIh6Rv30A09GbZpO5wvaPCKzCacvip-wn8l3hOAw rExra22oLriRSCbK3Y5Ynbww1NAPCGbwBmnCJP9BU2Nmsr21UOhcA-dR9KrM4nFHw1fDDU8thNVdO0dTHK6QFA Wi-scVLNefOr8CZw2sc_HDQvpdFVeBSWLJwDbTPB-Br7li00NZ55hHOs_L_BGb5waqQjcBqoaFRxd1gjadaiAQ iPnc8Xs_JZHfUy4KG_X5tQTjacSQiyIMLSISO1mdUyl4AQXvknVSJDJ4wYpOUe5Ya3SqgN8nOvyM50vLNyEPFw rP_UDf9oRN_Q6jlrI0r0001E5MSJMxQCH-xQjou7ofrHi4fBdqGwdf8Z8WKlVurXYv1Mf2vIIY8PI2EScrUpqQ]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[w3D1mJN8YOxfb1I-VXK7Ightymio1frNegPQsp61UhwL8PuGf2dDYVLKySFIWj3WxIaDsZNPtpQSeiS_8g9WFA MPybfMi1k3VoHYESexay6YKFwNOijIGKHfF2chYTovsIWTYnea1hjUa0nEFHMEvfsLjf8aQ0Yl8pZ8woyD9hsg R_zjmKCZrnHaZHnW1c_iI9CcHKf_xZgsRqBQ3BDZRbGfyGReAlhNMvcbYOualHSoOid3YSGppS-p7kDCIiN4xQ]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[w3D1mJN8YOxfb1I-VXK7Ightymio1frNegPQsp61UhwL8PuGf2dDYVLKySFIWj3WxIaDsZNPtpQSeiS_8g9WFA MPybfMi1k3VoHYESexay6YKFwNOijIGKHfF2chYTovsIWTYnea1hjUa0nEFHMEvfsLjf8aQ0Yl8pZ8woyD9hsg R_zjmKCZrnHaZHnW1c_iI9CcHKf_xZgsRqBQ3BDZRbGfyGReAlhNMvcbYOualHSoOid3YSGppS-p7kDCIiN4xQ]]
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
Collection created with first record: BsfVPISm1QzJTRvtrNij_ht8AAF2Sm2UkPSu8hGA8c5e0e3KBy0vu_0plVxoxt12WrHd9Fm8_qeRjDYLv-ZuEg

=== List Collections ===
Total collections: 15
Sample collections: [ws_ttl_test schema_employees_client_py test_accounts scripts__ek0_testing test_collection users schema_products_client_py demo_collection websocket_test schema_users_client_py chat_configurations__ek0_testing schema_documents_client_py batch_users ttl_cache chat_messages__ek0_testing]

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
✓ Inserted document: hkP5NnZ-WfSVBEn3koVg5AqflHUqGwNYBItKd0KMi_fXHoXkKAsoN_CgDGlu97yntYE-grZGDZZlgD4MEXOc_w

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: <nil>

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: 2xn8f_Gr_7CuWfEXC-HPQOb7uO1aUXuCZdLaFx9Z558xYJ1a2og0MQ_IXmjYA9Nq5KRb8_Wo4FtyOmgh218h_Q

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
✓ Inserted document with TTL: ws2akVxiv4pwUdHcBuBZy1JLjFj1Ip9bB4Hc5avOeB8FmAyinKt6A-4oW4OEE4JasQi9tNNLZLpTj4RzSSzX_w

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

✅ Script saved: GNcBEGoQdC0KH_j6MmgSRtqQgV7HzfWexkIrU_zrtA-mJXiSfx3Mqq3UiRqps7AkIqfS_YhVai-EZwhYNaEdCw
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: oJjzEHq_-aXKpeGNxGJkTBTQVxRwGGp3YPAdhLve_8tOEfGLU0RylKz2D-gyuYZdgY8yC_2GLIq0t3PBfYYIRQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: RN2U9BElh8hLVQitjAlwjz6Dm81PH5vY9CnzZkTdxpDdtdT-5WV0GjLmku3JaLCK1DhksvgoWzSa0I23gdWh9g
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
Created Alice: $1000 - ID: VvRLok4WkXnEtftpjVUAkXpHxJIGHfo7A8MgDxIX8sgRmGbGpWTElxIolVNJMJDzzH5VYIE_m61XbMRDYyXR-A
Created Bob: $500 - ID: W1ynuIOuJN4KyW4X2hEBgc3SsZUsK8nuy2NBQiGj0sxq1xvhvNHL5sCvfzjDteIKBANq38_ni2k8rCZ1fgOYHw

=== Example 1: Begin Transaction ===
Transaction ID: d70b6d04-7a88-462b-bcd3-7f1530d945ae

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
New transaction: ff9b22fd-48ea-4f0c-a2b4-e4b081d1b234
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
   ✅ Script saved: NeeDNLkJzwhh_BmimGEHTltWfcNCyRlgv2Yhp8ABCeZ99guBc_QLePZ2SdyfrdPbjh8qfZY_14ijNJZIzp4icQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: B9EM5-l4jiNoOxDPCwur-QulkF8iJGbqQydi1jNv-MoXomJVReu4S2ts5xuCu4IiMw6I85Btz6DsdV-gxnbtbA
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: QLFW1rWytedQifAdCVVniJpDGjlrLRG7SAk4v9O_xOj5C4n_G9bnOLf4TolKIsuYM967dnaW-BTWFs5Vi2Eqvg

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
   ✅ Script saved: N59xLX7StPhOzyjtEvlsUi8EKybCFSlL9qhErOASd_Un-3L2LfewNg2l_hl-HxkiPAJ8xyVUWinXI0Fkp8eR_w

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
   ✅ Script saved: 5T5iPAT4GqAplIsO6pcmCCw8GOq2Ra0kC733xwy5BzKfYLun-3aBHbdg3nSzr61cDmxgM-5BJdTHlDm5OZsGUg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: NeeDNLkJzwhh_BmimGEH...
   ✅ Deleted script: QLFW1rWytedQifAdCVVn...
   ✅ Deleted script: N59xLX7StPhOzyjtEvls...
   ✅ Deleted script: 5T5iPAT4GqAplIsO6pcm...
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
Inserted: map[id:v3BTjctST3hoD7ReC_wrfFBGPphbNz3tyk9a4RwJTtdyDmuFyo8vlwgrDntrBL2uzVnkobkPt5NTsaM1BWgnJw]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-03T05:27:15Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:v3BTjctST3hoD7ReC_wrfFBGPphbNz3tyk9a4RwJTtdyDmuFyo8vlwgrDntrBL2uzVnkobkPt5NTsaM1BWgnJw metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-03 05:27:15 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-03T05:27:15Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:v3BTjctST3hoD7ReC_wrfFBGPphbNz3tyk9a4RwJTtdyDmuFyo8vlwgrDntrBL2uzVnkobkPt5NTsaM1BWgnJw metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-03T05:27:15Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:v3BTjctST3hoD7ReC_wrfFBGPphbNz3tyk9a4RwJTtdyDmuFyo8vlwgrDntrBL2uzVnkobkPt5NTsaM1BWgnJw metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

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
Collection created with first record: knQchNNXuB1IJWVmfG6kVqfE88v6x8wO92ytsr91-9R4a6wsKl2-XDMfTQ9U5KhEPgymhHfk4kCPuD8064PvLQ

=== List Collections ===
Total collections: 14
Sample collections: [ws_ttl_test schema_employees_client_py test_accounts scripts__ek0_testing test_collection]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: Y3-LqToVGOoe1BiXYRh3obgMaucO8_mKdX8ZLXY1dJuKa3MqjQZZOzBQ32ZaCY8wkkDJBluNyigdlco9Mt7jOA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: MLVjW6oke4poOPaHsnH4Yf-b_DIHZjaZ4XR8LNAzQVqAiuDYyjHuV1oCjKSofdUgv1qfBPXdOY2uQdaRSRq5vw

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
✓ Inserted test record: DKepBHs6Vm04ln1sKI6DGWvDjPp6783fuld3w36BiNttZYfuFkc-mLtzKXb8WIgUSpGlf1hOIygmDLvqidW5ow

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
✓ Inserted document with TTL: NL4LoXjvVvdBnFps6fxcBMQg16xeV_k_92KmXFw3BuVIhd89nUSGJGG_chsYwJyyO1wCVT40k9MXUWl87kx2CA

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
✓ Created session: SBvNH3R38sKAL9Bc6P62j4zXLJyCdfX2OuFUr3FbjkmewR9MVcxDSE7U2FSqLYy7ramHAlvT9fpWecCMl0PwJQ

=== Sending Chat Message ===
Message ID: jhpk8ms_UG0Ov19UqMei8aO7I-tY6XmzJwhSI9E69dzrjmxQvRt9t75Q2nmBTjD_DS4D3PxjFSEgX9DNsgIUSQ

=== AI Response ===
The available products and their respective prices are as follows:

1. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

2. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

3. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:aATLxp8BDAfl52VGbhiLFi1HR7cxUGTD5l9rxRlM4ffWWmjTKyFbAxj-eVV4s_t2vd-hVEzG3nDPRRGHyFMo7w name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:5Gf2JllYD3F_tVy8gZ3yUs6TUkxbPZAaNtnh8yIrtq6ZWk4SF2eIPGpVfhmLKtvx2D2G050rIMJy-W8Frpy94w name:ekoDB price:99] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:bUyxvrIX6qFDtmwOAmG69zm-ze5YwmBsA7RsGb_D7haOMt_GPABXKuLJKaXsEqOeMRT503KgWYOkJYXhOWFhDQ name:ekoDB Pro price:299] score:0.1111111111111111]

Execution Time: 2620ms

=== Token Usage ===
Prompt tokens: 604
Completion tokens: 89
Total tokens: 693

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: eLr7gqxll0XkMo0fngGviRakNgisqU8DCS85DaBsZs8CS6vga5l2NFW3o8tiPi6Hqdv_I52Fb4aGQ7L50MET_A

=== Sending Messages ===
✓ Message 1 sent
  Response: Answer: One of the products available is "ekoDB". It is a high-performance database product and it costs $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 0ZFpLmRckrnONUJw0gtusDvjrBosVj3-gOdkpdvKBmpRERqYyFS4PCpiwhXnLCj0tIMOyyWQNFIlRWNrjO4NBg
  Parent: eLr7gqxll0XkMo0fngGviRakNgisqU8DCS85DaBsZs8CS6vga5l2NFW3o8tiPi6Hqdv_I52Fb4aGQ7L50MET_A

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: 0ZFpLmRckrnONUJw0gtusDvjrBosVj3-gOdkpdvKBmpRERqYyFS4PCpiwhXnLCj0tIMOyyWQNFIlRWNrjO4NBg (Untitled)
  Session 2: eLr7gqxll0XkMo0fngGviRakNgisqU8DCS85DaBsZs8CS6vga5l2NFW3o8tiPi6Hqdv_I52Fb4aGQ7L50MET_A (Untitled)
  Session 3: SBvNH3R38sKAL9Bc6P62j4zXLJyCdfX2OuFUr3FbjkmewR9MVcxDSE7U2FSqLYy7ramHAlvT9fpWecCMl0PwJQ (Untitled)
  Session 4: GcI_-JS9SW66hdDUtOx8VFC7p1LwmVDKuARk01Ave2Iv5cQ4tA5LKv4tnVsbWwxppCW-Vc4RbAEteKlXSvJ3aQ (Untitled)
  Session 5: qZMycTvr9bUB7YzLX4ZcWbXjNc0DhwM1cTVcywNKzMxIiU7HUYO1RD5mktQvJD1w7qRXgiu87_BhTzLj1_HUKQ (Untitled)
  Session 6: zzMnYpnbOdEiHo60NW7v27gF22aW9C9vI17DJPbTOj_dVJ8W5PrS1iXVv3DLxXuW_gp_SHv8N7h3_9yZz7UGpg (Untitled)
  Session 7: ufrTyD4PrTBEQptwzw4r9KilyotTBDWqwmk4QImGnlZfBf6Qx-Xc8uVTAkyswCKMpDmpE7KDxy8Iez2OfoOILQ (Untitled)
  Session 8: PNbBYjnZytJfK4lVrRLNRsiBxwy7a78bqkqjn6t5ALLHU37liYPxCrve7K80XGdiFzBiAyyIZnLhiCk0RpcR3Q (Untitled)
  Session 9: Qq23Wydaedyu53YVx8GKmxuBLGgTrRMMJJrULpbZ9S1uB9m90eEgQQja0jX0_QNEgwLr3vzJYz91qxxD6Dh-tQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 0ZFpLmRckrnONUJw0gtusDvjrBosVj3-gOdkpdvKBmpRERqYyFS4PCpiwhXnLCj0tIMOyyWQNFIlRWNrjO4NBg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: K2RRbgESugzlMeHPJZqA2yHhb4Ju9ijLrzGbONA4ccJYma5uLt4P6aqrbYk2pXWoLb3FiUasvNI2EIUNZkMvUg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, we have the "ekoDB". It is a high-performance database product. The price for this product is $99.

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
✓ Created second session: ElVzQrLNy8abDUqTzInc62FK1DQNQ5oWxkOaO6_uS9oZlfd78IvPRsDC01kg-5fKbgtzqgyw2gruwFjn6PhEBg
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
  1. Score: 0.759
  2. Score: 0.746
  3. Score: 0.730

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.698
  2. Score: 1.504
  3. Score: 0.292

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

✅ Script saved: S2nP6NKLht6bYZ3oTfElIXNkAQCDLU2KSFj-9IPRsRSRCdsJvX2VUnCeHJdRVcX4go8ysrYC-5sqwwYAUGJ_zw
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
Created Alice: $1000 - ID: 4HsYS5rbCLK1Olexa_6yejzwGtM9Rg7vif5AVkOi9qmMCpaB_WyVz9TjnDR_P60c2MB4riysgag_B2809wslGA
Created Bob: $500 - ID: J23MG0KtGp2GkveWUDd9g4S2ftILt5X--GRDwAhnBwmdCwF0PRb_UH9u2xofYepJdaozkqFedYT1_IGEkmfwJA

=== Example 1: Begin Transaction ===
Transaction ID: 87a36560-ee60-4f6f-bd3a-1156a6e9f556

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 2c8e7adc-4c3f-48fd-9df6-c84e9f651b25
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mGo client examples complete![0m
✅ [32mAll Go integration tests complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.6.1 prepare
> npm run build


> @ekodb/ekodb-client@0.6.1 build
> tsc


up to date, audited 7 packages in 826ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.6.1 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 686ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'kNF5PWkWtRlI30ab-QzdURd0kdDXD6DJD1Tr1kEaPx9gHFcSWqCdDlswoEC-0GVL3VYV7_lzcpVb9xfcAlzZ3g'
}

=== Find by ID ===
Found: {
  name: { value: 'Test Record', type: 'String' },
  value: { value: 42, type: 'Integer' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  created_at: { value: '2026-01-03T05:27:29.300Z', type: 'DateTime' },
  active: { type: 'Boolean', value: true },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  price: { value: 99.99, type: 'Float' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  id: 'kNF5PWkWtRlI30ab-QzdURd0kdDXD6DJD1Tr1kEaPx9gHFcSWqCdDlswoEC-0GVL3VYV7_lzcpVb9xfcAlzZ3g',
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-03T05:27:29.300Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  name: 'Test Record',
  value: 42,
  tags: [ 'tag1', 'tag2', 'tag3' ],
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  created_at: '2026-01-03T05:27:29.300Z',
  active: true,
  data: 'aGVsbG8gd29ybGQ=',
  price: 99.99,
  metadata: { key: 'value', nested: { deep: true } },
  id: 'kNF5PWkWtRlI30ab-QzdURd0kdDXD6DJD1Tr1kEaPx9gHFcSWqCdDlswoEC-0GVL3VYV7_lzcpVb9xfcAlzZ3g',
  categories: [ 'electronics', 'computers' ],
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  created_at: { value: '2026-01-03T05:27:29.300Z', type: 'DateTime' },
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  active: { value: true, type: 'Boolean' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  name: { type: 'String', value: 'Updated Record' },
  price: { value: 99.99, type: 'Float' },
  id: 'kNF5PWkWtRlI30ab-QzdURd0kdDXD6DJD1Tr1kEaPx9gHFcSWqCdDlswoEC-0GVL3VYV7_lzcpVb9xfcAlzZ3g'
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
Collection created with first record: HYV6-vvf-PlKh0BaoMWDyPU1Occ7nxqX5FYMYFRLgnn7NAQqBxXLFiuHAU6Bf3puUgBF7H27oncfrHG-KGprGQ

=== List Collections ===
Total collections: 18
Sample collections: ws_ttl_test,schema_users_client_go,client_collection_management_ts,schema_employees_client_py,test_accounts

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: e9adUdmjXFfHTDEshjUxK0A6v4kv-zYosky9P40JYq0eeP0bPXvE78PLdp6MlTuYmQHTstMz-Uiz7Ww2-sWMMw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: OGag60L9kxayK7Dz6xhZZC8BwA5ZZeh6z8hBiKZdm4kyQ86n6j5TKymGYt-SYs-TF1GZuaJVmRhYOF5I1jts0Q

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
✓ Inserted test record: v3lPsyX_odFY19uUOxrzHhPMXJBl9vzdh4sni-k9W69DhGGOAM51Sn-Qz_TxBE77dntbiB7WJIn37UT9Ec14Qg

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
✓ Inserted document with TTL: pUmZFoSP8grdCFvWnxjyiKzY3OfRkmaLvWZmqwT7izEZFjJaArzI7F2NN2S6prAdBlRA7mKxCSE9w6uBO-6u9A

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
✓ Created session: pdDITrnGlc2AA1r8vHiVWC4H4q8ee66esJLZiV_VwSS3wIb0pp09Ic4KLwP5LLUqvCGsxanygbvgG_kazNGnkQ

=== Sending Chat Message ===
Message ID: N_3G59vbqkLEZUmM0bOWXQyl5LzpuJHMRTbsI15Qit1JSSKUvfmecrp2CrF3T5-Kd2v67jP0nNHu1IoYTnv3yw

=== AI Response ===
Based on the information provided, the following products are available:

1. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

2. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

3. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    id: 'lpIjIjGRWXBmSSjxKsFng8KdLYexJ_Y0ModikJ_WwKZjNdRk4pmTsR11KO_uzxh6R6-hLBccZ1aXPnTu9U1ukA'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    price: 499,
    id: '9VW9rD1klIwzX7cnQPx-rxzbN5hXiw9f-UlHkZY4HK5BV7rpxUaNEoazOSCttI4EvfG0uZt_tASm2UhuNXYHIQ',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'aeE0pTP2QBcqp6wWtfiITyrakZsbdmqUck3F4W-7WrmP1nnpkRQaDKgzpb07MnpFdICPwrIj6rPGPOLzmfmNAQ',
    name: 'ekoDB Pro',
    price: 299,
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3035ms

=== Token Usage ===
Prompt tokens: 608
Completion tokens: 90
Total tokens: 698

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: UTay6yQdkYzsPM8WHQGqRzSlfngi3O3cgb0hnIy6CPgLxIw2ZhMlcRM203q4pMW_wiqqfCew5MqAGln-UrWqRw

=== Sending Messages ===
✓ Message 1 sent
  Response: The product available based on the provided information is "ekoDB", a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: nEzN0QjAyzE69XO3uYBiH0P79hfdMzlPgcDwVy1tbRCGqbAZgWjd-MRWeGcpm96u0DYKL13gEzaISyNbih1bJQ
  Parent: UTay6yQdkYzsPM8WHQGqRzSlfngi3O3cgb0hnIy6CPgLxIw2ZhMlcRM203q4pMW_wiqqfCew5MqAGln-UrWqRw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: nEzN0QjAyzE69XO3uYBiH0P79hfdMzlPgcDwVy1tbRCGqbAZgWjd-MRWeGcpm96u0DYKL13gEzaISyNbih1bJQ (Untitled)
  Session 2: UTay6yQdkYzsPM8WHQGqRzSlfngi3O3cgb0hnIy6CPgLxIw2ZhMlcRM203q4pMW_wiqqfCew5MqAGln-UrWqRw (Untitled)
  Session 3: pdDITrnGlc2AA1r8vHiVWC4H4q8ee66esJLZiV_VwSS3wIb0pp09Ic4KLwP5LLUqvCGsxanygbvgG_kazNGnkQ (Untitled)
  Session 4: ElVzQrLNy8abDUqTzInc62FK1DQNQ5oWxkOaO6_uS9oZlfd78IvPRsDC01kg-5fKbgtzqgyw2gruwFjn6PhEBg (Untitled)
  Session 5: eLr7gqxll0XkMo0fngGviRakNgisqU8DCS85DaBsZs8CS6vga5l2NFW3o8tiPi6Hqdv_I52Fb4aGQ7L50MET_A (Untitled)
  Session 6: SBvNH3R38sKAL9Bc6P62j4zXLJyCdfX2OuFUr3FbjkmewR9MVcxDSE7U2FSqLYy7ramHAlvT9fpWecCMl0PwJQ (Untitled)
  Session 7: GcI_-JS9SW66hdDUtOx8VFC7p1LwmVDKuARk01Ave2Iv5cQ4tA5LKv4tnVsbWwxppCW-Vc4RbAEteKlXSvJ3aQ (Untitled)
  Session 8: qZMycTvr9bUB7YzLX4ZcWbXjNc0DhwM1cTVcywNKzMxIiU7HUYO1RD5mktQvJD1w7qRXgiu87_BhTzLj1_HUKQ (Untitled)
  Session 9: zzMnYpnbOdEiHo60NW7v27gF22aW9C9vI17DJPbTOj_dVJ8W5PrS1iXVv3DLxXuW_gp_SHv8N7h3_9yZz7UGpg (Untitled)
  Session 10: ufrTyD4PrTBEQptwzw4r9KilyotTBDWqwmk4QImGnlZfBf6Qx-Xc8uVTAkyswCKMpDmpE7KDxy8Iez2OfoOILQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: nEzN0QjAyzE69XO3uYBiH0P79hfdMzlPgcDwVy1tbRCGqbAZgWjd-MRWeGcpm96u0DYKL13gEzaISyNbih1bJQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: K2_5noSZyktyNQRGr6b_MZwl31wPER2Y2tyizviMDLmUwzF4bNewTGra1_pn3tL53oej8kzObSGzXIwlEArwIA

=== Sending Initial Message ===
✓ Message sent
  Response: The available product in our database is the ekoDB. It's a high-performance database product. The price for this product is $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is 99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: lwsk8JG9Jgvz32hv-MNts3fZoTHIWFNy1Ba_Eo8yN89kIjrgxLNrSJsCMMPc-x530s9Gw4KximQIfz1mOWmrUA
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
  1. Score: 25.740, Matched: email.value, name.value, name, email
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio, title, bio.value
  2. Score: 26.400, Matched: title, bio.value, title.value, bio
  3. Score: 26.400, Matched: title.value, bio, bio.value, title
  4. Score: 26.400, Matched: title, title.value, bio.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title, title.value, bio
  2. Score: 39.600, Matched: title.value, bio.value, bio, title
  3. Score: 39.600, Matched: title.value, title, bio.value, bio
  4. Score: 39.600, Matched: bio, bio.value, title, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.769, Matched: 
  2. Score: 0.751, Matched: 
  3. Score: 0.737, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.707, Matched: content, content.value, title.value, title
  2. Score: 1.495, Matched: title, title.value, content, content.value
  3. Score: 0.300, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio.value, bio, skills.value, skills

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
🚀 ekoDB Scripts Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: XKPgbYfkU3cWsgLqekD_wocyjNs-SrEFp7k1R9KWTLRJ4wg9nxuh9Y62OF6dTidpJhl8WBNCRZaAe4bq2bYS9g
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: __mey_4527pndd8Ku0xr5t0sA7y8En5gXUNHsoTZwpOKN1ovUvYmTuCs70sbqbQ_684ZHg2U50GIijwQnQ9eBg
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 9vc5IbRA_DlrlC5yzkiDJQYY0jDZUReUd4d4YM3dZkC5dXbZcTAh4u4ZKWRt54iF3AEOlgANjeewA0nxrGMWDw
📊 Statistics: 2 groups
   {"avg_score":50,"status":"inactive","count":5}
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

✅ Script saved: 5oHYFHBAv9_TJyWpUxYA2D4RVvir1jpN1UW_6TQM3ezDdBGl2GQhhJA740GHFDZ8dI_yFD5MxK0J17FJko5IAA
📊 Found 2 product groups
   {"count":2,"category":"Furniture","avg_price":474}
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
   {"category":"Electronics","count":3}
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
Created Alice: $1000 - ID: aUGaDpOLLl93omzAOin4CVIgDZV5-TP1-AykbZ7jH0VLCxGkLhIUQYcJywgYV4O518dfCL_bEDbHTLtWLxl6RQ
Created Bob: $500 - ID: 7yYg_OVntPnSZy1tSPsV99ojVzYKJz9Ogbf6cmgjKI21X7nDa9atOJvw_hQxzt29f_HqNzd2bpukuNZg4cLA5Q

=== Example 1: Begin Transaction ===
Transaction ID: 26b6f681-c850-4c58-acb0-3162f3445e2c

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 56831a97-e9ed-4cea-ad8f-9f5ee4b091ac
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mTypeScript client examples complete![0m
✅ [32mAll TypeScript integration tests complete![0m
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 645ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
(node:73925) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
(node:73928) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Inserted: {
  id: 'bAL064FoR3d47JKFRbvRLoFEWXXpfi25NX2PlHJ1rTjnk9YiblRWiiQ0Ddsijj5KxShqq9_PrmZr9QPJz25PTA'
}

=== Find by ID ===
Found: {
  id: 'bAL064FoR3d47JKFRbvRLoFEWXXpfi25NX2PlHJ1rTjnk9YiblRWiiQ0Ddsijj5KxShqq9_PrmZr9QPJz25PTA',
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Test Record' },
  value: { type: 'Integer', value: 42 }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'bAL064FoR3d47JKFRbvRLoFEWXXpfi25NX2PlHJ1rTjnk9YiblRWiiQ0Ddsijj5KxShqq9_PrmZr9QPJz25PTA',
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 100 },
  name: { type: 'String', value: 'Updated Record' }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js ===[0m
✓ Authentication successful

=== Inserting Test Data ===
(node:73941) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted test record: oDJ_HExGK9xZwnyNk7gJKgs1uQMBorFqUq7ENsILVxbuLxInDFW3cAXmvYsOdlnsIOAuSPIfrbRoQwCtvn27Ag

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
        "id": "pgX3tN0CrrfdwZdfEP7IfhBBi2_OMTlOZKaFUVkClPD4NDbdz0ze2s_DBBIMQ5LTxd-lIMtq_vWFdt_XE62GnQ",
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
        "id": "hVJrJrqbHjCEisTnEM8A-FztQmjbfEL92UsvIUHFCyH4j45VgMzWEtQyv_b1grPXz9_tC-9hntazuuCZdw86tg",
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
        "id": "MC6awzEkQm2HxIFz8YHls2Ws7hmMZSn51NM_fz8xZSMVu3ZXvP8tNXq4OgMXARzOhP5i5qmyxItK6259PZlT4Q",
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
        "id": "oDJ_HExGK9xZwnyNk7gJKgs1uQMBorFqUq7ENsILVxbuLxInDFW3cAXmvYsOdlnsIOAuSPIfrbRoQwCtvn27Ag",
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
(node:73945) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
(node:73958) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {
  loginTime: '2026-01-03T05:27:52.841Z',
  userId: 123,
  username: 'john_doe'
}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { name: 'Product 1', price: 29.99 }
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
(node:73962) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Collection created with first record: IZvF87tZ9N5mtE_9FEASTChhKeEZS63zZxmBkE2P024n7cOzWtvB6AWAnml5mGK4Dxwkst70cAe7RCXjcURxvQ

=== List Collections ===
Total collections: 23
Sample collections: [
  'ws_ttl_test',
  'schema_users_client_go',
  'schema_employees_client_ts',
  'schema_employees_client_py',
  'test_accounts'
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
(node:73975) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document: Kpr9U0anU7tO9j_z9H_mHM--8V2lXTO69OVra35GjBhwKlhkQxRzn8r4GEj_zQPIjWOinlW3wKrUyK4WnsIVfA

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: created

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: Wacdyac1J4dthssRYCGRHqGp63zXAXszwn38m_4hqRSbaVldSPJY9mihEt6WBkTykyzXbu9deUwr2W3w0rrzEA

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
(node:73978) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document with TTL: KMVONVt8V2JVQp5_Fg3sGrAIenMjXaHmHTuj7fMIzcbzfSPR_tjDVVz99F7kTP4EQWLI6I8bEK3b9z66lqylOw

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

✅ Script saved: C80INp62a1v9l9f5N7luZXEayG0sGyWRj_jwLCzt69Vs4UFgNsxp-le-EBCznz-2MQxpPsF2XCyg4wummHjHPg
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: sS8i5FKFChznoRUBqwMkKXVsJAxduPmDYaJSWXPQ4l5T7qE6CfGP2hdkyOOvZyUlECeMblcnaT8EQrbF8HcHPw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: uDD2A7bx7SkeM-QU1v6196qWRFv7sRzr45sTlxxR-jLx10hhQbA5INxOitwFsgAz_BlXt48owgyl3f87NV1www
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":10,"status":"inactive","avg_score":50,"max_score":90}
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}

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
Created Alice: $1000 - ID: X5eGWh2KVDM4rvxJ1Sc1ThDCvhkHleiF15XlLIIXMCqSrrfS2Bj5tt4ieWFhs0QsaVjBJ7CdR0b7mHTAWAQ2gA
Created Bob: $500 - ID: uumF6o8r_DowftmezHPb6mq9A3m07_LBj-CJ1pdndinT6NobhH1jBuXUEgXAblQ6EOIPlr46qxdmRvhmR9R87g

=== Example 1: Begin Transaction ===
Transaction ID: 2c0a35fe-5071-41fe-8563-8f6f50fbd350

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
New transaction: c914606b-bac0-48e0-ab17-e6cfd73c9ad7
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
   ✅ Script saved: VTfI7EP5G5a5gvNelp_MdwtLiHnJcJXAul5QcczRJXnHFoPL8NzqIFW2F9im_LYGMlgab1V6KdF7LJNWtr_Ujg

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
   ✅ Script saved: 5QZ8ajoshV8euNjw3mHoSSjM2oqo2GM-ltuyo61c7K7x3K_rSx3x1Zlo5BVH8HHdtQtgYa9lqQ6i7oY7l4xeKg

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"value":"active","type":"String"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: ubVlSl69WTVIQt6TYfxZW8iY7QH7bicOF-3LbsYPNZXKFTC09Ay3IbFiLQCtCYtPas8T7FFtBqtjl70tASoIIA

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":0}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 1xWjnMPSNoRptFePLDMFxTVM2LABVPZnlTV8eWV9eRhoiFkGxqJSYArxL1WHK_wQCNfOnsnL2frttHx9dN3OmQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: VTfI7EP5G5a5gvNelp_M...
   ✅ Deleted script: 5QZ8ajoshV8euNjw3mHo...
   ✅ Deleted script: ubVlSl69WTVIQt6TYfxZ...
   ✅ Deleted script: 1xWjnMPSNoRptFePLDMF...
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

> @ekodb/ekodb-client@0.6.1 prepare
> npm run build


> @ekodb/ekodb-client@0.6.1 build
> tsc


up to date, audited 7 packages in 823ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.6.1 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 650ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'QaF-Hi-y0O7g4zJHoz8QqqAah0gXonnWMoGwVKPJo3cG0oc5N1EfBekv1uMamSIwehXHfzFS3GNPu-jGI8xUpg'
}

=== Find by ID ===
Found: {
  active: { type: 'Boolean', value: true },
  value: { type: 'Integer', value: 42 },
  name: { type: 'String', value: 'Test Record' },
  id: 'QaF-Hi-y0O7g4zJHoz8QqqAah0gXonnWMoGwVKPJo3cG0oc5N1EfBekv1uMamSIwehXHfzFS3GNPu-jGI8xUpg'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'QaF-Hi-y0O7g4zJHoz8QqqAah0gXonnWMoGwVKPJo3cG0oc5N1EfBekv1uMamSIwehXHfzFS3GNPu-jGI8xUpg',
  name: { type: 'String', value: 'Updated Record' },
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 100 }
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
cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
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
Collection created with first record: Q-wyi46JNKyhbwPKMHny7CITAC7p0uE4TbdH1tEC3Ez9YXMFbvWMFPVczBJ_DkKX2NjQmcaIlzHZ9dBCxL2Jgw

=== List Collections ===
Total collections: 22
Sample collections: ws_ttl_test,schema_users_client_go,schema_employees_client_ts,schema_employees_client_py,test_accounts

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: hlYayxwNSQB9laT4Sl4KX84F0sEC8RW_EN_nJtnqmaFFUW70oghq6YS4zKVhwjTuItHXARTK2ONfi9WAp64n8w

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 5LCFPraJ9wH5zqNyRldID-MVBAxAi1S0a1FYFOFWpANKKjr9Q6KYVSBlPRuTML9WMaH4thqHKAv7yJZGbwoEdA

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
✓ Inserted test record: qJN9XUhB-tRFxDa8atxjVxPSqo8_Cwa-p6Zw6nzAkPNaQwqznAXaHM8GSpVf4XedZ0Hbr-W-zKRWKnJ2h5Ze-w

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
✓ Inserted document with TTL: 0AJfLbMPQsFZ6quykgrr4pDwm_k-R8MWg9arY7d2KeInqkzrcr-HiSb3O2DjkIdmsCO4BRvdJDc3vgk5VV8OEg

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

✅ Script saved: xUFWGII_uuc1UDKTUgB9P_ORwGNo5nVR2xx4mHLIJbNXy0bnHkstzWtBYzJyyMk9wqu5z9iEkU1ec3ye2SKahg
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: iVD8_G3waUnERbKp8HQnyHHTfVdc7qLGGcJEM2CRujPgHIrkgNakyZv4-4rX7gtHoIH_0j59ZtloE6C7Tl4RAA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: MY1W5eSsBdHOcwLq5_Sc1BFgsBI3T-oUvMYPjjHW5F2Vp-UQa5Sc-rGoF-SNT1_cSGETfU7sIHc81Q-RzMMHMQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":5,"max_score":100,"status":"active","avg_score":60}
   {"avg_score":50,"count":5,"max_score":90,"status":"inactive"}

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

✅ Script saved: chf3WYGOReRJXwhe1gP9jaSKSYwfpgjitchBBcxcwrq9IfJKruyvFEZuzDpssOz0L8olDw007nVzKUP9XUc54w
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
   {"status":"inactive","avg_score":50,"count":10}
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
   1. Laptop Pro - $1299 (⭐4.8)
   2. USB-C Cable - $19 (⭐4.3)
   3. Wireless Mouse - $29 (⭐4.5)
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
   1. Introduction to Machine Learning (AI)
   2. Database Design Principles (Database)
   3. Natural Language Processing (AI)
   4. Vector Databases Explained (Database)
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
   1. Introduction to Machine Learning
   2. Database Design Principles
   3. Natural Language Processing
   4. Vector Databases Explained
   5. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Database Design Principles
   3. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Introduction to Machine Learning (AI)
   2. Database Design Principles (Database)
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
   1. Scalability: Vector databases can handle large amounts of data and scale as the data grows.
2. Precision: They allow for accurate computations as they store data in the form of vectors.
3. Speed: Vector databases often provide faster query response times.
4. Flexibility: They can handle different types of data including spatial, sequential, and time series data.
5. Efficient storage: They use less storage space compared to other database models. 
6. Simplified data operations: Operations like filtering, sorting, and aggregating become easier with vector databases.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Draft Article" - 1536D vector
   2. "Advanced Query Patterns" - 1536D vector
   3. "Getting Started with ekoDB" - 1536D vector
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
Created Alice: $1000 - ID: d7d5WPWnXOasnCZKG3dsSaX6aEamOSGHDU_Ax6clt8s9GJrh7XC7iawmnJhJgetaf46BsCBdZ9NR7nKuC2gC9Q
Created Bob: $500 - ID: axTplssmbD5UiyhJu1hVmL0td5Daw97U55YJ56ZLou01nZUqd21G6B6R0evNYUMbB3v70egsq32fjhv-H8NQvg

=== Example 1: Begin Transaction ===
Transaction ID: 29137f17-941a-4a31-8c9f-93bc98274908

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 94cde92a-1c6e-4e90-8271-2db4a8716b81
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
✓ Inserted user: Record(fields={id=StringValue(value=JjnscpJWu5GZF4Z45jrXMm3EL-q00ahrEwRIQOs6ZgxxXZrnJnW2s9Uzbn63VBC2Y2cGyGQqapnJrEudqwQZHw)})
  User ID: JjnscpJWu5GZF4Z45jrXMm3EL-q00ahrEwRIQOs6ZgxxXZrnJnW2s9Uzbn63VBC2Y2cGyGQqapnJrEudqwQZHw

=== Read ===
✓ Found user by ID: Record(fields={tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), created_at=ObjectValue(value={value=StringValue(value=Sat Jan 03 00:28:23 EST 2026), type=StringValue(value=String)}), id=StringValue(value=JjnscpJWu5GZF4Z45jrXMm3EL-q00ahrEwRIQOs6ZgxxXZrnJnW2s9Uzbn63VBC2Y2cGyGQqapnJrEudqwQZHw), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=28)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Sat Jan 03 00:28:23 EST 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: [tags, embedding, created_at, id, user_id, data, price, metadata, name, active, age, categories, email]

=== Update ===
✓ Updated user: Record(fields={user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), id=StringValue(value=JjnscpJWu5GZF4Z45jrXMm3EL-q00ahrEwRIQOs6ZgxxXZrnJnW2s9Uzbn63VBC2Y2cGyGQqapnJrEudqwQZHw), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Sat Jan 03 00:28:23 EST 2026)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})})})

=== Query ===
✓ Found 1 users matching query
  - Record(fields={user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), id=StringValue(value=JjnscpJWu5GZF4Z45jrXMm3EL-q00ahrEwRIQOs6ZgxxXZrnJnW2s9Uzbn63VBC2Y2cGyGQqapnJrEudqwQZHw), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), created_at=ObjectValue(value={value=StringValue(value=Sat Jan 03 00:28:23 EST 2026), type=StringValue(value=String)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})})})

=== Delete ===
✓ Deleted user with ID: JjnscpJWu5GZF4Z45jrXMm3EL-q00ahrEwRIQOs6ZgxxXZrnJnW2s9Uzbn63VBC2Y2cGyGQqapnJrEudqwQZHw

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
  IDs: b6SYK5C04udTApjnsCoyh6khGH2h3TBZHBiO1lnNyltFjv-fshBd3cgoIU1VMMniOGtrSRZZKOEm2vn4bQ88hw, YsgmcGyfZGYwwbRRS9k0Drx_ofkFsqMWQV-SICB-_C1MxJgqSbG6AMuiyCK2nw39NX5zd7kAT34UoVVNNHbFkQ, vxWDlKGtbDo-VXwgvYSWtO3QhnjUOBNvlfJPjy5KrZlDEf8BHQZda8JYo6P8b0Zx445aMs_LGxbimmn3pDwR2A...

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
  - Score: ObjectValue(value={value=IntegerValue(value=95), type=StringValue(value=Integer)})
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=92)})
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=88)})

=== Query 4: Complex (score > 80 AND age >= 25) ===
✓ Found 4 high-scoring adults

=== Query 5: IN (city IN ['NYC', 'LA']) ===
✓ Found 4 records in NYC or LA

=== Query 6: Pagination (skip 2, limit 2) ===
✓ Page 2 (2 records):
  - ObjectValue(value={value=StringValue(value=Charlie), type=StringValue(value=String)})
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
=== ekoDB Kotlin Client - KV Operations Example ===

=== KV Set ===
✓ Set key: user:123

=== KV Get ===
✓ Retrieved value: {"email":"alice@example.com","name":"Alice","role":"admin"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"user_id":"123","created_at":1767418116241}
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
  - ws_ttl_test
  - schema_users_client_go
  - schema_employees_client_ts
  - schema_employees_client_py
  - test_accounts

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-01-03T05:28:40.500005Z","last_modified":"2026-01-03T05:28:40.500005Z","bypass_ripple":false,"primary_key_alias":"id"}

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
  Document ID: WUiOKYHN-dnDHuRMZYScYqOKYLIxMS2qPVdJJmNXGk2vZV4n2SgQgntDNzcZCx4AyLJQI-Eos-6g022-8rb3Zw

=== Verify Document Exists ===
✓ Document found: [session_id, id, created_at, user_id, ttl]

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: apXW3HF1A4zZbvkhAsh47TTrmk0b_QHlCuNNcdWfN_4BPt7D_eE6fF_ADIBMteGknb_t1kLQuHzovnPIzSK6BA

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: [cache_key, value, id, ttl]

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
  {"payload":{"data":[{"id":"MEf5uRpmC07SN3JANElIV7B3OXzfCXd0SkXpdQ6h-rUdHkNieCvYCLyGdTYT-ZfE14cbeThIlLblQEo7OEMsiA","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

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
✓ Inserted user 1: StringValue(value=ZAQAdpmJGnCwLyaE9j2Tq9PBvtfkvaHSyDI2AthdiYzKRN8sIKS6SW8fmIgBXDHh_HonhMeJt1HAc9ztbSTluA)
✓ Inserted user 2: StringValue(value=jb_D-HzgTpKoeYPvJc5IeUfxGotE7KuzwyEZEGbFjqjErUzyvFxXftpUJ6kEdIfEA5xBO1k-l-hhzABD-I-qiw)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 22
  Sample: [ws_ttl_test, schema_users_client_go, schema_employees_client_ts, kotlin_schema_example, schema_employees_client_py]

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
  {"results":[{"record":{"description":"Learn Rust programming language with hands-on examples and best practices.","views":543,"tags":"programming,rust,tutorial","title":"Rust Programming","id":"xY6_FZr2w6ITxobIqGeDmTeWrQIqvi68RZ-jtAGmUSbDifMV7mT_bY9K1dGibB9JfD0j6lhYnmk2X_SHJfLJIQ"},"score":19.8,"matched_fields":["description","title","tags"]},{"record":{"tags":"programming,python,data-science","title":"Python for Data Science","id":"iDhbUDxMYRpH5juV4cgNRQGMs0pT03JvkwZnxT67sVlLSMPvEbcHHerLtcwDj5Qj2vkmDygyoFUfbHcjj4V7PA","description":"Master Python for data analysis, machine learning, and visualization.","views":681},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"tags":"programming,javascript,web","views":142,"id":"3r3o978BlEEVyOsRfHCOnkZTeE0a98R8PTRxHqTvbWnHigliXzWXSCfzMO3dXt1vL6cDkJ3SGuZOLRGuDpDjwQ","description":"Build modern web applications using JavaScript, React, and Node.js.","title":"JavaScript Web Development"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"views":992,"id":"vgCdxrNSDq7Sy8jrPowuMQo7gfl_lT3ksHfUiYjLpOSgX2B1n370eyxmw40aWzjSI3Dp-bevvqSM3M7LCPV7sw","title":"Machine Learning Basics","tags":"ai,machine-learning,python","description":"Introduction to machine learning algorithms and neural networks."},"score":2.7,"matched_fields":["title","description","tags"]},{"record":{"description":"Master Python for data analysis, machine learning, and visualization.","tags":"programming,python,data-science","id":"iDhbUDxMYRpH5juV4cgNRQGMs0pT03JvkwZnxT67sVlLSMPvEbcHHerLtcwDj5Qj2vkmDygyoFUfbHcjj4V7PA","title":"Python for Data Science","views":681},"score":1.0,"matched_fields":["description"]},{"record":{"title":"Rust Programming","description":"Learn Rust programming language with hands-on examples and best practices.","id":"xY6_FZr2w6ITxobIqGeDmTeWrQIqvi68RZ-jtAGmUSbDifMV7mT_bY9K1dGibB9JfD0j6lhYnmk2X_SHJfLJIQ","views":543,"tags":"programming,rust,tutorial"},"score":0.5,"matched_fields":["description"]},{"record":{"title":"Database Design","views":802,"id":"0arhInQGWsIvW9kZdBUpSr09tHfQAAltay_yG7oK6Ip7arP7ijRj66h4hcxJIL4epxYgfXCicVuPu6Zwuo689w","description":"Learn database design principles, normalization, and query optimization.","tags":"database,design,sql"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
✓ Inserted document with TTL: StringValue(value=OiVR2zwIOp1ZlsdcBE2H1BMA0EMFjnz4j-W4PrNf16c3aYVGtjkgBj_HZamYfLKnywLdpgr32Aw_yzpIcFxYUQ)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1767418141586},"id":"OiVR2zwIOp1ZlsdcBE2H1BMA0EMFjnz4j-W4PrNf16c3aYVGtjkgBj_HZamYfLKnywLdpgr32Aw_yzpIcFxYUQ","name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-01-03T06:29:00.713592Z","value":{"type":"Integer","value":42}}]},"type":"Success"}

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
✓ Created session: Xd9___vUsubdd_QfvLlBMx03GEYHCalOikdRF86kSS9_q4QH9edUlhAoN3hKMBsBpzYz_cXhZFuBB-MQQBxFKw

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "KXYlqTHyEpFVmKQhSZVJqqEXkX8Z2pj5sPuD7fchL2hWi41FLtvo-H_W62PJOyPRPJTDdYvkFYrIOEV2y3scsA"
  Responses: ["ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration. Some of its key features include an AI Chat Integration, which allows you to query your database using natural language and get AI-powered responses with relevant context. It also supports full-text search, vector search, and hybrid search with automatic context retrieval."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

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
=== ekoDB Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: TUgGbV1kA-M3p1MLSyYVe7vh9jeRpv27iE9BrCWr1D2za66NVPtxUTzMxKPbBiDXvoJAcZ8QDLDGFIVKJKCkkQ

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["Answer: The available product is ekoDB, a high-performance database product with AI capabilities. It is priced at $99."]

✓ Message 2 sent
  Responses: ["The price of the ekoDB product is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"TUgGbV1kA-M3p1MLSyYVe7vh9jeRpv27iE9BrCWr1D2za66NVPtxUTzMxKPbBiDXvoJAcZ8QDLDGFIVKJKCkkQ"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"aXh58MMQCgrC_q3sqml9HXhwBctMSee_lXT9yLoSCPOG2l_Xw-bECgpbCoA7Wc74QSYinKxjg0b8Ql4qaxTVIg","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-03T05:29:17.763486Z"},"id":"vL5_lRW8T7RONB9JgkM6IaT5OzMxPVMgqCwzmC8k0SUiDZH2eZvo3OvOFFPz6-hql3UZ6rdl7TRPdy4CD0Lb2g","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":26,"prompt_tokens":237,"total_tokens":263}},"updated_at":{"type":"DateTime","value":"2026-01-03T05:29:17.763486Z"}},{"chat_id":{"type":"String","value":"TUgGbV1kA-M3p1MLSyYVe7vh9jeRpv27iE9BrCWr1D2za66NVPtxUTzMxKPbBiDXvoJAcZ8QDLDGFIVKJKCkkQ"},"content":{"type":"String","value":"Answer: The available product is ekoDB, a high-performance database product with AI capabilities. It is priced at $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"aXh58MMQCgrC_q3sqml9HXhwBctMSee_lXT9yLoSCPOG2l_Xw-bECgpbCoA7Wc74QSYinKxjg0b8Ql4qaxTVIg","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-03T05:29:17.766999Z"},"id":"jcwHMUMyopJyxQ2NkDBkRUdxOY5YfP_ycBtE3XaZbvTQqy47J20_Q4gDCndSXw59hKtzAJkCkmCgn-nUylmUtw","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":26,"prompt_tokens":237,"total_tokens":263}},"updated_at":{"type":"DateTime","value":"2026-01-03T05:29:17.766999Z"}},{"chat_id":{"type":"String","value":"TUgGbV1kA-M3p1MLSyYVe7vh9jeRpv27iE9BrCWr1D2za66NVPtxUTzMxKPbBiDXvoJAcZ8QDLDGFIVKJKCkkQ"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"aXh58MMQCgrC_q3sqml9HXhwBctMSee_lXT9yLoSCPOG2l_Xw-bECgpbCoA7Wc74QSYinKxjg0b8Ql4qaxTVIg","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-03T05:29:18.868180Z"},"id":"WQkSvoO1y1CaRFvb9jfwsHzWW7xjxheBgTTZLt5gtlALu6yVMyjYMhZ_5zrX1OMocO3HvQXFcO0iVW9dUyvcKw","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":237,"total_tokens":249}},"updated_at":{"type":"DateTime","value":"2026-01-03T05:29:18.868180Z"}},{"chat_id":{"type":"String","value":"TUgGbV1kA-M3p1MLSyYVe7vh9jeRpv27iE9BrCWr1D2za66NVPtxUTzMxKPbBiDXvoJAcZ8QDLDGFIVKJKCkkQ"},"content":{"type":"String","value":"The price of the ekoDB product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"aXh58MMQCgrC_q3sqml9HXhwBctMSee_lXT9yLoSCPOG2l_Xw-bECgpbCoA7Wc74QSYinKxjg0b8Ql4qaxTVIg","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-03T05:29:18.872797Z"},"id":"LB1-Nm_558hvY-hnT_WJeKUJ5dkCtdDx6R1Sex-IEaMOi1v4D_YQI-4qTZYr7-31O7DSgQqfeYk8llCSAVvLUQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":237,"total_tokens":249}},"updated_at":{"type":"DateTime","value":"2026-01-03T05:29:18.872797Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 13

=== Branching Session ===
✓ Created branched session: d9QCXKN0gB7v-wG5EDovGXHZP27NBlTV2NihAcliUhBKbdffHN80mtGfBI78spzOt_ZsbCyDlVtSuu_5keuCSg

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
✓ Created session: 2G1_BNUU138gqYYZkXcvcyi2WaG33vAPghPZDbdN_nzN13T-lnIzHeB4IvyXlW5hsFpoFyURB-1SO3iyHfD_Zw

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Answer: One available product is \"ekoDB\", a high-performance database product. The price for this product is $99."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["The product available is a high-performance database product named ekoDB. It is priced at $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: hd76KCcs6Qo2CazCRJxDFbA7YUjtaqwFAb3Y-tamuorN8T9_a0dwUVNXGm6evtuD1idis2PApk-v9bGXdI4Rjg

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

✅ Script saved: qRBuuid3SGqG-Y9_pIQrtjDb_9V-EIrde3K1LHZC5kTNwzcFV-69SkAugAIMQohGP_VHIjoCbWj4KO9nJ2bsAQ
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"avg_score":60.0,"status":"active","count":5}
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
Created Alice: $1000 - ID: -meHDxzKbkMOedAPD1tDAX2nRiJgtFPcozpnvwkQ-J8aZKpYouLJUNW69piGb6S0LvleW4sZWEBDdkT7oTT5ow
Created Bob: $500 - ID: aexpzzSwtlfJWiKYrI0M9AMOfALorVxEuVnCGU4vw3mxaDmRikJNyFaAlu5IK41I118mrQKm37BBDSy0KlrVog

=== Example 1: Begin Transaction ===
Transaction ID: 3fc5832f-630d-4015-9849-5e56a37b8495

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 87b8f1f7-f6b0-4a7f-be1a-dc126103d77c
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
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.6.1-cp38-abi3-macosx_11_0_arm64.whl
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
    ✓ Generated embedding: 1536 dimensions in 305.396958ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 213.782458ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 375.067875ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 600.711875ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 501.972834ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 672.499708ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 498.2365ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 229.334917ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 246.655667ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 559.450167ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 189.400792ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 615.34275ms
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
    ✓ Generated embedding: 1536 dimensions in 208.471709ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 102.736417ms

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

Writing memory-safe high-performance database code involves various practices and considerations. Here are some tips:

1. **Use Prepared Statements**: Prepared statements can help prevent SQL injection attacks and improve performance by allowing the database to efficiently reuse the queries. They are available in both SQL and certain NoSQL databases.

2. **Proper Indexing**: Indexing is a method used to speed up the retrieval of records. However, it's important to only create indexes on columns that will be queried frequently, as unnecessary indexing can slow down insert, update, and delete operations.

3. **Database Normalization**: As mentioned in context 2 and 3, database normalization can help reduce redundancy and improve data integrity. This can result in improved performance and memory usage, as less space is used to store redundant data.

4. **Appropriate Use of SQL and NoSQL**: Depending on your needs, you may want to use SQL or NoSQL (as discussed in context 1 and 4). For example, if you need high write throughput or are working with unstructured data, NoSQL may be more appropriate.

5. **Optimize Queries**: As mentioned in context 5, optimizing your queries can greatly improve performance. This might involve using joins effectively, avoiding select *, limiting the amount of data returned, and more.

6. **Memory Management**: In your programming language of choice, make sure to properly manage memory. This includes effectively using data structures, avoiding memory leaks, and correctly handling object lifecycles.

7. **Connection Pooling**: Repeatedly opening and closing database connections can be costly. Connection pooling allows you to reuse existing connections, improving performance.

8. **Caching**: Implement caching strategies to store frequently accessed data in memory. This can significantly reduce database calls and improve performance.

Remember, the best practices can vary depending on the specific database system (MySQL, PostgreSQL, MongoDB, etc.), the programming language, and the specific use case. Always consider these factors when writing your database code.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 446.478792ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2121 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 288.562334ms
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
  ✓ Text search completed in 61.2235ms

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_new_question: Writing memory-safe high-performance database code involves various practices and considerations. Here are some tips:

1. **Use Prepared Statements**: Prepared statements can help prevent SQL injection attacks and improve performance by allowing the database to efficiently reuse the queries. They are available in both SQL and certain NoSQL databases.

2. **Proper Indexing**: Indexing is a method used to speed up the retrieval of records. However, it's important to only create indexes on columns that will be queried frequently, as unnecessary indexing can slow down insert, update, and delete operations.

3. **Database Normalization**: As mentioned in context 2 and 3, database normalization can help reduce redundancy and improve data integrity. This can result in improved performance and memory usage, as less space is used to store redundant data.

4. **Appropriate Use of SQL and NoSQL**: Depending on your needs, you may want to use SQL or NoSQL (as discussed in context 1 and 4). For example, if you need high write throughput or are working with unstructured data, NoSQL may be more appropriate.

5. **Optimize Queries**: As mentioned in context 5, optimizing your queries can greatly improve performance. This might involve using joins effectively, avoiding select *, limiting the amount of data returned, and more.

6. **Memory Management**: In your programming language of choice, make sure to properly manage memory. This includes effectively using data structures, avoiding memory leaks, and correctly handling object lifecycles.

7. **Connection Pooling**: Repeatedly opening and closing database connections can be costly. Connection pooling allows you to reuse existing connections, improving performance.

8. **Caching**: Implement caching strategies to store frequently accessed data in memory. This can significantly reduce database calls and improve performance.

Remember, the best practices can vary depending on the specific database system (MySQL, PostgreSQL, MongoDB, etc.), the programming language, and the specific use case. Always consider these factors when writing your database code.

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
    ✓ Generated embedding: 1536 dimensions in 0.282s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.207s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.234s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.188s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.285s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.248s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.252s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.300s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.235s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.293s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.259s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.221s
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
    ✓ Generated embedding: 1536 dimensions in 0.240s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.085s

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
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code often involves a combination of good practices in database design, query optimization, and efficient code. Here's what you can do:

1. Database Design: Start with a well-designed database. Use database normalization techniques to organize your data and reduce redundancy, as this can greatly improve the performance of database operations. This involves dividing large tables into smaller ones and defining relationships between them using foreign keys, as mentioned in Context 1.

2. Choose the Right Database: Depending on your use case, select either SQL or NoSQL. As stated in Context 2, SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships, while NoSQL is more suitable for flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data.

3. Query Optimization: Efficient queries can significantly improve the performance of your database. You can optimize your queries by:
   - Using Indexes: Indexes can enhance query performance by reducing the amount of data that needs to be scanned for each query.
   - Limiting Result Set: Only fetch the data that you need. Using limits and offsets can help in fetching only a certain number of rows.
   - Using Joins Appropriately: Joins can be expensive in terms of performance. Make sure you're using them only when necessary and in the most efficient manner.
   - Avoiding N+1 Queries: N+1 queries can be a common performance problem where the system makes a separate query for each record in a result set.

4. Memory Management in Code: Be aware of how your programming language handles memory management. Use proper data structures, avoid unnecessary object creation, and deallocate resources that are no longer in use.

5. Use a Profiler: Profiling your code can help identify bottlenecks and inefficient parts of your code.

6. Database Connection Management: Efficiently manage your database connections. Use connection pooling and close connections when they're no longer needed.

Remember, writing memory-safe, high-performance code is not just about using the right tools and techniques, but also about understanding the underlying principles and trade-offs.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.296s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2258 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.279s
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
    ✓ Generated embedding: 1536 dimensions in 0.277s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.325s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.289s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.306s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.330s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.216s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.218s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.217s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.404s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.297s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.282s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.275s
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
    ✓ Generated embedding: 1536 dimensions in 0.278s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.083s
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
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves several best practices. Here are some tips:

1. **Normalization**: As discussed in Context 1, normalization is crucial for data integrity and minimizing redundancy, which can help improve the performance of your database.

2. **Choosing the right database management system**: As mentioned in Context 2, depending on your specific use case, you may want to choose between SQL and NoSQL. SQL is better suited for complex queries and structured data with well-defined relationships, which can help improve performance and safety. NoSQL, on the other hand, is better for flexible schemas, horizontal scaling, and high write throughput.

3. **Optimize your queries**: You asked in Context 5 about optimizing database queries. Making sure your queries are efficient is key to high-performance, memory-safe code. Here are a few ways to optimize your queries:
    - **Indexing**: Create indexes on tables to speed up data retrieval. This can significantly reduce the amount of data that needs to be processed.
    - **Avoiding full table scans**: Where possible, avoid SELECT * commands, and instead specify the exact fields you need.
    - **Using joins wisely**: Avoid unnecessary joins as they can lead to performance problems.
    - **Batching**: If you're writing multiple rows, consider batching them together in a single query.

4. **Memory management**: Be mindful of how your code uses memory. Avoid loading large amounts of data into memory all at once. If you're using an ORM, consider disabling automatic tracking to save memory.

5. **Monitoring and Profiling**: Regularly monitor your database performance and use profiling tools to identify and fix any slow operations. 

6. **Concurrency**: Use concurrency control mechanisms to manage simultaneous data operations and avoid conflicts, which can lead to better performance and memory safety.

Remember, the specific approaches might change based on whether you are using SQL or NoSQL databases, so always make sure to use best practices appropriate for your chosen database system.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.272s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2101 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.244s
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
  ✓ Text search completed in 0.051s
✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_new_question: Writing memory-safe, high-performance database code involves several best practices. Here are some tips:

1. **Normalization**: As discussed in Context 1, normalization is crucial for data integrity and minimizing redundancy, which can help improve the performance of your database.

2. **Choosing the right database management system**: As mentioned in Context 2, depending on your specific use case, you may want to choose between SQL and NoSQL. SQL is better suited for complex queries and structured data with well-defined relationships, which can help improve performance and safety. NoSQL, on the other hand, is better for flexible schemas, horizontal scaling, and high write throughput.

3. **Optimize your queries**: You asked in Context 5 about optimizing database queries. Making sure your queries are efficient is key to high-performance, memory-safe code. Here are a few ways to optimize your queries:
    - **Indexing**: Create indexes on tables to speed up data retrieval. This can significantly reduce the amount of data that needs to be processed.
    - **Avoiding full table scans**: Where possible, avoid SELECT * commands, and instead specify the exact fields you need.
    - **Using joins wisely**: Avoid unnecessary joins as they can lead to performance problems.
    - **Batching**: If you're writing multiple rows, consider batching them together in a single query.

4. **Memory management**: Be mindful of how your code uses memory. Avoid loading large amounts of data into memory all at once. If you're using an ORM, consider disabling automatic tracking to save memory.

5. **Monitoring and Profiling**: Regularly monitor your database performance and use profiling tools to identify and fix any slow operations. 

6. **Concurrency**: Use concurrency control mechanisms to manage simultaneous data operations and avoid conflicts, which can lead to better performance and memory safety.

Remember, the specific approaches might change based on whether you are using SQL or NoSQL databases, so always make sure to use best practices appropriate for your chosen database system.

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
    ✓ Generated embedding: 1536 dimensions in 0.196s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.241s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.316s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.277s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.240s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.702s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.397s
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
    ✓ Generated embedding: 1536 dimensions in 0.281s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.220s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.272s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.242s
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
    ✓ Generated embedding: 1536 dimensions in 0.280s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.083s

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
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves several practices:

1. Use Prepared Statements: Prepared statements or parameterized queries help prevent SQL injection attacks, which can leak memory or compromise your database. They also improve performance as they can be compiled once and executed multiple times.

2. Normalization: As mentioned in Context 1, database normalization can help improve data integrity and reduce redundancy. This can lead to more efficient use of memory and better performance.

3. Indexing: Proper indexing can greatly improve query performance. However, remember that indexes take up memory, so it's crucial to find the right balance between the number of indexes and query speed.

4. Optimize Queries: As per Context 5, optimizing database queries is key to high performance. Avoid using SELECT *, reduce the use of joins where possible, and use LIMIT to prevent returning more data than necessary.

5. Use Appropriate Database Type: Depending on your data and use case, NoSQL may be more appropriate than SQL or vice versa. As per Context 2, NoSQL is good for flexible schemas, horizontal scaling, high write throughput, and unstructured data. SQL is better for complex queries, ACID transactions, and structured data.

6. Manage Connections: Reuse database connections instead of opening and closing them for each query. Connection pools can help manage multiple connections efficiently.

7. Monitor and Tune: Regularly monitor your database performance and adjust parameters, indexes, and queries as necessary. Tools like SQL Profiler for SQL Server can help find bottlenecks.

8. Use Transactions: Transactions can help ensure data integrity and prevent memory leaks in case of failures.

9. Limit In-Memory Data: If you're working with large data sets, fetch and process data in chunks rather than loading it all into memory at once.

10. Error Handling: Proper error handling can help prevent memory leaks and ensure your application recovers gracefully from unexpected situations.

Remember, writing high-performance, memory-safe code often involves trade-offs, so it's important to understand your application's requirements and constraints.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.292s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2197 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.228s
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
  ✓ Text search completed in 0.052s

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
    ✓ Generated embedding: 1536 dimensions in 0.596s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.285s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.214s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.215s
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
    ✓ Generated embedding: 1536 dimensions in 0.26s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.204s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.212s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.487s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.302s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.236s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.292s
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
    ✓ Generated embedding: 1536 dimensions in 0.221s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.089s

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
✓ Context prepared from search results
✓ AI would use this context to generate comprehensive response

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.249s
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
  ✓ Text search completed in 0.046s

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

> @ekodb/ekodb-client@0.6.1 prepare
> npm run build


> @ekodb/ekodb-client@0.6.1 build
> tsc


up to date, audited 7 packages in 860ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.6.1 build
> tsc

✅ [32mTypeScript client built![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning TypeScript SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (UIDyw75C8eNg0pSRz4JQ0mT_PNeLMMtjdyiAcBB7hW76x1oRZGA2Q99DacdmnjCKPYik5IEqDVJKlIwXgAlhMA)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [
    {
      "id": "9qo2Kv20kNBODGHcKrpEG_QrB73BnEFcuELvA17o_1fOVkRdCO7XCPrPpbR2x7qGLc1UiN1WEhyg1d4eJWd2Sw",
      "cached_at": "2026-01-03T05:30:47.704Z",
      "data": "{\"login\":\"torvalds\",\"id\":1024025,\"node_id\":\"MDQ6VXNlcjEwMjQwMjU=\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/1024025?v=4\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/torvalds\",\"html_url\":\"https://github.com/torvalds\",\"followers_url\":\"https://api.github.com/users/torvalds/followers\",\"following_url\":\"https://api.github.com/users/torvalds/following{/other_user}\",\"gists_url\":\"https://api.github.com/users/torvalds/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/torvalds/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/torvalds/subscriptions\",\"organizations_url\":\"https://api.github.com/users/torvalds/orgs\",\"repos_url\":\"https://api.github.com/users/torvalds/repos\",\"events_url\":\"https://api.github.com/users/torvalds/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/torvalds/received_events\",\"type\":\"User\",\"user_view_type\":\"public\",\"site_admin\":false,\"name\":\"Linus Torvalds\",\"company\":\"Linux Foundation\",\"blog\":\"\",\"location\":\"Portland, OR\",\"email\":null,\"hireable\":null,\"bio\":null,\"twitter_username\":null,\"public_repos\":9,\"public_gists\":1,\"followers\":269522,\"following\":0,\"created_at\":\"2011-09-03T15:26:22Z\",\"updated_at\":\"2025-11-24T04:16:14Z\"}"
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
Response time: 71ms (served from cache)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (4vcIm6gq13ljUb6Lbr0b6wOXDZHU-17_oOfZKUur7TMO-AmfNAWrztYjmGVejHxSwwOWGAGaxr0sjpzOulZWVw)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "enriched_at": "2026-01-03T05:30:47.957Z",
      "id": "TEE81IkmxuhKs1cl35-vleASkqknJPSRIi5SLejiva852MszojNgKFi9c6E0wVKNWACQsZhLyIheaSMui-5r_w",
      "enriched_data": "{\"id\":1,\"title\":\"Essence Mascara Lash Princess\",\"description\":\"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.\",\"category\":\"beauty\",\"price\":9.99,\"discountPercentage\":10.48,\"rating\":2.56,\"stock\":99,\"tags\":[\"beauty\",\"mascara\"],\"brand\":\"Essence\",\"sku\":\"BEA-ESS-ESS-001\",\"weight\":4,\"dimensions\":{\"width\":15.14,\"height\":13.08,\"depth\":22.99},\"warrantyInformation\":\"1 week warranty\",\"shippingInformation\":\"Ships in 3-5 business days\",\"availabilityStatus\":\"In Stock\",\"reviews\":[{\"rating\":3,\"comment\":\"Would not recommend!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"},{\"rating\":4,\"comment\":\"Very satisfied!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Lucas Gordon\",\"reviewerEmail\":\"lucas.gordon@x.dummyjson.com\"},{\"rating\":5,\"comment\":\"Highly impressed!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"}],\"returnPolicy\":\"No return policy\",\"minimumOrderQuantity\":48,\"meta\":{\"createdAt\":\"2025-04-30T09:41:02.053Z\",\"updatedAt\":\"2025-04-30T09:41:02.053Z\",\"barcode\":\"5784719087687\",\"qrCode\":\"https://cdn.dummyjson.com/public/qr-code.png\"},\"images\":[\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp\"],\"thumbnail\":\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp\"}"
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
✓ Edge cache script created: vmrHltvKFTfdDCZGMgvp2hjrsOHaeZk5NKdG1vjPhEiU4wefMRldWXuha18WLBXpQM8DF8haSiD4mEztK5xsYA

Call 1: Cache miss (fetches from API)
Response time: 552ms
Result: {
  "records": [
    {
      "id": "DyccNUHKLtoHAkeHywuvhmz1ph8S71jllJJs9v6IvLf74yeOb5u9lLSyDnU4GPKWxp-tjQe2x-R_K4gDghpc3g",
      "cached_at": "2026-01-03T05:30:48.479Z",
      "data": "{\"latitude\":40.710335,\"longitude\":-73.99309,\"generationtime_ms\":0.021457672119140625,\"utc_offset_seconds\":0,\"timezone\":\"GMT\",\"timezone_abbreviation\":\"GMT\",\"elevation\":32.0,\"current_units\":{\"time\":\"iso8601\",\"interval\":\"seconds\",\"temperature_2m\":\"°C\"},\"current\":{\"time\":\"2026-01-03T05:30\",\"interval\":900,\"temperature_2m\":-3.8}}"
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
Response time: 527ms (1x faster!)
Result: {
  "records": [
    {
      "id": "fkuYvqCmZSBvf00i2BhYYsTrR1feZz9PyufhS5vE2DxC_NbLXTmEzK1-_niq3rMHl7W4PR2Mlz_3cI-0ehGRPg",
      "cached_at": "2026-01-03T05:30:48.479Z",
      "data": "{\"latitude\":40.710335,\"longitude\":-73.99309,\"generationtime_ms\":0.020384788513183594,\"utc_offset_seconds\":0,\"timezone\":\"GMT\",\"timezone_abbreviation\":\"GMT\",\"elevation\":32.0,\"current_units\":{\"time\":\"iso8601\",\"interval\":\"seconds\",\"temperature_2m\":\"°C\"},\"current\":{\"time\":\"2026-01-03T05:30\",\"interval\":900,\"temperature_2m\":-3.8}}"
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
    Finished `release` profile [optimized] target(s) in 0.11s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.6.1-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.6.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
Successfully installed ekodb-client-0.6.1
✅ [32mPython client package built and installed![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (2F552zK1g_rS8u1zXLSNexOMhdvMfpv_ixIs679qP8jEh-mrPuimP6AgnM02LTi8ruLAVFj3SP9A9x4KJOzlPQ)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 74ms
Result: [
  {
    "cached_at": "1767418251",
    "data": "{\"login\":\"torvalds\",\"id\":1024025,\"node_id\":\"MDQ6VXNlcjEwMjQwMjU=\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/1024025?v=4\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/torvalds\",\"html_url\":\"https://github.com/torvalds\",\"followers_url\":\"https://api.github.com/users/torvalds/followers\",\"following_url\":\"https://api.github.com/users/torvalds/following{/other_user}\",\"gists_url\":\"https://api.github.com/users/torvalds/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/torvalds/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/torvalds/subscriptions\",\"organizations_url\":\"https://api.github.com/users/torvalds/orgs\",\"repos_url\":\"https://api.github.com/users/torvalds/repos\",\"events_url\":\"https://api.github.com/users/torvalds/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/torvalds/received_events\",\"type\":\"User\",\"user_view_type\":\"public\",\"site_admin\":false,\"name\":\"Linus Torvalds\",\"company\":\"Linux Foundation\",\"blog\":\"\",\"location\":\"Portland, OR\",\"email\":null,\"hireable\":null,\"bio\":null,\"twitter_username\":null,\"public_repos\":9,\"public_gists\":1,\"followers\":269522,\"following\":0,\"created_at\":\"2011-09-03T15:26:22Z\",\"updated_at\":\"2025-11-24T04:16:14Z\"}",
    "id": "GpEDzSH-igpnLItCA6tPN6yKfdlvbcBnOo_ru2JlWPwDpOrzoxK4PaJLXre-t01shjAcXOihHSysBA6Ejginhw"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 78ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (HoWdfM3J47ZXx5EU7oTennvY-qAbI4W4kMu1ZydEw4gjSglz69kpd4bHXxSZYKwRuCePO-9-TVFeLpQ00749KQ)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "1767418251",
    "enriched_data": "{\"id\":1,\"title\":\"Essence Mascara Lash Princess\",\"description\":\"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.\",\"category\":\"beauty\",\"price\":9.99,\"discountPercentage\":10.48,\"rating\":2.56,\"stock\":99,\"tags\":[\"beauty\",\"mascara\"],\"brand\":\"Essence\",\"sku\":\"BEA-ESS-ESS-001\",\"weight\":4,\"dimensions\":{\"width\":15.14,\"height\":13.08,\"depth\":22.99},\"warrantyInformation\":\"1 week warranty\",\"shippingInformation\":\"Ships in 3-5 business days\",\"availabilityStatus\":\"In Stock\",\"reviews\":[{\"rating\":3,\"comment\":\"Would not recommend!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"},{\"rating\":4,\"comment\":\"Very satisfied!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Lucas Gordon\",\"reviewerEmail\":\"lucas.gordon@x.dummyjson.com\"},{\"rating\":5,\"comment\":\"Highly impressed!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"}],\"returnPolicy\":\"No return policy\",\"minimumOrderQuantity\":48,\"meta\":{\"createdAt\":\"2025-04-30T09:41:02.053Z\",\"updatedAt\":\"2025-04-30T09:41:02.053Z\",\"barcode\":\"5784719087687\",\"qrCode\":\"https://cdn.dummyjson.com/public/qr-code.png\"},\"images\":[\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp\"],\"thumbnail\":\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp\"}",
    "id": "vVJpTfF34HR-1B58OQfU7hx7JXCQCDO-UithePxbioJ0rK8OHPT4kR8wJ9fNM584qvhxX1DAQWbzs_KvHXJTbg"
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
✓ Created SWR script: fetch_github_user (AQAEkR6aJw7izKkXeYFJqpdOwawlhjfYgcmi5g3ljaJPFTvaCNjB0FhUJLDIMHwP2kOOFk8Xnq5z94dre1GDvw)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 80.372875ms
Result: [
  {
    "cached_at": "1767418251",
    "data": "{\"login\":\"torvalds\",\"id\":1024025,\"node_id\":\"MDQ6VXNlcjEwMjQwMjU=\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/1024025?v=4\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/torvalds\",\"html_url\":\"https://github.com/torvalds\",\"followers_url\":\"https://api.github.com/users/torvalds/followers\",\"following_url\":\"https://api.github.com/users/torvalds/following{/other_user}\",\"gists_url\":\"https://api.github.com/users/torvalds/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/torvalds/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/torvalds/subscriptions\",\"organizations_url\":\"https://api.github.com/users/torvalds/orgs\",\"repos_url\":\"https://api.github.com/users/torvalds/repos\",\"events_url\":\"https://api.github.com/users/torvalds/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/torvalds/received_events\",\"type\":\"User\",\"user_view_type\":\"public\",\"site_admin\":false,\"name\":\"Linus Torvalds\",\"company\":\"Linux Foundation\",\"blog\":\"\",\"location\":\"Portland, OR\",\"email\":null,\"hireable\":null,\"bio\":null,\"twitter_username\":null,\"public_repos\":9,\"public_gists\":1,\"followers\":269522,\"following\":0,\"created_at\":\"2011-09-03T15:26:22Z\",\"updated_at\":\"2025-11-24T04:16:14Z\"}",
    "id": "A3IbLwMsRskXDpScQazQxqbUodYM3m92-ZeYzU6w6ll_IyXWf_WUqVS_d4v0E7fmHSrzpJSYyI8YNEyZEQRIyg"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 72.292792ms (1.1x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (jjRdrsoIG9r4S7S6lyZFiLm5YswcjjrWB6sAWjENR-9nz2wpYDTsRzz1bWG_GnH8Iy38kIdvd6dK2Bynjocwkw)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "2026-01-03T00:30:51-05:00",
    "enriched_data": "{\"id\":1,\"title\":\"Essence Mascara Lash Princess\",\"description\":\"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.\",\"category\":\"beauty\",\"price\":9.99,\"discountPercentage\":10.48,\"rating\":2.56,\"stock\":99,\"tags\":[\"beauty\",\"mascara\"],\"brand\":\"Essence\",\"sku\":\"BEA-ESS-ESS-001\",\"weight\":4,\"dimensions\":{\"width\":15.14,\"height\":13.08,\"depth\":22.99},\"warrantyInformation\":\"1 week warranty\",\"shippingInformation\":\"Ships in 3-5 business days\",\"availabilityStatus\":\"In Stock\",\"reviews\":[{\"rating\":3,\"comment\":\"Would not recommend!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"},{\"rating\":4,\"comment\":\"Very satisfied!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Lucas Gordon\",\"reviewerEmail\":\"lucas.gordon@x.dummyjson.com\"},{\"rating\":5,\"comment\":\"Highly impressed!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"}],\"returnPolicy\":\"No return policy\",\"minimumOrderQuantity\":48,\"meta\":{\"createdAt\":\"2025-04-30T09:41:02.053Z\",\"updatedAt\":\"2025-04-30T09:41:02.053Z\",\"barcode\":\"5784719087687\",\"qrCode\":\"https://cdn.dummyjson.com/public/qr-code.png\"},\"images\":[\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp\"],\"thumbnail\":\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp\"}",
    "id": "IaqtUyaz0zRfNSA-3oc26JzdxbRnQtuFx3f-18aJcFzZAPoRk5olwMhOpweHWtIK2iv08BYVCoguXxADGl5Eug"
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (Nsy8Klrmubwg7pkLB4KnfLs3EalIKV460cU35mXAmS4iDz4eprQA2U2ZlYXLOAX_N2egs-oUQS3tf_j4vWsFiw)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 78ms
Result: {
  "cached_at": "1767418251",
  "data": "{\"login\":\"torvalds\",\"id\":1024025,\"node_id\":\"MDQ6VXNlcjEwMjQwMjU=\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/1024025?v=4\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/torvalds\",\"html_url\":\"https://github.com/torvalds\",\"followers_url\":\"https://api.github.com/users/torvalds/followers\",\"following_url\":\"https://api.github.com/users/torvalds/following{/other_user}\",\"gists_url\":\"https://api.github.com/users/torvalds/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/torvalds/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/torvalds/subscriptions\",\"organizations_url\":\"https://api.github.com/users/torvalds/orgs\",\"repos_url\":\"https://api.github.com/users/torvalds/repos\",\"events_url\":\"https://api.github.com/users/torvalds/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/torvalds/received_events\",\"type\":\"User\",\"user_view_type\":\"public\",\"site_admin\":false,\"name\":\"Linus Torvalds\",\"company\":\"Linux Foundation\",\"blog\":\"\",\"location\":\"Portland, OR\",\"email\":null,\"hireable\":null,\"bio\":null,\"twitter_username\":null,\"public_repos\":9,\"public_gists\":1,\"followers\":269522,\"following\":0,\"created_at\":\"2011-09-03T15:26:22Z\",\"updated_at\":\"2025-11-24T04:16:14Z\"}",
  "id": "q-mVOdkhu5t9Lm-VqbvRfV5vYd0rrERCxz-7FbgZ4t71cSIkydG5U2CPuqFTBfsONeZg5003mEkLOZPJf3cr0w"
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 80ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (L-2xFCorXia1ejsgVHPBmy7Zsrsn0rarRw73z2U6nX9SqG05OuyBlE5U2BMYLTpCo3mgZrP9PGT2Y9PZnZnKEg)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "enriched_data": "{\"id\":1,\"title\":\"Essence Mascara Lash Princess\",\"description\":\"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.\",\"category\":\"beauty\",\"price\":9.99,\"discountPercentage\":10.48,\"rating\":2.56,\"stock\":99,\"tags\":[\"beauty\",\"mascara\"],\"brand\":\"Essence\",\"sku\":\"BEA-ESS-ESS-001\",\"weight\":4,\"dimensions\":{\"width\":15.14,\"height\":13.08,\"depth\":22.99},\"warrantyInformation\":\"1 week warranty\",\"shippingInformation\":\"Ships in 3-5 business days\",\"availabilityStatus\":\"In Stock\",\"reviews\":[{\"rating\":3,\"comment\":\"Would not recommend!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"},{\"rating\":4,\"comment\":\"Very satisfied!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Lucas Gordon\",\"reviewerEmail\":\"lucas.gordon@x.dummyjson.com\"},{\"rating\":5,\"comment\":\"Highly impressed!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"}],\"returnPolicy\":\"No return policy\",\"minimumOrderQuantity\":48,\"meta\":{\"createdAt\":\"2025-04-30T09:41:02.053Z\",\"updatedAt\":\"2025-04-30T09:41:02.053Z\",\"barcode\":\"5784719087687\",\"qrCode\":\"https://cdn.dummyjson.com/public/qr-code.png\"},\"images\":[\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp\"],\"thumbnail\":\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp\"}",
  "id": "Y2iH9iSvoQw1x_nrs5JUdxAONAJfGriBr-g7be8KBnBTLDyIKxbSADGbqlNXBVtMkU8bZFp6zYQD0-VQkscr7A",
  "enriched_at": "2026-01-03T00:30:51-05:00"
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
✓ Created SWR script: fetch_github_user (1KmHUJSYJur0XjXljHNW2_P-LOHV8a5L6EsQbS0f8ExVv2w11DEeSYzcE8uI10xtT3hIfsSzpEsfYinXfU33LQ)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 85ms
Result: [{"id":"UcubsvtPltbKWPWSuPKUNIW8Bj9Q98vkWD-8o__itGSFYNlaaZuD9xg771bbFAMZjIEX3eyaxEBdER7iSWFzGw","cached_at":"1767418257897","data":"{\"login\":\"torvalds\",\"id\":1024025,\"node_id\":\"MDQ6VXNlcjEwMjQwMjU=\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/1024025?v=4\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/torvalds\",\"html_url\":\"https://github.com/torvalds\",\"followers_url\":\"https://api.github.com/users/torvalds/followers\",\"following_url\":\"https://api.github.com/users/torvalds/following{/other_user}\",\"gists_url\":\"https://api.github.com/users/torvalds/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/torvalds/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/torvalds/subscriptions\",\"organizations_url\":\"https://api.github.com/users/torvalds/orgs\",\"repos_url\":\"https://api.github.com/users/torvalds/repos\",\"events_url\":\"https://api.github.com/users/torvalds/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/torvalds/received_events\",\"type\":\"User\",\"user_view_type\":\"public\",\"site_admin\":false,\"name\":\"Linus Torvalds\",\"company\":\"Linux Foundation\",\"blog\":\"\",\"location\":\"Portland, OR\",\"email\":null,\"hireable\":null,\"bio\":null,\"twitter_username\":null,\"public_repos\":9,\"public_gists\":1,\"followers\":269522,\"following\":0,\"created_at\":\"2011-09-03T15:26:22Z\",\"updated_at\":\"2025-11-24T04:16:14Z\"}"}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 83ms
Cache hit was 1.0x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (1MikHevlt49bjhDFEaRgLhT8L_TUwjBiaM417Kk-jnbPB2vCf2mTvtLRr_ckWAU_y15Ntl94r-uSg0CTZMa7ag)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"enriched_data":"{\"id\":1,\"title\":\"Essence Mascara Lash Princess\",\"description\":\"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.\",\"category\":\"beauty\",\"price\":9.99,\"discountPercentage\":10.48,\"rating\":2.56,\"stock\":99,\"tags\":[\"beauty\",\"mascara\"],\"brand\":\"Essence\",\"sku\":\"BEA-ESS-ESS-001\",\"weight\":4,\"dimensions\":{\"width\":15.14,\"height\":13.08,\"depth\":22.99},\"warrantyInformation\":\"1 week warranty\",\"shippingInformation\":\"Ships in 3-5 business days\",\"availabilityStatus\":\"In Stock\",\"reviews\":[{\"rating\":3,\"comment\":\"Would not recommend!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"},{\"rating\":4,\"comment\":\"Very satisfied!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Lucas Gordon\",\"reviewerEmail\":\"lucas.gordon@x.dummyjson.com\"},{\"rating\":5,\"comment\":\"Highly impressed!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"}],\"returnPolicy\":\"No return policy\",\"minimumOrderQuantity\":48,\"meta\":{\"createdAt\":\"2025-04-30T09:41:02.053Z\",\"updatedAt\":\"2025-04-30T09:41:02.053Z\",\"barcode\":\"5784719087687\",\"qrCode\":\"https://cdn.dummyjson.com/public/qr-code.png\"},\"images\":[\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp\"],\"thumbnail\":\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp\"}","id":"4FE1tSRp9zqNwoDVOBsla0PwNG5g8lywEgTHz7YsKMQ8P3_Ah92c-x56ilZQVbg37eBmUx3S-lPDPLQah7lptw","enriched_at":"2026-01-03T00:30:51-05:00"}]
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
   Name: Some(Object({"value": String("User 1"), "type": String("String")}))
   Department: Some(Object({"value": String("engineering"), "type": String("String")}))

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 82.179042ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 84.618167ms
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

> @ekodb/ekodb-client@0.6.1 prepare
> npm run build


> @ekodb/ekodb-client@0.6.1 build
> tsc


up to date, audited 7 packages in 844ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.6.1 build
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
   ⏱️  Duration: 81ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 78ms
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
✅ [32mTypeScript function composition examples complete![0m
🐍 [36mBuilding Python client package...[0m
🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings with abi3 support
🐍 Not using a specific python interpreter
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.07s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.6.1-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.6.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.6.1
    Uninstalling ekodb_client-0.6.1:
      Successfully uninstalled ekodb_client-0.6.1
Successfully installed ekodb-client-0.6.1
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
   ⏱️  Duration: 83.4ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 79.9ms
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
   ⏱️  Duration: 83.513834ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 84.012667ms
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
   ⏱️  Duration: 88ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 89ms
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

