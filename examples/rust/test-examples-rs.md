make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("FqW1dkHbuB6a2s5WJyK_F-4HkYDEU83edHztOprK8_ptdYFi_rNVcbQsb7wic6OdDAJpYPBChJPycEWpDRPndA")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("FqW1dkHbuB6a2s5WJyK_F-4HkYDEU83edHztOprK8_ptdYFi_rNVcbQsb7wic6OdDAJpYPBChJPycEWpDRPndA"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("FqW1dkHbuB6a2s5WJyK_F-4HkYDEU83edHztOprK8_ptdYFi_rNVcbQsb7wic6OdDAJpYPBChJPycEWpDRPndA"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("FqW1dkHbuB6a2s5WJyK_F-4HkYDEU83edHztOprK8_ptdYFi_rNVcbQsb7wic6OdDAJpYPBChJPycEWpDRPndA"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "uNNfPR9zCQ_Wj1E21yvsz6eduv194YhIWwL8E2fBMxqSqmMVPCRQjSoBSn6ipWjaTp5S-QAoqOl_dwGFXKUdCw"

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
        "id": "uNNfPR9zCQ_Wj1E21yvsz6eduv194YhIWwL8E2fBMxqSqmMVPCRQjSoBSn6ipWjaTp5S-QAoqOl_dwGFXKUdCw",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
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
Collection created with first record: "A36dxdlNX5V9EQaGEzyxPj8AL9K5EgUnC11H685SisZRg6kT44Afyaa-D9J9X4XUCjIN8YiO9X5fCtXba8Sxtw"

=== List Collections ===
Total collections: 4
Sample collections: ["batch_users", "test_collection", "demo_collection", "websocket_test"]

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
✓ Inserted document: "IhH4R4-jKQrjUDH4837nEMXCI3E8wVUenMSuPwGTUFFufsJV7Jcm8mfP6JzhBGQXrIoch21mCYiH1jcVKy3-yA"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: null

=== Query Documents ===
✓ Found 1 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("996ScuPwsAA1SI49hp6IQciS8ZM6huTsSNeVJCbkyqtrWtBbkf1-Hy8DHIu2_cBWLq84L4PPkwEDPCcqHdftpw")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: xxGq99O-XglKqevgNYNOt7i67h6VNReYtKikxLHJ1F4H8SnFcqWPKQDFVpX9iebWPf0xl9oh3Sqo7FSzgyxT4A
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "umURZO6K-UCEwx8I_Ya8vb5N4OPcTJ7Rzm9C-ocpF3PQKBaL7s-UoWjVm7-EH_qYrEzJFGCZ8P9FeKbqTRKMfA"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: 5FqJ9PaYTf34s0wIR8ohRpWS2zEKGXM3WGUBtiq0ll-NGUNBnghgYWL68Dgr3DSEvMpIBTQiHT1yzSgRZ92-lg
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: qOQZ3f4Fn_B06d1kMrgTUkFM1f0XFJF_Vm3KXHSzW2bHquBMBh6ePzuG9HeRlIoiG7K-2x_U3MHZjbtYV6e6Dw
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: D0hZODdmPUQfV5t0X0OkjQTxVYOPV0rpFWQPht887o02peOw9ly6CM0tf6xLpF2jRVnuhfoBm5hchcrsUYujuA
Created Bob: $500 - ID: GMRF7pIm_f4Eh0EJZ7ovA6sP4KzuDcnOLXEKjt2rMZPyvXtQ6TsrzApEYIfo_F8TZXzl76jCMnJ0xR2cPIhU7A

=== Example 1: Begin Transaction ===
Transaction ID: 4ddc2efc-9850-46e3-8bac-c5053f18e878

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
New transaction: 5e996e8b-c21e-4241-8ae9-1940e92c04f3
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record { fields: {"id": String("oYvivBWG2SFouuAL4KhuDijk0qrekNuQgBh6SJ-UBLSyjVj75W8i2ogmn6IxvPMxxthX6pBDSgEokRKIVv0UZQ")} }

=== Find by ID ===
Found: Record { fields: {"name": Object({"type": String("String"), "value": String("Test Record")}), "id": String("oYvivBWG2SFouuAL4KhuDijk0qrekNuQgBh6SJ-UBLSyjVj75W8i2ogmn6IxvPMxxthX6pBDSgEokRKIVv0UZQ"), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "value": Object({"value": Integer(42), "type": String("Integer")})} }

=== Find with Query ===
Found documents: [Record { fields: {"id": String("oYvivBWG2SFouuAL4KhuDijk0qrekNuQgBh6SJ-UBLSyjVj75W8i2ogmn6IxvPMxxthX6pBDSgEokRKIVv0UZQ"), "value": Object({"value": Integer(42), "type": String("Integer")}), "name": Object({"value": String("Test Record"), "type": String("String")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)})} }]

=== Update Document ===
Updated: Record { fields: {"active": Object({"type": String("Boolean"), "value": Boolean(true)}), "id": String("oYvivBWG2SFouuAL4KhuDijk0qrekNuQgBh6SJ-UBLSyjVj75W8i2ogmn6IxvPMxxthX6pBDSgEokRKIVv0UZQ"), "value": Object({"type": String("Integer"), "value": Integer(100)}), "name": Object({"type": String("String"), "value": String("Updated Record")})} }

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
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

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "8M8ZmVYiJJ7mN3JwNTBWqLr8WEmi7dODCORZHv3neZEnnEzJezT9zzAqyFn75CkZE7CMTtntPVr6DfQS-wCR8g"

=== List Collections ===
Total collections: 9
Sample collections: ["batch_users", "test_collection", "users", "scripts__ek0_testing", "ttl_cache"]

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
✓ Inserted document: "30kfIWA7BHiJilyzE1L7Zp-J8zQV39oQfbB9Yf11L_zjSJ80y6XZT7C8nu8_bya_zc_Bv8CCZPf5hoJOeqwi0g"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("Wtx-3Ywps3P2KqvmSv62aBRUoYZZA2h3tIPool0MYVg0r-5m9LvCf6WIbINetn84ChDg58-uT0mKCU6GPHfSMQ"))

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
✓ Inserted test record: P83-xw6x3-KLeZlhjm3s1n355iMqU5z7ID_WNq0Qqdg0OcPkZ5innFZrfJ11Pgz27P2zw8tOnL5zuebMHttmPQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("uu13PmUf2jubk33Z7LyEQ7kz0mamPotMfEObG9J9PceZvuxeaN3jwP-1sRT6J8v9dGPqcOXhy_nNTipq6LSZqw"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Regex Query ===
✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"value": String("Charlie"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
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
     Matched: ["title", "description", "title.value", "description.value"]
  2. Score: 2.0000
     Title: Some(String("Python for Data Science"))
     Matched: ["description", "description.value"]
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
  2. Score: 6.6000 - Some(String("JavaScript Web Development"))

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 3ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_schema_management`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("MJeH8UwVJOlX7ZdEXNTX1EhcbqogRcTLxFpL-HIvaGjFYq8XeF0v-rxEOSOdm0CkNkDjiW3KgQeHFBgD-dkU5A"))
✓ Inserted user 2: Some(String("jf31Ttq_hcayKVLKdPskUd7W7N77Ga6x1H_ZFKpu4ZhwYH0zbyot_13TTk4AaFyHRTpgxpdI85JMf0bUMeH_hQ"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - title: String
    (required)
  - age: Integer
  - email: String
    (required)
  - status: String

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["batch_users", "test_collection", "users", "scripts__ek0_testing", "schema_client_rust"]

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
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

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
✓ Created session: GnurYMIVmn2xtI9pusFTuqQgQau7C3-kRSIfPGO4vyRy9X02Wz0z5Z9WVCW6GeXPqSHyE8Vz98kCZRoZPeD1xw

=== Sending Chat Message ===
Message ID: XI4qCOo1fd7V1Yp2djulDbiZz7cidWToOysdR4DJFDysIpmbSiNuIn41m4cVj8-jdAWSKHHmeb10D_UrcF_Svw

=== AI Response ===
Response 1: ekoDB is a high-performance database that offers intelligent caching, real-time capabilities, and AI integration. One of its notable features is the AI Chat Integration, which allows you to query your database using natural language and receive AI-powered responses with relevant context. Additionally, ekoDB supports a variety of search features, including full-text search, vector search, and hybrid search with automatic context retrieval. This wide range of features makes ekoDB a flexible and efficient tool for managing and querying data.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["content", "category", "title"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("7rW4qJtLk3DUq4lvianXWjjmRDXNBJdHFDkW1dl7wTIlOSMI16X3MQc02r1FtqQQGTQogqCJbVGw59o9BXQTpg"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("l0SjHDhqFiV_E-_khG-tR-CTzObx5C8xVVD83yYOpkqCv3mpGO5R4WSlVPx8a-JxHH7DHZhO6NAPeDLN-vMmZw"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title", "category"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("iNdzCKamCwPD0vS8KdzTUooa20x_J7zd7PW2Y48Sh9MDBDdaZK0eYbpSVRkyCQxENzzSd2qzswhG2X4aysFjSA"), "title": String("Search Features")}

Execution Time: 3036ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: GfXPY1xW_iSnX8H5hW76Z1_5bILggPqR4oM7Ap3fqJKBs31el-N2GY98rtL2-lOX7PCZ27GZhqT72QRHcbW-Rw
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is "ekoDB", which is a high-performance database product with AI capabilities. The price for this product is 99.

✓ Message 2 sent
  Response: The price of the product "ekoDB" is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: jsNFl2ATyvHAudCfiBn6rhqzJD9lH62inEzZwuh3GC7XKmyufD-IPtyFVWv95YXct8UxR5rfemtbT8eOT8qj2g
  Parent: GfXPY1xW_iSnX8H5hW76Z1_5bILggPqR4oM7Ap3fqJKBs31el-N2GY98rtL2-lOX7PCZ27GZhqT72QRHcbW-Rw

=== Listing Sessions ===
✓ Found 3 sessions
  Session 1: jsNFl2ATyvHAudCfiBn6rhqzJD9lH62inEzZwuh3GC7XKmyufD-IPtyFVWv95YXct8UxR5rfemtbT8eOT8qj2g (Untitled)
  Session 2: GfXPY1xW_iSnX8H5hW76Z1_5bILggPqR4oM7Ap3fqJKBs31el-N2GY98rtL2-lOX7PCZ27GZhqT72QRHcbW-Rw (Untitled)
  Session 3: GnurYMIVmn2xtI9pusFTuqQgQau7C3-kRSIfPGO4vyRy9X02Wz0z5Z9WVCW6GeXPqSHyE8Vz98kCZRoZPeD1xw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: jsNFl2ATyvHAudCfiBn6rhqzJD9lH62inEzZwuh3GC7XKmyufD-IPtyFVWv95YXct8UxR5rfemtbT8eOT8qj2g

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: ke4yHZ1iaNCRvex76rONj9ZgyudV1C5KewNt46zX3DQJogyZhaoFyrmnQLx0qpkB1KVqtrVLAummfV4jCG_ypA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information I have, there is a high-performance database product available named "ekoDB". It is priced at $99.

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
✓ Created second session: hy9LUO2nl-RZUbTZUgrqYDj1mubOHylpZk3swm78FojI3ne4KfoLyDAjFLBhNK3WVW37L9ChwSPfRWPCGo3Ezg
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
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: XV3rmvObqNLZCt5joaVKsVpxpUssKk7R5O49d0grVghxf8Mv4Sep7zbhOWPFtmQNbq4o-eR_-80O7Nwe6h79YQ
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: CntMOu9BHzkP9n2Go2i1aWTnuHG7y3wXfb2HoXomwnex5X9BSqUYpwMlquMl38LL60v9fonBGOvzb7O6BtgOVA
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: Qjpqm3dtzmuzd2dj16d6MCSIXL7NOfB4Kq_5qkFTd96ptPnUBhLdX7uQwWA1gTr4VordF4l_rFBNo7kQVdI0Cw
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 6
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
✅ [32mRust client examples complete![0m
