make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("Hx40Bo2AMC-R5-sCDJs0maAGBoUi0bnK-TEMsJIo3XLzOa6du-AC6N4OSphDGH6d3Z9Ib8CHDRIClRz6WUjyBg")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("Hx40Bo2AMC-R5-sCDJs0maAGBoUi0bnK-TEMsJIo3XLzOa6du-AC6N4OSphDGH6d3Z9Ib8CHDRIClRz6WUjyBg"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("Hx40Bo2AMC-R5-sCDJs0maAGBoUi0bnK-TEMsJIo3XLzOa6du-AC6N4OSphDGH6d3Z9Ib8CHDRIClRz6WUjyBg"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("Hx40Bo2AMC-R5-sCDJs0maAGBoUi0bnK-TEMsJIo3XLzOa6du-AC6N4OSphDGH6d3Z9Ib8CHDRIClRz6WUjyBg"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "OIcUeukTb5AoykKpQDNuJeW36cIo-7vd-q8gp7gPtlnHoXGHToZKYM_zNufISLbfCfwaSx6cT6aRhWPJ5ALqJA"

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
        "id": "OIcUeukTb5AoykKpQDNuJeW36cIo-7vd-q8gp7gPtlnHoXGHToZKYM_zNufISLbfCfwaSx6cT6aRhWPJ5ALqJA",
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
Collection created with first record: "I6ngAtJrG_98dRekoCgd5Bh7K2z_ctE786XTgCjPcgbX7pCEqm3Ez04A5UMLtZwSY9kkKAsqFKkD1QzjcICNYQ"

=== List Collections ===
Total collections: 4
Sample collections: ["demo_collection", "test_collection", "websocket_test", "batch_users"]

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
✓ Inserted document: "TwQ_9NZkiAke-L0vc7eyaoBXWREX0fgGeJRy7_Yw937F5mDR4abOr_sMffm7T_ZtWg2Xv3mdotPDmMEogVdbJw"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: null

=== Query Documents ===
✓ Found 1 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("be5e5cvqOv73th6f8OguMy9Q50pqesV5eRSPsIJUtBHPKMpP9Q23-MtoHGVxJCs1dhlEVYn8UZDQcH0zSK-ASA")

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

✅ Script saved: jkpizSO0niS31d47cfIf-k0ZRJtvr2sJ9-8sSbY0q4kZkR9aEwZJJ6rgwIKOHKkqFWwsxWewQE3xTTQfslPdrg
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "xWQ5LTAPl7NvZMdYSleCfUz1kFSJ0jCphr0K1l52awUh89EBGcdop38F6AlhOjH1qTY7GaLJBq739KyHcCQqTQ"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: dsoVvCM-FB_jTKSO341yuu7hxr3ax_2a-VAWsfMrgYG_1_xtbUbKNDlBDnk_cpSgr-MZeZs1Xk7eXdGwbdRO8g
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: ENmKt076TnX0SqCvez6hunrLOTDuvWx3nGQtsTxnEBgeAK4upovE1OLc1aPp3d7MKBOhIch3yhF2T5yrOkA03g
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: R5JsK0DVv4EpwopaRaBndNg4wJwkvgnnD-MqFeQWWnMBOxSJWCPtMKV0IQmyOI_8FW4l8V3ItUxxeQlQCl2lkQ
Created Bob: $500 - ID: tXGnnHxRYi0h8sIbYQtOising762MsTTV6fBnOsQJEmYsqCgQ7ilm59DVSn5X3D-olG6rzImxA2-Q5OSxGOBlA

=== Example 1: Begin Transaction ===
Transaction ID: 3e020d80-bdaa-4803-a6e4-529e91d85daf

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
New transaction: b8bdc079-fe3c-4efb-aefb-16775233f87d
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record { fields: {"id": String("JpVtHk5pyReUL5x6rCS1Sr7zO76HY6J1QkJ6gr5-22iPvytUzFOLvcar7bo0K9MJALKJYuHmLD6MpMdar3G12A")} }

=== Find by ID ===
Found: Record { fields: {"value": Object({"type": String("Integer"), "value": Integer(42)}), "name": Object({"type": String("String"), "value": String("Test Record")}), "id": String("JpVtHk5pyReUL5x6rCS1Sr7zO76HY6J1QkJ6gr5-22iPvytUzFOLvcar7bo0K9MJALKJYuHmLD6MpMdar3G12A"), "active": Object({"value": Boolean(true), "type": String("Boolean")})} }

=== Find with Query ===
Found documents: [Record { fields: {"value": Object({"type": String("Integer"), "value": Integer(42)}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "id": String("JpVtHk5pyReUL5x6rCS1Sr7zO76HY6J1QkJ6gr5-22iPvytUzFOLvcar7bo0K9MJALKJYuHmLD6MpMdar3G12A"), "name": Object({"type": String("String"), "value": String("Test Record")})} }]

=== Update Document ===
Updated: Record { fields: {"name": Object({"type": String("String"), "value": String("Updated Record")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "value": Object({"value": Integer(100), "type": String("Integer")}), "id": String("JpVtHk5pyReUL5x6rCS1Sr7zO76HY6J1QkJ6gr5-22iPvytUzFOLvcar7bo0K9MJALKJYuHmLD6MpMdar3G12A")} }

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

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "L7jWn19Z8Kqibg-vVUcEVSZux96qFhf__tLKdN4z4ufyG9u6ksw1ViX1XR2dNHOmDUakWncoRShja4QD87cL6w"

=== List Collections ===
Total collections: 9
Sample collections: ["test_accounts", "ttl_cache", "test_collection", "websocket_test", "scripts__ek0_testing"]

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
✓ Inserted document: "S1-8oN8jgKjnSN2QKw4IuljIUEdmKMzl42EEU2CrCOmI-AEykd4ALK5OEKkkxYmGZzf3AtRhEF7UB1USA2au-A"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("AyVNni5hMCeVIfEfA8SYpQkaTUbW8s32vUO1qxpvzt1j7HtBPcb6DvDRB3QjP_-45xHONP50HpkQ7mAYGFu2dQ"))

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
✓ Inserted test record: a_0-Prqbwve5VSnqs77NuhGLB9sZ7fqexG9MSsSykGK0uFUwvhI1Z74evswjpJ1SngGAcPo2JhIQXAeZBuNP5g

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
✓ Inserted document with TTL: Some(String("NmELNX_qy1QyFKuOWdYqMxFiWjmyTYEYERPJsnT0E0JW8qGZbw_1iEXEExhWt0FnIUrkazUKdvIp8M0cbwQvlQ"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Regex Query ===
✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"type": String("String"), "value": String("Charlie")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"value": String("Bob"), "type": String("String")}))
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
     Matched: ["description.value", "title", "description", "title.value"]
  2. Score: 2.0000
     Title: Some(String("Python for Data Science"))
     Matched: ["description", "description.value"]
  3. Score: 1.0000
     Title: Some(String("Database Design"))
     Matched: ["description", "description.value"]
  4. Score: 1.0000
     Title: Some(String("Rust Programming"))
     Matched: ["description.value", "description"]

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
✓ Inserted user 1: Some(String("puGZMZs5RcczWXb3-q1mTofrHrbXPBMApe_O-6JjkrkZnvcgd2UYSpi9Ca9H0gAVlshNn_vFj4HCPiZkwGO8OA"))
✓ Inserted user 2: Some(String("gTY-dYQF8YTn2FFx90ZGkeo6TmY4_wB1i6bsi7qCdexaSSPyvVipjwrHu1bMQUEPLhhc6-OS9f-v526WpDjh3Q"))

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
  Sample: ["test_accounts", "ttl_cache", "test_collection", "websocket_test", "scripts__ek0_testing"]

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
✓ Created session: OYjJR7pVbKoYxcBDhajVQoErv88QB3r08vuGOXPIKr3sG79g5agf2DVqZivkEABQRz95uBQ_hm8b9krr1yc3mw

=== Sending Chat Message ===
Message ID: NZEbHw542gIVRsnjgVxMpCvuXoW2bdbKgFnDy_KlEv9wJFqYPi9p0VMi-r5PF3ImE5AuptKzj8NhT1rZqXWoXg

=== AI Response ===
Response 1: Answer: ekoDB is a high-performance database that features intelligent caching, real-time capabilities, and AI integration. It comes with a variety of features including the ability to perform full-text search, vector search, and hybrid search with automatic context retrieval. Additionally, ekoDB has an AI Chat Integration feature that enables you to query your database using natural language and receive AI-powered responses with relevant context.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("snS5TnI0G0T99dJCV7e24v99F__oIBwQVuiaZyGDlUfKyZ0rPta-NFE7kcIn7yWgDkljjVZ5nPm3ZXHXImVuGA"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("AGfTjpY5EKoC95V7FTrVFrtaDWDT1G2wy7OHrDbJO1Gt2K_jz9UyNvt2cWCRhn2XBYBbZLfXlAl_dA5cvCOHlQ"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content", "category"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("EGH-grwbn3G2pH4OSArt0eMHvUR73sGkUzY-1hQQD2xbvoLvuARQy37_AHTBCDwwqva35vlDxIak7kFujlZ-HQ"), "title": String("Search Features")}

Execution Time: 2472ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: LDJLO-5YuyhN72uoGjH8oZkLQWc_B7ARiZYwyG-8rDhRttuosxeeXgu_2eRIr0XKRfQH1xKZk8_QkxAiUoAEeA
=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, the product we have is ekoDB. It's a high-performance database product with AI capabilities, and it's priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: G9ehqkdfB5H5FIHa8g5r83NLBf509yw_b4CX84hSL-1_oTKvBF9L8A-TXRrl2t2xsOPNsY9InsNACRvTxQelkA
  Parent: LDJLO-5YuyhN72uoGjH8oZkLQWc_B7ARiZYwyG-8rDhRttuosxeeXgu_2eRIr0XKRfQH1xKZk8_QkxAiUoAEeA

=== Listing Sessions ===
✓ Found 3 sessions
  Session 1: G9ehqkdfB5H5FIHa8g5r83NLBf509yw_b4CX84hSL-1_oTKvBF9L8A-TXRrl2t2xsOPNsY9InsNACRvTxQelkA (Untitled)
  Session 2: LDJLO-5YuyhN72uoGjH8oZkLQWc_B7ARiZYwyG-8rDhRttuosxeeXgu_2eRIr0XKRfQH1xKZk8_QkxAiUoAEeA (Untitled)
  Session 3: OYjJR7pVbKoYxcBDhajVQoErv88QB3r08vuGOXPIKr3sG79g5agf2DVqZivkEABQRz95uBQ_hm8b9krr1yc3mw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: G9ehqkdfB5H5FIHa8g5r83NLBf509yw_b4CX84hSL-1_oTKvBF9L8A-TXRrl2t2xsOPNsY9InsNACRvTxQelkA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: CdaCzhheiyQNx5hbI7AgaKxc6xX5g08Fs7Uv3xtPj-iXYZADAks6MHvZIfNl_TpBdSxpvn8LC-jS8vPuGqq2Tg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the data provided, the available product is "ekoDB". This is a high-performance database product. The price for this product is $99.

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
✓ Created second session: hyGmrj0Z3bTf09rh4aobr5bED81Zs2p4vAC3-HmLu1uJjUvxMG6N94KSZw2qnG7vBqdW60KHJcX7K5l_eD_Yag
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
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: 3HOEOB0b8BcY-4yx3hvIciMw9Y1D6JCB-Qn4qWdgIQ1MuDiLGjQeCBy6ntyDpvs9ruDMQyUYBjnaSUBIhoCGmw
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: 4wR7HEkPJp57JsvHJvoyEk-o-v0ZU00ND5_WkSSWPjtTicD7g8Tg3RT9MBfBAEbMKM1AE47JC6LvEQQuGZElHQ
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: uY3KBQ-9uOP7Q_C-tiBZespriJ9UntDphQ7dxi2bLUPgPBheMoRyyAQvvyq9gD9Wxv1He97hDO3mV_YJagw7ZQ
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 6
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
✅ [32mRust client examples complete![0m
