make test-examples
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
✅ [32mAll Rust integration tests complete![0m
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'fGZzLK6DlHlYw_vuW5BdF8YQXrma1EQ4ocsUMij_eg6Oxl8iprLKJ2foPQMZseIXJcAQJgniME-aUHRmuWuP7g'}

=== Find by ID ===
Found: {'id': 'fGZzLK6DlHlYw_vuW5BdF8YQXrma1EQ4ocsUMij_eg6Oxl8iprLKJ2foPQMZseIXJcAQJgniME-aUHRmuWuP7g', 'active': {'type': 'Boolean', 'value': True}, 'value': {'value': 42, 'type': 'Integer'}, 'name': {'value': 'Test Record', 'type': 'String'}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'value': {'value': 100, 'type': 'Integer'}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'id': 'fGZzLK6DlHlYw_vuW5BdF8YQXrma1EQ4ocsUMij_eg6Oxl8iprLKJ2foPQMZseIXJcAQJgniME-aUHRmuWuP7g', 'active': {'value': True, 'type': 'Boolean'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: a3HSAeDJ4dkj4hErQeiyqFAv339ot0i90jIBizcekkcFWpdStzLfwrQA-ejOxfbPFxFuF9w977m4x7DpOIeeUA

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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "a3HSAeDJ4dkj4hErQeiyqFAv339ot0i90jIBizcekkcFWpdStzLfwrQA-ejOxfbPFxFuF9w977m4x7DpOIeeUA",
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
✓ Verified: Record updated with status="{'type': 'String', 'value': 'active'}"

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
Collection created with first record: RmN4VZwuFS0ghNDKfu4LGk_fnkXJ__TNbuAJ_22wzYmLLtLizi_anQQYY91_Y0kTy-7SBqC7t278T9KKBvaelA

=== List Collections ===
Total collections: 11
Sample collections: ['batch_users', 'chat_messages__ek0_testing', 'test_collection', 'users', 'scripts__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: ddnT9nDPDw_cAbQ8UN6H0xcisg5xZ7ueY_nnwhAz9cLsrKHbxZEbPJoK3q-pw4dyLaQwW9j-_Bci0IXENxkPWg

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: created

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: n74H60Jy-8ItOmBWLaOqrdF3DjQPBFZKBCSM1WYW6TBwkjYoZv1tOPrwuAYpF1I2Ym1m_FhZWGyKC2X9Dc-8ig

=== Query Documents ===
✓ Found 5 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: DRlQMjPPYPXnMTos_Z9TYq34I-7tUR5s0E6su4FeO3TkjVLsnZTY5NSBix66RSE8ma-tcUh0aIURYmfHRGMBJg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: YEpdKoHVrexVtGpvAz_TG8ypFA16j7U5NObeihfZKGfxxlOVBRJyqe1El3jNjKzOGsmIGT6MASPqPw6Z7-wnGw
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: vyeA1eo-_tDqYgBFgaRahqRWOtTF6GorJJX5tJQBO0CY5DaA5VOOn58rYRRHK873j-Ru1GrxMN3ZqQKtQx15LA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: -tN6QXzrPJF_KZn108nKqjXmS4O0E9crBCKgxrZhH1QlsNXGAv3gcN3ECk3_w0MbQWUh1qbXcT916fMbdhCK8Q
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'count': 15, 'avg_score': 50.0, 'status': 'inactive', 'max_score': 90}
   {'avg_score': 60.0, 'count': 15, 'status': 'active', 'max_score': 100}

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
Created Alice: $1000 - ID: dauWCYmH0KIOJpRRAowt2PWcshgqB3RzQcz_n_4BGxO8HTuBW1jLuGLHKJZk4BAMd_VEd7uW4MKw-18PFxULyw
Created Bob: $500 - ID: iX01rzu7JSSgVTg_q6OjEe3a8POrLLzo7CocLuz-Sm8z61frReeOkmgaLALfpblvF0R92mi4r_NFGj2ih_CrNQ

=== Example 1: Begin Transaction ===
Transaction ID: 3f01a7a0-1a09-41f5-b6cf-6e13ff6bed3d

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
New transaction: 5529e99f-17df-4453-a247-ba64128ead6a
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: ObA6f-Cn1FRqOqlgia9kkTXfH9dCiM5GGyQ41fZR6DRVw27MKGodyiPtcsJZiDdEKKWDeL5vCmtQPYB6zRgjqQ
Created Bob: $500 - ID: 8GOCsHbtquT7xBuge5Bnvg_5C5IEX6JY0KOK5lLTWXXq1dDjoJ5zB3euGGaSBpFzvO4RjJJ58yxjBzHU0guEgg

=== Example 1: Begin Transaction ===
Transaction ID: 5167ef1a-dfda-4fd3-9c50-f323cce76cd7

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
Created Alice: $1000 - ID: S5t2oSUdGFc1DtGKonhjVheGA6OVDvaX0D4V8VbVszsF1y7NhNd5O6gcD6RbX85R38cfXBzP3vy4VcyoK08Gzw
Created Bob: $500 - ID: k93wbIUSKsSNDOiIJYR_xIn2sdrYVKLtIVUJDyC4x7NMVefmRhmYJOl1nALbfEpu3VgqUUWmtvP-1t4Fhs-Wbg

=== Example 1: Begin Transaction ===
Transaction ID: 0ab85737-5b2a-49a4-8134-e0ce5e3339bd

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
New transaction: c7f509a2-0a21-43fc-aefa-347587f45f48
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Wi_TWJVL9j3_X3dScIrAJNtBuyaNxkhgT8u3WCV2Xy_dM9cTUB3qB8nQJZugMhZnS_xHTuAEyCTO-USt3-r_LQ
Created Bob: $500 - ID: dlhrEcuskjlasNag4KeZjxvGhM1k7sLoiqJdz4p70Jtk4HaVpeEvb31HACHSK6Une8PuJPzwnweveXucZ3vThg

=== Example 1: Begin Transaction ===
Transaction ID: 185c89d1-2ab3-49df-abd2-681951b23fe7

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
Created Alice: $1000 - ID: QEA_lRRjpyqHu2289FE76Ckes6aMWWH4GN7QXWc9UgCVRaDJSy7YCjkdzp0jIqUl0mw2piR9zhHz2OjjB_SVtg
Created Bob: $500 - ID: GujXfca1YzbYJXXUcKUv1Uo-R5tLcwfvsB9mdjDCwFHOR7r7pIGxWeJmw5em5_TTHDLrAjF18DWRs9CkEb4vBQ

=== Example 1: Begin Transaction ===
Transaction ID: df101bc6-c2f6-4c6d-b9de-39392ed8e7e3

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
New transaction: 67a9a696-576d-42b3-a408-abb955d4dac8
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 6tpxBeAKTWt7fn_SPXuXpN_GFY2TDf867PNjPvVuqpu45tvxuSD7uBfrSYRF0TUlVmN-HKn8wJ0v31CYF5oPyQ
Created Bob: $500 - ID: pmb1py6dC9-9sKYZIy_6YrVtX3DoWXFAu96SAdXsMRailMpEakGqcob2OzHG7mRdBfr2kNQSgWDysaUud9QvWw

=== Example 1: Begin Transaction ===
Transaction ID: 29c71e98-b41c-4806-a12f-c29dd26605aa

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
   ✅ Script saved: wn_6rC1cGRj2rLCUAO8HWv_fUJd5dkf37rY5bihHtqZSbfaZiMdGzOvGwflbJPVWJtxcd6CfaylnP9wS1XYexA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: tVVzOeW_eADjf6DlOnm8tJJzSuLWCmQIMDVgC7rHxjqDMH4K9LdcXn6myyu-ryFSGL8sR5kUx2ui1heZgytiyA
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: gBNmcbWvZots_ZvRxLD3BeaIACdHvMTDlDdNE8e6Jwgsoi7kUE2DrIK7hYgX3VgbfogKjdh5xY6iJZ3GCv2v8Q

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
   ✅ Script saved: TwPl6J0gSYUAPOtUzF5MPPrzpAYHCsj7eR1lml41hDMlB_NUtzWpnyzfUlbnqMguA1VZjMOqIlbKHhCFsRcSzQ

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'value': 0, 'type': 'Integer'}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: gcfz4TNpRfKBS0CFU1Xx3yfOqP9rBTzORLPQG-R0_PzfIKhAV_YMmUWPmodbLYctNxZ1YS-YRP9oyAH04PRmSA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: wn_6rC1cGRj2rLCUAO8H...
   ✅ Deleted script: gBNmcbWvZots_ZvRxLD3...
   ✅ Deleted script: TwPl6J0gSYUAPOtUzF5M...
   ✅ Deleted script: gcfz4TNpRfKBS0CFU1Xx...
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
    Finished `release` profile [optimized] target(s) in 0.09s
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
Inserted: {'id': 'M2bXYIA1JLzj_XQOkVarsxpU1w3mrEuZKTBy7jt6p2E-bJE1dGX2MdrqTFuaZM0PSdohpAGhcnshHPa01eA6uQ'}

=== Find by ID ===
Found: {'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'price': {'type': 'Float', 'value': 99.99}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'id': 'M2bXYIA1JLzj_XQOkVarsxpU1w3mrEuZKTBy7jt6p2E-bJE1dGX2MdrqTFuaZM0PSdohpAGhcnshHPa01eA6uQ', 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'active': {'value': True, 'type': 'Boolean'}, 'created_at': {'type': 'String', 'value': '2026-01-04T01:32:12.725936'}, 'value': {'value': 42, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Test Record'}, 'metadata': {'value': {'nested': {'deep': True}, 'key': 'value'}, 'type': 'Object'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-04 01:32:12.725936
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'data': 'aGVsbG8gd29ybGQ=', 'price': 99.99, 'categories': ['electronics', 'computers'], 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'id': 'M2bXYIA1JLzj_XQOkVarsxpU1w3mrEuZKTBy7jt6p2E-bJE1dGX2MdrqTFuaZM0PSdohpAGhcnshHPa01eA6uQ', 'tags': ['tag1', 'tag2', 'tag3'], 'active': True, 'created_at': '2026-01-04T01:32:12.725936', 'value': 42, 'name': 'Test Record', 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'user_id': '550e8400-e29b-41d4-a716-446655440000'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'created_at': {'type': 'String', 'value': '2026-01-04T01:32:12.725936'}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'value': {'value': 100, 'type': 'Integer'}, 'price': {'type': 'Float', 'value': 99.99}, 'active': {'value': True, 'type': 'Boolean'}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'id': 'M2bXYIA1JLzj_XQOkVarsxpU1w3mrEuZKTBy7jt6p2E-bJE1dGX2MdrqTFuaZM0PSdohpAGhcnshHPa01eA6uQ', 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}}

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
Collection created with first record: "5FQm46vMFAm8OKrHXHOACQOBP28JcH6GLIcI6Vwzgn0FtLR-ouX0_AGxz6fsZv3Erj_zQQj1VAsVavQDNLhxRw"

=== List Collections ===
Total collections: 10
Sample collections: ['batch_users', 'chat_messages__ek0_testing', 'test_collection', 'scripts__ek0_testing', 'websocket_test']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: J3U4wAFw2CDWAc5Y7fC5kuVURyMgXMLfXZYcMtfMo313P29yJ_dlYvLHIcGybpwjM3rcXlLiT4G43BT89LrmCA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: bPQ_VHXK6P3X_Cif9Zg8ub0l-kbvdFF9F9eJhUPn0rAHxNgvahWQsYcHnzhesPmTno14ulpiaxDIKjcJR_-g_w

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
✓ Inserted test record: PUjOqbvpsM_ZBfQqmPdYQRdu4tNCBDXVlYzYIDA2STJMN_QMpE-yBiHnah6FQ61V-_H8q7NUy18gYUQhBiwo2w

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
✓ Inserted document with TTL: CJkeoPPnffAQulPgK2HOos_cZF4yagC1Drpz2dFomVzEZDEy72Ezsuyo23-XjbnlGTJq6uf_chWSQZMfOkXSjQ

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
  1. Score: 25.740, Matched: email, email.value, name.value, name
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, title, bio.value, title.value
  2. Score: 26.400, Matched: bio.value, title.value, title, bio
  3. Score: 26.400, Matched: bio.value, title, bio, title.value
  4. Score: 26.400, Matched: title, bio, title.value, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title, bio.value, title.value
  2. Score: 39.600, Matched: bio.value, title, bio, title.value
  3. Score: 39.600, Matched: title, title.value, bio, bio.value
  4. Score: 39.600, Matched: title, bio, title.value, bio.value

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
  1. Score: 0.781
  2. Score: 0.753
  3. Score: 0.737

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.716, Matched: title, content.value, content, title.value
  2. Score: 1.147, Matched: content, content.value, title, title.value
  3. Score: 0.527, Matched: 

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
✓ Created session: rWDdGI5jTi6Tm3bG4At0VPgu8m0mVWOpMz-31zOK6dkYgequ9swcbxoeAfq68y86Op-7mF-ee8-n4vDcw5fGCA

=== Sending Chat Message ===
Message ID: zFE6iw3UwvD7X50oxScmIKjGYsW-rxunC3iTGcYshk7jN-JG2Zw_LIU5AylL6wUy6qcv5I99s0eGnG1JJKhT9w

=== AI Response ===
The products available are:

1. ekoDB Cloud: This is a fully managed cloud database service product. The price of this product is $499.

2. ekoDB: This is a high-performance database product with AI capabilities. The price of this product is $99.

3. ekoDB Pro: This is an enterprise edition product with advanced features. The price of this product is $299.

Execution Time: 2700ms

=== Token Usage ===
Prompt tokens: 606
Completion tokens: 83
Total tokens: 689

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: BGziR8-8gkMr75-xFjh9ze6J3YD333YrHqF2Rj1S2EGv9Nmw4wg9zAVCZqH3ftp7Snp1WZ2xdzgRPwCwswUE_g

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the available data, one product option is the "ekoDB", which is a high-performance database product. The price for it is $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: XnsJ8gn5nhWAWWMWu7OE_aXjbkDw0Sr6kY8AJa_vYGdsoj5GCHMuAmvgsMaDabHCVO0re21WZ-E0aVouZnnEjg
  Parent: BGziR8-8gkMr75-xFjh9ze6J3YD333YrHqF2Rj1S2EGv9Nmw4wg9zAVCZqH3ftp7Snp1WZ2xdzgRPwCwswUE_g

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: XnsJ8gn5nhWAWWMWu7OE_aXjbkDw0Sr6kY8AJa_vYGdsoj5GCHMuAmvgsMaDabHCVO0re21WZ-E0aVouZnnEjg (Untitled)
  Session 2: BGziR8-8gkMr75-xFjh9ze6J3YD333YrHqF2Rj1S2EGv9Nmw4wg9zAVCZqH3ftp7Snp1WZ2xdzgRPwCwswUE_g (Untitled)
  Session 3: rWDdGI5jTi6Tm3bG4At0VPgu8m0mVWOpMz-31zOK6dkYgequ9swcbxoeAfq68y86Op-7mF-ee8-n4vDcw5fGCA (Untitled)
  Session 4: ckT-vsSlrRFV67BvKFS30PxVBECc1qDvSkEeAZIHWGmSDdCBnk96BxjiLq-EJhaQMWUZQ-4ePmY9X4BckHHBMw (Untitled)
  Session 5: xi8NGvCFxkY8run1C7czBm8Zusrp3ulWQll5bARwCYBvRo77ERcVKgiJZg_PaNTblnAnCPp4cNn3XbX42Fahfw (Untitled)
  Session 6: AtwBzakJFSuDzeoN5C86fY2ZlXZBQj1kjD1Tjxl0iHObhCP8EL4aRdenEGal4OwDAUBIP79dvRi523bjUx4vTA (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: XnsJ8gn5nhWAWWMWu7OE_aXjbkDw0Sr6kY8AJa_vYGdsoj5GCHMuAmvgsMaDabHCVO0re21WZ-E0aVouZnnEjg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: r1RhDCNcLRqZ7Lvp5PBLdubZHGV_ARKALNwmG6kJKvkjmCWXaL-Qp_uKM_gSewu22Y1N2C2lMqc9pHASvkJ7wQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, one product that is available is ekoDB. It's a high-performance database product and it's priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['created_at', 'updated_at', 'content', 'token_usage', 'chat_id', 'context_snippets', 'role', 'id'])
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
✓ Created second session: hIhtRI8S4wACiDzJQuUxJAcNLdKhXI3vCzy7Zx2uvCBYmr-lHtb8oxO_AQ4pU8uXdwKlEqOb0Gt7PX7MWaCmww
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

✅ Script saved: F4TwKiI3ezE0EHHY1IN4qkedxWsUThEceQWQwZuCEGIdZoxV5ENy455wt5dRBUPvAmEDCbirGJtDBX2Tl6Rhjw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: WF4OQ1VbNz8TLgnyVRrwFHsyRrcTQfxkvXEU9jPSxPXhGV6EQpHbslLwtvNN381uj0Hxk6cVsvfEqe0Kemuw5A
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 2QCEPbK_BBBq-c3gUAL1MUurejHnLial8CiK-0RME8VSbVJhk7JiDSplj1-I7q77rrn2FAI9-TXlcELyLOFZ5Q
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

✅ Script saved: 0wpgM1aqiN03KerbfiKBXthAEPb5NyZ76fkL0gO90Y70fgUUzJbNvEgPQVykzdLHy-cN0my9x2Q8jtvyaXjhtg
📊 Found 2 product groups
   {'avg_price': 474.0, 'category': 'Furniture', 'count': 2}
   {'avg_price': 575.6666666666666, 'category': 'Electronics', 'count': 3}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {'category': 'Furniture', 'count': 2}
   {'category': 'Electronics', 'count': 3}
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
Created Alice: $1000 - ID: EeuBPIbSRgPp_HbJ4jFqiBMtPLtcO3HM63RX_b9-pkrHFim-bCXTvqH5z6GvIRwrI-uGNi7IoFFog4iEEvuX0A
Created Bob: $500 - ID: UWst16YOTDJniCVKmllwWUOyhkL9GoGwlfY8daf8E6X-wGnrGna0-Zk-tISIOEIwydL9mMS4U_x2HZqmR8zgmQ

=== Example 1: Begin Transaction ===
Transaction ID: 7aecde4e-7ebb-413c-ae0f-609fa112e676

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: a868a556-086d-405e-89e6-ec60edb44d4a
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
Inserted: map[id:42hQBr-K1do5CWsdo3bzNuFxQyEV6rckCDBQpCDtAbfAXybUw7Z-TqN8aYeo9-E0FC1ii8hWg5GnCLwH9bqsww]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:42hQBr-K1do5CWsdo3bzNuFxQyEV6rckCDBQpCDtAbfAXybUw7Z-TqN8aYeo9-E0FC1ii8hWg5GnCLwH9bqsww name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:42hQBr-K1do5CWsdo3bzNuFxQyEV6rckCDBQpCDtAbfAXybUw7Z-TqN8aYeo9-E0FC1ii8hWg5GnCLwH9bqsww name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: z9LcWWNnG9qb3sE9I9ROC59IDx8hjNWn2hIqE9g6JS3nfKdpi4aLfs8iMwbAqZy7BhMTRTR6uqt3iSx83kVaIw

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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "a3HSAeDJ4dkj4hErQeiyqFAv339ot0i90jIBizcekkcFWpdStzLfwrQA-ejOxfbPFxFuF9w977m4x7DpOIeeUA",
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
        "id": "z9LcWWNnG9qb3sE9I9ROC59IDx8hjNWn2hIqE9g6JS3nfKdpi4aLfs8iMwbAqZy7BhMTRTR6uqt3iSx83kVaIw",
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
Batch insert response: map[failed:[] successful:[Jc8xk6_U68NfhaUnIKyd682tkICc0WraZylJiYQeLfxKzafwubi7mRO1Lnl6zMWupFxnIN6QES5_2CmGeg5vKQ IN3fThyBehxqX5C5TgkPsOXtYZ1zUfzgKGSQCK4rHdUigrZ-304i_WyUvap9JnTV7M6P4TFrvZaghjy_pnbXyw XDpegAgaQQssBdQ3ZnzbhQeT9-H9RW4LLkcJ34kkecXLcVs3hjKrIonGDwYVcWpZspXFrPIePAeCbQBODigFmg d_qwnswluZO_-W1wP7JQrzHLPUJlL46pglkUMoNelotS9TkB48vOOqMruVEPwa16QFOZX0YOzRedHwANTWwi9Q iJ-aGNbJ5xWUOOFm3BH9HLWamkPPbzBJ1qt6wANk7E-vVNt23oxpvbm9LSQm_ddjFOV86hTzbRXgyqBzqvZ1Pw]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[usPZUuFjxtalMEItumQwQVVr6Y1bSJjK12vTDaGgisyD5_BNc8rcBSpVfoGSvtcJd1bKx1DDykSDo5CiHkrvuQ -4Q-Du7fq9Ytzxxm_7w1aHZQtsG8Ce03464kJ1NR7ktp_jf1f_mHxoYwWyWBVR9Mk89UnZHeT5k7AOH1WFO9KQ souSs-xEo9ohjKK9YrkBNoee3JX_uTGFScclC2w4ewS-4yHAyI07ztfGzc82A0R7viBTTGxBO_AXkiBCp_Ff1w]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[usPZUuFjxtalMEItumQwQVVr6Y1bSJjK12vTDaGgisyD5_BNc8rcBSpVfoGSvtcJd1bKx1DDykSDo5CiHkrvuQ -4Q-Du7fq9Ytzxxm_7w1aHZQtsG8Ce03464kJ1NR7ktp_jf1f_mHxoYwWyWBVR9Mk89UnZHeT5k7AOH1WFO9KQ souSs-xEo9ohjKK9YrkBNoee3JX_uTGFScclC2w4ewS-4yHAyI07ztfGzc82A0R7viBTTGxBO_AXkiBCp_Ff1w]]
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
Collection created with first record: eiYb5o4lzXJwVo2Pg-tcUC9mMS9KLuiTYMIaushlS8Ai4fU-cQHoqjBmGGetTB53cqBOf-Fc8U0fBwmvgg8heg

=== List Collections ===
Total collections: 15
Sample collections: [schema_documents_client_py batch_users chat_messages__ek0_testing test_collection schema_users_client_py users scripts__ek0_testing websocket_test ttl_cache test_accounts chat_configurations__ek0_testing schema_employees_client_py demo_collection ws_ttl_test schema_products_client_py]

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
✓ Inserted document: Wf5KL0xVueg5myVivUP5GrtM0VED7Jk_SZEVb9HusRep58iIin1jhC5yxpAemSzLZ9-9ii8HGPUYxjBVPqc9ww

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: <nil>

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: pvuNPqBtvqDtUsKUZXQJ_EaVYzmWv9LAkXY-CoJ2R9TGJqg5F4pWgYCKxwTSqybuSPEhqUO7-anhjDlU_GxMgA

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
✓ Inserted document with TTL: tTCUtfcvOjaefWWxGT8r-uTjiuX4jBJuHraAhjYbBSOw4N5_zJ2tlWjcWmPdodphEogjylkUH409I1bXWAf3ZA

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

✅ Script saved: pWLLmv4AinBEnwggFzoRyFuM5M-sxkQ4tfsGe_R6KOU-Qh06I_QelaIs8K-xH1_HUi7MVxv1l_nn6y_HC_kmMw
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: kII-H96vZIex6YhSu12qcqQ4zO4x0aSVBE9x7sT3MNi9SY-seSNXNkK5lH-9X5wozEHKZ2fWLFjAEneziV5Ixg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: KggbccaX8Mrt4IkXgsUTyAaJGU2W-iuqTNMTzA7ypFbn_OVRvcuN_b1jhNVwpSopGNo-zmkfDftWM-op9gASXQ
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
=== Running transactions.go ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Y8dVaHLQ0_GrJfEvYNq-_jGcDTaCyPWg0ckVbyDiZ2vx70WDeoxC2c5vKwdxnwJSxwZeKSTQ1MsHWkqHwQ51hA
Created Bob: $500 - ID: X4jeL4iUIdAOk2AMLf-LZt3gPvT0Pmod3kb5RQ1iupeCN3QLhvLowWO4RUKL6A_YVgZWA08BxkTujYoCcGjRBA

=== Example 1: Begin Transaction ===
Transaction ID: 6bcbba6d-d19c-4517-a346-067bea458d3c

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
New transaction: d47d41cb-cd5f-4bbe-8173-143857b2dd4c
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
   ✅ Script saved: jdU2z_I6Xn0e_YY6nm3IQRCnabqllNQKDRjlrk7SSCFzkeFq-HA0Sm6DIoyy_2xQgJhK8Q3XwHp0dK8H1yTS1Q

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: hfo5FDbcaCOThwtHZUjdKTDMNAVqVCMSCS09GTyJD8qlMOe_Fax_3vzwpPwpYrix2EyujQh3lvuRfd-oXw_hwQ
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: kkSgiJ1XbYyu9nIfHjqE2cVw0NfbcUJFBIaDhctXy6IdrziXRttFOwtaBGxXLtaO97y-JWPIOrg4Gbr9WXtLDw

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
   ✅ Script saved: JEcFsI6jAAeAo23tpXopVx82EiDfpw7x62xeAMBr9UHhP_Pjoyr6niExjf5MJjf9lso_L8gUSHac3mecRXxTeg

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
   ✅ Script saved: GGHQx6wkSOaJyP9sr1vfRgJ9SCimWbUBmcydcmHw-u54ngjJY2xibu66vE2pR5Qea4AamoT12izaVoJgfhrYag

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: jdU2z_I6Xn0e_YY6nm3I...
   ✅ Deleted script: kkSgiJ1XbYyu9nIfHjqE...
   ✅ Deleted script: JEcFsI6jAAeAo23tpXop...
   ✅ Deleted script: GGHQx6wkSOaJyP9sr1vf...
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
Inserted: map[id:Cl4dcMupgQpOA9Dm7Cu3ial5XePtSz3UTm2x_5oqu7OeQ3YsKV_mylF8d6n6zwWd-crcaJxjFiVX7-SBofsI5w]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-04T06:32:45Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:Cl4dcMupgQpOA9Dm7Cu3ial5XePtSz3UTm2x_5oqu7OeQ3YsKV_mylF8d6n6zwWd-crcaJxjFiVX7-SBofsI5w metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-04 06:32:45 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-04T06:32:45Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:Cl4dcMupgQpOA9Dm7Cu3ial5XePtSz3UTm2x_5oqu7OeQ3YsKV_mylF8d6n6zwWd-crcaJxjFiVX7-SBofsI5w metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-04T06:32:45Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:Cl4dcMupgQpOA9Dm7Cu3ial5XePtSz3UTm2x_5oqu7OeQ3YsKV_mylF8d6n6zwWd-crcaJxjFiVX7-SBofsI5w metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

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
Collection created with first record: bcAx-Gxmaf6CoKnIkqRqTBHaMG--aAdbpW2ok2pN9T5JQqQKHQIEc25T5QDqlgorR4fdYQIX5eUpWyxfJXL0lg

=== List Collections ===
Total collections: 14
Sample collections: [schema_documents_client_py batch_users chat_messages__ek0_testing test_collection schema_users_client_py]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: PQ5PJuzxyCIGkE2KgcTucdpZUaS4Jsp6yKzH-6VjqGZAFDFjeeKSDWUFB69qPWuhztYNVw7hshuCbgbzr6b1YA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: eZj1mjbwa0E8PsyPjloRpiyoFGYavXJZIMu7Lb56DdmgSu-yR_ZyZ_GZwUm-2awINs1YCa8lF6zLLqnjXwT1-A

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
✓ Inserted test record: sz_RlPQXr_xJBLbHnZoiUzLFkVZ4aCgnWskFn88dcSw6UAHWejDSFBxYCCFZzU-qyhOelYsbrheyU0Nv6h_YCA

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
✓ Inserted document with TTL: aA8zTBFzNabVQm01skFAsAF_PSonHckucPUzxlHJC8Lq_T0S97hJuA-eimUXO9L9mGrO7z3zaf9PEDBXvYflEg

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
✓ Created session: iFUlZ1cqvlI-L5mwayMbK6rRYZHtvP0C1ZkjPMznuqaHhq4fUa3SisMopOjLCPYjrpobzAeyVmHK1WkSnbFhvQ

=== Sending Chat Message ===
Message ID: t1q88S88iC1ugcng3ig0N9Jp3azRz6dvNTp3vm6qwSKbAttKRNi8fEEjj0tANcMuLZKDlmdN8hjZ4os3j4bx0w

=== AI Response ===
Three products are available:

1. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

2. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

3. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:MnvqaOLriV1YRDi8k7ck_Er1nrr5TZ2loo1ge1nDuc8o0MpJ_l7UbIQn7ZEx0Nvv-Pm61sZiNwrQyf0E04Df4Q name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:ojUjjAKYXiqxpGG4toP70NhiDeVoLYPvjyPad3tlvr7CD2bCqCa1Ux9pmRKPUKoqv1B-7YEeI___JwBoSAYmPg name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:QFtzKenzRcIFKHrKi0VeSOGTV0b9pLz3smDWA55WTSP0H9TU-bV1iR7P-pI_2JJADlEl6g4W2OQZnnQm5gVAOQ name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 3041ms

=== Token Usage ===
Prompt tokens: 607
Completion tokens: 83
Total tokens: 690

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: BAfXzFTQQ1fyRUxlY_HZ0T5wLMlnNo2y5ZrgIRBDG8qIaDl9HssTKUdCrinchUSF9kJ9ZLJIY4Vhom3zZVrw_A

=== Sending Messages ===
✓ Message 1 sent
  Response: Answer: The available product is a high-performance database product named ekoDB. It is priced at 99 units.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: s2T0G4G1vr7a8KXYXB9LCWQ2IOuZVaYaY9pScz9sUoIk2rj2xAZDojVFN85Nl1ShrUeGBKQICfkWgNm8LpY0MQ
  Parent: BAfXzFTQQ1fyRUxlY_HZ0T5wLMlnNo2y5ZrgIRBDG8qIaDl9HssTKUdCrinchUSF9kJ9ZLJIY4Vhom3zZVrw_A

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: s2T0G4G1vr7a8KXYXB9LCWQ2IOuZVaYaY9pScz9sUoIk2rj2xAZDojVFN85Nl1ShrUeGBKQICfkWgNm8LpY0MQ (Untitled)
  Session 2: BAfXzFTQQ1fyRUxlY_HZ0T5wLMlnNo2y5ZrgIRBDG8qIaDl9HssTKUdCrinchUSF9kJ9ZLJIY4Vhom3zZVrw_A (Untitled)
  Session 3: iFUlZ1cqvlI-L5mwayMbK6rRYZHtvP0C1ZkjPMznuqaHhq4fUa3SisMopOjLCPYjrpobzAeyVmHK1WkSnbFhvQ (Untitled)
  Session 4: hIhtRI8S4wACiDzJQuUxJAcNLdKhXI3vCzy7Zx2uvCBYmr-lHtb8oxO_AQ4pU8uXdwKlEqOb0Gt7PX7MWaCmww (Untitled)
  Session 5: BGziR8-8gkMr75-xFjh9ze6J3YD333YrHqF2Rj1S2EGv9Nmw4wg9zAVCZqH3ftp7Snp1WZ2xdzgRPwCwswUE_g (Untitled)
  Session 6: rWDdGI5jTi6Tm3bG4At0VPgu8m0mVWOpMz-31zOK6dkYgequ9swcbxoeAfq68y86Op-7mF-ee8-n4vDcw5fGCA (Untitled)
  Session 7: ckT-vsSlrRFV67BvKFS30PxVBECc1qDvSkEeAZIHWGmSDdCBnk96BxjiLq-EJhaQMWUZQ-4ePmY9X4BckHHBMw (Untitled)
  Session 8: xi8NGvCFxkY8run1C7czBm8Zusrp3ulWQll5bARwCYBvRo77ERcVKgiJZg_PaNTblnAnCPp4cNn3XbX42Fahfw (Untitled)
  Session 9: AtwBzakJFSuDzeoN5C86fY2ZlXZBQj1kjD1Tjxl0iHObhCP8EL4aRdenEGal4OwDAUBIP79dvRi523bjUx4vTA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: s2T0G4G1vr7a8KXYXB9LCWQ2IOuZVaYaY9pScz9sUoIk2rj2xAZDojVFN85Nl1ShrUeGBKQICfkWgNm8LpY0MQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: J1NBTYJRQfNfowVBsBXX_0R2WfMfz10Ia0gvrsp_-Tx0zxkLcnbPq50veF1LMuYq3dfgHSBarNdogD87CMaZkw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, one product that is available is "ekoDB". It's a high-performance database product and is priced at $99.

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
✓ Created second session: VsNDCHqfYmt3Nf653dwidqprl7O2ITkbnM1QbK71O_rGwgwCv8SHLolCOC7Z0uRjJIeYv3eqkk8KCgzjJ_9_Iw
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
  1. Score: 0.769
  2. Score: 0.744
  3. Score: 0.729

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.698
  2. Score: 1.507
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

✅ Script saved: jtapenMzS7tAqtKp1q8z2hE60WTTAFwx1d8eH9-WzmNTGDQC5YoqUYPgaaHLTBmPgFg8ZoLgbJTC6P9fZ25h8w
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
Created Alice: $1000 - ID: mMSuGtLYnW1WqJPBST6grYPw-iV3ns892QljgLck4yVLAdQ9Faqkp5LWKBpUV5UgqnWAvtIDs6t4g21ejDd5QA
Created Bob: $500 - ID: LF65xWQokWhfWOCo9Dp9fcHUOfam_EgOkpiUKe_tATlbLxBCy16Yy9nhDrZCxGVjDCu03450xsuDWZph_g2F4w

=== Example 1: Begin Transaction ===
Transaction ID: a68cbe10-6d5c-4ffe-8f3f-1a9f01e4960b

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 36f1eb15-d590-46af-b9b8-c5b975fe33cf
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

added 1 package, removed 1 package, and audited 13 packages in 836ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'YLk29H1juS2IPSVqUjJczCJp7gZ3B53G8c0ZhREJJJscZWZSzf2HDbPkfAUj-SXu1Nesxfi6nwVtZApLhw0IUQ'
}

=== Find by ID ===
Found: {
  active: { value: true, type: 'Boolean' },
  price: { value: 99.99, type: 'Float' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  created_at: { value: '2026-01-04T06:32:59.457Z', type: 'DateTime' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  id: 'YLk29H1juS2IPSVqUjJczCJp7gZ3B53G8c0ZhREJJJscZWZSzf2HDbPkfAUj-SXu1Nesxfi6nwVtZApLhw0IUQ',
  value: { value: 42, type: 'Integer' },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  name: { type: 'String', value: 'Test Record' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-04T06:32:59.457Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  active: true,
  price: 99.99,
  categories: [ 'electronics', 'computers' ],
  created_at: '2026-01-04T06:32:59.457Z',
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  id: 'YLk29H1juS2IPSVqUjJczCJp7gZ3B53G8c0ZhREJJJscZWZSzf2HDbPkfAUj-SXu1Nesxfi6nwVtZApLhw0IUQ',
  value: 42,
  metadata: { nested: { deep: true }, key: 'value' },
  name: 'Test Record',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  tags: [ 'tag1', 'tag2', 'tag3' ],
  data: 'aGVsbG8gd29ybGQ='
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  price: { value: 99.99, type: 'Float' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  id: 'YLk29H1juS2IPSVqUjJczCJp7gZ3B53G8c0ZhREJJJscZWZSzf2HDbPkfAUj-SXu1Nesxfi6nwVtZApLhw0IUQ',
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  value: { type: 'Integer', value: 100 },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  created_at: { value: '2026-01-04T06:32:59.457Z', type: 'DateTime' },
  name: { type: 'String', value: 'Updated Record' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
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
Collection created with first record: MTGeaoKRAeftQBRFtozWNpuRyCG9lpLtdQgCN3utnR4Z6zHrota-Ftcgfcv7-6lSSJUEQlENgUoQ12D_6dMjNg

=== List Collections ===
Total collections: 18
Sample collections: client_collection_management_ts,schema_documents_client_py,schema_products_client_go,batch_users,chat_messages__ek0_testing

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: wmMyuQqlVCqrpu9Py2oYSSBtHO980Da4pgvpxxUXMRoV9RVvGAYUtfiJBDHB5UIS_GqI1LSbXti2m2kO1IIrHw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 6atOzEAUo_gFgRWf9JV0mFymWjRFyxTHaAgngralDuD2tzZdWRBvQ5bF1rFdDRlug0u_1vKEclWAu6Cd458rgQ

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
✓ Inserted test record: yat_5MBPwjXVfKjnjnPXu4gdDhg1KcWoflMwvgGbWJyd4b8Hk8VjdIEhOeqn_bj4Nlbbbq1Syn9DK5F_WXHtMA

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
✓ Inserted document with TTL: 0RhoQzRE4gX6aoLUFzQGGraOWvicZ3Wd5IgZJhSDU3QAzvis_h0Kw1XN92jdesuaWrSgQca9ipzhrwdGNFbDnA

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
✓ Created session: _Q3z9V4ral49KsLaZCPRhQLTypHlHhLrSiavxOY5Ft_uNeE_kCw0H3ReiELoGkEQ4exN3_SdDk00VedvAMNXdg

=== Sending Chat Message ===
Message ID: 9a7jKYyX8JJvvA3SUB1W3piIfLe0f6jY-Ow-ThSsHjwSM8sZqFt-vbksrhFYQZj9oxUKZn_2neMZMQD8imHXNg

=== AI Response ===
We have three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. It costs $99.

2. ekoDB Cloud: This is a fully managed cloud database service product. It costs $499.

3. ekoDB Pro: This is an enterprise edition product with advanced features. It costs $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    id: 'Eb7DvvgzW7OQ18vX7pckD8kGB6LWncMr-THb50Qwx0ObYexezM1yQnYdP9EshCwFOdJGiE9GDf_zoGgYEFZCiA'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'UkkK2L6tvg7FgqorKVEsAXUjrW0P0-S0l4fZIpGi9MODjvQbmtsy_GjO36KLuBIA-hMmlGRbieBJHHP71CYs3g',
    name: 'ekoDB Cloud',
    price: 499,
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Enterprise edition product with advanced features',
    price: 299,
    name: 'ekoDB Pro',
    id: 'KvFMvM81Jo_hv9A65DNfVQPGgj6n7eRWNB9YnFMXBHGHzImLDtovhe0wHFSfj-EHpm1Hjq_yFrv3RR8pIFhV5Q'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2493ms

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
✓ Created session: T8IFMGiKq7J04dr7uPnsXGcrdC4u0PqP8QH0M0J7Yqvf2zLSLGX3vDPzM5hCYkUqpjCSmpt1LtnB1YQlwhNxQA

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, one available product is "ekoDB," which is a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: tl-UXSRgx8q-H-PAZLxAPKwww-Iyp9gPeLdoHrczSWljeqMZrkpp7PGWkTLxVqc_AXzz6mbIwbrpP2HgodBCcg
  Parent: T8IFMGiKq7J04dr7uPnsXGcrdC4u0PqP8QH0M0J7Yqvf2zLSLGX3vDPzM5hCYkUqpjCSmpt1LtnB1YQlwhNxQA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: tl-UXSRgx8q-H-PAZLxAPKwww-Iyp9gPeLdoHrczSWljeqMZrkpp7PGWkTLxVqc_AXzz6mbIwbrpP2HgodBCcg (Untitled)
  Session 2: T8IFMGiKq7J04dr7uPnsXGcrdC4u0PqP8QH0M0J7Yqvf2zLSLGX3vDPzM5hCYkUqpjCSmpt1LtnB1YQlwhNxQA (Untitled)
  Session 3: _Q3z9V4ral49KsLaZCPRhQLTypHlHhLrSiavxOY5Ft_uNeE_kCw0H3ReiELoGkEQ4exN3_SdDk00VedvAMNXdg (Untitled)
  Session 4: VsNDCHqfYmt3Nf653dwidqprl7O2ITkbnM1QbK71O_rGwgwCv8SHLolCOC7Z0uRjJIeYv3eqkk8KCgzjJ_9_Iw (Untitled)
  Session 5: BAfXzFTQQ1fyRUxlY_HZ0T5wLMlnNo2y5ZrgIRBDG8qIaDl9HssTKUdCrinchUSF9kJ9ZLJIY4Vhom3zZVrw_A (Untitled)
  Session 6: iFUlZ1cqvlI-L5mwayMbK6rRYZHtvP0C1ZkjPMznuqaHhq4fUa3SisMopOjLCPYjrpobzAeyVmHK1WkSnbFhvQ (Untitled)
  Session 7: hIhtRI8S4wACiDzJQuUxJAcNLdKhXI3vCzy7Zx2uvCBYmr-lHtb8oxO_AQ4pU8uXdwKlEqOb0Gt7PX7MWaCmww (Untitled)
  Session 8: BGziR8-8gkMr75-xFjh9ze6J3YD333YrHqF2Rj1S2EGv9Nmw4wg9zAVCZqH3ftp7Snp1WZ2xdzgRPwCwswUE_g (Untitled)
  Session 9: rWDdGI5jTi6Tm3bG4At0VPgu8m0mVWOpMz-31zOK6dkYgequ9swcbxoeAfq68y86Op-7mF-ee8-n4vDcw5fGCA (Untitled)
  Session 10: ckT-vsSlrRFV67BvKFS30PxVBECc1qDvSkEeAZIHWGmSDdCBnk96BxjiLq-EJhaQMWUZQ-4ePmY9X4BckHHBMw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: tl-UXSRgx8q-H-PAZLxAPKwww-Iyp9gPeLdoHrczSWljeqMZrkpp7PGWkTLxVqc_AXzz6mbIwbrpP2HgodBCcg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Qpgf19xdg3pHQPz4KpFRgpRMkO-R0zPG7yIklfd6EmT5MZe3T6_vrKVzu7_FCmEDlDfVJr42eK5ZA3v_Fs9y6Q

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, we have the ekoDB, which is a high-performance database product. It is priced at $99.

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
✓ Created second session: kwBXnbkiLJGSxp87uoiu6Kt7FaM6KTRboe_ci49g690Wy9iswDroZRN9UzErRYVwFb7T3ldAzdK73WPxF3cCrw
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
  1. Score: 25.740, Matched: email, name.value, email.value, name
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio, bio.value, title
  2. Score: 26.400, Matched: title.value, title, bio, bio.value
  3. Score: 26.400, Matched: bio.value, bio, title.value, title
  4. Score: 26.400, Matched: title, title.value, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio, title.value, bio.value
  2. Score: 39.600, Matched: title, bio, title.value, bio.value
  3. Score: 39.600, Matched: title.value, bio, bio.value, title
  4. Score: 39.600, Matched: title.value, bio.value, bio, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.783, Matched: 
  2. Score: 0.761, Matched: 
  3. Score: 0.760, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.713, Matched: content, title.value, content.value, title
  2. Score: 1.504, Matched: title, title.value, content.value, content
  3. Score: 0.304, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, skills.value, bio, bio.value

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

✅ Script saved: M3eaDszLi_h_WVzd57NfSuQk5fRHEPAAvAgdf7YGg47Z579SjeEBfkfSJ_XaW_UPN5SasroPIe6BYkEDksZKRw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: DULeK047uGNjIVCBCxK8fJGwEdKW_qRQXSxoeg76xg-XOohcyQxUNCrId1jQ_5BcqdF3vI0HLcfBN4u3uOC33w
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: pyu6_c0SogozlSR0ZrODHz8XwR3D7c8NLwVLjUnw4w9olA9PXpLdk6gNJD8Hx5q4j4cOc0x0Ost_hx0IEmH9dQ
📊 Statistics: 2 groups
   {"status":"inactive","count":5,"avg_score":50}
   {"count":5,"avg_score":60,"status":"active"}

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

✅ Script saved: HoUFwxbkjCK-gTA8lKt5b1qL1wD6Z6CbCxYt-ePBUGnowPkv2HqV9RZ3aIRq4cGcT2FdnLKkLWg_6oFmFTXIUg
📊 Found 2 product groups
   {"category":"Furniture","count":2,"avg_price":474}
   {"category":"Electronics","avg_price":575.6666666666666,"count":3}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"count":2,"category":"Furniture"}
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
Created Alice: $1000 - ID: YcefyFgxatVBf1xzVKN_EbfIbhrar1BfspCR4uKFA7qXBp7_lZiGKK1AytENbTdC9QN1aArU_qcLBPTHt10OPA
Created Bob: $500 - ID: FplAbMOmvrY4oWsvk-d5FX9vQD0yYzPd22JLH8-uOygukK9UpkBpEp_YjRcdh1XsDoXuxUONIMNIWUo7VNtaqQ

=== Example 1: Begin Transaction ===
Transaction ID: 555ade7f-c448-452b-bf2a-e5d9aa3bf848

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 0af2a318-716c-4b70-99b5-9b84a2b17258
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mTypeScript client examples complete![0m
✅ [32mAll TypeScript integration tests complete![0m
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 991ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
(node:6053) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
(node:6056) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Inserted: {
  id: '85AtX5Wb5lKSOv0R8nycBd_HB1kzDp13N4G1b3-stnKzJ7AuprctHdYRaTbXhoEdmDJxiZZkxyO943nmZH--ew'
}

=== Find by ID ===
Found: {
  value: { value: 42, type: 'Integer' },
  name: { type: 'String', value: 'Test Record' },
  id: '85AtX5Wb5lKSOv0R8nycBd_HB1kzDp13N4G1b3-stnKzJ7AuprctHdYRaTbXhoEdmDJxiZZkxyO943nmZH--ew',
  active: { value: true, type: 'Boolean' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  id: '85AtX5Wb5lKSOv0R8nycBd_HB1kzDp13N4G1b3-stnKzJ7AuprctHdYRaTbXhoEdmDJxiZZkxyO943nmZH--ew',
  value: { value: 100, type: 'Integer' },
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
(node:6061) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted test record: muEECKr4iZ9jcK8ybmHjWSqSJfvxiG99pBBEmhzm4bXdJ-tE5goFE_eDB2IETcvlZeGzBk_FqKEhYI_A50QKgg

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
        "id": "muEECKr4iZ9jcK8ybmHjWSqSJfvxiG99pBBEmhzm4bXdJ-tE5goFE_eDB2IETcvlZeGzBk_FqKEhYI_A50QKgg",
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
        "id": "a3HSAeDJ4dkj4hErQeiyqFAv339ot0i90jIBizcekkcFWpdStzLfwrQA-ejOxfbPFxFuF9w977m4x7DpOIeeUA",
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
        "id": "z9LcWWNnG9qb3sE9I9ROC59IDx8hjNWn2hIqE9g6JS3nfKdpi4aLfs8iMwbAqZy7BhMTRTR6uqt3iSx83kVaIw",
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
✓ Retrieved 4 record(s) via WebSocket

✓ WebSocket example completed successfully
[32m✓ simple_websocket.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/batch_operations.js ===[0m
✓ Authentication successful

=== Batch Insert ===
(node:6074) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
(node:6077) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {
  loginTime: '2026-01-04T06:33:24.098Z',
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
(node:6080) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Collection created with first record: kmuijEmiUkof8O94g1pNJhrGi233YqPMMQF1UGyf9-zCiM6o4K71J0Xlldqeu3GtcliW5kh8YuquO6SzRruGxA

=== List Collections ===
Total collections: 23
Sample collections: [
  'schema_documents_client_py',
  'schema_products_client_ts',
  'schema_products_client_go',
  'batch_users',
  'schema_employees_client_ts'
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
(node:6094) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document: lnF4IkLAqcvvzQb1abbQEiVj8rOsEThCiLnYweCOjeOCJS-1GqOfbalBmFT64ireQK14sCqFefJiMbFDVAFQxQ

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: created

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: n3_u4F92HpeF6kOo1g6hiZg2DNmgw31yl_RwyXSmCD1uCqS_UqvJh5-wR2aWG2T82w0bXWIHXIC0a0J5R0wuEQ

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
(node:6097) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document with TTL: aGh43GD9b_dtbPmqFaBMZ6OM-nzJcHU68khMCAJ6aPLQZMoeJJpM2ldtt96kFdm9CmiChDk_jP7yBD6BIsVa5w

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

✅ Script saved: 2x0l4W1SLiuDovKBULs0NcxtMhCly-Qi8bTDmxpdDcmjFGadFJuApITNGuxumUl9gHon-8uKRKvuxORd0Wr8Qg
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: bFtwY3-W1Bg5rXL4CfMzwnBS9PSv0EH7j8RllOxrpFk0maed_tlhQj1_g1k2lMUMjWxIfAFAx6ftMyIjRrCAnA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: P7SW6WBXgrIMTtMpRa6aq8BSzgkWynGVs6U0Y-8vfSE-nX6LH1jYg44ie97zlpz0EGWnJu6miXaV6QVa-W7DIA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"max_score":100,"status":"active","avg_score":60,"count":10}
   {"count":10,"max_score":90,"status":"inactive","avg_score":50}

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
Created Alice: $1000 - ID: OXv4WZ8Ile04sWTgfoFQE3aa4tfhzY5-VkGXT41eaKrLjVBZT2QzVFXWHlK1I3pQC8MK4Sw2zHIgPdBZPg2dBw
Created Bob: $500 - ID: HUxYOHyN77EhGoPxjmR1gxQp4s27tfPIVycBRMpOrx00uBtqBiu7nF32tjcYJNZibfxA99aBJsUcyQsFUB5pUw

=== Example 1: Begin Transaction ===
Transaction ID: a1972fb2-1775-41bb-9187-73bafd39ecc5

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
New transaction: 537b36e9-fbd1-49d8-811c-7350b4243247
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
   ✅ Script saved: t9S4S1LHW_T1q15-uzuiAEkKqor0KaIWeFTXTgnwR6V1kvosTN74vDduX6ulDrDyCzeTKGThqhu6NbRT5MEvuQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Name: [object Object]
   📋 Email: {"value":"alice@example.com","type":"String"}
   📋 Status: [object Object]
   📋 Credits: [object Object]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: KCzZHzjjyzIMcc2GXfgPZJ8eyWt5yLEWDisQV4kyLpryDwq_gqqoSEP10Ut5PsdokomvpjTu9QV3r1YsdI3UVw

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
   ✅ Script saved: -IsIHyrxh5GrAUuVRekDCoHmhfX0-z0nZMG5ptwFtkLK9bRLEIjU4jsnwTTNAPpT50MrxSPMIZhBg53ZgB_rqg

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"value":0,"type":"Integer"}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: MB-pwX-AvMx4vovJxO7KpfX_xN454mXwsndjPPV4PWvD2ioqLPkucriGA4X6OOdLsqMn7sTnZEZdERKZvJXk_A

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: t9S4S1LHW_T1q15-uzui...
   ✅ Deleted script: KCzZHzjjyzIMcc2GXfgP...
   ✅ Deleted script: -IsIHyrxh5GrAUuVRekD...
   ✅ Deleted script: MB-pwX-AvMx4vovJxO7K...
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

added 1 package, removed 1 package, and audited 9 packages in 772ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'VWboFIkP7aDhsApB85jtSbtzJwjJdTCKVzhCgblfr-ZmUQYLBRKBTVXf5lt_uBP3fA5gqLuyhjp5vqZRPhJYEA'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  value: { type: 'Integer', value: 42 },
  id: 'VWboFIkP7aDhsApB85jtSbtzJwjJdTCKVzhCgblfr-ZmUQYLBRKBTVXf5lt_uBP3fA5gqLuyhjp5vqZRPhJYEA',
  active: { type: 'Boolean', value: true }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  active: { type: 'Boolean', value: true },
  id: 'VWboFIkP7aDhsApB85jtSbtzJwjJdTCKVzhCgblfr-ZmUQYLBRKBTVXf5lt_uBP3fA5gqLuyhjp5vqZRPhJYEA',
  name: { type: 'String', value: 'Updated Record' },
  value: { value: 100, type: 'Integer' }
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
Collection created with first record: lJnMmG5x2SQzsyEj7C1p4cleOJE7jCfOikO-gs8pZHwIHNMCMOGgagpnBxDcftkOrvfmYH6BNoUYZdFEmMpyPQ

=== List Collections ===
Total collections: 22
Sample collections: schema_documents_client_py,schema_products_client_ts,schema_products_client_go,batch_users,schema_employees_client_ts

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: hMRCtReQscKHNHVuH9GlQH0hXuLD0Nkps0dAQW1qo4PBJW3YFgcIwWfFgkGJqL_XHjCDDV0_5pUSNILkpcdHjQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: DkVzau5S2WC61Oat2SVXKg909TJpsosbzsQWdeJczTyNqXTUtHl_lmYt8ZjbaqQ_ANj7R56AcXLFr9Fjsk1oeg

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
✓ Inserted test record: bE-cZ1uRa9w4AmpX4nsnveajSQQ364Q25TAd8wnPEmpFl48GRIXOLykMWiPquZyqK0s9VpAj42nZPkidktVAgw

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
✓ Inserted document with TTL: Vy-hNqH_i7jw7kutDjuaSKmCU0DI3qA8NMpI6r3HXmhOvGKHbN-oJkleo1YPPCMqG8OkqbYsKnEEO4kz0GCryA

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

✅ Script saved: wDLZJeZQoYtEqgsRADseirHlHVY-Tb48XPc5Layp4EmGCG9VJahaSRTRUJ-hB71RYd0DVicylNEdJxDJg8k_Mw
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: lGY_uQHuy-IClbQLoqvuDbJ55jWsY4M8ra4jtf85VKypAzB5n9_8nSFLNRCwJ_ufyrCJxVCFOD5sFUYM7T5LJQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: LiKUjLZeriwUIebNfgMX-O6HuZkxMrMV7WF0VgkzFJxtC4EW3g2zvAlx9ReGG8lY5pJyldQbXqKi8iCu9cL4sQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":"active","max_score":100,"avg_score":60,"count":5}
   {"max_score":90,"status":"inactive","count":5,"avg_score":50}

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

✅ Script saved: pLehHpCGlCug2dCMGm4_18nDODevk8SsS0Qw_cbzbBKM6OLFMsr8O1C9dhIRXLewsYuLHEHJpV-3f2CkweQehw
📊 Found 20 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 20 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"count":10,"avg_score":60,"status":"active"}
   {"avg_score":50,"status":"inactive","count":10}
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
   1. Desk Chair - $349 (⭐4.2)
   2. Bookshelf - $149 (⭐4.1)
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
   2. Natural Language Processing (AI)
   3. Vector Databases Explained (Database)
   4. Getting Started with ekoDB (Database)
   5. Database Design Principles (Database)
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
   2. Natural Language Processing
   3. Vector Databases Explained
   4. Getting Started with ekoDB
   5. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Natural Language Processing
   3. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Introduction to Machine Learning (AI)
   2. Natural Language Processing (AI)
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
   Vector databases offer several benefits including:

1. High Precision: They store data in points, lines, and polygons, allowing for the representation of geographical features with high accuracy and detail.

2. Scalability: Vector data can be easily scaled up or down without loss of quality or resolution.

3. Small Size: They generally consume less storage space compared to raster databases.

4. Attribute Storage: Vector databases can store attribute information such as names, categories, or other characteristics, directly linked to each vector feature.

5. Better Analysis: They are suitable for advanced spatial analysis and operations such as network analysis, proximity analysis, and topological relationships.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Getting Started with ekoDB" - 1536D vector
   2. "Advanced Query Patterns" - 1536D vector
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
Created Alice: $1000 - ID: g1D5KCmIF6wjK6ppOG7WAUrXRNtfXpu1TLnw8wneIkeLtWZAOZb4ZaulhMVG0mS3cWCDreoyLVz-kANCeNVKOg
Created Bob: $500 - ID: YZrjj0lSZo5FYKoGveSjHXhO57M2IbZ1N7kplaSh0l2iDmbeiD_EWa2ZNxAgU0mThs24rgQb2zD_58yZhT1HzQ

=== Example 1: Begin Transaction ===
Transaction ID: 264e345e-5c5a-4ab2-ac82-785ef56ec501

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 9f5e0de0-76f4-488a-b313-333fe834109b
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
✓ Inserted user: Record(fields={id=StringValue(value=HsW7ubaL7_cQSOC8rK2KJthdj_HrOukDPta1uQcxuw3Uc7OI7yUsjinkrs0c3A6h59F8drG1lPbNcteyHP0U6w)})
  User ID: HsW7ubaL7_cQSOC8rK2KJthdj_HrOukDPta1uQcxuw3Uc7OI7yUsjinkrs0c3A6h59F8drG1lPbNcteyHP0U6w

=== Read ===
✓ Found user by ID: Record(fields={id=StringValue(value=HsW7ubaL7_cQSOC8rK2KJthdj_HrOukDPta1uQcxuw3Uc7OI7yUsjinkrs0c3A6h59F8drG1lPbNcteyHP0U6w), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), created_at=ObjectValue(value={value=StringValue(value=Sun Jan 04 01:33:58 EST 2026), type=StringValue(value=String)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=28)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Sun Jan 04 01:33:58 EST 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: [id, user_id, active, email, embedding, name, price, data, tags, created_at, age, metadata, categories]

=== Update ===
✓ Updated user: Record(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), created_at=ObjectValue(value={value=StringValue(value=Sun Jan 04 01:33:58 EST 2026), type=StringValue(value=String)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), id=StringValue(value=HsW7ubaL7_cQSOC8rK2KJthdj_HrOukDPta1uQcxuw3Uc7OI7yUsjinkrs0c3A6h59F8drG1lPbNcteyHP0U6w), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)})})

=== Query ===
✓ Found 1 users matching query
  - Record(fields={price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), id=StringValue(value=HsW7ubaL7_cQSOC8rK2KJthdj_HrOukDPta1uQcxuw3Uc7OI7yUsjinkrs0c3A6h59F8drG1lPbNcteyHP0U6w), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Sun Jan 04 01:33:58 EST 2026)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)})})

=== Delete ===
✓ Deleted user with ID: HsW7ubaL7_cQSOC8rK2KJthdj_HrOukDPta1uQcxuw3Uc7OI7yUsjinkrs0c3A6h59F8drG1lPbNcteyHP0U6w

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

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
  IDs: JJPY3XVWSUKQb_RnLEvSLHR3vKh-CPSjJ6udTG1nfFv21h5wmADU8zMIfm8EocRcn538EcWJy-3F3S1G5KZ40w, fzg9MUtJZ7ZWTGrwys3yE9X5eZlIi2vdvSGsSP6YEIopQwDxk9vA3qjccfIJpGSzFL1cRAzV8Ll7u3fWXyTlwg, U7tSjj1Ay2W2F2rtqkc9jKcav_DyEANVxZE_MyT4vu9jExj3BFrrE6RzqJ_M_cXqVU0vxovkK9WUkWb5YITFIg...

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
  - ObjectValue(value={value=StringValue(value=Charlie), type=StringValue(value=String)})
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
✓ Session value: {"user_id":"123","created_at":1767508452075}
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
  - schema_documents_client_py
  - schema_products_client_ts
  - schema_products_client_go
  - batch_users
  - schema_employees_client_ts

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-01-04T06:34:16.322583Z","last_modified":"2026-01-04T06:34:16.322584Z","bypass_ripple":false,"primary_key_alias":"id"}

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
  Document ID: t6pYjAzKbL9arxFRk2ZiQUG2uhd4bXu53g6fbgEDoaaENnaJJFo46tRtPKubYqEiQvMA9--OeLggMZpbLwi2_w

=== Verify Document Exists ===
✓ Document found: [session_id, created_at, user_id, id, ttl]

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: JxhPeZgRm5_CuqjWAEQY9UF7yqXzOqW9fvlbu964ANcTU2AOIVUb8f2B9cU2D9_xnvsRR_dmcmYBEmYYYz7RKQ

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: [value, cache_key, id, ttl]

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
  {"payload":{"data":[{"id":"42FfHcn4lCSxf502tM2axQH2TW_snYd4_pcTBNobjp9t2SyTyx45ESVGHRG7YGfWa3ZctXpMG5EVWH7InSCcIA","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

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
✓ Inserted user 1: StringValue(value=xiDIgIFaLNtwS1vKuKJpt1_yRuVhAredVdSow3uSANsK_H1_GpnfjJo3WLkTf_K6WA1yN0UMY0GlsrZtwV3NXA)
✓ Inserted user 2: StringValue(value=x1-E2ZapXftCRS0hhGsfAoMcwJ_GzBMaeBUIL1cgohfjJR1UxBhl4is1oaqTeBmBTetTRvXrskuss9wW8buYXw)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 22
  Sample: [schema_documents_client_py, schema_products_client_ts, schema_products_client_go, batch_users, schema_employees_client_ts]

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
  {"results":[{"record":{"id":"rkQ0Ex_Hn-rHV1VVpqAVA-gbYg8wa-5zTVgvdJCUS7qrPkKiTl5vAPmJuwc6z64OFIf_4MKDVaxNKU1vCMsHrA","description":"Learn Rust programming language with hands-on examples and best practices.","views":297,"tags":"programming,rust,tutorial","title":"Rust Programming"},"score":19.8,"matched_fields":["description","tags","title"]},{"record":{"views":320,"description":"Build modern web applications using JavaScript, React, and Node.js.","title":"JavaScript Web Development","id":"-j33icJkVjXbbTM407Jc3rdoSVuO3jHuqYI5Fce6a2NfqlwG0IMR_Wgnug4DjgT8neH5teQof-XDd9H6wqTm5Q","tags":"programming,javascript,web"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"tags":"programming,python,data-science","title":"Python for Data Science","description":"Master Python for data analysis, machine learning, and visualization.","id":"6Dz7jzwgPjD5ZEk84iHZwQPZbdLyQ70xo7pfRTUv5Y7Eys4dMlLfyGBPu3yCoHE9khDwprKMiBr-8n7i_T04jw","views":339},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"id":"Dx8kEOeSwkxerYSFtsV3qRPcfbaNwJfAKkSxSoc7J3UG2hTZcRkERGYQD0mZRF-tMidY2tBVL1tLhcbeUtgoZw","views":622,"title":"Machine Learning Basics","tags":"ai,machine-learning,python","description":"Introduction to machine learning algorithms and neural networks."},"score":2.7,"matched_fields":["title","tags","description"]},{"record":{"id":"6Dz7jzwgPjD5ZEk84iHZwQPZbdLyQ70xo7pfRTUv5Y7Eys4dMlLfyGBPu3yCoHE9khDwprKMiBr-8n7i_T04jw","title":"Python for Data Science","tags":"programming,python,data-science","description":"Master Python for data analysis, machine learning, and visualization.","views":339},"score":1.0,"matched_fields":["description"]},{"record":{"id":"HH90TGXY7Jo3Dy_2N10Qfdz-G97GbWGqKdvHcGHk1wFB8G3m_8ZUboVM9ZWnhidOaaJ04Av6LV1Kls0c8Zu5qA","title":"Database Design","views":74,"tags":"database,design,sql","description":"Learn database design principles, normalization, and query optimization."},"score":0.5,"matched_fields":["description"]},{"record":{"views":297,"id":"rkQ0Ex_Hn-rHV1VVpqAVA-gbYg8wa-5zTVgvdJCUS7qrPkKiTl5vAPmJuwc6z64OFIf_4MKDVaxNKU1vCMsHrA","tags":"programming,rust,tutorial","description":"Learn Rust programming language with hands-on examples and best practices.","title":"Rust Programming"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
✓ Inserted document with TTL: StringValue(value=s-lBh7OMpqCAbY-jcKrtenGk_PtpBaD5aecDed1gsCwnY5Fz3DfHPO1ZufJuqlXNUdpi8V_NMrTQWbvFRYVhCQ)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1767508477870},"id":"s-lBh7OMpqCAbY-jcKrtenGk_PtpBaD5aecDed1gsCwnY5Fz3DfHPO1ZufJuqlXNUdpi8V_NMrTQWbvFRYVhCQ","name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-01-04T07:34:37.012318Z","value":{"type":"Integer","value":42}}]},"type":"Success"}

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
✓ Created session: AoGV2ETq9ih_wHDgmEjg_EgzNgujHFIKDL8uN3J9rzARdCJa5tJMwlD7OuncU176W7jsmB-2x1lKf55SS8OJSw

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "RnZ8sCr2cXX4AwOe1uniXjQZsmvHelfMcl5Udw0IqAK80PSR8r4_pSloE41IiNwR-b2b8X48jxxOY4BFsocYBw"
  Responses: ["ekoDB is a high-performance database known for its intelligent caching, real-time capabilities, and AI integration. It provides several unique features to enhance the user experience and efficiency. One of the features is the AI Chat Integration, which allows you to query your database using natural language and receive AI-powered responses with relevant context. This integration makes it easier to interact with the database while ensuring accurate results. Additionally, ekoDB supports various search features, including full-text search, vector search, and hybrid search with automatic context retrieval. These features facilitate efficient and precise data retrieval from the database."]

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
✓ Created session: hT1lAcig8obFH6P8SazVCEQEthVjaOwrnLF_uMnrQ1wx3RSRGB4020Q_MYyS_ISMkfTQkdjLs02i3ri6cDqSCw

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The product available is ekoDB, a high-performance database product with AI capabilities. It is priced at $99."]

✓ Message 2 sent
  Responses: ["The price of the ekoDB product is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"hT1lAcig8obFH6P8SazVCEQEthVjaOwrnLF_uMnrQ1wx3RSRGB4020Q_MYyS_ISMkfTQkdjLs02i3ri6cDqSCw"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"saZp9sqjExiwthQgIGaqXKIof5DRyUeKziz6jsK5Pz2Fb34fqQg55rEEr03G2c8fIwisARqCywYvrbytBiZGsA","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-04T06:34:55.918241Z"},"id":"e1_FEZJLxgghWcxjAlQJEdHZiq6DTUnNwyf9OIqu86Pv5Y4e3YzzFWZgu1QtPvWMB5y3cSgNX97n9PygnWoaUg","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":24,"prompt_tokens":239,"total_tokens":263}},"updated_at":{"type":"DateTime","value":"2026-01-04T06:34:55.918241Z"}},{"chat_id":{"type":"String","value":"hT1lAcig8obFH6P8SazVCEQEthVjaOwrnLF_uMnrQ1wx3RSRGB4020Q_MYyS_ISMkfTQkdjLs02i3ri6cDqSCw"},"content":{"type":"String","value":"The product available is ekoDB, a high-performance database product with AI capabilities. It is priced at $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"saZp9sqjExiwthQgIGaqXKIof5DRyUeKziz6jsK5Pz2Fb34fqQg55rEEr03G2c8fIwisARqCywYvrbytBiZGsA","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-04T06:34:55.920772Z"},"id":"au04CoJKShHyHo9bfdk-7xlBeHD7lUVQxAbPobVCBEmmh-zCUniLCShURBItTY2jguBoY5Tj0nJECwTLyhKD6Q","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":24,"prompt_tokens":239,"total_tokens":263}},"updated_at":{"type":"DateTime","value":"2026-01-04T06:34:55.920772Z"}},{"chat_id":{"type":"String","value":"hT1lAcig8obFH6P8SazVCEQEthVjaOwrnLF_uMnrQ1wx3RSRGB4020Q_MYyS_ISMkfTQkdjLs02i3ri6cDqSCw"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"saZp9sqjExiwthQgIGaqXKIof5DRyUeKziz6jsK5Pz2Fb34fqQg55rEEr03G2c8fIwisARqCywYvrbytBiZGsA","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-04T06:34:56.943074Z"},"id":"PvlEiUjSeMCyTBumjxx4CAcwth41SbsLGJMrgUqvcuKUdEpfpsWln4yLd4IxBUc5MV3DIyUALExnE3IchXO0VQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":239,"total_tokens":251}},"updated_at":{"type":"DateTime","value":"2026-01-04T06:34:56.943074Z"}},{"chat_id":{"type":"String","value":"hT1lAcig8obFH6P8SazVCEQEthVjaOwrnLF_uMnrQ1wx3RSRGB4020Q_MYyS_ISMkfTQkdjLs02i3ri6cDqSCw"},"content":{"type":"String","value":"The price of the ekoDB product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"saZp9sqjExiwthQgIGaqXKIof5DRyUeKziz6jsK5Pz2Fb34fqQg55rEEr03G2c8fIwisARqCywYvrbytBiZGsA","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-04T06:34:56.945827Z"},"id":"unUhf1xQ2zmo8HdHC-PrNuBkZQYRpTXHw3o_gzt6KCRKfBEAiLe-8-jPk3sgbto12KyBwgW9DB7bS2lpwcxWqw","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":239,"total_tokens":251}},"updated_at":{"type":"DateTime","value":"2026-01-04T06:34:56.945827Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 13

=== Branching Session ===
✓ Created branched session: -qOqrDapUmDpvxL40unnfIqVK_dRWujHqNqwUwprl3EDaHj4U86WA1NM4TQejZsve3vQHw20BuDYiO8eUh99XA

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
✓ Created session: oy0Lo2TqtLhfKIlRzvnhS2xo3LAY5e-92LJWKt4H2JnqDyNaUFN7nLEP5__4o6D6GZECMofZVMHoDEHJsq28Zg

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the provided context, the available product is \"ekoDB\", a high-performance database product. It is priced at 99."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Based on the context provided, the available product is \"ekoDB\", which is a high-performance database product. It costs $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: QF_Zw_2Yu6RRp-Ricv76ErmMbErtehEj-G8178313rJ6PelLuImC-4S1U_J-paVvS2orrGleiVJ3Bm94KhHg8w

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

✅ Script saved: 9Pp6l4RX_kIji9rYWKkBa0fkfZ_EBgBkSScAJ59oScJuuILq9BfxDcuhdVfxetCuDNnGDSn2PuZKobFsFFBajw
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"count":5,"avg_score":60.0,"status":"active"}
   {"status":"inactive","avg_score":50.0,"count":5}
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
Created Alice: $1000 - ID: EZDL1oLbDdDbTGSOjcba_hzrXxHlvevz0ROSP_KQ_VCdCxWk0rsDmQAz6PHaKgXqeNCSYfwjP2I2fRU22MCayg
Created Bob: $500 - ID: iiF7sQFbuyslaO_m4PJ4OpeORH5WnDEfstbUKZLXdCVRGIry3SbHuPmBBfiiIW54c1I2Pd2JQMubTZKEaBZuEw

=== Example 1: Begin Transaction ===
Transaction ID: e37789f3-3c13-45a9-8e8d-a019d20024e1

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: fffa2fa3-a93d-4cf7-878c-fdf13c07805d
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
    ✓ Generated embedding: 1536 dimensions in 208.77725ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 254.343125ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 244.90075ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 240.68825ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 234.642042ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 943.453459ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 609.7295ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 508.168167ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.028662458s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 348.257667ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 596.014167ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 286.876292ms
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
    ✓ Generated embedding: 1536 dimensions in 258.7915ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 95.57025ms

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

Writing memory-safe, high-performance database code involves several principles and techniques:

1. **Choose the Right Database Type**: Based on previous conversations, we know that different types of databases (NoSQL vs SQL) are suited for different types of tasks. NoSQL is better for flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships. So, choosing the right database based on your application requirements can lead to better performance.

2. **Normalization**: Normalize data to reduce redundancy and improve data integrity. As mentioned before, this involves dividing large tables into smaller ones and defining relationships between them using foreign keys. However, keep in mind that over-normalization can also lead to performance issues due to join operations.

3. **Optimize Queries**: Write efficient SQL queries to speed up database performance. This can involve techniques like:
   - Using indexes on columns that are frequently queried.
   - Avoiding SELECT * and instead specifying the columns you need.
   - Using JOINs wisely and avoiding Cartesian products.
   - Using LIMIT to restrict the amount of data that your query returns.

4. **Memory Management**: In terms of ensuring memory safety, it depends on the language you're using to interact with the database. If you're using a language like C++, it's important to properly manage memory to avoid leaks. In languages like Java and Python, the garbage collector manages memory, but you should still be mindful of creating unnecessary objects.

5. **Connection Pooling**: Creating a new connection for every database operation is resource-intensive. Connection pooling allows you to reuse existing connections, reducing the overhead of establishing a new connection every time.

6. **Caching**: Implementing a caching system can drastically improve the performance of your database by keeping frequently accessed data in a cache, reducing the need for database calls.

Remember, the best methods for optimizing your database will depend on your specific use-case, data, and system architecture.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 414.796209ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2221 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 492.418375ms
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
  ✓ Text search completed in 53.751958ms

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_new_question: Writing memory-safe, high-performance database code involves several principles and techniques:

1. **Choose the Right Database Type**: Based on previous conversations, we know that different types of databases (NoSQL vs SQL) are suited for different types of tasks. NoSQL is better for flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships. So, choosing the right database based on your application requirements can lead to better performance.

2. **Normalization**: Normalize data to reduce redundancy and improve data integrity. As mentioned before, this involves dividing large tables into smaller ones and defining relationships between them using foreign keys. However, keep in mind that over-normalization can also lead to performance issues due to join operations.

3. **Optimize Queries**: Write efficient SQL queries to speed up database performance. This can involve techniques like:
   - Using indexes on columns that are frequently queried.
   - Avoiding SELECT * and instead specifying the columns you need.
   - Using JOINs wisely and avoiding Cartesian products.
   - Using LIMIT to restrict the amount of data that your query returns.

4. **Memory Management**: In terms of ensuring memory safety, it depends on the language you're using to interact with the database. If you're using a language like C++, it's important to properly manage memory to avoid leaks. In languages like Java and Python, the garbage collector manages memory, but you should still be mindful of creating unnecessary objects.

5. **Connection Pooling**: Creating a new connection for every database operation is resource-intensive. Connection pooling allows you to reuse existing connections, reducing the overhead of establishing a new connection every time.

6. **Caching**: Implementing a caching system can drastically improve the performance of your database by keeping frequently accessed data in a cache, reducing the need for database calls.

Remember, the best methods for optimizing your database will depend on your specific use-case, data, and system architecture.

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
    ✓ Generated embedding: 1536 dimensions in 0.249s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.024s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.556s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.459s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.229s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.221s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.283s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.559s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.254s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.317s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.294s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.274s
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
    ✓ Generated embedding: 1536 dimensions in 0.292s
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

Writing memory-safe high-performance database code involves a range of good practices, here are some of them:

1. **Normalization:** As defined in context 3, normalization is a process that helps reduce data redundancy and improve data integrity. Proper normalization can help optimize the speed and efficiency of most of your database operations.

2. **Optimize Queries:** The performance of a database is often limited by the efficiency of the queries it runs. You can optimize queries by using indexing, avoiding unnecessary columns in SELECT statements, limiting the use of wildcards, and using JOINs wisely.

3. **Choose the Right Database Type:** Depending on your needs, either SQL or NoSQL may be more appropriate (as described in context 1 and 4). SQL databases are good for structured data with complex queries and ACID transactions, while NoSQL databases are better for flexible schemas, horizontal scaling, high write throughput, and unstructured data.

4. **Use Prepared Statements:** Prepared Statements are used to execute the same or similar database statements repeatedly with high efficiency. They also help avoid SQL injection attacks, making your code more secure.

5. **Memory Management:** Be careful with memory allocation and deallocation. Unnecessary memory usage can lead to memory leaks which can slow down your database performance. Make sure to free up memory that is no longer needed.

6. **Concurrency Control:** Efficient handling of multiple simultaneous operations can significantly improve performance in a multi-user and multi-transaction environment. Techniques can include locking, multiversion concurrency control (MVCC), and transaction isolation levels.

7. **Caching:** Implement caching strategies to store frequently accessed data in memory for faster retrieval.

8. **Batch Processing:** Instead of writing or updating data one record at a time, consider batch processing. This can be much faster for large amounts of data.

Remember, the specifics of how you implement these will depend on the specific database system you are using.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.438s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2079 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.186s
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
    ✓ Generated embedding: 1536 dimensions in 0.209s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.408s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.231s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.272s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.302s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.222s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.303s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.443s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.261s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.298s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.204s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.223s
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
    ✓ Generated embedding: 1536 dimensions in 0.201s
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
     What is database normalization?

  3. [Score: 0.000] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  4. [Score: 0.000] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.000] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves several strategies:

1. **Use Efficient Queries**: As discussed in Context 5, optimizing your queries is crucial for high performance. Avoid using SELECT * queries; instead, specify the exact fields you need. Use JOINs sparingly and wisely, as they can be expensive. Index your tables appropriately based on the fields you most commonly query on.

2. **Normalize Your Database**: Referencing Context 1, database normalization can help reduce redundancy and improve data integrity, which can lead to better performance. It makes your database leaner and more efficient by ensuring data is not duplicated unnecessarily.

3. **Choose the Right Database**: Depending on your needs, the choice between SQL and NoSQL (as discussed in Context 3 and 4) can impact performance. SQL databases are great for structured data and complex queries, while NoSQL databases are more suited for unstructured data, high write throughput, and horizontal scaling.

4. **Manage Connections Efficiently**: Connections to databases are expensive to create and maintain. Use connection pooling to reuse existing connections rather than establishing a new one every time.

5. **Memory Management**: In terms of memory safety, it's crucial to avoid memory leaks in your code. This can be achieved by proper resource allocation and deallocation, and by utilizing garbage collection where applicable. 

6. **Use Prepared Statements**: These can not only improve performance by reducing the time needed to execute the same query multiple times, but they also help prevent SQL injection attacks, thereby improving the safety of your code.

7. **Use Batch Processing**: If you need to perform a large number of similar operations (like INSERTs or UPDATEs), doing them in a batch rather than individually can significantly improve performance.

Remember, the best practices can vary depending on the specific database system (like MySQL, PostgreSQL, MongoDB, etc.) and the programming language you're using. Always refer to the relevant documentation for the best practices.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.300s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2101 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.351s
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
    ✓ Generated embedding: 1536 dimensions in 0.276s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.300s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.318s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.505s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.223s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.228s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.228s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.277s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.423s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.715s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.251s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.257s
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
    ✓ Generated embedding: 1536 dimensions in 0.350s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.091s

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

Writing memory-safe, high-performance database code involves several key considerations:

1. **Optimize your Queries**: As mentioned in Context 5, optimizing database queries is crucial for high performance. Use indexing appropriately, avoid full table scans, limit the data you're fetching, and write efficient joins. Also, use the EXPLAIN command to understand how your queries are being executed and where you can make improvements.

2. **Normalization**: As noted in Context 1 and 2, database normalization reduces redundancy and improves data integrity. It can also enhance the performance of some types of queries. 

3. **Choose the Right Database Type**: As pointed out in Context 3 and 4, choosing between SQL and NoSQL depends on your specific needs. SQL databases are better for complex queries and structured data, while NoSQL databases excel at handling unstructured data, scaling horizontally, and providing high write throughput.

4. **Memory Management**: Ensure your code is memory-safe by properly managing your database connections. Make sure to close connections when they're no longer needed to free up resources. Also, use appropriate data structures to store data in memory and avoid memory leaks.

5. **Concurrency Control**: Use transactions where necessary to ensure data consistency and integrity. SQL databases follow ACID properties (Atomicity, Consistency, Isolation, Durability) as mentioned in Context 3.

6. **Caching**: Cache frequently accessed data to reduce the load on your database and increase performance. But, be careful to manage cache invalidation correctly to ensure data consistency.

7. **Batch Processing**: If you are writing or reading large amounts of data, it might be more efficient to do it in batches rather than one record at a time.

8. **Monitoring and Profiling**: Regularly monitor and profile your database performance. This will help you identify slow queries or memory issues and fix them proactively.

9. **Use Prepared Statements**: Prepared statements not only help in preventing SQL injections but can also improve performance as they're precompiled and reusable.

Remember, each application has unique requirements, so there are no one-size-fits-all solutions. Always profile and test different approaches to find what works best for your specific situation.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.319s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2325 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.325s
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
  ✓ Text search completed in 0.041s

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

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
    ✓ Generated embedding: 1536 dimensions in 0.353s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.248s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.258s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.26s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.361s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.326s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.261s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.282s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.468s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.309s
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
    ✓ Generated embedding: 1536 dimensions in 0.242s
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
    ✓ Generated embedding: 1536 dimensions in 0.308s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.088s

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
    ✓ Generated embedding: 1536 dimensions in 0.221s
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
  ✓ Text search completed in 0.044s

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

> @ekodb/ekodb-client@0.7.1 prepare
> npm run build


> @ekodb/ekodb-client@0.7.1 build
> tsc


up to date, audited 58 packages in 2s

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
✓ Created SWR script: fetch_github_user (OwAtWfVa5ReSIWv0koPBBcHYbq2rjzU0Wio9oPWzrzxTVDa9bs-81Elf1iaE15XVnxosUpd30dCdqPUJ_2W5xQ)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [
    {
      "data": {
        "type": "Object",
        "value": {
          "node_id": "MDQ6VXNlcjEwMjQwMjU=",
          "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
          "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
          "repos_url": "https://api.github.com/users/torvalds/repos",
          "followers": 269793,
          "user_view_type": "public",
          "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
          "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
          "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
          "company": "Linux Foundation",
          "login": "torvalds",
          "site_admin": false,
          "received_events_url": "https://api.github.com/users/torvalds/received_events",
          "name": "Linus Torvalds",
          "twitter_username": null,
          "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
          "gravatar_id": "",
          "location": "Portland, OR",
          "id": 1024025,
          "hireable": null,
          "html_url": "https://github.com/torvalds",
          "updated_at": "2025-11-24T04:16:14Z",
          "public_repos": 9,
          "bio": null,
          "created_at": "2011-09-03T15:26:22Z",
          "email": null,
          "organizations_url": "https://api.github.com/users/torvalds/orgs",
          "public_gists": 1,
          "followers_url": "https://api.github.com/users/torvalds/followers",
          "type": "User",
          "following": 0,
          "blog": "",
          "url": "https://api.github.com/users/torvalds"
        }
      },
      "id": "WH4r85C_1w6jVeN5xrpqJrjWRNYN5wPSoZeaXTbJVjxjunTBRluTGNz-Cb3kLiq63M0e3lsojLWZKYrbfvFZVA",
      "cached_at": "2026-01-04T06:36:29.173Z"
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
Response time: 81ms (served from cache)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (TbxnOb3ubCGQQc1KfCFIrR68rUqbKT7CQbFujkakAf0OsuskHRJmwgnJOUkMWLMLlfKIFMj3vDj9ufd865krGQ)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "enriched_at": "2026-01-04T06:36:29.460Z",
      "enriched_data": {
        "type": "Object",
        "value": {
          "reviews": [
            {
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "rating": 3,
              "reviewerName": "Eleanor Collins",
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Would not recommend!"
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Very satisfied!",
              "reviewerName": "Lucas Gordon",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "rating": 4
            },
            {
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Eleanor Collins",
              "comment": "Highly impressed!",
              "rating": 5
            }
          ],
          "shippingInformation": "Ships in 3-5 business days",
          "meta": {
            "createdAt": "2025-04-30T09:41:02.053Z",
            "barcode": "5784719087687",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "updatedAt": "2025-04-30T09:41:02.053Z"
          },
          "dimensions": {
            "height": 13.08,
            "depth": 22.99,
            "width": 15.14
          },
          "title": "Essence Mascara Lash Princess",
          "sku": "BEA-ESS-ESS-001",
          "returnPolicy": "No return policy",
          "weight": 4,
          "minimumOrderQuantity": 48,
          "stock": 99,
          "availabilityStatus": "In Stock",
          "discountPercentage": 10.48,
          "rating": 2.56,
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "warrantyInformation": "1 week warranty",
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "brand": "Essence",
          "price": 9.99,
          "id": 1,
          "category": "beauty",
          "tags": [
            "beauty",
            "mascara"
          ]
        }
      },
      "id": "eyUcgo2gPfurWlEgbEuFVKyCuGBSZyr_aHoVQxmYLVt1BFMW5Ro3U6L3FedUXhT9Jo5mDJD-OVM7xk1577OpaA"
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
✓ Edge cache script created: H3QEQkVXrP00FPAcpjF_sMeJlQQnuqPGUQUhoVZy2Y22tE7X8S-ExijIUF-RlppQ4N1-1oBZysmTo0vWNjTsew

Call 1: Cache miss (fetches from API)
Response time: 570ms
Result: {
  "records": [
    {
      "id": "IlZ1WVTysfsTpa8NEyxQf8DUp42DW7Q9UYasnUImfdQH_1iqffI-3cR39sdcD1sSsz-MYq_tOx52CIt1mHsynA",
      "data": {
        "type": "Object",
        "value": {
          "generationtime_ms": 0.019550323486328125,
          "utc_offset_seconds": 0,
          "timezone": "GMT",
          "elevation": 32,
          "latitude": 40.710335,
          "timezone_abbreviation": "GMT",
          "current_units": {
            "temperature_2m": "°C",
            "interval": "seconds",
            "time": "iso8601"
          },
          "longitude": -73.99309,
          "current": {
            "time": "2026-01-04T06:30",
            "interval": 900,
            "temperature_2m": -2.1
          }
        }
      },
      "cached_at": "2026-01-04T06:36:29.928Z"
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
Response time: 601ms (0.9x faster!)
Result: {
  "records": [
    {
      "data": {
        "value": {
          "utc_offset_seconds": 0,
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "°C",
            "time": "iso8601"
          },
          "elevation": 32,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "longitude": -73.99309,
          "current": {
            "interval": 900,
            "time": "2026-01-04T06:30",
            "temperature_2m": -2.1
          },
          "generationtime_ms": 0.014901161193847656,
          "latitude": 40.710335
        },
        "type": "Object"
      },
      "id": "3fACYhpxO0bUfsaWWAQpMajjEqK2Ns6IpwQ7MOxDw7QS8t0pMdnMunGQPQuXFDFENQS-HOPPfphJuSvAFdlOeA",
      "cached_at": "2026-01-04T06:36:29.928Z"
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
    Finished `release` profile [optimized] target(s) in 0.08s
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
✓ Created SWR script: fetch_github_user (Z1nVjJxkR-ZAWr4NeZcOo_9s-dF1bckyK9KkUb0DuMM7I0GdlgeItsATiOyAAtRGmfCCBS2igAGuq36gRV-yGg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 87ms
Result: [
  {
    "cached_at": "2026-01-04T06:36:29.173Z",
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
        "followers": 269793,
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
    "id": "wJ0JVFxbRKAMUnItqDY4blwA30fd6hIqYFxrr_JvhLoZjG7wvy3CtI1FKbO5lhyTFHw4ccSAW3hbzbjv1bYdug"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 85ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (awtnca_lyl6XlqWTd9azoFr5bysMz2vk2_jU-qUyz8hax396rfH992wYid-1prwAt4RZdlT2i6Fp5DBMmrqexw)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "1767508592",
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
    "id": "ynIdSLcVOfQ6s_qklwDkBkvdl3SVN6TheCkNH4qKbz-z5IZAB059OZWA0fPAWJpi_18BupWcrh0zlMj9Fbpy0A"
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
✓ Created SWR script: fetch_github_user (JGdxMioVTY-_fLL6GFVSSf4dEtvTMqZiGTJ0WIulhNOyOC8i3XAW81ZmJiuqZGXfwc72hn__88mhLIP-abMasA)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 70.130417ms
Result: [
  {
    "cached_at": "2026-01-04T06:36:29.173Z",
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
        "followers": 269793,
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
    "id": "LSjWluvIzX3s3sfbYgoPBF2u0EOi4874IPdWVioXBItX9DG2NFTUH_gJXainNarB1Y3L6I2tYxlj1LXscX8-ig"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 85.357208ms (0.8x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (-CB1iqLfHW-BaVZp9lALnEG0kDCyn1eI3IfeXKOVZzCQl_Uh00mzJVUe7Hbf7yoKZWHoTy-Ve_XwEleLRKrJhw)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "1767508592",
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
    "id": "vX1MJQhDA6xLTNXQZL5PBXoUAGdxJKLlD4WxZ38ZOnxH1uLLyXJCzy-y5XGdNlsNsb2AfR4yw_T0LOoqCjbRLA"
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.45s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (kd5VI_Eq4ZZtatq1Xlzr12ReCj73EdFormQxiUbCZghnq6qawhQ8H-zPXeccG9dEpG6qzhLgUgfjkM1xBpyWxA)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 77ms
Result: {
  "data": {
    "type": "Object",
    "value": {
      "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
      "bio": null,
      "gravatar_id": "",
      "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
      "name": "Linus Torvalds",
      "site_admin": false,
      "followers_url": "https://api.github.com/users/torvalds/followers",
      "url": "https://api.github.com/users/torvalds",
      "location": "Portland, OR",
      "repos_url": "https://api.github.com/users/torvalds/repos",
      "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
      "email": null,
      "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
      "updated_at": "2025-11-24T04:16:14Z",
      "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
      "public_repos": 9,
      "following": 0,
      "blog": "",
      "html_url": "https://github.com/torvalds",
      "organizations_url": "https://api.github.com/users/torvalds/orgs",
      "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
      "login": "torvalds",
      "twitter_username": null,
      "type": "User",
      "hireable": null,
      "company": "Linux Foundation",
      "node_id": "MDQ6VXNlcjEwMjQwMjU=",
      "created_at": "2011-09-03T15:26:22Z",
      "received_events_url": "https://api.github.com/users/torvalds/received_events",
      "user_view_type": "public",
      "followers": 269793,
      "id": 1024025,
      "public_gists": 1
    }
  },
  "id": "oj7QELFIgPZaoI9sMcqRQO-WTmmCmTxebuaKaf2ij9B0FX4RmuDpenyhu4mXYnbMlyvmcK-hkp3ovSiqTHYNCQ",
  "cached_at": "2026-01-04T06:36:29.173Z"
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 79ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (7SRyrFoqUSpci_VXzK0rgTgzeTyHz4I3PNDryiprZOoJJu_qN3w6NHHpht5EQxkfEiA7QEXZE_QhfwrY9STFAQ)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "enriched_data": {
    "value": {
      "stock": 99,
      "discountPercentage": 10.48,
      "minimumOrderQuantity": 48,
      "weight": 4,
      "price": 9.99,
      "sku": "BEA-ESS-ESS-001",
      "rating": 2.56,
      "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
      "dimensions": {
        "width": 15.14,
        "depth": 22.99,
        "height": 13.08
      },
      "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
      "title": "Essence Mascara Lash Princess",
      "shippingInformation": "Ships in 3-5 business days",
      "category": "beauty",
      "meta": {
        "barcode": "5784719087687",
        "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
        "createdAt": "2025-04-30T09:41:02.053Z",
        "updatedAt": "2025-04-30T09:41:02.053Z"
      },
      "availabilityStatus": "In Stock",
      "returnPolicy": "No return policy",
      "id": 1,
      "images": [
        "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
      ],
      "reviews": [
        {
          "rating": 3,
          "comment": "Would not recommend!",
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerName": "Eleanor Collins",
          "reviewerEmail": "eleanor.collins@x.dummyjson.com"
        },
        {
          "comment": "Very satisfied!",
          "rating": 4,
          "reviewerEmail": "lucas.gordon@x.dummyjson.com",
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerName": "Lucas Gordon"
        },
        {
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "date": "2025-04-30T09:41:02.053Z",
          "rating": 5,
          "reviewerName": "Eleanor Collins",
          "comment": "Highly impressed!"
        }
      ],
      "tags": [
        "beauty",
        "mascara"
      ],
      "warrantyInformation": "1 week warranty",
      "brand": "Essence"
    },
    "type": "Object"
  },
  "id": "xTOi2uCsmXiqO5uRoDiofH0dLZauGU_h-RKB60JjGZTYQhXjmcc1eFYxVz6aDbNLGWXckZEhzL4ZBUII_YE-Ng"
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
✓ Created SWR script: fetch_github_user (XRvA9UnpEx0fJueXaqjqVdMy5pOtq_mFsB3TbnAPGveY6Ae5RTj3wFrev5Xf99TQpfka5M0u7SPLN5fl35V-iw)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 91ms
Result: [{"id":"ZwimsmYHeIqoCXQWYbAbWfztrHPd56UsxdPiDjFPcBEeakjMP9N9viFVWiKfEumrUGodq_d9BpUxrzcgERWczA","cached_at":"2026-01-04T06:36:29.173Z","data":{"value":{"events_url":"https://api.github.com/users/torvalds/events{/privacy}","gists_url":"https://api.github.com/users/torvalds/gists{/gist_id}","company":"Linux Foundation","followers":269793,"login":"torvalds","type":"User","starred_url":"https://api.github.com/users/torvalds/starred{/owner}{/repo}","repos_url":"https://api.github.com/users/torvalds/repos","followers_url":"https://api.github.com/users/torvalds/followers","gravatar_id":"","html_url":"https://github.com/torvalds","user_view_type":"public","email":null,"bio":null,"hireable":null,"organizations_url":"https://api.github.com/users/torvalds/orgs","url":"https://api.github.com/users/torvalds","public_gists":1,"id":1024025,"avatar_url":"https://avatars.githubusercontent.com/u/1024025?v=4","blog":"","updated_at":"2025-11-24T04:16:14Z","public_repos":9,"following_url":"https://api.github.com/users/torvalds/following{/other_user}","following":0,"location":"Portland, OR","subscriptions_url":"https://api.github.com/users/torvalds/subscriptions","site_admin":false,"created_at":"2011-09-03T15:26:22Z","node_id":"MDQ6VXNlcjEwMjQwMjU=","twitter_username":null,"name":"Linus Torvalds","received_events_url":"https://api.github.com/users/torvalds/received_events"},"type":"Object"}}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 74ms
Cache hit was 1.2x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (zObqPfLN85VNDdrvircn2jfaJAfqBzKPE2Za3E21L8nc8APk8ZTA8fP21wc3LSREQq8qIEtrBX8Md3QJMAcVyg)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"enriched_data":{"type":"Object","value":{"shippingInformation":"Ships in 3-5 business days","sku":"BEA-ESS-ESS-001","dimensions":{"width":15.14,"depth":22.99,"height":13.08},"thumbnail":"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp","warrantyInformation":"1 week warranty","discountPercentage":10.48,"description":"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.","price":9.99,"availabilityStatus":"In Stock","brand":"Essence","category":"beauty","returnPolicy":"No return policy","rating":2.56,"minimumOrderQuantity":48,"meta":{"updatedAt":"2025-04-30T09:41:02.053Z","barcode":"5784719087687","createdAt":"2025-04-30T09:41:02.053Z","qrCode":"https://cdn.dummyjson.com/public/qr-code.png"},"reviews":[{"reviewerEmail":"eleanor.collins@x.dummyjson.com","comment":"Would not recommend!","rating":3,"date":"2025-04-30T09:41:02.053Z","reviewerName":"Eleanor Collins"},{"reviewerName":"Lucas Gordon","comment":"Very satisfied!","date":"2025-04-30T09:41:02.053Z","rating":4,"reviewerEmail":"lucas.gordon@x.dummyjson.com"},{"reviewerName":"Eleanor Collins","comment":"Highly impressed!","rating":5,"reviewerEmail":"eleanor.collins@x.dummyjson.com","date":"2025-04-30T09:41:02.053Z"}],"tags":["beauty","mascara"],"weight":4,"images":["https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"],"id":1,"stock":99,"title":"Essence Mascara Lash Princess"}},"id":"0elq-S31JJH3HIyJE9odqlQO0aNo3TtnVozBjtRB0EOsA3ncTYz8CtCkiV7cwoFD31OEJbIqyfZ9Iq1qvhHVXQ"}]
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.99s
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
   Name: Some(Object({"type": String("String"), "value": String("User 1")}))
   Department: Some(Object({"type": String("String"), "value": String("engineering")}))

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 84.16925ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 79.900542ms
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


up to date, audited 58 packages in 1s

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
   ⏱️  Duration: 92ms
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
    Finished `release` profile [optimized] target(s) in 0.08s
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
   ⏱️  Duration: 85.2ms
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
   ⏱️  Duration: 77.115708ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 74.040916ms
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
   ⏱️  Duration: 80ms
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

