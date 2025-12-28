make test-examples
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
✅ [32mAll Rust integration tests complete![0m
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
✓ Authentication successful

=== Insert Document ===
Inserted: {'id': '5chMg3FBZhiEHLOM9a_RIArVq6TZQ-RYh07mEWI4q4LqF16QdsGm4I7XpvIzx6ODQobMpTsbaGpRhbTZCj8-BA'}

=== Find by ID ===
Found: {'active': {'value': True, 'type': 'Boolean'}, 'value': {'type': 'Integer', 'value': 42}, 'name': {'value': 'Test Record', 'type': 'String'}, 'id': '5chMg3FBZhiEHLOM9a_RIArVq6TZQ-RYh07mEWI4q4LqF16QdsGm4I7XpvIzx6ODQobMpTsbaGpRhbTZCj8-BA'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'id': '5chMg3FBZhiEHLOM9a_RIArVq6TZQ-RYh07mEWI4q4LqF16QdsGm4I7XpvIzx6ODQobMpTsbaGpRhbTZCj8-BA', 'value': {'type': 'Integer', 'value': 100}, 'active': {'type': 'Boolean', 'value': True}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: Kd7MuNthn11mq4fPbeELKVfNy-FciWdlj1aoIA441hvPMlKU0ZCKc0mBikFpaf1fbEYf6jXXevgjFsdeggqXNQ

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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "Kd7MuNthn11mq4fPbeELKVfNy-FciWdlj1aoIA441hvPMlKU0ZCKc0mBikFpaf1fbEYf6jXXevgjFsdeggqXNQ",
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
Retrieved value: {'username': 'john_doe', 'userId': 123}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: {'name': 'Product 1', 'price': 29.99}
cache:product:2: {'name': 'Product 2', 'price': 39.989999999999995}
cache:product:3: {'price': 49.989999999999995, 'name': 'Product 3'}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: 1f9b6fdqxtOzPnlVxKo7lYhgt2iw9itioreUTcxwz-eq5ebT1P9UygTFTpwwxci_ZHYFG9EY_nSDqNR_aFlvnQ

=== List Collections ===
Total collections: 11
Sample collections: ['batch_users', 'test_collection', 'chat_configurations__ek0_testing', 'users', 'chat_messages__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: VfPhyagjMcKIMASkVin6ICioZ07poS-WJJvu_rrWW8sbVLucnjVJfpvmJM4wlMLhNMhKHUZKnCtHyJ6bHeXZEQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: created

=== Query Documents ===
✓ Found 2 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: ufaGNf2T7vfO6S13_KTH1_47RSvovcFsXVz_vHkY33E_HT2A1yak9hwOMu74zvDT0T3msV_DWHYVrrkUYdkLpw

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: C8d_K9sd3GSQmqvpJgoiyDPod3JTWpZBwRcRNgsJtDW7AQBqwTOPfDrgmIroBLRi_0qLUJh7t36xpCUB4As2rA
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: BKtCSovW84FhcFc_KeuB68VO3nzXWOqolExk9mG8bOpv7x7urayKLERFJIMthExSZUiHCqW8v29Qw7BmvB5BqA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: VtMJja3BTUslQwEjyTgU3vQIzIx2cRh36eBznwkUmItDxqrsHciqVCp_WQDl9q-jSVfQ06_Iy7Ra11PwmGIxpQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'max_score': 90, 'status': 'inactive', 'avg_score': 50.0, 'count': 15}
   {'count': 15, 'avg_score': 60.0, 'status': 'active', 'max_score': 100}

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
Created Alice: $1000 - ID: QW4eXfYoTZvYKtzTnjLdMp3XdRFF187p_pqzA1zM2aW6YiS7jHnrxjPEPQzGdFTBheRiFDIhdWxw1kbxoLoaQw
Created Bob: $500 - ID: Mv0586kPvqSeJl62A9pgqasZBXLbtjx6JA33voTe4Q-IZijP5zdbWWfiJQo_7-42UDNcmh0Jjlt7ALEGJnng5w

=== Example 1: Begin Transaction ===
Transaction ID: ab9039de-ddc5-45e5-96d8-2b632ba82957

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
New transaction: 71ccef37-6e01-4a21-be01-7c403c8616b0
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: u7Gb2Sn9-7c-DHYlrNAuKoMzIAXIJIWgR0yvCYyRVcyRXLHn4C-oN4PB78wM2F3aiO9SoOEe4E2u0r9h9TEQKg
Created Bob: $500 - ID: e9KY3FbjZB6xUyCKWsCHcIx0A2ECQqOpI9UH2V4CZ3SANxDcNsQbdao36xELd5-0Ajr12DDVaF6lZgt2kXpnQg

=== Example 1: Begin Transaction ===
Transaction ID: c08dffd7-d26d-4c3f-b0c4-bbccbdb75510

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
Created Alice: $1000 - ID: KQsmvZEDG-k1OK5FJb8b1lJW08Sw8V5zhI8JHdbzsjK9tYJN6KfK3ExEaA-sCMRWgmoHwSKymILnvq7OCP4wEg
Created Bob: $500 - ID: XaRPYEJwihgnVI26SSjf-bF_YHuCzHfypi_d66V6zIhTMuo5RpsJWTRYBAKwBEFg1dcK0gKYYWnGNSg9OuhfPw

=== Example 1: Begin Transaction ===
Transaction ID: a367e9a6-a5f9-4688-95ab-4db5c2414abc

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
New transaction: a06d9bdb-ab54-47ea-910b-8ed2c201e11b
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: slIIWa7SbiNEMuJsEy-DSkzCd-TXqPYcndR-73IuntnVfRkYcapYe1kJy-DTaYrxTZjdV2FTuHye8WiOkDoj6g
Created Bob: $500 - ID: OdX6-NlUZ2nwNR1G2HFgkq-2QMaCXHKcuw6KvZ9GDg-13tiB0mVLGL0OsWb9M8z5_iUXJUHYYdF5I-lAdlahKA

=== Example 1: Begin Transaction ===
Transaction ID: c25f7999-2f30-47a8-ac46-36f19516ce86

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
Created Alice: $1000 - ID: eN5cz7BfxWsjtNLldhPDq0ieecCBDrnWlR-glSdk5K_LFkbUD_r0CwCgcuasjkh0HbcUOXyAEtOldGLPTCNedQ
Created Bob: $500 - ID: W9DAfBqZ4oq03wuZxXcUAxxdN2rGg520GzeTGANVj5XC79A7qWFEbqu8HCi20t-RkI98-2Hqo-ALbTy1zH-YTQ

=== Example 1: Begin Transaction ===
Transaction ID: 8f0c4d42-99a6-420b-9812-bc4e47b2073c

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
New transaction: e1c058a4-8937-4d53-932d-96b738d5c8c8
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: coUAlSgkXqlG1smA8MQAYaPivz_PDYL9Bzdznv1rnZ3kldAYbvgxvAaGOpFN9GnrWHVtUeCThoVF0pFKVknjjw
Created Bob: $500 - ID: kUEm5nJHXRbBZ4BafSBRgEFXkYUkza_HsMCeA_ZOi7M9KhifIokdrftIkHvGH3DrIuwpt6b-QdRayy8eDroKDw

=== Example 1: Begin Transaction ===
Transaction ID: 12f6a053-8c2a-4013-9456-3ada9e5c9c98

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
   ✅ Script saved: 6UseiYyeIMC7d_UnJRWeDPaupd8x0wWb0EB5Qd0fjXl8yEkFqPxWq3KCVnN7qK7RYpZnv7RZYMbaWDzZpjhmVA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: hjS3oC0q9i9gGgJA5L5eq90LMz06uXujmA6cNPEuzhQNIdTtuFX6gFqt_cpTfhnNuviJ5SLB0g2ma_xjaJavMQ
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: m2MutAC5UEniAW6TsOpwStxO0vW2mXBnVB8kV8VLXCXNg5LWgx1ERt45vqD_HciwoWr7yYxBqZ5y7FvevoMcKQ

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 8v5HPcklLwwO5ebmIDBmbyG9AyMHx3YTzoIXQdO36IZYH10oWNdw5--6IKLMPoi5Cj02DvA13ZnH0C0MnvTorA

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'value': 'active', 'type': 'String'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: NlhrnLqQUJTqNVUwY56qQBjkjgeSDLwfIqPUuuP-QcbfaVFbrMk869PBf9_vuQPx4TS6hjfqfxbdECav3wnSLg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 6UseiYyeIMC7d_UnJRWe...
   ✅ Deleted script: m2MutAC5UEniAW6TsOpw...
   ✅ Deleted script: 8v5HPcklLwwO5ebmIDBm...
   ✅ Deleted script: NlhrnLqQUJTqNVUwY56q...
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
    Finished `release` profile [optimized] target(s) in 0.15s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.5.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.5.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
Successfully installed ekodb-client-0.5.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'd-CD9ir8Uffp2EiQaISWqZaESFkdCRIhDcctoKH-bgs5uCtV4iI1n8feFZEgOfJRz3MTyKSovD1Usx2OoicIRA'}

=== Find by ID ===
Found: {'value': {'type': 'Integer', 'value': 42}, 'id': 'd-CD9ir8Uffp2EiQaISWqZaESFkdCRIhDcctoKH-bgs5uCtV4iI1n8feFZEgOfJRz3MTyKSovD1Usx2OoicIRA', 'name': {'value': 'Test Record', 'type': 'String'}, 'active': {'value': True, 'type': 'Boolean'}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'active': {'value': True, 'type': 'Boolean'}, 'value': {'value': 100, 'type': 'Integer'}, 'id': 'd-CD9ir8Uffp2EiQaISWqZaESFkdCRIhDcctoKH-bgs5uCtV4iI1n8feFZEgOfJRz3MTyKSovD1Usx2OoicIRA', 'name': {'type': 'String', 'value': 'Updated Record'}}

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

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "MtT5zy7pNP-xIqlJq7kdcmzg6lqkrtnotoC5CuKUBZWT5-5lYvUqsjzueLpcom2M-V1RE4oXiPCo-JCQZxgcvw"

=== List Collections ===
Total collections: 10
Sample collections: ['client_collection_management_python', 'batch_users', 'test_collection', 'chat_configurations__ek0_testing', 'chat_messages__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: foqK5T2JwGkDK8wrybjVryiEGgyrwrIs4io7c0NCvR6rtBrCvomOWRFCuGZ-QQaNaSlZ_ftRsXe_FyTXKHBm1Q

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: P5dDd5YPFBUQJcJBL3J5SHGDmz7AWMfMwY30CGxEu157tkcPJYepdh3fghWEz3QFakWB7T4NR5rIQNL8jkNUpw

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
✓ Inserted test record: uoEugP5sLCWj-v4CEUyWih52evrcw750-plwClxug-CT1g59fCn848a1HSWwesowvVafLiwcROldh4eGindRtw

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
✓ Inserted document with TTL: prx_XHvMKPoclRlL6m9kJx4HyaAYoGTk_JR9rUM_-O88SHHHsNxI2QS7Y_2DHQJ_ZbqK4Wd7Uw2By3gAHxovfA

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
  1. Score: 25.740, Matched: name, email, email.value, name.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, bio.value, title, title.value
  2. Score: 26.400, Matched: bio.value, title.value, title, bio
  3. Score: 26.400, Matched: title, bio.value, bio, title.value
  4. Score: 26.400, Matched: title, title.value, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, bio, title, title.value
  2. Score: 39.600, Matched: title, title.value, bio, bio.value
  3. Score: 39.600, Matched: title.value, bio.value, title, bio
  4. Score: 39.600, Matched: title.value, bio.value, title, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.767
  2. Score: 0.760
  3. Score: 0.745

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.732, Matched: title.value, content, content.value, title
  2. Score: 1.121, Matched: title, title.value, content, content.value
  3. Score: 0.537, Matched: 

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
✓ Created session: _1CPODs-8nj298P5KnJz-t7BEQVsQOo6EyCog69m8gTNz_pANx0bruuIZ7qnAoLYTGBamNY1ZO3vaCNcVPD0Wg

=== Sending Chat Message ===
Message ID: bA46-Y5k5Axi3TvrxVLfvAdLmDMyjl73YIuFD__xukFJ91XaFnT1MIf84W5zhnSiEZxHO7sxuO15KeywjY0aqQ

=== AI Response ===
There are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. The price for ekoDB is $99.

2. ekoDB Pro: This is an enterprise edition product with advanced features. The price for ekoDB Pro is $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price for ekoDB Cloud is $499.

Execution Time: 2824ms

=== Token Usage ===
Prompt tokens: 609
Completion tokens: 89
Total tokens: 698

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: wm2yusSL9We4iCWtVQ8OgiRhxOTYG68-HqstgTHWeEx7_p2tmw4Wp59ymCN4bMg5hHgv9tlwAeuZZYkvGMQJiQ

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, we have the ekoDB, which is a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 9mMiWkgfMMCkgBgT1_kIRwdebmhWFhfSXVMXm6iTGiykGfhD-CYtLoT_K2R92YpC3EI0VeGLF7YpXY4CMDTQTQ
  Parent: wm2yusSL9We4iCWtVQ8OgiRhxOTYG68-HqstgTHWeEx7_p2tmw4Wp59ymCN4bMg5hHgv9tlwAeuZZYkvGMQJiQ

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: 9mMiWkgfMMCkgBgT1_kIRwdebmhWFhfSXVMXm6iTGiykGfhD-CYtLoT_K2R92YpC3EI0VeGLF7YpXY4CMDTQTQ (Untitled)
  Session 2: wm2yusSL9We4iCWtVQ8OgiRhxOTYG68-HqstgTHWeEx7_p2tmw4Wp59ymCN4bMg5hHgv9tlwAeuZZYkvGMQJiQ (Untitled)
  Session 3: _1CPODs-8nj298P5KnJz-t7BEQVsQOo6EyCog69m8gTNz_pANx0bruuIZ7qnAoLYTGBamNY1ZO3vaCNcVPD0Wg (Untitled)
  Session 4: hy9LUO2nl-RZUbTZUgrqYDj1mubOHylpZk3swm78FojI3ne4KfoLyDAjFLBhNK3WVW37L9ChwSPfRWPCGo3Ezg (Untitled)
  Session 5: GfXPY1xW_iSnX8H5hW76Z1_5bILggPqR4oM7Ap3fqJKBs31el-N2GY98rtL2-lOX7PCZ27GZhqT72QRHcbW-Rw (Untitled)
  Session 6: GnurYMIVmn2xtI9pusFTuqQgQau7C3-kRSIfPGO4vyRy9X02Wz0z5Z9WVCW6GeXPqSHyE8Vz98kCZRoZPeD1xw (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: 9mMiWkgfMMCkgBgT1_kIRwdebmhWFhfSXVMXm6iTGiykGfhD-CYtLoT_K2R92YpC3EI0VeGLF7YpXY4CMDTQTQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: ysATYJhDjp7xGC3dSD5lizyRhizW8zn9sRFqfgQu5E3179aPG6a0U29qFsZZeh9WtAKvgVhUeN5dx0QzlHtm3Q

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the product available is "ekoDB". It's a high-performance database product priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['context_snippets', 'id', 'chat_id', 'token_usage', 'created_at', 'updated_at', 'content', 'role'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: KVh-vuJkxdu82QEem9mEJqlsehZ-MC_e1SnOJ7Xgd_lrHTjQZhpjlMgkO11L2jZt3heiLfxZmTU0K21jAkDofA
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

✅ Script saved: h3Znj1FcDpiO7_YSwvyYS8hrhat_z7YkN-lBjLn5zl9nQRrlrZZ87n8sU2scjXJ_PulBcRtnA4jPhgR3rcSm9g
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: 0TtruoLAc-4TAn-D_M6MG8HpBYTON-negELMqGvpwKFkb96SCNI6-XAYcrsAjw468TaX69tmtP2NeC2iq1nAAw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: QQIZQYxV5Vz4F9NnBLMCIWLzrTCK3koSdQnlLSgq8cJbBAnXxJLr_Q2mC7rDFrUMlLw_huvNyZUbbEVXRpr0sA
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

✅ Script saved: nDYipfawWk_loiBdsqQb4a9Ax-Uf9ra4ROxd0Tf6osk9uPMW5C_92WWWwNNpM_ZrB5RbtEVCvLgwdQcd9n58hQ
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
Inserted: map[id:IAdTinjF5zvcvQbUt1DCvE9EQVsW46mBjaveZ4GCjE4vlsX1HTDNg-H3aNXWWdDXwPPUIsultVfVCjsQ4mcW2g]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:IAdTinjF5zvcvQbUt1DCvE9EQVsW46mBjaveZ4GCjE4vlsX1HTDNg-H3aNXWWdDXwPPUIsultVfVCjsQ4mcW2g name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:IAdTinjF5zvcvQbUt1DCvE9EQVsW46mBjaveZ4GCjE4vlsX1HTDNg-H3aNXWWdDXwPPUIsultVfVCjsQ4mcW2g name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: rOrZMWqDh7WWVTLDxIikzWv8TgnmEIw4VFgPZB6WZt6mnTipX67B6IVw3iRtb_yijeZztQjufYQyn0AN2m3wDQ

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
        "id": "rOrZMWqDh7WWVTLDxIikzWv8TgnmEIw4VFgPZB6WZt6mnTipX67B6IVw3iRtb_yijeZztQjufYQyn0AN2m3wDQ",
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
        "id": "uNNfPR9zCQ_Wj1E21yvsz6eduv194YhIWwL8E2fBMxqSqmMVPCRQjSoBSn6ipWjaTp5S-QAoqOl_dwGFXKUdCw",
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
        "id": "Kd7MuNthn11mq4fPbeELKVfNy-FciWdlj1aoIA441hvPMlKU0ZCKc0mBikFpaf1fbEYf6jXXevgjFsdeggqXNQ",
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
Batch insert response: map[failed:[] successful:[382IwybyM0tvE0YZUIGa7nJEGoCx-wvQCrBJh41VwpsY2JNi6wq_upbRKjfjeYEEreUCKnwHhgBrRLEb1nKJGw 8B-PM6dxkmSXhPyRSW3Q2gRSmNTtpA44bZHqMogIE09IVfE20b4qkx6zHzOtgZbFhePxRe6KTp8y_jbhCe4DDQ L1cNlX9jycAiwPvw3Wo9arLWAj-yUlXXV1clUJko8B0gfoS3xXe_yW6gJYX5A3_wNY8PudMgDmjTJCcPBQOV9Q 2kLrHD-2FEXT3dRWgt8UJMkmcpdBE5tIvECQVphaoOJebhNgH1U8Z_hsLennWKRA-Ipo3L7fleRKwziJHbbrLw S_pR5qXm4i8mt1h-5DBzvk-wH7Ni675zFEinjC7NgMcmrjVWiZlS6aUHxiFMT0UVJO8pTl22X7s9OH9hdE23zQ]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[xgDfSJG3zpm_HcvsF-BNjX5eN52_LEQ_9QTsD4zom6Wu_MrfS9kl4Bi-pYU1vi-L1IOwoH-iZbUeOq-78BhFeg 39Z1_xvwY1XVNdp3xU5-9_5ZetXcfhNODyJaJ059mR0yyGnxoifNK-cCyT3ZU4RGNxllhMhQKJkRljHCGtx4Zg 97keQXbfeIy1JKfvWHCNA2G79HEvbVGvFyOrZxZ4POkMn7P8_2Ua_qDdyjUAqCBJ1SqVMFjP6SXhOiNVIw48EA]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[xgDfSJG3zpm_HcvsF-BNjX5eN52_LEQ_9QTsD4zom6Wu_MrfS9kl4Bi-pYU1vi-L1IOwoH-iZbUeOq-78BhFeg 39Z1_xvwY1XVNdp3xU5-9_5ZetXcfhNODyJaJ059mR0yyGnxoifNK-cCyT3ZU4RGNxllhMhQKJkRljHCGtx4Zg 97keQXbfeIy1JKfvWHCNA2G79HEvbVGvFyOrZxZ4POkMn7P8_2Ua_qDdyjUAqCBJ1SqVMFjP6SXhOiNVIw48EA]]
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
Collection created with first record: 6UfpxQiASc3Ow_UTkXqXIvEVt3_Y42fsaQIm2bEee5O9o63pzgoRg7MQUOaQA9G5cvc-nGersLYt_xYbWJM9bg

=== List Collections ===
Total collections: 15
Sample collections: [schema_employees_client_py batch_users schema_products_client_py test_collection schema_users_client_py chat_configurations__ek0_testing users chat_messages__ek0_testing scripts__ek0_testing ttl_cache ws_ttl_test demo_collection schema_documents_client_py test_accounts websocket_test]

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
✓ Inserted document: BBoPo9m7BA_iYXZ_TNO7WwlSOyqYR8yY047BDRbyoEXx1W8p8qQynBfcpURtp3HICNwsL4UQYSTyNrF962UCHQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: created

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully
[32m✓ document_ttl.go completed successfully[0m
[34m
=== Running websocket_ttl.go ===[0m
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: 0FSOpIc4KNAZ9JZtKUCzvPLjyp97nmXTyecMiYyyWMOifZoKhZX5XWhpF4vGEds8V5dRnZZyEBZEdVKEIU1QyQ

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

✅ Script saved: RIzRGTCJyqlPup7GrteMGxpvuRsf5bZHrm09XcQZkUNcVi0Tv8FU6S7K2onT-t5v-fbWzZgpFbXRVUGHOlPSvA
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: DP8A1qSnaEunuP4U4Wn41TjFrxS_BII9r1WMQmehTa_z9mmIxMK0HBfCWFGmd6K-3zn24YGItdz72QDN4pQgIg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 8sft0dvqtlAoqs9x4aFHvimisLL6cjk8HappHS1YWqlG9zzllflP9ekxmnwLS667LL86v_QlA1ApjeBxaQX7yw
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
Created Alice: $1000 - ID: cb3s5hnZZbg7T93Pf99JYYnkFjZexoaABhkEiNvjKjEQPKf2jv1ACb98XmY5-g6EjqQ0AU8ZyyKdMrRWXUIIrQ
Created Bob: $500 - ID: zAu2NbeZmXTIkGI2RJBERuZ9iF0UyX6x4LKqj-PnzIfcFu_OSe1Z6QGpOAKIYt9li6jX4EOY-kdSDuiKEmMzFg

=== Example 1: Begin Transaction ===
Transaction ID: 1ed1cd29-55ba-424d-9e14-cc59ee9081f4

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
New transaction: c76867b5-b1d6-40df-a785-18ceba8806de
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
   ✅ Script saved: y69Tgj7y5GsPx1EJ07IDhjFYA90xsrQ-rNtMsSQzWgPSbA7-QoVc8oFOf2ZiNtoZGypXWLBstZSP-3zP_Rfkvg

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: MAc4tIxWK1kkaQXf800kOlwCei7VJR-GyqwCNqnPbnXvEepzTAT--M2uOr6HgHGYhIQGU7ftJVD27dQ1truXTQ
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: PuhJEte6F3K-rFRrmbqdm_jPURMB8-gPVICYMG2Adn_Tdg2eJ4cVdoMyvP4oHR65AgySDZqzxeyJkue9oYe7TA

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
   ✅ Script saved: z02mFxp5uw5rUyEdDgDcGOciwQ12pjU4ODlpww6MUdK3cwu8zUdtFagJFdt2ygCN50YH4Lrokb8UM-eVFrlaOQ

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
   ✅ Script saved: dEnGrokF-NjwG9LhIxq0ZdIlgkIabaQ9Hr8y3huht2GcZvVAsnAOPu8n_aMHE-JcLuKewIUJB2k17SHSyq7fMA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: y69Tgj7y5GsPx1EJ07ID...
   ✅ Deleted script: PuhJEte6F3K-rFRrmbqd...
   ✅ Deleted script: z02mFxp5uw5rUyEdDgDc...
   ✅ Deleted script: dEnGrokF-NjwG9LhIxq0...
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
Inserted: map[id:wd_LDAm2W9a8YXPclgme-xLeyaXHZ2vuqxhhHML1gfZjK_KGwf3TgdDAKz2_p2YbASnc7Qu-18tpNGXaXI5RDw]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:wd_LDAm2W9a8YXPclgme-xLeyaXHZ2vuqxhhHML1gfZjK_KGwf3TgdDAKz2_p2YbASnc7Qu-18tpNGXaXI5RDw name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:wd_LDAm2W9a8YXPclgme-xLeyaXHZ2vuqxhhHML1gfZjK_KGwf3TgdDAKz2_p2YbASnc7Qu-18tpNGXaXI5RDw name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

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

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: WcYDDv-X-lGtUNcCC40QLNYn5Nd_M8wpTRJ-0lsu6FwM0kPby9Y7ntor90ord_QoRmqSpc9UvgMjnbWWOWGXpg

=== List Collections ===
Total collections: 14
Sample collections: [schema_employees_client_py batch_users schema_products_client_py test_collection schema_users_client_py]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: gY6qel2SErD-lp8OwGcomm1xW-YoGJnhP1wApQqXXx55MNUcnpa2UOE0VjFGFqCqiwKnroChdMxFPmccPSNZDw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: ZKxIlbRJOiUORXYGdXMsr5Q8xL7MGTNPFCEpL4u7FT0r5EfclSuFL00rgXZAS7fNUc1-g18_sZvmjmFzCq5x7w

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
✓ Inserted test record: RHHLBNb8Er9CwnXKCJyQwGMpFdlPuv2Ln_wWFKVdsKDyXkWdWDq5KUC7dd5pnJefZ8rOEwlR63RJSUo483IojQ

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
✓ Inserted document with TTL: 9yW6-C-d-p3QDRZK8JJyVzxkcMH4hkdohgRSIhyN0nW8d35UmVQWkM5JumyONfpO97PgHrCiJ7-KUsBHfzfkuA

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
✓ Created session: v2sHPzLi65qdQ6hf-Js9BfbjCdw237_ldwjv4O9gOAJak9wodMbphJdBNk3fkb5WCgpDQ8I3EbhOABOA8t5SbA

=== Sending Chat Message ===
Message ID: zHOdZh4Xf_TbI6h0OvjVFJ4lCasHmUypSRuYBY6cwFMLxqM2RnSCoftv03fWh5zIBmYOYMumdopw9EK9CeurMA

=== AI Response ===
We have three products available:

1. ekoDB Cloud: This is a fully managed cloud database service product. The price is $499.

2. ekoDB Pro: This is an enterprise edition product with advanced features. The price is $299.

3. ekoDB: This is a high-performance database product with AI capabilities. The price is $99.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:diGrfUFKemLJCcvuCi8L6dO3DjkKLkUGqxrCcvjd3UXXfvxBxX4UkR4qbkApmNkTSDvyP1Lj8PcChti4sLuVZg name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:V2YPOAX9lRUVMlrf3ATGwBZNpGoQ0nTzlQoYrYjfofZ1xDOp2lmcw9CQV-9Z3CMDttTxIHQWj1bM-0o4Max5gQ name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:FLK5ygejihitT6yvxzN37uPUR-jfuI6tNqkoxKH1oYjwgpio4brAf9ZlSBZ4e7UTjNPYIIbeH2xHVSEnJS2Gug name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 2241ms

=== Token Usage ===
Prompt tokens: 613
Completion tokens: 75
Total tokens: 688

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Nl1MXe_ym3CqGP0-zV4n4927SZunPuOrsjYNzqtKLYRSh1xVU96TXy7VOfn9nmXIpAN3UJcY80zW23qzl_fKhg

=== Sending Messages ===
✓ Message 1 sent
  Response: The product available according to the provided context is ekoDB, a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: H2WPCLfx3tnaOKw6qNE9eNJJdPtsHwxXCpYHoV-f3xt7el2rQFGELa6I9HJZThmNQ2UpfAh9ehJz8_ec0-iu3g
  Parent: Nl1MXe_ym3CqGP0-zV4n4927SZunPuOrsjYNzqtKLYRSh1xVU96TXy7VOfn9nmXIpAN3UJcY80zW23qzl_fKhg

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: H2WPCLfx3tnaOKw6qNE9eNJJdPtsHwxXCpYHoV-f3xt7el2rQFGELa6I9HJZThmNQ2UpfAh9ehJz8_ec0-iu3g (Untitled)
  Session 2: Nl1MXe_ym3CqGP0-zV4n4927SZunPuOrsjYNzqtKLYRSh1xVU96TXy7VOfn9nmXIpAN3UJcY80zW23qzl_fKhg (Untitled)
  Session 3: v2sHPzLi65qdQ6hf-Js9BfbjCdw237_ldwjv4O9gOAJak9wodMbphJdBNk3fkb5WCgpDQ8I3EbhOABOA8t5SbA (Untitled)
  Session 4: KVh-vuJkxdu82QEem9mEJqlsehZ-MC_e1SnOJ7Xgd_lrHTjQZhpjlMgkO11L2jZt3heiLfxZmTU0K21jAkDofA (Untitled)
  Session 5: wm2yusSL9We4iCWtVQ8OgiRhxOTYG68-HqstgTHWeEx7_p2tmw4Wp59ymCN4bMg5hHgv9tlwAeuZZYkvGMQJiQ (Untitled)
  Session 6: _1CPODs-8nj298P5KnJz-t7BEQVsQOo6EyCog69m8gTNz_pANx0bruuIZ7qnAoLYTGBamNY1ZO3vaCNcVPD0Wg (Untitled)
  Session 7: hy9LUO2nl-RZUbTZUgrqYDj1mubOHylpZk3swm78FojI3ne4KfoLyDAjFLBhNK3WVW37L9ChwSPfRWPCGo3Ezg (Untitled)
  Session 8: GfXPY1xW_iSnX8H5hW76Z1_5bILggPqR4oM7Ap3fqJKBs31el-N2GY98rtL2-lOX7PCZ27GZhqT72QRHcbW-Rw (Untitled)
  Session 9: GnurYMIVmn2xtI9pusFTuqQgQau7C3-kRSIfPGO4vyRy9X02Wz0z5Z9WVCW6GeXPqSHyE8Vz98kCZRoZPeD1xw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: H2WPCLfx3tnaOKw6qNE9eNJJdPtsHwxXCpYHoV-f3xt7el2rQFGELa6I9HJZThmNQ2UpfAh9ehJz8_ec0-iu3g

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: tBRQWmMEPalc6ts1ua2Z57OBzn6PsKfCqXtA_0TiAOTiVF25VFYVSl5iUINgDOeG5rV6ols7k7sPqw9V28bGDQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, we have the "ekoDB" product. It's a high-performance database product. The price for this product is $99.

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
✓ Created second session: EN1u8WYJ-XYaHi9TyUf54f-TTSu0EMrkz__RCp9y29vsugdqMwNrgyEQPBPnox61oymGm6v6jMBP1LKSqwQpoA
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
  1. Score: 0.784
  2. Score: 0.745
  3. Score: 0.730

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.713
  2. Score: 1.498
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
  - Laptop ($0) by Alice Johnson
  - Mouse ($0) by Alice Johnson

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

✅ Script saved: JPLmgOeDGz7Fp4uPjLL7AQrqhTwCBbZc-1VajkgnnRWc1y_owbMji_UM_RWb3RYIrHFoFy2-wyA8465hq7WN0g
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
✅ [32mGo client examples complete![0m
✅ [32mAll Go integration tests complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.5.0 prepare
> npm run build


> @ekodb/ekodb-client@0.5.0 build
> tsc


up to date, audited 7 packages in 1s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.5.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 636ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'RwtfcKoNUr_fWiEJwI1KkdtHtPKGcVIee5nqphru3gkIGT4isOhpW8T7tkPS3vMMK8w0nxFa4jhqJDb1jN8naA'
}

=== Find by ID ===
Found: {
  id: 'RwtfcKoNUr_fWiEJwI1KkdtHtPKGcVIee5nqphru3gkIGT4isOhpW8T7tkPS3vMMK8w0nxFa4jhqJDb1jN8naA',
  value: { type: 'Integer', value: 42 },
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Test Record' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  id: 'RwtfcKoNUr_fWiEJwI1KkdtHtPKGcVIee5nqphru3gkIGT4isOhpW8T7tkPS3vMMK8w0nxFa4jhqJDb1jN8naA',
  active: { type: 'Boolean', value: true },
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
cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: VqcRojpDRxFQKPEwEHPniRelbuV_VLgMU9gmA4-E8Y-2qC57oLqvfsMLbRStvvXRoCb_B8HZ_RjAYLBQmUb_BQ

=== List Collections ===
Total collections: 18
Sample collections: schema_employees_client_py,batch_users,schema_products_client_py,test_collection,schema_users_client_py

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: fzI1WdkCgYnxfA7XE38XgIi_us9JSKk-yVJSS9iykg0iSbIvVikUr0PxvgsSBqEZKIbJWepdIbtOjhiaraaROA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: UARZYU1Mp-RHRiDRZRDVxZndlcfqkjBbMvnuEHmk3qtr_ruY2v-z_-Kk-h4RuVNch-uoGL5R-gXNMT8yjdVniA

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
✓ Inserted test record: RvrdMyjxRJMOPLJ8R-MxcpzP4p5WgRm6AD_NQISarNL52I1IsYiPKCEjY58pbsUn4VXM5j3QvYDXaYaTLuh-mA

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
✓ Inserted document with TTL: 5XGiAhp8FLl7M2MhevFYTCNzyDgRcSNyrFHjZGbLDqHIRH-w316t9lqAF-f3nbIiKt7zJvfyVLMX9KxEsLdE5g

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
✓ Created session: IzHf7BsJ4jQQZYkHnASDuH6Q-GE-MrC6PNjI4rRQqbXLf4vAglyIV2uiNJpPHY_tBwbJPi64ny1Gg_fZpcJMbg

=== Sending Chat Message ===
Message ID: vB44l134IKgUcKoY2AGx4hMhHpQpUJCeo8wYTKygpksPhH9UCX__jCRqemC0w_gYTJjGYon8GMsGXYHcDAxyWA

=== AI Response ===
There are three products available:

1. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

2. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

3. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 499,
    name: 'ekoDB Cloud',
    id: '8e9Lj1d4zgc1U9nahSV6Fo9D15oOZjNDqIK7c3DSmjEezDHrfHIfmx3KW8fkf4iTQSLJ-GaCI3bOgIeR4JXA0g',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    id: 'uKEPWu9zDSE7S4ZuBWB8mRJ9_stN9Z44_Dgi1kTYBJAyZf20CEF-gj7IzoD5sVa82_NB5PeoT-I9bZKrwA1o_w'
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
    id: 'KhnBgp13MWwbxUBYhbysrm9BBS4QfFXbLUChUrjGWC1TMTyEmP8AOv0gV7JC1FUPt9A7XEDMytidvzFJcQwq1Q'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2860ms

=== Token Usage ===
Prompt tokens: 609
Completion tokens: 84
Total tokens: 693

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Ac3k0yzskAnHczgWYQ4h5sEGvcZAe2ABZHX2z1V5Lxw7ftnRDcNl5CDJM3fIm7Lhi5HoOIOk0NCFC40S6wvPmg

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, one product that is available is "ekoDB". It is described as a high-performance database product. The price for this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: wItvB5T0wCB0DDfREuxr_JBLkqb3Ixu6UhXub0iL3kSNTnuoAJ4inZeCZrXsddoferEzCPYMcErnH4nusH9_lw
  Parent: Ac3k0yzskAnHczgWYQ4h5sEGvcZAe2ABZHX2z1V5Lxw7ftnRDcNl5CDJM3fIm7Lhi5HoOIOk0NCFC40S6wvPmg

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: wItvB5T0wCB0DDfREuxr_JBLkqb3Ixu6UhXub0iL3kSNTnuoAJ4inZeCZrXsddoferEzCPYMcErnH4nusH9_lw (Untitled)
  Session 2: Ac3k0yzskAnHczgWYQ4h5sEGvcZAe2ABZHX2z1V5Lxw7ftnRDcNl5CDJM3fIm7Lhi5HoOIOk0NCFC40S6wvPmg (Untitled)
  Session 3: IzHf7BsJ4jQQZYkHnASDuH6Q-GE-MrC6PNjI4rRQqbXLf4vAglyIV2uiNJpPHY_tBwbJPi64ny1Gg_fZpcJMbg (Untitled)
  Session 4: EN1u8WYJ-XYaHi9TyUf54f-TTSu0EMrkz__RCp9y29vsugdqMwNrgyEQPBPnox61oymGm6v6jMBP1LKSqwQpoA (Untitled)
  Session 5: Nl1MXe_ym3CqGP0-zV4n4927SZunPuOrsjYNzqtKLYRSh1xVU96TXy7VOfn9nmXIpAN3UJcY80zW23qzl_fKhg (Untitled)
  Session 6: v2sHPzLi65qdQ6hf-Js9BfbjCdw237_ldwjv4O9gOAJak9wodMbphJdBNk3fkb5WCgpDQ8I3EbhOABOA8t5SbA (Untitled)
  Session 7: KVh-vuJkxdu82QEem9mEJqlsehZ-MC_e1SnOJ7Xgd_lrHTjQZhpjlMgkO11L2jZt3heiLfxZmTU0K21jAkDofA (Untitled)
  Session 8: wm2yusSL9We4iCWtVQ8OgiRhxOTYG68-HqstgTHWeEx7_p2tmw4Wp59ymCN4bMg5hHgv9tlwAeuZZYkvGMQJiQ (Untitled)
  Session 9: _1CPODs-8nj298P5KnJz-t7BEQVsQOo6EyCog69m8gTNz_pANx0bruuIZ7qnAoLYTGBamNY1ZO3vaCNcVPD0Wg (Untitled)
  Session 10: hy9LUO2nl-RZUbTZUgrqYDj1mubOHylpZk3swm78FojI3ne4KfoLyDAjFLBhNK3WVW37L9ChwSPfRWPCGo3Ezg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: wItvB5T0wCB0DDfREuxr_JBLkqb3Ixu6UhXub0iL3kSNTnuoAJ4inZeCZrXsddoferEzCPYMcErnH4nusH9_lw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: rbT6xWmWKC2HItnoYS6kigTi5-vXWEtSo0--BG9dkPxgFAfAR-28PXgMV8WXQCi4temkscrMLutdtrBGWFvIiw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, one of the available products is ekoDB. It is a high-performance database product. The price for this product is $99.

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
✓ Created second session: 5gbl1VA8jEF5e8MibX2ZTwQwzLS9ELlWLLDovjgR9ZdY8veJbNXe8C4tBix4KaN1ZUc_1sMkPHgPJip88luaEA
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
  1. Score: 25.740, Matched: name, email, email.value, name.value
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title.value, bio, title
  2. Score: 26.400, Matched: bio.value, title.value, title, bio
  3. Score: 26.400, Matched: title.value, bio.value, title, bio
  4. Score: 26.400, Matched: bio, title, title.value, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, bio.value, title, title.value
  2. Score: 39.600, Matched: title.value, bio, bio.value, title
  3. Score: 39.600, Matched: title.value, title, bio, bio.value
  4. Score: 39.600, Matched: bio, title, bio.value, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.753, Matched: 
  2. Score: 0.747, Matched: 
  3. Score: 0.717, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.687, Matched: title, title.value, content.value, content
  2. Score: 1.499, Matched: title, title.value, content, content.value
  3. Score: 0.301, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, bio, skills.value, bio.value

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

✅ Script saved: u4dH2OjN4u7UcTsjlHazzvVdAhoNofiVMZn7G8iNAybf0siIMyth6zl-K6FeQM4wpl7wzkv99zR8eI4vzGGUaA
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: lnGE4IV9ViLzIEGcYeI11x1XNTsVipTEaTBFln1vNno3AJyTqEZX0fS6L95-uc9R_uY3ybEbTyEwJvvwgAJzlw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: p5DMInLi8KLBI9sz5Z4qrgM4mef0M8nKyVIfMkVb3LznA0r3UoAtGDEzZvihi8wkDRoEGKkcnHk8CYlLOxxcqQ
📊 Statistics: 2 groups
   {"count":5,"status":"inactive","avg_score":50}
   {"status":"active","avg_score":60,"count":5}

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

✅ Script saved: 2mBau_CKOBJ9O5y2PcJ7CgcdU6B1X6tpML9cSj0iIjCRYK9_ZUG0nIwEtU-1Y1zj1GQYOyYiRcnNEgVXWP8cnw
📊 Found 2 product groups
   {"category":"Electronics","avg_price":575.6666666666666,"count":3}
   {"avg_price":474,"category":"Furniture","count":2}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"category":"Electronics","count":3}
   {"count":2,"category":"Furniture"}
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
✅ [32mTypeScript client examples complete![0m
✅ [32mAll TypeScript integration tests complete![0m
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 629ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
(node:40357) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
(node:40360) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Inserted: {
  id: '7o2uUhZL5-bmHrfmnsKQOfBGp7EttkhG0fwxjO6B6v2oUDe4LVRlN43w8RE7XhBH0ijs9TdLLVnaCMga-YHUYw'
}

=== Find by ID ===
Found: {
  id: '7o2uUhZL5-bmHrfmnsKQOfBGp7EttkhG0fwxjO6B6v2oUDe4LVRlN43w8RE7XhBH0ijs9TdLLVnaCMga-YHUYw',
  active: { value: true, type: 'Boolean' },
  name: { value: 'Test Record', type: 'String' },
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  id: '7o2uUhZL5-bmHrfmnsKQOfBGp7EttkhG0fwxjO6B6v2oUDe4LVRlN43w8RE7XhBH0ijs9TdLLVnaCMga-YHUYw',
  active: { type: 'Boolean', value: true },
  value: { value: 100, type: 'Integer' }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js ===[0m
✓ Authentication successful

=== Inserting Test Data ===
(node:40382) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted test record: oSyC2C2D8L2hZE9j_Ho030C0ubOVbSDAbXEnU-y-WVhshprVbA1mPP-zl3KwtqEf3sq7lgksYClPBMyC-Wjm6w

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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "Kd7MuNthn11mq4fPbeELKVfNy-FciWdlj1aoIA441hvPMlKU0ZCKc0mBikFpaf1fbEYf6jXXevgjFsdeggqXNQ",
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
        "id": "oSyC2C2D8L2hZE9j_Ho030C0ubOVbSDAbXEnU-y-WVhshprVbA1mPP-zl3KwtqEf3sq7lgksYClPBMyC-Wjm6w",
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
        "id": "rOrZMWqDh7WWVTLDxIikzWv8TgnmEIw4VFgPZB6WZt6mnTipX67B6IVw3iRtb_yijeZztQjufYQyn0AN2m3wDQ",
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
(node:40398) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
(node:40411) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {
  loginTime: '2025-12-28T23:18:36.082Z',
  userId: 123,
  username: 'john_doe'
}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

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
(node:40418) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Collection created with first record: uBBeQZKUfxhEo4KXFgTnRRgcMFoFTi-lcUnv_Y3nH3gknJiQLb5Pgs9pWqEJo2dl-J4Az4VQo2iaDtUILfr1qA

=== List Collections ===
Total collections: 23
Sample collections: [
  'schema_employees_client_py',
  'batch_users',
  'schema_products_client_py',
  'test_collection',
  'schema_documents_client_ts'
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
(node:40422) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document: xfGbx_SLsXqWJvQ4qzQBWvAAF9hzgXQ6VEmKHpKW5Y9jXkq_pFEz8eFlg8zTKu9DCDdczfVmAk1jEMyBGgJ4Qg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: created

=== Query Documents ===
✓ Found 4 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully
[32m✓ document_ttl.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js ===[0m
✓ Authentication successful

=== Insert Test Data with TTL ===
(node:40437) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document with TTL: dbOBgMKN483WF3fzpDI0rCrIgICl5ecghsjXp87C0vPDG9AMisVAsDtzBrq5jk1eTcjvjUGkSwouA4gHzpYg2w

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

✅ Script saved: Y9zsBjOJ-t0D5S2ossC6bXE3lLawKJD5K2qJteBIexu8fD6aYpw9_-cpnuVdKZzsTGn4KMRMrPxHcARqYCwRbg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: pXnadNXvQMF2CJ5Uv0Nxv5qujrq9WeVgqn58kv3hr2XZlapzOHKzWec61z_XCZRjxCPr91-XXq9TL2COJLgAKg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 8B0DHwPmyW4sMjbC_3mAG-iCfqAAy7hFA-efRXLqGbDS3pEVNHeIQeLUz_kJlxLcAPxxjFPy_GIB5JROE3Y1Lw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":"inactive","count":10,"avg_score":50,"max_score":90}
   {"avg_score":60,"status":"active","count":10,"max_score":100}

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
Created Alice: $1000 - ID: E38LWi88fGe2V8pCX1dL57efqfeo42u_Gtid9phym6asmsanAXuYLMCK-pMQ-wDm43PKDq9Sz6ekHkVp2GTIgA
Created Bob: $500 - ID: JahHKpprr7MdOuA7amqZ1JuwEbXAzB9_m824PI5Ac-v2oALtOhZkSoALDItS12W2XBcQG9MgIsnXXKUL1YioCA

=== Example 1: Begin Transaction ===
Transaction ID: 2fc03ec3-f837-4023-a3af-d5c1b5029d18

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
New transaction: fe05f818-2ece-42ed-b2ae-55fa762781bf
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
   ✅ Script saved: wR-Pj1lSODT9IFD7RTg2u7668YjfYhg2AGYLRCldLLHVwDFQBBQIW6mFusDErxsq9PGh2kFtJnJOPxmxh6eUzA

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
   ✅ Script saved: mxLQwvrmKOwnvi8z21qOhqCiTXvg1ntEBsMDTkMC_rwCWobB_AzF6a68fuVU4J5CHJkCyURT_lvC9gb2cpaJUg

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
   ✅ Script saved: THsSDujnA8XZz0FbDorYsFyriZdXMrWZPou0phYqYQvyKzEl6VARhJUjagm0iVnQu7B4zh1X259JwdzcZE2Nig

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
   ✅ Script saved: HvGweZMHpvcvpko7NU6_2tZwH8Or_OQdkVGY9H0lb0vL5pF4hSrHHkA8EsH5Qu8yQC1TMjYUfIinBybyEhhFew

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: wR-Pj1lSODT9IFD7RTg2...
   ✅ Deleted script: mxLQwvrmKOwnvi8z21qO...
   ✅ Deleted script: THsSDujnA8XZz0FbDorY...
   ✅ Deleted script: HvGweZMHpvcvpko7NU6_...
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

> @ekodb/ekodb-client@0.5.0 prepare
> npm run build


> @ekodb/ekodb-client@0.5.0 build
> tsc


up to date, audited 7 packages in 849ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.5.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 669ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'JTmdZ2iMz3AOHgnXyHG1yvkrkQzfpeO1C0JmO-3FCLvnugB3O-rkGeQghai9LR9aoL_wbJIx8pOnBe9cD5iSqg'
}

=== Find by ID ===
Found: {
  value: { type: 'Integer', value: 42 },
  name: { type: 'String', value: 'Test Record' },
  id: 'JTmdZ2iMz3AOHgnXyHG1yvkrkQzfpeO1C0JmO-3FCLvnugB3O-rkGeQghai9LR9aoL_wbJIx8pOnBe9cD5iSqg',
  active: { type: 'Boolean', value: true }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  active: { type: 'Boolean', value: true },
  name: { value: 'Updated Record', type: 'String' },
  id: 'JTmdZ2iMz3AOHgnXyHG1yvkrkQzfpeO1C0JmO-3FCLvnugB3O-rkGeQghai9LR9aoL_wbJIx8pOnBe9cD5iSqg'
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
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: E3YcJZpDiWHTUdSo7ofUkXJGJp38OjantvnWsMcZ3QAPB3Nn7kh6Gj54jYgIGZwl1BbvOUDffYv_FEXqg10-6A

=== List Collections ===
Total collections: 22
Sample collections: client_collection_management_js,schema_employees_client_py,batch_users,schema_products_client_py,test_collection

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: AJzCLtqfo1LYxVfJVQ8AQonx6X9CHzHEEKAwvhVgR6Nvj3L0rFZ49ZAg4lEcHXXMBwmPkseDa6bwtZe0qTsbGA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 0UQrVjkCKrEc9Yc0Ew0oAEaD0GWLhH4ny6FUbCVI58qFI8kTzBjFY2mPcWemxtPUZu1l3OWzYzBjz01akIDGtA

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
✓ Inserted test record: bLNOTApLb41EeGMAs-w34aWZql0MpY_YakaMt3yo-SHGSNyugXAo1zSLT8hjQQf2Yu8-9kFjj6KlB1MUh31Otg

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
✓ Inserted document with TTL: N4-FiVGrwZULeu7ILlp35OuI5CHHZ3KVmIkXVkipu3vQd-zpiO-P0xty0Gf2z9FWk358EixgulMkFcTuKFdUfA

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

✅ Script saved: rUFPt5_cCtt77CL-7zsF1pa1VJqCRotKcfO9iKF1BQNQ3yItawSvFrmscNqjeoPGiRNqPLstFg9ryASBJWNwzg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: js6mFiPDU8D5MMRjxlfEqlFWN3ydzpfvPb4Sj54_vBBa3Aiq2UqZGJbELgKRyoaSllvMq9nrBiFY_lyRPs1Ypw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 3gCHJWkOJMMAUOh4_zADXucd8yC93K2i3f6ZNTE-lFxf9gPn2DJhqJtEd7kJbN1pJFJyXIW4hiofdWdOoasqzA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":5,"avg_score":50,"max_score":90,"status":"inactive"}
   {"count":5,"max_score":100,"avg_score":60,"status":"active"}

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

✅ Script saved: WnAu3G9pLhdhUIR5HM1MGB8H9vjNuna-8q2T1Q4y47FhzObMeQTjIY_u25RSD62lC63XXIePkFpZv6jNPp1n8w
📊 Found 20 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 20 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"avg_score":50,"status":"inactive","count":10}
   {"count":10,"avg_score":60,"status":"active"}
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
   1. Monitor 27" - $399 (⭐4.6)
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
📊 Users (showing first 5 of 10):
   1. User 10 (user10@example.com) - Score: 100
   2. User 2 (user2@example.com) - Score: 20
   3. User 5 (user5@example.com) - Score: 50
   4. User 1 (user1@example.com) - Score: 10
   5. User 7 (user7@example.com) - Score: 70
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved
📊 User summary (2 groups):
   inactive undefineds: 3 users, Total Score: 180
   active undefineds: 7 users, Total Score: 370
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
   2. Vector Databases Explained (Database)
   3. Introduction to Machine Learning (AI)
   4. Database Design Principles (Database)
   5. Getting Started with ekoDB (Database)
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
   2. Vector Databases Explained
   3. Introduction to Machine Learning
   4. Database Design Principles
   5. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Natural Language Processing
   2. Vector Databases Explained
   3. Introduction to Machine Learning
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Natural Language Processing (AI)
   2. Vector Databases Explained (Database)
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
   Vector databases offer several benefits:

1. High Precision: Vector data is represented in points, lines, and polygons, providing a high level of detail and accuracy.

2. Scalability: Vector data can be easily scaled up or down without losing quality, which is crucial for zooming or mapping at various scales.

3. Geographical Analysis: Vector databases allow for complex geographical operations like calculating area, perimeter, or distance.

4. Data Storage: They use less storage space compared to raster databases for representing simple, sparse geographical features.

5. Integration Ability: Vector databases can easily integrate with other data types, allowing for more complex and layered GIS analysis.
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
✓ Inserted user: Record(fields={id=StringValue(value=WDmcWxvOvgBa7AkmWgmBZTZbH_g4J1DdV5v9P0fqWH-FDccF1knTGYDFg2YV-eewkHLaUVjVsuk6ZTH8J79OtQ)})
  User ID: WDmcWxvOvgBa7AkmWgmBZTZbH_g4J1DdV5v9P0fqWH-FDccF1knTGYDFg2YV-eewkHLaUVjVsuk6ZTH8J79OtQ

=== Read ===
✓ Found user by ID: Record(fields={age=ObjectValue(value={value=IntegerValue(value=28), type=StringValue(value=Integer)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), id=StringValue(value=WDmcWxvOvgBa7AkmWgmBZTZbH_g4J1DdV5v9P0fqWH-FDccF1knTGYDFg2YV-eewkHLaUVjVsuk6ZTH8J79OtQ), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)})})

=== Update ===
✓ Updated user: Record(fields={id=StringValue(value=WDmcWxvOvgBa7AkmWgmBZTZbH_g4J1DdV5v9P0fqWH-FDccF1knTGYDFg2YV-eewkHLaUVjVsuk6ZTH8J79OtQ), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)})})

=== Query ===
✓ Found 1 users matching query
  - Record(fields={age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), id=StringValue(value=WDmcWxvOvgBa7AkmWgmBZTZbH_g4J1DdV5v9P0fqWH-FDccF1knTGYDFg2YV-eewkHLaUVjVsuk6ZTH8J79OtQ), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)})})

=== Delete ===
✓ Deleted user with ID: WDmcWxvOvgBa7AkmWgmBZTZbH_g4J1DdV5v9P0fqWH-FDccF1knTGYDFg2YV-eewkHLaUVjVsuk6ZTH8J79OtQ

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
  IDs: jTegHbTaUmQJ5O_dPQ29NE6eRWwqgLcTOQPH4At-u0rz22TTE78YBSJt08vsRVCxXeH_fxiELmWkNYIG26Ub3w, nR4LaRuVnKtuTm2tglJO3luOZsawJFZzkgnZdiNQgpIsEeTx2v_LKlbo6jRiXgDplQqFXSzzMpgu3Ilqzj-srw, ToHS9AVdv-UElaz22ZbSt_zdZpIr7IhtFPHA1SrpaByeW6f3YGlZ2a_DKz4OBBh1NjjfJZwhJq8y5d7FySaAnw...

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
  - Score: ObjectValue(value={value=IntegerValue(value=92), type=StringValue(value=Integer)})
  - Score: ObjectValue(value={value=IntegerValue(value=88), type=StringValue(value=Integer)})

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
✓ Retrieved value: {"name":"Alice","email":"alice@example.com","role":"admin"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"created_at":1766963961041,"user_id":"123"}
  (Will expire in 10 seconds)

=== Set Multiple Keys ===
✓ Set 3 configuration keys

=== KV Delete ===
✓ Deleted key: user:123

=== Verify Deletion ===
✓ Confirmed key was deleted

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
  - schema_employees_client_py
  - batch_users
  - schema_products_client_py
  - test_collection
  - schema_documents_client_ts

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-12-28T23:19:25.256180Z","last_modified":"2025-12-28T23:19:25.256180Z","bypass_ripple":false,"primary_key_alias":"id"}

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
  Document ID: VLd5FiRemabrELtmbE1F59EK0tobUOvWj1QzjBIMVc_BGxMJ3z0aTUpOmAG0pA7zM1k65VTQIcHsx_r2nA1jDQ

=== Verify Document Exists ===
✓ Document found: [user_id, session_id, created_at, id, ttl_duration]

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: D2LyG9hjvb6NV56JWqXausUfIAWTn0QLHRHDvvQiufwsR1eNHdxa0CQr1hywkIgGC0UYd7xz_TjZGlaUDkCdfw

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: [id, ttl_duration, cache_key, value]

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
=== ekoDB Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client ===
✓ WebSocket client created

=== Connect to WebSocket ===
✓ Connected to WebSocket

=== Insert Test Record ===
✓ Inserted test record

=== Find All via WebSocket ===
✓ WebSocket findAll result:
  {"payload":{"data":[{"id":"rjjPwwTsnoing3tdEsMKQRL6XiLzccfzFmaFgTnBwC0qAsHCXJAplvGvBw2n0zu_DAbRv_kvuLS_9hZfGrOTLw","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

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
✓ Inserted user 1: StringValue(value=ykOeUb2yG4bH7C1JSfeUROvQ-l9YR33GbxIq3RoyirkPasw85Evf-_uXuipBoO9LQVVQnQ2uOa2GcYWlwMxVGQ)
✓ Inserted user 2: StringValue(value=wkQ6ZncNOAW9V1KRN9Ute2-uiv7xt4sNRrfx8UhqxC1msFGAzFtLsw_V78vbtAyI47Ev5byY8HWWIib-gdbzAg)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 22
  Sample: [schema_employees_client_py, batch_users, schema_products_client_py, test_collection, schema_documents_client_ts]

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
  {"results":[{"record":{"title":"Rust Programming","description":"Learn Rust programming language with hands-on examples and best practices.","id":"igciM42xxBu6kuvOW83TRpRmnWWIFNF3m1QcDOE-zbQRyMulRLWmU0fnjnl4DCFMwZlX09-cfpaZ7WAyrAc2KA","views":424,"tags":"programming,rust,tutorial"},"score":19.8,"matched_fields":["title","description","tags"]},{"record":{"title":"Python for Data Science","id":"saj7TVQ185BxxZFZucm0zU4zhojmVmKajgNcGwDNmltCRHFXc5GYzAIhV3FR9-ywESPONrz58Zy8oYTjOLf7SA","views":443,"tags":"programming,python,data-science","description":"Master Python for data analysis, machine learning, and visualization."},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"description":"Build modern web applications using JavaScript, React, and Node.js.","tags":"programming,javascript,web","title":"JavaScript Web Development","id":"CyBH1uhu2i4FNHHAaamfS9B2Eo6CNiuneNdQlLuyJMflu1SLspmBHal_AVHNWd9cZZzK2XUAG6yJohypwUx7jQ","views":255},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"id":"6rAtUdOZXVrZPrKXYr9lDJyXJfd7ki_ckvZWf7JhynWF9AI2fa-w9zosyLOJNFAm3Fw5925RHLhJwSNODwXubA","tags":"ai,machine-learning,python","title":"Machine Learning Basics","views":611,"description":"Introduction to machine learning algorithms and neural networks."},"score":2.7,"matched_fields":["tags","description","title"]},{"record":{"title":"Python for Data Science","description":"Master Python for data analysis, machine learning, and visualization.","tags":"programming,python,data-science","views":443,"id":"saj7TVQ185BxxZFZucm0zU4zhojmVmKajgNcGwDNmltCRHFXc5GYzAIhV3FR9-ywESPONrz58Zy8oYTjOLf7SA"},"score":1.0,"matched_fields":["description"]},{"record":{"tags":"programming,rust,tutorial","description":"Learn Rust programming language with hands-on examples and best practices.","title":"Rust Programming","views":424,"id":"igciM42xxBu6kuvOW83TRpRmnWWIFNF3m1QcDOE-zbQRyMulRLWmU0fnjnl4DCFMwZlX09-cfpaZ7WAyrAc2KA"},"score":0.5,"matched_fields":["description"]},{"record":{"description":"Learn database design principles, normalization, and query optimization.","title":"Database Design","tags":"database,design,sql","views":942,"id":"g8Vl-Jp64G1Rke3Uxh6bxIVPv5p_mJtWw1v2s1c0cuamQCcwitv_E33szT4BIAhyMaL0JU9MuMMGHZSrhXh7Qw"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
✓ Inserted document with TTL: StringValue(value=BrRmN_twtPw9teHUiQckWrMI83hrPetll_sQy1NNA2Ifnk-7ARJ3VahAGTo7jOowkYwuJCVcjxSqyyRIyAM1eg)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1766963987164},"id":"BrRmN_twtPw9teHUiQckWrMI83hrPetll_sQy1NNA2Ifnk-7ARJ3VahAGTo7jOowkYwuJCVcjxSqyyRIyAM1eg","name":{"type":"String","value":"WebSocket TTL Test"},"ttl_duration":{"type":"String","value":"1h"},"value":{"type":"Integer","value":42}}]},"type":"Success"}

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
  - Mouse ($25) by Alice Johnson
  - Laptop ($1200) by Alice Johnson

=== Example 5: Complex join with multiple conditions ===
✓ Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✓ Deleted test collections

✓ Join operations example completed successfully

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
=== ekoDB Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: -fJGYNHaUc4qGM-6FVy-ZITOaXRVHbs_AiO1GE6zz_yBr74YiUS4-WPVgAa7g84Od848WjrPUmbJeXYEfDzjFA

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "NnIoGbin41mL9NBlO7IR84nQoDVn3vJMUrXXAEBf_uyV3cK-J0oTYG3tD1mHDXjPy97JRNr1tABVYBTC4dUOhg"
  Responses: ["Answer: ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration. It offers several notable features. One of these is the AI Chat Integration, which allows you to query your database using natural language and receive AI-powered responses with relevant context. ekoDB also supports different types of search features, including full-text search, vector search, and hybrid search with automatic context retrieval."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

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
=== ekoDB Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: olkmFM_uc66QC_cz1eotsyc8efUQ5ob_OTp8DnjkwV2T4GAoi3e1RNsH9K72_cnZOzK44bkur9vbhoEc_rSClw

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The available product is \"ekoDB\", a high-performance database product with AI capabilities. The price for this product is $99."]

✓ Message 2 sent
  Responses: ["The price of the ekoDB product is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"olkmFM_uc66QC_cz1eotsyc8efUQ5ob_OTp8DnjkwV2T4GAoi3e1RNsH9K72_cnZOzK44bkur9vbhoEc_rSClw"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"x9cEHn6Xkh0aBng_xYDKiWVf08yWnyD6oZJs1Y1gpsfdLgMiWpFGMYEbXXfsQh-RyHqZUM6HyDWfQcGZO-MThg","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-28T23:20:06.327938Z"},"id":"9L9v0fXygQchb44D29hoc6FR_7ONs_0umf0mitR0aG5qg8itWAnaIC9BL6hP5Cw8T3kGynCCowz5PV_aQJ0n-g","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":26,"prompt_tokens":240,"total_tokens":266}},"updated_at":{"type":"DateTime","value":"2025-12-28T23:20:06.327938Z"}},{"chat_id":{"type":"String","value":"olkmFM_uc66QC_cz1eotsyc8efUQ5ob_OTp8DnjkwV2T4GAoi3e1RNsH9K72_cnZOzK44bkur9vbhoEc_rSClw"},"content":{"type":"String","value":"The available product is \"ekoDB\", a high-performance database product with AI capabilities. The price for this product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"x9cEHn6Xkh0aBng_xYDKiWVf08yWnyD6oZJs1Y1gpsfdLgMiWpFGMYEbXXfsQh-RyHqZUM6HyDWfQcGZO-MThg","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-28T23:20:06.333539Z"},"id":"NzQbXv2IXRHiiwJ_xGjW8rwnsAGtKGeFsgq18JZHIen61qbNaBfgI_vb0ebdM7OCoRPqnpyD2rUSzDV9wA-XoQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":26,"prompt_tokens":240,"total_tokens":266}},"updated_at":{"type":"DateTime","value":"2025-12-28T23:20:06.333539Z"}},{"chat_id":{"type":"String","value":"olkmFM_uc66QC_cz1eotsyc8efUQ5ob_OTp8DnjkwV2T4GAoi3e1RNsH9K72_cnZOzK44bkur9vbhoEc_rSClw"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"x9cEHn6Xkh0aBng_xYDKiWVf08yWnyD6oZJs1Y1gpsfdLgMiWpFGMYEbXXfsQh-RyHqZUM6HyDWfQcGZO-MThg","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2025-12-28T23:20:07.097744Z"},"id":"z-SYW1ZrQWNu6HbCJzGP2v0tpaJEkL7CsAZMvygfFOjJslmDvK4TX3iA44FH_JXrV_eZcWL8__8t9T4GLtWtvQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":240,"total_tokens":252}},"updated_at":{"type":"DateTime","value":"2025-12-28T23:20:07.097744Z"}},{"chat_id":{"type":"String","value":"olkmFM_uc66QC_cz1eotsyc8efUQ5ob_OTp8DnjkwV2T4GAoi3e1RNsH9K72_cnZOzK44bkur9vbhoEc_rSClw"},"content":{"type":"String","value":"The price of the ekoDB product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"x9cEHn6Xkh0aBng_xYDKiWVf08yWnyD6oZJs1Y1gpsfdLgMiWpFGMYEbXXfsQh-RyHqZUM6HyDWfQcGZO-MThg","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2025-12-28T23:20:07.103944Z"},"id":"l3nOK1ZM9S0tY-Tmy8W3l1WslDqIp5aLlt0TP4ull-jGd5RiXAVS3bE4OjBjNpHEqcUZWyq-Vesn-SQiTBYPjg","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":240,"total_tokens":252}},"updated_at":{"type":"DateTime","value":"2025-12-28T23:20:07.103944Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 13

=== Branching Session ===
✓ Created branched session: h8trlpy3sCIEw-e-2V0xhgj64yRknHkHtFzBw1TxWhhsj_WJeq_VAAaPXmsdIL56VWxL7s5lOJR5fk8CC--0gg

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
✓ Created session: tzh5ke_VnHbtP3caQvNKiEFf-kz1e9pcu0tUkA_I-S7Arl7WARjT8YtS2hj7y5huaONXzql6Us8T6elA11Pw4A

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the context provided, one of the available products is ekoDB. This is a high-performance database product."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Based on the context provided, the available product is ekoDB. It's a high-performance database product priced at $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: FQTloqDAr6ok5cs9xZWxuxtBeNt53grehmxcwBY4QIpS7vSVhuR_FA8_3zBFEsEAsOK2OAY3Hlt9p5GlhFeMdw

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

✅ Script saved: NAvCqRxeiN2CZ9d5MNz8ytZZTBaNyFa1JYYcnfyNJbtbfrf6IxXS9LogsguL_UP9hLzSlavUol0lDMZOgjBSKg
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"status":"active","count":5,"avg_score":60.0}
   {"status":"inactive","count":5,"avg_score":50.0}
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
/bin/sh: kotlinc: command not found
Kotlin example (requires kotlinc installed)
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
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.5.0-cp38-abi3-macosx_11_0_arm64.whl
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
    ✓ Generated embedding: 1536 dimensions in 458.951875ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 788.180291ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 606.630875ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 287.829333ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 267.182084ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 422.509625ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 404.699834ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 312.177958ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 618.995458ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 477.622917ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 292.184458ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 349.5575ms
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
    ✓ Generated embedding: 1536 dimensions in 644.758167ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 107.970292ms

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

Writing memory-safe, high-performance database code requires a fair understanding of both the database system you are using (whether it's SQL or NoSQL) and the programming language you're coding with. Here are some general steps you can follow:

1. **Understand Your Database**: Whether you're using SQL or NoSQL, understand the strengths and weaknesses of your database. For example, if you're using SQL, remember to use its ability to make complex queries and ACID transactions. If you're using NoSQL, take advantage of its flexible schemas and ability to handle high write throughput.

2. **Normalize Your Database**: As mentioned in context 3, database normalization is essential to reduce redundancy and improve data integrity. This not only makes your database more efficient but also makes your code safer as it reduces the chances of data inconsistency.

3. **Optimize Your Queries**: The efficiency of your database code is often determined by the efficiency of your queries. Make sure your queries are as precise as possible, retrieve only what's necessary, and avoid unnecessary calculations. Indexing can also greatly improve your query performance.

4. **Connection Handling**: Ensure that your code properly opens, uses, and closes database connections. Improper handling of database connections can lead to memory leaks and other performance issues.

5. **Use Prepared Statements**: Prepared statements not only help prevent SQL injection attacks, but they also often perform better than plain text SQL queries.

6. **Error Handling**: Proper error handling is crucial to writing memory-safe code. Make sure your code is equipped to handle any database errors that may occur.

7. **Profiling and Monitoring**: Regularly profile and monitor your database performance. This can help you identify any bottlenecks or memory issues.

Remember, the specifics of how you implement these steps will depend on the specific database system and programming language you're using.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 629.121875ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 1983 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 341.6645ms
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
  ✓ Text search completed in 68.187292ms

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_new_question: Writing memory-safe, high-performance database code requires a fair understanding of both the database system you are using (whether it's SQL or NoSQL) and the programming language you're coding with. Here are some general steps you can follow:

1. **Understand Your Database**: Whether you're using SQL or NoSQL, understand the strengths and weaknesses of your database. For example, if you're using SQL, remember to use its ability to make complex queries and ACID transactions. If you're using NoSQL, take advantage of its flexible schemas and ability to handle high write throughput.

2. **Normalize Your Database**: As mentioned in context 3, database normalization is essential to reduce redundancy and improve data integrity. This not only makes your database more efficient but also makes your code safer as it reduces the chances of data inconsistency.

3. **Optimize Your Queries**: The efficiency of your database code is often determined by the efficiency of your queries. Make sure your queries are as precise as possible, retrieve only what's necessary, and avoid unnecessary calculations. Indexing can also greatly improve your query performance.

4. **Connection Handling**: Ensure that your code properly opens, uses, and closes database connections. Improper handling of database connections can lead to memory leaks and other performance issues.

5. **Use Prepared Statements**: Prepared statements not only help prevent SQL injection attacks, but they also often perform better than plain text SQL queries.

6. **Error Handling**: Proper error handling is crucial to writing memory-safe code. Make sure your code is equipped to handle any database errors that may occur.

7. **Profiling and Monitoring**: Regularly profile and monitor your database performance. This can help you identify any bottlenecks or memory issues.

Remember, the specifics of how you implement these steps will depend on the specific database system and programming language you're using.

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
    ✓ Generated embedding: 1536 dimensions in 0.251s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.468s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.564s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.289s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.263s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.237s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.268s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.277s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.306s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.221s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.325s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.330s
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
    ✓ Generated embedding: 1536 dimensions in 0.300s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.099s

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

Writing memory-safe, high-performance database code requires a combination of good coding practices, careful database design, and a deep understanding of the database system you're using. Here are a few general tips:

1. **Use prepared statements**: Prepared statements not only help prevent SQL injection attacks, but they can also improve performance by allowing the database to cache and reuse query plans.

2. **Optimize your queries**: Avoid fetching more data than you need. For example, use SELECT specific columns instead of SELECT *. Also, take advantage of indexes for faster data retrieval. 

3. **Database normalization**: As per context 2 and 3, database normalization is a process to organize data to reduce redundancy and improve data integrity. This can lead to better overall performance.

4. **Use the right database type**: If your data is unstructured or if you need high write throughput, horizontal scaling, and flexible schemas, a NoSQL database could be more efficient, as per Context 1.

5. **Close Connections**: Always close database connections when they're not in use. Open connections consume resources.

6. **Use transactions wisely**: Transactions help maintain data integrity, but they can also lock the database and cause performance issues. Use them only when necessary.

7. **Memory Management**: In your program, always free up resources that are no longer needed, especially large data structures, to avoid memory leaks.

8. **Concurrency**: Utilize multithreading or multiprocessing where applicable to allow multiple operations to run simultaneously.

In addition to these, the specific methods to improve performance can greatly depend on the particular database system being used. For instance, different databases have different ways to define indexes, partition data, manage memory, etc. Therefore, understanding the specific database documentation well can be highly beneficial.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.245s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 1923 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.350s
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
    ✓ Generated embedding: 1536 dimensions in 0.211s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.256s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.501s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.339s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.397s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.557s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.214s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.258s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.478s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.420s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.254s
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
    ✓ Generated embedding: 1536 dimensions in 0.293s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.102s
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

Writing memory-safe, high-performance database code involves a combination of best practices from both database management and software development. Here are some general tips:

1. **Optimize your queries:** As discussed in Context 1, optimizing your queries is crucial. This can be done by removing any unnecessary columns, using WHERE clauses to filter data properly, using JOINs effectively, and avoiding nested queries where possible. Using indexes can also significantly speed up data retrieval.

2. **Database Design:** Your database design plays a key role in performance. As mentioned in Context 5, normalization can reduce data redundancy and improve integrity. However, it's also important to note that in some high-performance applications, a degree of denormalization might be beneficial, as it can reduce the number of table joins required.

3. **Choose the right database system:** As mentioned in Context 2 and 3, the choice between SQL and NoSQL databases can impact performance. SQL is good for structured data with complex queries and ACID transactions. NoSQL is good for unstructured data with high write throughput, flexible schemas, and horizontal scaling.

4. **Memory Management in Code:** When writing your code, be mindful of memory usage. Avoid loading large amounts of data into memory at once, and free up memory that's no longer needed. In a garbage-collected language like Java or Python, this usually means making sure there are no remaining references to objects you're done with. In a language with manual memory management like C++, it means properly using `delete` or smart pointers.

5. **Use Efficient Data Structures:** Certain data structures are more efficient than others for certain tasks. For instance, using a Hash Map can provide constant-time lookups, which can be beneficial when dealing with large datasets.

6. **Concurrency Control:** When multiple users or processes are accessing your database simultaneously, concurrency control becomes important. Techniques like locking, optimistic concurrency control, or using transactions can help ensure data integrity while maintaining high performance.

7. **Caching:** Frequently accessed data could be stored in cache to improve performance. Database systems often include their own caching mechanisms, but application-level caching can also be beneficial.

8. **Batch Operations:** Instead of performing database operations one at a time, batch them together where possible. This can reduce the number of round trips between your application and the database, improving performance.

9. **Monitoring and Profiling:** Regularly monitor and profile your code to find bottlenecks. Tools like slow query logs, performance schema, or explain plans can help you understand where your database code might need optimization.

Remember, high-performance code also needs to be safe and maintainable. It's important to balance these considerations to create robust, efficient applications.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.346s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2975 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.238s
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
  ✓ Text search completed in 0.059s
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
    ✓ Generated embedding: 1536 dimensions in 0.233s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.445s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.308s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.257s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.222s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.295s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.322s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.249s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.243s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.368s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.283s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.346s
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
    ✓ Generated embedding: 1536 dimensions in 0.365s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
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
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code can vary based on the type of database you are using (SQL or NoSQL), but here are a few general guidelines:

1. **Optimize your queries**: This was discussed in Context 1. To do this, you can use techniques like indexing, avoiding the use of wildcard characters at the beginning of LIKE patterns, using the LIMIT clause to limit the number of rows returned by a query, and running EXPLAIN PLAN to understand how your query is being executed.

2. **Choose the right database type**: As discussed in Contexts 2 and 3, the choice between SQL and NoSQL can significantly impact performance. If you need to handle unstructured data, require high write throughput, or need horizontal scaling, a NoSQL database may be more efficient. Conversely, if you are working with structured data and need to perform complex queries or ACID transactions, SQL might be the better choice.

3. **Normalize your database**: As mentioned in Context 4 and 5, database normalization can improve data integrity and reduce redundancy, which can in turn improve performance. By dividing large tables into smaller ones and defining relationships with foreign keys, you can ensure more efficient operations.

4. **Memory Management**: Be mindful of how your database operations use memory. Avoid loading large datasets into memory all at once, instead, try to process data in chunks. In SQL, avoid using large temporary tables when not necessary as this can consume significant memory.

5. **Use prepared statements**: Prepared statements not only protect against SQL injection attacks but they also can improve performance by allowing the database to cache the query plan and reuse it.

6. **Close connections**: Always ensure database connections are closed promptly after use. This can prevent memory leaks and improve performance by freeing up system resources.

7. **Concurrency control**: Implement concurrency control mechanisms like locks, semaphores, or optimistic concurrency control depending on your use-case to ensure memory-safe operations.

8. **Consider using a connection pool**: Connection pooling can greatly enhance performance for applications that require frequent short interactions with the database by reducing the overhead of establishing a new connection every time.

Remember, each application and system can have unique requirements, so it's essential to benchmark and test different strategies to identify what works best for your specific use case.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.336s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2501 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.277s
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
  ✓ Text search completed in 0.059s

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
    ✓ Generated embedding: 1536 dimensions in 0.358s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.234s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.28s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.333s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.262s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.272s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.461s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.304s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.279s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.344s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.255s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.273s
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
    ✓ Generated embedding: 1536 dimensions in 0.264s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.09s

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
    ✓ Generated embedding: 1536 dimensions in 0.322s
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
  ✓ Text search completed in 0.047s

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
