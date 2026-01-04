make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("HL1dIV1NyAw3lpRfJExuBcCXPW9LpxG_1M82NrcYL2U00saKtzROAQLaS2zc-CipJHX7Q4mBlnjaS8JRL1dyrw")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("HL1dIV1NyAw3lpRfJExuBcCXPW9LpxG_1M82NrcYL2U00saKtzROAQLaS2zc-CipJHX7Q4mBlnjaS8JRL1dyrw"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("HL1dIV1NyAw3lpRfJExuBcCXPW9LpxG_1M82NrcYL2U00saKtzROAQLaS2zc-CipJHX7Q4mBlnjaS8JRL1dyrw"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("HL1dIV1NyAw3lpRfJExuBcCXPW9LpxG_1M82NrcYL2U00saKtzROAQLaS2zc-CipJHX7Q4mBlnjaS8JRL1dyrw"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "nM-4Yeiy_NNKrQ8nYlvtKtF7PcI8tNE5I1l7Gsolj9YOSEoI5ZHPa-lL6fSZAOlMYl6LvJkAmUO3NNZv-wV4Aw"

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
        "id": "nM-4Yeiy_NNKrQ8nYlvtKtF7PcI8tNE5I1l7Gsolj9YOSEoI5ZHPa-lL6fSZAOlMYl6LvJkAmUO3NNZv-wV4Aw",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "6SaGPcl4WuNpW2pMQ0QPr5sa8fTeUEltF2J1abuRj_6z5LVKKsUhNXm9PGI195kIsvsxC1bU75qP7hzuMJ1HEg"

=== List Collections ===
Total collections: 4
Sample collections: ["batch_users", "test_collection", "websocket_test", "demo_collection"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "ZhnB3pM3b1anSQM-wS72OoqUyygE0a7p__KiCW6zifbAQn9zjicWUMkJNPUsUAkF3QR1MEAAUJi22yEWtB-Cww"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "3Ba9F4whqOs61wTGBYEuHc7mn_FTX7oAP9EzHxbrI6pRZqM6TuV7oZmICKs0GBEegSLoWNb5l_G3X63wLHrNvQ"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "jw-g1cAh1f-bEJR8-KvlzvVfSBc-MWuAA07AT-2S3pCl8o4uL1RO3KG6hIMGcxKdIjYtITw65aYT793BLIzeoQ"

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("bfMs_KaDnM0wnj4TlRXN3LWD8qHQgAZ3frPIzETFU3ATndA0ONEs-gr8oWPvYOODphSBO1TxNQkmO2zmyoRAgA")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: wBuFceSeHg1-50GmIJtOY0pcCJulSnVnaVMsQu2pWQa9ds6yRT6yAJS3a8QR7zB_ysku1uq65HcI27y50TCTlA
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "z5dI7qEYb1Fkg8HdYJZ9_IzlNjZvPqro3t_zHgQG02UwQocDIZ6dZpDPvRvYwMfIo00w8hIxmENfxrwvtk3khQ"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: lHFXYCG2c1YRO9fbSBclKWRtPYRsDf9yP3rKbKiVzbzo6KWiQNMcIa4I6RKNXPzVtSNZSqu0i3Y7rQqjLH6LXQ
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: kAm3uhXJn61E4bjh0el5r1vvmCSuekvWBSWtD6_boNBLJ5Q24IBck2VUCQMg96TEVMVmqRc2PkvSnjNOIGVDVw
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: vXGaaPL4IHRGIWoT_1RnyOgp0uRLHz4VKKFgaytF90-KtuMMk7jzHcSPBeq3ULM8hPNYPFRisDgN2K10wPYE4Q
Created Bob: $500 - ID: cd5rIDN8-ARWFoHSWiRh73Z3Tq7eNDFD7cisx74zu6ipDrwe-JAbiP3q4sFv598tl71FWPGDjxa8fBGLlU_32A

=== Example 1: Begin Transaction ===
Transaction ID: d8f6c42c-6d39-4c78-8db1-80f4d3cd1051

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
New transaction: 46494fad-5cee-4e26-885d-177a1153aab5
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record { fields: {"id": String("f-kylgGEF3naAxih03OBVd0BLkYq8AqbklSv4CMxdDMcHtJPX8Z9CUBe9ltV9UgPu43cWcs-rVI5mdfTve55rQ")} }

=== Find by ID ===
Found: Record { fields: {"metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-01-04T06:31:20.310951Z")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "id": String("f-kylgGEF3naAxih03OBVd0BLkYq8AqbklSv4CMxdDMcHtJPX8Z9CUBe9ltV9UgPu43cWcs-rVI5mdfTve55rQ"), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "name": Object({"value": String("Test Record"), "type": String("String")})} }

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-01-04T06:31:20.310951Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record { fields: {"user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "id": String("f-kylgGEF3naAxih03OBVd0BLkYq8AqbklSv4CMxdDMcHtJPX8Z9CUBe9ltV9UgPu43cWcs-rVI5mdfTve55rQ"), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "name": Object({"value": String("Test Record"), "type": String("String")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "created_at": Object({"value": String("2026-01-04T06:31:20.310951Z"), "type": String("DateTime")}), "value": Object({"value": Integer(42), "type": String("Integer")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")})} }]

=== Update Document ===
Updated: Record { fields: {"data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "created_at": Object({"type": String("DateTime"), "value": String("2026-01-04T06:31:20.310951Z")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "name": Object({"value": String("Updated Record"), "type": String("String")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "id": String("f-kylgGEF3naAxih03OBVd0BLkYq8AqbklSv4CMxdDMcHtJPX8Z9CUBe9ltV9UgPu43cWcs-rVI5mdfTve55rQ"), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "value": Object({"value": Integer(100), "type": String("Integer")})} }

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "KCNh5ChD3LEKR9eRL7Va3BPoov84BU6rWrdPKe119mUjOT0Rjwg56gM4gx0ojSJ-8ScOZug5Zpw2BkjIm7K9gw"

=== List Collections ===
Total collections: 9
Sample collections: ["batch_users", "test_collection", "client_collection_management_rust", "users", "scripts__ek0_testing"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "FGrKoHyoCFglRQzPpZqfggp5_WuL2YOzO8sBtptvyxaYSs3CPLt45hzSZvHEQUbg94yZszH5xmym1ZDCkI7Wjg"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("lAy0EiRmFSqTqszK-JRLseAohUKMo4sgp4Ps3VR9eRj4z5SRUI_0OTX16Fg_49qrljCOC36jpPq-QJWurHbDxg"))

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: NgdyNrxYyVKMmutAAy2E_lg-tP72MMNTGeOcLx1Q1r8o_yRTz-5c7eIPnP8csKmNYt5VHB9KlIydJDI5NU2OiQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("ljftEFK92ePQlnzx8QDjsCOXgT9zEjk-JrW3AXy0u79sXRzJtkO2ucGQ4G7zQEdfHr-uOH7kTVP-8pNVBhH7nQ"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.57s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

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
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"type": String("String"), "value": String("Charlie")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"value": String("Bob"), "type": String("String")}))
  2. Some(Object({"value": String("David"), "type": String("String")}))
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
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
     Matched: ["title", "title.value", "description", "description.value"]
  2. Score: 2.0000
     Title: Some(String("Python for Data Science"))
     Matched: ["description", "description.value"]
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
  2. Score: 6.6000 - Some(String("JavaScript Web Development"))

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 1ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/examples/client_schema_management`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("N6z27HvxoYiCGpL8aePb1XplIl-IoKSVRaIbQjf3jBP57huYebhIkcQ_kXeowvMuY47zyveuowHcVWXKvfoFGw"))
✓ Inserted user 2: Some(String("dhyzJN1uNLWTseLqee7qBAPgsylfbywcg_D5F9LYbXbsGWy_uQ8TdDS2w3eJU2Ps2qlBWt0I1yuTr5NHDGJnwg"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - age: Integer
  - title: String
    (required)
  - email: String
    (required)
  - status: String

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["batch_users", "test_collection", "users", "scripts__ek0_testing", "websocket_test"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: AtwBzakJFSuDzeoN5C86fY2ZlXZBQj1kjD1Tjxl0iHObhCP8EL4aRdenEGal4OwDAUBIP79dvRi523bjUx4vTA

=== Sending Chat Message ===
Message ID: VdHlIN5oVlZX3hxY6zWccJj8fYvrhjLTbeESiTZRoHdHFgcKIv0UB5dbf2AO_-m2gLEtEmjvGISvxhiVd5_PTQ

=== AI Response ===
Response 1: ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration. It offers a range of features including an AI Chat Integration that allows you to query your database using natural language to get AI-powered responses with relevant context. It also supports full-text search, vector search, and hybrid search with automatic context retrieval.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("u77QAYjH7OO14UAUq5rctvvmSrne7yXCYB2SbyN5oiL9LmQJFQs2wQ2LsJGr2wcGWORVB8VHFU7kk9dJFl516Q"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("ICot9AvVSKPBkZHI7zS8_SzpbOzzxiTqyrKOjS6aCdRZ39yhmiikEdUi1PY6L4faqyejF_v47RbAO89YhgPXEA"), "title": String("Search Features")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("q5ciHBwrMj2rkTAbjTZuaeW5HJZevR-KGFQ1sDTrtyvMrTcGXmpUf0syFj8gZLJ_JfEs-4e-Q601u7yghk3KSg"), "title": String("Introduction to ekoDB")}

Execution Time: 2468ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: xi8NGvCFxkY8run1C7czBm8Zusrp3ulWQll5bARwCYBvRo77ERcVKgiJZg_PaNTblnAnCPp4cNn3XbX42Fahfw
=== Sending Messages ===
✓ Message 1 sent
  Response: Answer: The product available is ekoDB, a high-performance database product with AI capabilities.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: ZtiozsqAaPp2uhueVZUcY3N-AVtW1qi67IHKJ31PABt9OxmHWyiMIUzrggTrHbokmPi5K3YNb249A1h3oA58fg
  Parent: xi8NGvCFxkY8run1C7czBm8Zusrp3ulWQll5bARwCYBvRo77ERcVKgiJZg_PaNTblnAnCPp4cNn3XbX42Fahfw

=== Listing Sessions ===
✓ Found 3 sessions
  Session 1: ZtiozsqAaPp2uhueVZUcY3N-AVtW1qi67IHKJ31PABt9OxmHWyiMIUzrggTrHbokmPi5K3YNb249A1h3oA58fg (Untitled)
  Session 2: xi8NGvCFxkY8run1C7czBm8Zusrp3ulWQll5bARwCYBvRo77ERcVKgiJZg_PaNTblnAnCPp4cNn3XbX42Fahfw (Untitled)
  Session 3: AtwBzakJFSuDzeoN5C86fY2ZlXZBQj1kjD1Tjxl0iHObhCP8EL4aRdenEGal4OwDAUBIP79dvRi523bjUx4vTA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: ZtiozsqAaPp2uhueVZUcY3N-AVtW1qi67IHKJ31PABt9OxmHWyiMIUzrggTrHbokmPi5K3YNb249A1h3oA58fg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: NEj4xMSIMq0ixARHRISNA9CqdRCZcrxV1XWBpW-ZnMDr4GYeSJg8EYwdARl29tYJ5nSNVFx2IqoYqXdeJIZybQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the available product is "ekoDB". It is a high-performance database product.

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
✓ Created second session: ckT-vsSlrRFV67BvKFS30PxVBECc1qDvSkEeAZIHWGmSDdCBnk96BxjiLq-EJhaQMWUZQ-4ePmY9X4BckHHBMw
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

✅ Script saved: n2myM7eUap0f4DPp_FY2xgnjdm7BEJHluyHlnZBetYAcPoKKIJCYLpmVAkwhtwEegdkAKAs6dDlJ37VAPdpalQ
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: hR0PRL9TuXDB7bWSSrivcMWPA0anuvL8NfguAVu7zvAVOjpAs9P-3loi-L4y21Is7LjuMW8PlDXuZ1vj6DwVdw
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: r1ocg8CJvl5IYo9u-ZLtWuegOFgTMUtlOkDn02e2kzyV_95OCTikbwycHg-2tJjXuvlz2DsFT1pC5NzlMKXqfg
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 6
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: d6wUU0QuFZ4MX_oezI3632fhQR2BfiyGKZjTRG5os1knXgxU2ipX5kt-qeGrbklDOubKNw5a9AJAgUqgXbI_Tg
Created Bob: $500 - ID: IbksYNt2Ntr2dhp6b9-VAkLMBywDc7TECeGSsLQswPkJQB0tEUr0-JCCSNxYdeZI12jOkVEusiaYeyDqeB7kaA

=== Example 1: Begin Transaction ===
Transaction ID: 87e7ffcb-67c9-4bbc-9973-df904d0b471f

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: c3a1d689-01de-4f2b-9e6d-ffcaad05c519
Status before rollback: "Active"
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mRust client examples complete![0m
