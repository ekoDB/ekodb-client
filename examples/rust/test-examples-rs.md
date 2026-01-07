make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("ouYiNTXLXrsDpf5FU-tJbQHZMOaxGuf-jfldj_o6C4PuEDMjkPAuc4ART4hLuffgGTl1CUefP3eU_yUY_28bvQ")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("ouYiNTXLXrsDpf5FU-tJbQHZMOaxGuf-jfldj_o6C4PuEDMjkPAuc4ART4hLuffgGTl1CUefP3eU_yUY_28bvQ"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("ouYiNTXLXrsDpf5FU-tJbQHZMOaxGuf-jfldj_o6C4PuEDMjkPAuc4ART4hLuffgGTl1CUefP3eU_yUY_28bvQ"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("ouYiNTXLXrsDpf5FU-tJbQHZMOaxGuf-jfldj_o6C4PuEDMjkPAuc4ART4hLuffgGTl1CUefP3eU_yUY_28bvQ"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "tNBjyp_C6q70lcZ7609iKWcnWLYe6Agolq1MREc9kt4JQ_fBMKBfF6Ge8qdVEoTvnBCarg8W5rkDNl7RomCAZg"

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
        "id": "tNBjyp_C6q70lcZ7609iKWcnWLYe6Agolq1MREc9kt4JQ_fBMKBfF6Ge8qdVEoTvnBCarg8W5rkDNl7RomCAZg",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "CK4S2Bfq6fdUMcDTQl4AVAIb0SFiX8070YRo9Q6MmUNRtYPghMnb2rH9zSv-9F2QAeqg9RT_zS1WU5eQNzX15w"

=== List Collections ===
Total collections: 4
Sample collections: ["batch_users", "websocket_test", "test_collection", "demo_collection"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "_ev5i8ul4C-xgD6RxxeVvLlGFQy_1-vieyeSAxzo93p-nGe97S7xOOG8d7y1NF_rU6EtnAy7xPudEQDU9iPhxw"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "8GiSFHV2rvq4_r_vj6IAz3bFSavtLz-G4pxy7PoiY6BhWV9VgYVr7bRv87znQ0ghMxwXknnFHxLzs1MoT3uibA"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "HZn4Fg_q9aaj-KIrSRQkHmrrdTGAeXm-oiguervHd8G1Zp8cjzeFLSsQjYbfFXCTDknXNpYbHq37hsEqhDFsOA"

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("YH6tnJCTCQfLQ_Ja36jKWcioQ8jWFxvm1KQHUuApKRVyCdWFPOZGn00Y9AossSkzE5fp4tbKZN8p_Syn5GmWNw")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: bIU5Ks5PZ9vD2Twalca-fWsu3JEfq6MHbYsagLoyDdCV4-LqYL1as3unQBJVUl4nJlFfHi9IUlzO4fh0srY0sQ
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "WaaNWE57Ig5LHTVGrGNvzCF_OpGM21Q8oXuMeqthrVMq1z8HfvOJX9Wl_98HNkiAK3QfSWfS9jUHQcEsmpQMgA"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: U_1seo-Qtf9rHW3WTcYPCC8thsj4c-gw-9DK2AOByQPTFN55pLf2wEBauXvqF71ZTSRxM-C-lIOE8nEGWhDDbg
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: paUYF4xN-ZKSqgXtj-ddVl5ay_4KCfGvkO1uHTvEqKyIsWWywDMm7xPHueYRh-K-EbYs2y66VcEbvqBLuJHT8g
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: tnINpOWhBNHrSCpiZQyb-eHeNws4jWVvUUk_RFcwRK-S8qGUU1TI2EYJ-D0NynCr6t-RWMXklZO_mRRwx4Zh5g
Created Bob: $500 - ID: qI2_Xgv-lTv3pM_Y_L-Nbb8gLxaqvJ_6HY9jrWvHrmcxSbeFSGi65Yv35aybGuV75yUwPJZXE2uwEGHZ54b66g

=== Example 1: Begin Transaction ===
Transaction ID: d324bbc2-b75a-4d42-a32d-d0d70ddd460e

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
New transaction: 7da9c6f6-d2d2-4858-9828-321187d7d855
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record { fields: {"id": String("XGiihV_8IKbktxoCYALV8e-_nwLYCV5wYRZDYayxdeWNyTxLAzBwtzMON1pfttXHPlneNB-dgnE-Hpoc9pYe5Q")} }

=== Find by ID ===
Found: Record { fields: {"user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-01-07T17:32:34.864866Z")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "value": Object({"value": Integer(42), "type": String("Integer")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "name": Object({"value": String("Test Record"), "type": String("String")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "id": String("XGiihV_8IKbktxoCYALV8e-_nwLYCV5wYRZDYayxdeWNyTxLAzBwtzMON1pfttXHPlneNB-dgnE-Hpoc9pYe5Q")} }

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-01-07T17:32:34.864866Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record { fields: {"price": Object({"type": String("Float"), "value": Float(99.99)}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "name": Object({"type": String("String"), "value": String("Test Record")}), "value": Object({"value": Integer(42), "type": String("Integer")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "created_at": Object({"value": String("2026-01-07T17:32:34.864866Z"), "type": String("DateTime")}), "id": String("XGiihV_8IKbktxoCYALV8e-_nwLYCV5wYRZDYayxdeWNyTxLAzBwtzMON1pfttXHPlneNB-dgnE-Hpoc9pYe5Q"), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])})} }]

=== Update Document ===
Updated: Record { fields: {"active": Object({"type": String("Boolean"), "value": Boolean(true)}), "name": Object({"value": String("Updated Record"), "type": String("String")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "id": String("XGiihV_8IKbktxoCYALV8e-_nwLYCV5wYRZDYayxdeWNyTxLAzBwtzMON1pfttXHPlneNB-dgnE-Hpoc9pYe5Q"), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "created_at": Object({"value": String("2026-01-07T17:32:34.864866Z"), "type": String("DateTime")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "value": Object({"type": String("Integer"), "value": Integer(100)}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")})} }

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: bTE67yNpI2Tmdg_2O4av3BuQU9QgJ63Yu74J88S_NXsYahTXYRnf0mYKBv9RWv84dFpkZAqO16n-5YPUniIgnA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "eMP6brZr0v9tnuupZB7b4KMItQDF2qd9TxAfLfQHSkofxdzRIQu1mPQ7LJHiMcoTKXSAB6rC1P9zANlQRsepLw"

=== List Collections ===
Total collections: 9
Sample collections: ["batch_users", "websocket_test", "test_collection", "client_collection_management_rust", "users"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: V76hhoAUZeITKLEJiYfAUqeynfO-bcz8pKdfslKZpQ-0Ldog-75VXPQpwtE0lPc4mQcx_CRDMdjHN6Vb9QoU-g
Created Bob: $500 - ID: DQiYyADXC70LGQR5bbxeE5k0ZQ8vpdll-aO6_GkA-zIAdFsVfCg0gTkoKkledx4ai3xmyHHoxBmPcjlKHyR7hg

=== Example 1: Begin Transaction ===
Transaction ID: e221195c-7869-473f-87cf-c39d9856a143

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: b0a978e1-cfca-42d1-8c34-8f0d0f34fe69
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
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

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
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"value": String("Bob"), "type": String("String")}))
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
     Matched: ["title.value", "title", "description.value", "description"]
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
Execution time: 1ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("vjwKYH3NicaF1xlJdGXbH-w2ZT1EolMCN9dSFu1hJOTV08T6TTX6-zOu65r8Mo-hdjeK44v68_3hC0JHwYQH1Q"))
✓ Inserted user 2: Some(String("K8o8Rthy5luSLCijGRljpYGZKrxzhrNlte8s6YVfLhQng_RJOfMBqebqDyaq9cf_vn2W65egsV8OeoUMtKiv7g"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - age: Integer
  - status: String
  - email: String
    (required)
  - title: String
    (required)

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["batch_users", "websocket_test", "test_collection", "schema_client_rust", "users"]

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
  - Mouse ($25) by Alice Johnson
  - Laptop ($1200) by Alice Johnson

=== Example 5: Complex join with multiple conditions ===
✓ Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✓ Deleted test collections

✓ Join operations example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "ucdobkmWD4Ozicj7oLfqDpOOy-7efRxFrOGm-cl5EwlFjrvw1x6Bag-ST5uxsu_8hBIZph_EUBWeGSvBnU-j7A"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("nJPM4RkfS5Gd-jRYCsqRr44XP_w8EQLEYKPtWgaNIEznZX2Qx50Fa5VxkO7xN1oBWqvYS3WP51OH140rlPmljQ"))

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
✓ Inserted document with TTL: Some(String("rX-CgqfbrhK4m9fcXzTSnLVGlFGxNHz-gO2MfP4_N_vsSW7v8eDD-TcNyaNYQ57UJZo80H91Nh3QhKvh-RdxQQ"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_edge_cache`
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: WVMZG0ImaAab0yihSCBbB-PqVi-EYIH5PoEjLqmaH8RfTgoxh0ZRxi8vE6tTF3lm2pJ7ezeBmzScIaLOECzgPQ

Call 1: Cache lookup
Response time: 3ms
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: 7QisHyzDOniT3615xGjK-ECQ4t7ZidDU83nitCineEDQc6vn0Aed3wvyNgWqtc2QcUMYQLkflie3Q4TjU80_7w
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: _VIf1ZeJxTRQDC9zJalku-EULPhk9ZaZ4kSlEMZbuNYo4Q-HPiPLYTYp7uGXrcGnyb0Q4qmUiOP35chp-L_DGg
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: fSmLxBhhlzyhiuClmp0Ggyb9_nuLzFtTMZxIGIaxgOQYxllNcNyV8y3xSysyZ2xiVuxfaUY-p2DulS6fY_rCug
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 6
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
warning: unused import: `get_int_value`
 --> examples/client_function_composition.rs:6:21
  |
6 |     extract_record, get_int_value, get_string_value, Client, FieldType, Function,
  |                     ^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: `ekodb-examples` (example "client_function_composition") generated 1 warning (run `cargo fix --example "client_function_composition"` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.22s
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

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 166.100791ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 86.999ms
   📊 Records: 1
   🚀 Cache speedup: 1.9x faster!

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_functions_complete`
🚀 ekoDB Rust Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: 1PtCZRM0mci0mf5M6im-cuoCMpBtwm7JZbwphgiF9SwagQmrIMTEumlKzMHqGQMFElw4vgDosfA43Roond0aIA
📊 Found 2 product groups
   Record { fields: {"category": String("Furniture"), "avg_price": Float(474.0), "count": Integer(2)} }
   Record { fields: {"count": Integer(3), "avg_price": Float(575.6666666666666), "category": String("Electronics")} }
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   Record { fields: {"count": Integer(2), "category": String("Furniture")} }
   Record { fields: {"category": String("Electronics"), "count": Integer(3)} }
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("camLORXd-LiPWW8bI8g9-kZ7NaD1SyKH5KeOEaZWRfhLIVBQmyU1kWG9HeP6qaF0V1O-jeqNDP9EqoTAgeskFA"))
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: n2bXTTxwtkmkiLgsPvSEuUI8SuMqs3IRvdsMs923TMV5LwU2zrjkQ-AKB6X19TC26lZMPkVgMQiZ1ZuTcXERZQ
📊 Script executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: eUBLieA68NnQTfIh7EZfbR6ZGKkn5PPbg5r5YWTR4aT2ftNngX1wIfM7wDWDMs976tozT44alcSAA18_ppPPqA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: XBPlPI9QSuoZP5uAt5LNjLd6gr2ql2km4Iq1i2V93rmKQdILy1TrkdMHHwm5D-msfxw88N9o0ALTAAJIsSx5rw
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user_rs (vChXjSdeb7tsizlmqoBsP4hweHbdQ92bZg2rdzmfSfEEpahvaETkZjjEB8EnjnTfWfLY7OA6xKjl_E7f8G71pg)

Step 2: First call - Cache miss, fetches from GitHub API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 85ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
   Record { fields: {"avg_price": Float(365.6666666666667), "category": String("Furniture"), "count": Integer(3)} }
   Record { fields: {"category": String("Electronics"), "count": Integer(5), "avg_price": Float(367.0)} }
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits including:

1. High Accuracy: They provide precise representations of points, lines, and polygons.
2. Scalability: They handle data scaling without loss of detail or accuracy.
3. Attribute Storage: They allow storage of additional information (attributes) with each vector.
4. Complex Analysis: They enable complex spatial analysis and operations like network analysis.
5. Flexibility: They allow for easy modification and manipulation of data.
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
   Record { fields: {"status": String("active"), "count": Integer(7)} }
   Record { fields: {"status": String("inactive"), "count": Integer(3)} }
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. Introduction to Machine Learning (AI)
   2. Natural Language Processing (AI)
   3. Database Design Principles (Database)
   4. Getting Started with ekoDB (Database)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   Record { fields: {"category": String("AI"), "count": Integer(2)} }
   Record { fields: {"category": String("Database"), "count": Integer(3)} }
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
✓ Created session: XHx678zFzvQBxtql0wn1OII_AuXV1GhvjLYrLZQ-gZRnwHowwqAolqCpshLu2jznaATv0NVlDDydDsxdxcb62A

=== Sending Chat Message ===
Message ID: PubTujoB-nHHVcAszsfh2TplNbEDk8L5mr9QqQbGu70GaUoNu_notb6ocaSDSAg_Si2lKJhZAnCEVteLHhKK-Q

=== AI Response ===
Response 1: ekoDB is a high-performance database that offers intelligent caching, real-time capabilities, and AI integration. It comes with various features that enhance its functionality and usability. 

One of its features is the AI Chat Integration that allows you to query your database using natural language. Through this feature, you can get AI-powered responses with relevant context. 

ekoDB also supports full-text search, vector search, and hybrid search, all with automatic context retrieval. These search features make it easier to find specific information in your database quickly and efficiently.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["category", "title", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("qNzKqSAAi1ItDsHF4mM3xEWFGpdoevML_pFdMU-mHGOboU48HxsbMCaAKgEwL752LVTeaT6TuxEQxR0opg9kVQ"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title", "category"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("_n3U8erATZEaSnjuDSUSpr4U6FIoRCGXnNL23JKFls_SVzODxaNBbZp17OItMoSZd-uARdjtMezloND_QACnlQ"), "title": String("Search Features")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("s7yezjpArNVfzNergmDBvaRgMdiLb3wBWwjq1IBylQLnYi54SoWM0_sR8u85FzsoIAUqmHcKICpY3za2f50AdQ"), "title": String("Introduction to ekoDB")}

Execution Time: 4781ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: xNmZDswZwB0HUhir1v9a4sEBtlqSFKI9hq5KuNT55AG35MYv1CmXSLt8mNyLFsGbBpG4fCAd7leGFzDx_q7r8Q

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, one of the available products is ekoDB. This is a high-performance database product with a price of $99.

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
✓ Created second session: 3h-6AMxcAOH6aXNJCEWHuC8pSInT-bQ-XBvtDVTHgWaOxnQEY4mQsFcceY0rYuoOCEiGywzY5Kro29PFn5d83A
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: LIEA05AnUr968XyOI6ITsQB6rrZIHfW4n8RjqjkQQEMvQJ47Xm7oU1zaUcyaEezVomhaP-4JpmKSp7atv64SwQ
=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is called "ekoDB". It is a high-performance database product with AI capabilities. The price of this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB database product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 20ZCS2wTkRdKAduWtbiM-1VwpYg1le742fsrSGobjl_ntYKGhR9wyTWSki_HKmK3KhjYWZlWWwjiGD16WCnMCA
  Parent: LIEA05AnUr968XyOI6ITsQB6rrZIHfW4n8RjqjkQQEMvQJ47Xm7oU1zaUcyaEezVomhaP-4JpmKSp7atv64SwQ

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: 20ZCS2wTkRdKAduWtbiM-1VwpYg1le742fsrSGobjl_ntYKGhR9wyTWSki_HKmK3KhjYWZlWWwjiGD16WCnMCA (Untitled)
  Session 2: LIEA05AnUr968XyOI6ITsQB6rrZIHfW4n8RjqjkQQEMvQJ47Xm7oU1zaUcyaEezVomhaP-4JpmKSp7atv64SwQ (Untitled)
  Session 3: 3h-6AMxcAOH6aXNJCEWHuC8pSInT-bQ-XBvtDVTHgWaOxnQEY4mQsFcceY0rYuoOCEiGywzY5Kro29PFn5d83A (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 20ZCS2wTkRdKAduWtbiM-1VwpYg1le742fsrSGobjl_ntYKGhR9wyTWSki_HKmK3KhjYWZlWWwjiGD16WCnMCA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record { fields: {"id": String("fH924aiTsGyA_qBBLgvZOIMHcEfchXx-3g3lG1rvtZ5nYipVrsfmqviGZSpQzu5Mxpk-XgDTL25rOsf_ekWziA")} }

=== Upsert Operation ===
✓ First upsert (insert): Record { fields: {"id": String("fH924aiTsGyA_qBBLgvZOIMHcEfchXx-3g3lG1rvtZ5nYipVrsfmqviGZSpQzu5Mxpk-XgDTL25rOsf_ekWziA"), "email": Object({"value": String("bob@example.com"), "type": String("String")}), "name": Object({"type": String("String"), "value": String("Bob Smith")}), "age": Object({"value": Integer(35), "type": String("Integer")}), "active": Object({"value": Boolean(true), "type": String("Boolean")})} }
✓ Second upsert (update): Record { fields: {"name": Object({"value": String("Bob Smith"), "type": String("String")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "id": String("fH924aiTsGyA_qBBLgvZOIMHcEfchXx-3g3lG1rvtZ5nYipVrsfmqviGZSpQzu5Mxpk-XgDTL25rOsf_ekWziA"), "email": Object({"type": String("String"), "value": String("bob.smith@newdomain.com")}), "age": Object({"value": Integer(36), "type": String("Integer")})} }

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/bypass_ripple_example`
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: Record { fields: {"id": String("LMvyMsZl5N7_PKyRxL-53mEPd2IxQgHxW7U2StFXOGDM7-tIoVSIAoj2dU8JweiYRSPsnIlrNb-uYP7tiyKsQQ")} }

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record { fields: {"id": String("haPvWUn_C7NL9GtXmD5kWAZnG6-aTXRA1dAuzgP01dXn322Sw0LuhymEMLlkTo_nvJ_MxEnYBpkBSgnypfcG3w")} }

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record { fields: {"name": Object({"type": String("String"), "value": String("Product 1")}), "id": String("LMvyMsZl5N7_PKyRxL-53mEPd2IxQgHxW7U2StFXOGDM7-tIoVSIAoj2dU8JweiYRSPsnIlrNb-uYP7tiyKsQQ"), "price": Object({"type": String("Integer"), "value": Integer(150)})} }

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record { fields: {"id": String("f4vVq83YYJZiF3dxQpBfyePY0sImwukyNMjIOzLKL70auvxLnXIdPYpVACuvYsIdHB9m0UsfxoKx71elR1vEMA")} }

✅ All bypass_ripple operations completed successfully!
✅ [32mRust client examples complete![0m
