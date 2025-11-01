# ekoDB Examples - Test Run

```bash
make test-examples

🔍 Checking examples inventory against committed snapshot...
🔍 Generating comprehensive examples list...
📋 Counting Direct API examples...
  javascript: 7 direct examples
  python: 7 direct examples
  go: 7 direct examples
  rust: 7 direct examples
📚 Counting Client Library examples...
  rust: 14 client examples
  python: 14 client examples
  go: 14 client examples
  typescript: 14 client examples
  javascript: 7 client examples
  kotlin: 14 client examples
✅ Examples list generated!
  Total: 105 examples
  Direct: 28 | Client: 77
  Files: examples_list.txt.tmp, examples_list.json.tmp
✅ Examples inventory is up to date!
🧪 Running Rust examples (direct HTTP/WebSocket)...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("8uXj_OBYVqMJxi6g7caEZK_ugLjfqqKcckGgTyhufNXu1KRegZQLSVHJ-o4C8PwGUrSi7Uej6HNbUX5asZ6_Cg")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("8uXj_OBYVqMJxi6g7caEZK_ugLjfqqKcckGgTyhufNXu1KRegZQLSVHJ-o4C8PwGUrSi7Uej6HNbUX5asZ6_Cg"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("8uXj_OBYVqMJxi6g7caEZK_ugLjfqqKcckGgTyhufNXu1KRegZQLSVHJ-o4C8PwGUrSi7Uej6HNbUX5asZ6_Cg"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("8uXj_OBYVqMJxi6g7caEZK_ugLjfqqKcckGgTyhufNXu1KRegZQLSVHJ-o4C8PwGUrSi7Uej6HNbUX5asZ6_Cg"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "u58b8k85ctDidCO_rD1ii3VsNqtNwbmL0RcesNOJIgti_hY65pxOa0KESCYIkOS41DDuIdR-WnnATUrDv0hsbQ"

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
        "id": "u58b8k85ctDidCO_rD1ii3VsNqtNwbmL0RcesNOJIgti_hY65pxOa0KESCYIkOS41DDuIdR-WnnATUrDv0hsbQ",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "vFA3fSFFuUPUq_6aJc5uG2EDPSXAJt3Jc41tiCLTgIec7G8yNz1_WofMQFth7jfoEHHceV85YMdNCG4UNckdjw"

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "lbbwJ5-Oh_pLk1O_70PPEiNJXvJ8G9ELcKX35M5fyMIgKkuvAXJuJuvOjQ7NmbdBfqMqMteLgve1QhHn7G41gg"

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
✓ Inserted document with TTL: String("it-6OoSVLhSilvL71kskpkCBkrr4cikG3N1YzbEKPxl5IViQoGokTJZNF809p1Qw2V3DmJX3j_ptjZi0LnqC3g")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
✅ Rust direct examples complete!
🛠️  Building client library...
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
✅ Client build complete!
🧪 Running Rust client library examples...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record { fields: {"id": String("gIAUceYnsAGoARNqaXpFMD9Tu1PndbCBNLx9Vg5ZtygpUgCdICAqQ9Wqm-MGeJCB6mUSkV_aLfioA1om7Yr5Eg")} }

=== Find by ID ===
Found: Record { fields: {"name": Object({"value": String("Test Record"), "type": String("String")}), "id": String("gIAUceYnsAGoARNqaXpFMD9Tu1PndbCBNLx9Vg5ZtygpUgCdICAqQ9Wqm-MGeJCB6mUSkV_aLfioA1om7Yr5Eg"), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "value": Object({"value": Integer(42), "type": String("Integer")})} }

=== Find with Query ===
Found documents: [Record { fields: {"id": String("gIAUceYnsAGoARNqaXpFMD9Tu1PndbCBNLx9Vg5ZtygpUgCdICAqQ9Wqm-MGeJCB6mUSkV_aLfioA1om7Yr5Eg"), "name": Object({"value": String("Test Record"), "type": String("String")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "value": Object({"value": Integer(42), "type": String("Integer")})} }]

=== Update Document ===
Updated: Record { fields: {"value": Object({"type": String("Integer"), "value": Integer(100)}), "id": String("gIAUceYnsAGoARNqaXpFMD9Tu1PndbCBNLx9Vg5ZtygpUgCdICAqQ9Wqm-MGeJCB6mUSkV_aLfioA1om7Yr5Eg"), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "name": Object({"type": String("String"), "value": String("Updated Record")})} }

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
Collection created with first record: "MARb8qMEQ9qbTVOLfqP-6ZPi-YHRHHyjh7L0hE5S0LSSxjTYHOUaJdgnzn9JqjmS1YtMJHdW_QVTfXD0jN_fpA"

=== List Collections ===
Total collections: 6
Sample collections: ["ttl_cache", "batch_users", "ws_ttl_test", "websocket_test", "test_collection"]

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
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "REIueAau2AD7IjGpnxJH4KEh8xW3-OrOLpzz50rURTuDq_PAPBg13pRf5am_-koInaeCoWttf-IJcuEUd9gNGA"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("Ey5YumnMoOgp9ADlSbbW9thHjF-FnAaojdCgy3gVkk-p1uvA4Qn-pIz85mcr4DM__Q6fzP8yv8IEvVd6s1jKWw"))

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
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: UQXR4iZouVdcSE6RIUYGl6OnPVUnPno4v0ZmhVxSQE4ReexqEoe8FhBgoKquBWFBWxOLo03VqK0nyRGA8l1d7g

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
✓ Inserted document with TTL: Some(String("_sUj1w9WMXpfOkCva30USOPaa5CGVcsVgTJSwZHSr2xuqbvooU6EaGo27RVSH2Hwpg17mfGAHIkiem_-Lqq-RA"))

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
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

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
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"value": String("Charlie"), "type": String("String")}))

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
     Matched: ["title", "title.value", "description", "description.value"]
  2. Score: 2.0000
     Title: Some(String("Python for Data Science"))
     Matched: ["description.value", "description"]
  3. Score: 1.0000
     Title: Some(String("Rust Programming"))
     Matched: ["description", "description.value"]
  4. Score: 1.0000
     Title: Some(String("Database Design"))
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
  2. Score: 6.6000 - Some(String("Python for Data Science"))

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 2ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_schema_management`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("oztH7lOWDAT3Jf5n_QQmWoDuX6Za0Sf2qpE7rosPUkqGN8PDMHBER9QyOJECOnkNglmoxjHmCtcZSfIHzUDjXQ"))
✓ Inserted user 2: Some(String("abe2Z-SLlb0ulJFDFMVrKtQAkN4ymf8FWhbpoEjEzs_f10IhmAAucZeJ7pGCQ9_22drtraoJ_mxhGdKyAHWP0g"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - title: String
    (required)
  - email: String
    (required)
  - age: Integer
  - status: String

=== Listing Collections ===
✓ Total collections: 6
  Sample: ["ttl_cache", "batch_users", "ws_ttl_test", "websocket_test", "test_collection"]

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
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: LP69FWffXdjHxYSF4SKMr15EV00jLeVBy4w8WMXNdiNbT5UoJpKKrJGO5I_J9f8bGeQKmNXWkiFHXkBiGt8mbg

=== Sending Chat Message ===
Message ID: c8roxU9_Eb2xikZOfi6gTib2csMoCLWNI8hm0E8LHhQCLl1nJQqgi6zDCfyAGjxOLDYauYG_SfSE7bJQCFyqeA

=== AI Response ===
Response 1: Answer: ekoDB is a high-performance database that comes with intelligent caching, real-time capabilities, and AI integration. Some of its notable features include an AI chat integration, which allows you to query your database using natural language and receive AI-powered responses. It also supports full-text search, vector search, and hybrid search with automatic context retrieval for efficient data management and retrieval.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["content", "title", "category"]
  Record: Object {"category": Object {"type": String("String"), "value": String("features")}, "content": Object {"type": String("String"), "value": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context.")}, "id": String("ZqHhuxmwbx2t7GW6is88PTQSxwwFqxT6XAbSBDgJLI3TQZhX2YKTcFqak5-BGpzbSnrzRjr19ZgTIYPI54tXow"), "title": Object {"type": String("String"), "value": String("AI Chat Integration")}}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "category", "title"]
  Record: Object {"category": Object {"type": String("String"), "value": String("features")}, "content": Object {"type": String("String"), "value": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval.")}, "id": String("485d3cYZqPFtfcDRqMGJO6MGfheNuqnfeI6Z8u6BIQ4M38Yj2iEMvwPI9Zg_Cmuv6NZFDlBt2XKI00H7YtXaOg"), "title": Object {"type": String("String"), "value": String("Search Features")}}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": Object {"type": String("String"), "value": String("documentation")}, "content": Object {"type": String("String"), "value": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration.")}, "id": String("zYaxDy9e8l7LBq4TrTiLPPq9SHn4QoPdliYVSqccWYjU2lXA6c7O7aJdZJIjqu8CaY7MVzzZRmPaTwdDFkM1Yg"), "title": Object {"type": String("String"), "value": String("Introduction to ekoDB")}}

Execution Time: 3421ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: SXNBtMRQLSdg68c2t9Ax20DDO4pv61R9VmnwKNuTOvy50TXA_t33_NI3ybxx6TrsZtkJR56I4As9Klt6rURSmA
=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the product available is ekoDB, a high-performance database product with AI capabilities. The price of this product is $99.

✓ Message 2 sent
  Response: As an assistant, I'm sorry but I can't provide the price as there's no specific product mentioned in the context provided. Could you please specify which product you're referring to?

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 5If0yqAVtaO_aIYe1TwfwQvlK2iEbaYwku7QH8q2-4jeyIW7EQ9_XxXYxSvytBosF5T0ajhnOAqIH_EDaNQRZA
  Parent: SXNBtMRQLSdg68c2t9Ax20DDO4pv61R9VmnwKNuTOvy50TXA_t33_NI3ybxx6TrsZtkJR56I4As9Klt6rURSmA

=== Listing Sessions ===
✓ Found 3 sessions
  Session 1: 5If0yqAVtaO_aIYe1TwfwQvlK2iEbaYwku7QH8q2-4jeyIW7EQ9_XxXYxSvytBosF5T0ajhnOAqIH_EDaNQRZA (Untitled)
  Session 2: SXNBtMRQLSdg68c2t9Ax20DDO4pv61R9VmnwKNuTOvy50TXA_t33_NI3ybxx6TrsZtkJR56I4As9Klt6rURSmA (Untitled)
  Session 3: LP69FWffXdjHxYSF4SKMr15EV00jLeVBy4w8WMXNdiNbT5UoJpKKrJGO5I_J9f8bGeQKmNXWkiFHXkBiGt8mbg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 5If0yqAVtaO_aIYe1TwfwQvlK2iEbaYwku7QH8q2-4jeyIW7EQ9_XxXYxSvytBosF5T0ajhnOAqIH_EDaNQRZA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 8Yv4xBrOVbSfyi6ZewIL7C-BvUIlNNdCiZL-CYX20Kho-vve_Mj3S_MMKS6uzVRISacrwYCgwWH7Uav5ku6uYw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, the product ekoDB is available. This is a high-performance database product priced at $99.

✓ Second message sent
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: I'm sorry, but the context provided does not include information on the price of ekoDB. I recommend checking the latest price on the product's official website or contacting the product's customer service for the most accurate information.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: eNqHkJ2MZg4gvX96oNbOKGRzQLaFSbJSZ8PZ7cHp4vMBvWDLbAtwAiriVksCmOcMp_ONa-G4nqgfoM6_DBqxcQ
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
✅ Rust client examples complete!
✅ All Rust integration tests complete!
🧪 Running Python examples (direct HTTP/WebSocket)...

╔════════════════════════════════════════╗
║   ekoDB Python Examples Test Suite    ║
╚════════════════════════════════════════╝

=== Checking Server Connection ===
✓ Server is ready

=== Getting Authentication Token ===
✓ Authentication successful

=== Running 7 Examples ===

=== Running simple_crud.py ===
✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'xt99sG8ohEqElVQrWclovazHHzGf4w1alY8NLP_b6WQJA99o_fCNK_PA3DYnPCqG1mVhLAkW3G76P8BH1__jKA'}

=== Find by ID ===
Found: {'active': {'type': 'Boolean', 'value': True}, 'name': {'value': 'Test Record', 'type': 'String'}, 'value': {'type': 'Integer', 'value': 42}, 'id': 'xt99sG8ohEqElVQrWclovazHHzGf4w1alY8NLP_b6WQJA99o_fCNK_PA3DYnPCqG1mVhLAkW3G76P8BH1__jKA'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'id': 'xt99sG8ohEqElVQrWclovazHHzGf4w1alY8NLP_b6WQJA99o_fCNK_PA3DYnPCqG1mVhLAkW3G76P8BH1__jKA', 'active': {'type': 'Boolean', 'value': True}, 'value': {'value': 100, 'type': 'Integer'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.py completed successfully

=== Running simple_websocket.py ===
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: r18nKHVTruUYT6ullGjZE4TagZiudfGWn3YfI31OQpifIMSGzSHL2c_60kPko2n5j0gtJnQFvsCKyub2E0sb2Q

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
        "id": "r18nKHVTruUYT6ullGjZE4TagZiudfGWn3YfI31OQpifIMSGzSHL2c_60kPko2n5j0gtJnQFvsCKyub2E0sb2Q",
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
        "id": "u58b8k85ctDidCO_rD1ii3VsNqtNwbmL0RcesNOJIgti_hY65pxOa0KESCYIkOS41DDuIdR-WnnATUrDv0hsbQ",
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
✓ simple_websocket.py completed successfully

=== Running batch_operations.py ===
✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 10 total records in collection

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records
✓ Verified: Record updated with status="None"

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
✓ batch_operations.py completed successfully

=== Running kv_operations.py ===
✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {'username': 'john_doe', 'userId': 123}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: {'price': 29.99, 'name': 'Product 1'}
cache:product:2: {'name': 'Product 2', 'price': 39.989999999999995}
cache:product:3: {'name': 'Product 3', 'price': 49.989999999999995}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ kv_operations.py completed successfully

=== Running collection_management.py ===
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: _woIOsBkxemdJyF97VKrdWNjB1EwJKcd_1UvbEVemhuTTZUNa4wo8aYLkNuuDmfyhBUsN1IS8rDrjjTlv8maRA

=== List Collections ===
Total collections: 8
Sample collections: ['ttl_cache', 'chat_messages__ek0_bench', 'batch_users', 'ws_ttl_test', 'websocket_test']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ collection_management.py completed successfully

=== Running document_ttl.py ===
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: lnkjehhpPPiRMdF5ZfBImvkE0ZYI7vbW4ThlCsOis9uzjq1y-X6Cn2Ky-QqKqTfiKskKj2N5tRClDov0l9ugPg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: created

=== Query Documents ===
✓ Found 2 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully
✓ document_ttl.py completed successfully

=== Running websocket_ttl.py ===
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: KxTdMY0orBpjbJgLOQKEz2jRo7n0tLpqd8BZl4m0wKS2PwVSxk4SzVKAXJUE7hAAo3d0dvqN-GinEqO8eKYuHQ

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
✓ websocket_ttl.py completed successfully

╔════════════════════════════════════════╗
║           Test Summary                 ║
╚════════════════════════════════════════╝
Total: 7
Passed: 7
Failed: 0
✅ Python direct examples complete!
🐍 Building Python client package...
🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings with abi3 support
🐍 Not using a specific python interpreter
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.08s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.2.1-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel for current platform...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.2.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.2.1
    Uninstalling ekodb_client-0.2.1:
      Successfully uninstalled ekodb_client-0.2.1
Successfully installed ekodb-client-0.2.1
✅ Python client package built and installed!
🧪 Running Python client library examples...
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': '6FuKUnIU70eoZblymurWfI5dlj_4go1nOa7ITp0SGySHShFJp7pa1LvVufvxkUVruLx8CbwSod7qIfszEPC5Aw'}

=== Find by ID ===
Found: {'active': {'type': 'Boolean', 'value': True}, 'name': {'value': 'Test Record', 'type': 'String'}, 'value': {'value': 42, 'type': 'Integer'}, 'id': '6FuKUnIU70eoZblymurWfI5dlj_4go1nOa7ITp0SGySHShFJp7pa1LvVufvxkUVruLx8CbwSod7qIfszEPC5Aw'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'active': {'value': True, 'type': 'Boolean'}, 'value': {'value': 100, 'type': 'Integer'}, 'id': '6FuKUnIU70eoZblymurWfI5dlj_4go1nOa7ITp0SGySHShFJp7pa1LvVufvxkUVruLx8CbwSod7qIfszEPC5Aw', 'name': {'type': 'String', 'value': 'Updated Record'}}

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
Collection created with first record: "isoUaBy-cJHhxvn8c3_Vdx4EKu_Muf3vXjMcA7SVSXO21FHU2Whwad2zA-MaE-UdQ4JwODk4vVa7_4zCZ8oFTA"

=== List Collections ===
Total collections: 8
Sample collections: ['ttl_cache', 'chat_messages__ek0_bench', 'batch_users', 'ws_ttl_test', 'websocket_test']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: TQ5albGFFxgt3PsziGxMElSc0eFOGfM8vUAAxR1pK6oDXQSn5knLpAYahJfwdj3vSQCtCvtB6xB_TX7NXWX0Uw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: pskGOPNK-U8o-egk8daevJ7uhSS4jJ8Cb6tKuxLXAzYPV26acDTDdC1tBxkeAdcnLanJYIHAQy188pEINk-vGg

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
✓ Inserted test record: LAGjLYm0UOozygNX-0obh0HJqgaNnZcpAeIrbm0Vg9mt427ock7T6R6JYkGWef-fapuDxHe24qQtcXks5Ec69w

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
✓ Inserted document with TTL: Z43XrxJhsTgRx4_NUjKY2szANNJK45L095FxjYjColuBHzXycRw7MqK2Ge7Ld7P8mIGZK4g5gOM7zhsLXaji0w

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
  1. Score: 25.740, Matched: email.value, name, name.value, email
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio.value, bio, title.value
  2. Score: 26.400, Matched: title, title.value, bio, bio.value
  3. Score: 26.400, Matched: bio, title.value, bio.value, title
  4. Score: 26.400, Matched: title.value, title, bio.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio.value, title.value, bio
  2. Score: 39.600, Matched: bio, bio.value, title.value, title
  3. Score: 39.600, Matched: title, bio.value, title.value, bio
  4. Score: 39.600, Matched: bio, title.value, title, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 0 semantically similar documents

7. Hybrid search (text + vector):
Found 2 results using hybrid search (text + vector)
  1. Score: 1.200, Matched: title.value, title, content.value, content
  2. Score: 0.600, Matched: title.value, title, content, content.value

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
✓ Created session: F17zdv9PM_0zUilrEclCjxCpMgMW_Br9ZvTv1rF0ubKj6iCZ4hoIYDhqXtXhDr0n6lry4K_qu7ZXaOlLyE1dug

=== Sending Chat Message ===
Message ID: 3WUCUWO4XQNouAgzv0_CT8zGEfwlC6aohgAEqJLYvDNfgbq1OnG7iFeDsxkTENOC1coyPx1hCBH0ElrSnt4p0A

=== AI Response ===
Based on the provided information, there are three products available:

1. ekoDB: This high-performance database product with AI capabilities is priced at $99.
2. ekoDB Cloud: This fully managed cloud database service product is priced at $499.
3. ekoDB Pro: This is the enterprise edition product with advanced features and it is priced at $299.

Execution Time: 2672ms

=== Token Usage ===
Prompt tokens: 381
Completion tokens: 76
Total tokens: 457

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: E7-I8XAnUeMaZODbqD7IbQN-IgAo2buyNzndQ67BHVxOVjry2cEu5Hxb6iUvkluIGg3824LMHQ-lyA-wATFXBA

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is ekoDB. It is a high-performance database product priced at $99.

✓ Message 2 sent
  Response: Apologies for the inconvenience, but no specific product was mentioned in the context provided. Could you please specify which product's price you are inquiring about?

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: TMMb0Cflo9B4lWp5capj4igNpcmjv9OMgthpwl1dTjdVqKq5juQ7kv7742c9LEGupvz6BnvlSkHPttooSpzZMg
  Parent: E7-I8XAnUeMaZODbqD7IbQN-IgAo2buyNzndQ67BHVxOVjry2cEu5Hxb6iUvkluIGg3824LMHQ-lyA-wATFXBA

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: TMMb0Cflo9B4lWp5capj4igNpcmjv9OMgthpwl1dTjdVqKq5juQ7kv7742c9LEGupvz6BnvlSkHPttooSpzZMg (Untitled)
  Session 2: E7-I8XAnUeMaZODbqD7IbQN-IgAo2buyNzndQ67BHVxOVjry2cEu5Hxb6iUvkluIGg3824LMHQ-lyA-wATFXBA (Untitled)
  Session 3: F17zdv9PM_0zUilrEclCjxCpMgMW_Br9ZvTv1rF0ubKj6iCZ4hoIYDhqXtXhDr0n6lry4K_qu7ZXaOlLyE1dug (Untitled)
  Session 4: eNqHkJ2MZg4gvX96oNbOKGRzQLaFSbJSZ8PZ7cHp4vMBvWDLbAtwAiriVksCmOcMp_ONa-G4nqgfoM6_DBqxcQ (Untitled)
  Session 5: SXNBtMRQLSdg68c2t9Ax20DDO4pv61R9VmnwKNuTOvy50TXA_t33_NI3ybxx6TrsZtkJR56I4As9Klt6rURSmA (Untitled)
  Session 6: LP69FWffXdjHxYSF4SKMr15EV00jLeVBy4w8WMXNdiNbT5UoJpKKrJGO5I_J9f8bGeQKmNXWkiFHXkBiGt8mbg (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: TMMb0Cflo9B4lWp5capj4igNpcmjv9OMgthpwl1dTjdVqKq5juQ7kv7742c9LEGupvz6BnvlSkHPttooSpzZMg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: mWiNREly8L-YHKcDAgMls0JHQJ_s9TfOwiBd3mPlQThUVLd0Rqr_5LYuVZ6KLMg29_GEip6_HBd-GNtpd0-0sA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the available product is ekoDB. It is a high-performance database product priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['created_at', 'token_usage', 'id', 'updated_at', 'chat_id', 'content', 'role'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: I'm sorry, but there's no information provided in the context about the price of ekoDB. You may need to check the latest price on the product's official website or contact the product's customer service for the most accurate information.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: 2DARvf8VaJEYmR3ZQTg3wNjCYD1qm-UfiS_swV6zw4U-iYdZr3S_tdKIxcYEWQQc20fcu5Z5-wkIJkzYAYE5oA
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
✅ Python client examples complete!
✅ All Python integration tests complete!
🧪 Running Go examples (direct HTTP/WebSocket)...

╔════════════════════════════════════════╗
║     ekoDB Go Examples Test Suite      ║
╚════════════════════════════════════════╝

=== Checking Server Connection ===
✓ Server is ready

=== Running 7 Examples ===

=== Running simple_crud.go ===
✓ Authentication successful

=== Insert Document ===
Inserted: map[id:p_9OTeItXT8mXlmWFxvc0NRf6gp6PKsXxyo80tOws-kBSwUOtPotM13HdV4xVp3Vc9UvmzBzPaEO2PNuYxvQpg]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:p_9OTeItXT8mXlmWFxvc0NRf6gp6PKsXxyo80tOws-kBSwUOtPotM13HdV4xVp3Vc9UvmzBzPaEO2PNuYxvQpg name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:p_9OTeItXT8mXlmWFxvc0NRf6gp6PKsXxyo80tOws-kBSwUOtPotM13HdV4xVp3Vc9UvmzBzPaEO2PNuYxvQpg name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.go completed successfully

=== Running simple_websocket.go ===
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: mx1JFaLb4gTtxeTgdH9UaJLpr0vKTa46mpbTkqSNvKDYbl_1USaor2hbrjaYl-4eaYkbo7YpdW6uLPUsRhM62Q

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
        "id": "r18nKHVTruUYT6ullGjZE4TagZiudfGWn3YfI31OQpifIMSGzSHL2c_60kPko2n5j0gtJnQFvsCKyub2E0sb2Q",
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
        "id": "mx1JFaLb4gTtxeTgdH9UaJLpr0vKTa46mpbTkqSNvKDYbl_1USaor2hbrjaYl-4eaYkbo7YpdW6uLPUsRhM62Q",
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
        "id": "u58b8k85ctDidCO_rD1ii3VsNqtNwbmL0RcesNOJIgti_hY65pxOa0KESCYIkOS41DDuIdR-WnnATUrDv0hsbQ",
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
✓ simple_websocket.go completed successfully

=== Running batch_operations.go ===
✓ Authentication successful

=== Batch Insert ===
Batch insert response: map[failed:[] successful:[dY-_caP70Y9kUwlaHlwnWzSqtn4LurbNmNaFAdD_qeTJDFkIxKoOWY7c8WVoP_mTWJd6Yjce9j5Vo3-cXBgKxg wwaXsTxlNANo3gaVrtDT_vKPxHaMBZO1dvC_qaZ_Wiixp-XOyeCmoZusmFqlPPjBkXK9Wks3asvCVQ6cAQdcqg azVcyiPDgnIfbx9GRyZDV81yWI40jZLC3_DzpGJ59cYn7Jy0NTNw6jJBAfxjk6y77hHTQD5x8p-X6K4cL1JPoQ unmwN6ebmQbWbzHs3DaiqSYLAufM9oAbnC1a4onXcGA7ZbBOtrRZDQvhwuT7vk6XAvuutBJBNCHvGH3h92MEGQ 39ryaV1M4yH-SDkAy-cHFYqf3gTI38QpsRojBxyD97Ek3F-knpfCC1lVF80EhC1hS2-V4KFhefjIcFo6vqkpew]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[MX-3Wf1NjkhIJNBWZ3AyfXE-wozZME2eLrqu32Z0t3RVVF06o7oOJFPIJupsmfUaR_C1WmK5xPD2pZC4cCFhcQ D_RmonvU8STVS7T1cSmlF44Rc_Phswp9hFaEtM1p8eZ37qZJ6aSdbD3IrlGnm1xwsjKqOj-bedhr9SLX2vc8iQ su7JD7X8PwdBwBc7gd3GnOZfmuUhz-xJPapmt-cb7h_AlmfpvlThPETbhDJpD15rTUhDHncUGKZ1jxqOiPHo1w]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[MX-3Wf1NjkhIJNBWZ3AyfXE-wozZME2eLrqu32Z0t3RVVF06o7oOJFPIJupsmfUaR_C1WmK5xPD2pZC4cCFhcQ D_RmonvU8STVS7T1cSmlF44Rc_Phswp9hFaEtM1p8eZ37qZJ6aSdbD3IrlGnm1xwsjKqOj-bedhr9SLX2vc8iQ su7JD7X8PwdBwBc7gd3GnOZfmuUhz-xJPapmt-cb7h_AlmfpvlThPETbhDJpD15rTUhDHncUGKZ1jxqOiPHo1w]]
✓ Batch delete completed

✓ All batch operations completed successfully
✓ batch_operations.go completed successfully

=== Running kv_operations.go ===
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
✓ kv_operations.go completed successfully

=== Running collection_management.go ===
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: _a25tcByxQTzpO7-3IqepcbVAOwVWsQTdPYORWYr5NCSZlH5uycDX3tIMea0vkAWjBqdiH9celcO38IlhIznBw

=== List Collections ===
Total collections: 12
Sample collections: [ttl_cache chat_messages__ek0_bench batch_users schema_users_client_py ws_ttl_test schema_documents_client_py websocket_test schema_employees_client_py chat_configurations__ek0_bench test_collection schema_products_client_py demo_collection]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ collection_management.go completed successfully

=== Running document_ttl.go ===
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: h3Kw7Hj3shefLS7XNj7o27w3_r8gPHs08TW_MuI_oAjtuhG8Y785hk1jjIsla8BVGm-x-RmTitaRldaAijfKkg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: created

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully
✓ document_ttl.go completed successfully

=== Running websocket_ttl.go ===
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Be-7DofEf5lA7Wt3HSxhONaAcH4fI3Go8xZCuhFp35Yz58y6fw0QzBGMud04BfmgtBvfhnDS_IMLqMJ4KX-zDA

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 3 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
✓ websocket_ttl.go completed successfully

╔════════════════════════════════════════╗
║           Test Summary                 ║
╚════════════════════════════════════════╝
Total: 7
Passed: 7
Failed: 0
✅ Go direct examples complete!
🧪 Running Go client library examples...
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: map[id:adn8DbHtZo2PHNtTk68e9KnFsrKbUuuX3ReP9egXEKf-S6Vu2vB9c6glM5NlluyOLReg58DlwSQmYqXEWIVSVw]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:adn8DbHtZo2PHNtTk68e9KnFsrKbUuuX3ReP9egXEKf-S6Vu2vB9c6glM5NlluyOLReg58DlwSQmYqXEWIVSVw name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:adn8DbHtZo2PHNtTk68e9KnFsrKbUuuX3ReP9egXEKf-S6Vu2vB9c6glM5NlluyOLReg58DlwSQmYqXEWIVSVw name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

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
Collection created with first record: Aj37wVEcvX_7KB9nMB85HVjEZXtWNf2b-MxNN00tYL00T-58Xp8TEtNEuhPkeopbz26eEmLsl8XpEqZAC093EA

=== List Collections ===
Total collections: 12
Sample collections: [ttl_cache chat_messages__ek0_bench batch_users schema_users_client_py ws_ttl_test]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: xA_y85qE8euzLKYeZ3cjWSjm7BJ2phvqu-srwRIpijtUiFpTR7apWwuJzVQ6sppBMoCpnqti2tde0dCWB-iJNQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: zWWQjmHLhwzflRM3HprZYkAtYXlWZMaQdhjj3fXXn9v4hDahM3MTjXTVIeBViz4LAJ_BvbSCFjLVgsdczUK3jA

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
✓ Inserted test record: cd9J_Sf9VudAF52C9Pt1wQA0TQFU-Ay3Jpu1ZADkxU_1ZXeWOMDcIpo_klckwx4Tz6Dh7YCI-tJJbG03QzElvg

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
✓ Inserted document with TTL: h-tGiDSGmMLO7Ev9AElstuubdb_mIjDcp0PSpjjBH5h543PlFo4yWgYzoaF52m0JaNK98abmyIuWqYyEwYcK8A

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
✓ Created session: keM1bJy7_oN35pNQCrc1NnPplg9Dti5WmPaLJOm2iQy3z2CP2hCv4v5339FBBMLAu3Rz2pW_twW--WmJaQQd8Q

=== Sending Chat Message ===
Message ID: 18ShSq2aJNIU-aaQ7Tv7tOiEqJVFIYUJpWK-eglNcknYcX_aJbKAzuATtUJPuZICpIYuCr2HNUmjI6fiDRS6dg

=== AI Response ===
There are three products available:

1. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.
2. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.
3. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:map[type:String value:Fully managed cloud database service product] id:U1UJlm9oSBK47NStpEV3dks-k36E5ZNBLelxS1fN-32BmBgzniIywh5eZOgCNXvfDot_b6HB547O99II4ag9ZQ name:map[type:String value:ekoDB Cloud] price:map[type:Integer value:499]] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:map[type:String value:A high-performance database product with AI capabilities] id:ZD-J71GcrgmWvWnZEZQbL6nr0Cit0ijisMNgxOuIIAVtGdSDbaJrqfJrSeABNdFBWe0tjyWi89jUuaPxgDkRhw name:map[type:String value:ekoDB] price:map[type:Integer value:99]] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:map[type:String value:Enterprise edition product with advanced features] id:GgjJqk6vbrn9lnz8sq0oA7qethZ_QMxmqrugtq4NSpfU--wq4rwAPwz2E5vicHhagu3E_TYfUTqOvkj3mcridA name:map[type:String value:ekoDB Pro] price:map[type:Integer value:299]] score:0.1111111111111111]

Execution Time: 2667ms

=== Token Usage ===
Prompt tokens: 376
Completion tokens: 84
Total tokens: 460

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: gj-BOvC1tAHksPg2Zz6zPzcU_em-WhV4Tb-hytcHRPaR_JacjL4KI_k_IO3KHHiCdhxrdks8j14ZqwJDHgObUA

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is a high-performance database product named ekoDB. It is priced at $99.

✓ Message 2 sent
  Response: I'm sorry, but there's no information provided about the product you're asking about. Could you please provide more details?

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: SUxJR9RFcEhrS2zeHhqMHpwDEXUTGslqWY74TVRHnxdwmQRxqitsShlmNsr4rtG9CGiYtmQQ7F25zc9JCkTpqw
  Parent: gj-BOvC1tAHksPg2Zz6zPzcU_em-WhV4Tb-hytcHRPaR_JacjL4KI_k_IO3KHHiCdhxrdks8j14ZqwJDHgObUA

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: SUxJR9RFcEhrS2zeHhqMHpwDEXUTGslqWY74TVRHnxdwmQRxqitsShlmNsr4rtG9CGiYtmQQ7F25zc9JCkTpqw (Untitled)
  Session 2: gj-BOvC1tAHksPg2Zz6zPzcU_em-WhV4Tb-hytcHRPaR_JacjL4KI_k_IO3KHHiCdhxrdks8j14ZqwJDHgObUA (Untitled)
  Session 3: keM1bJy7_oN35pNQCrc1NnPplg9Dti5WmPaLJOm2iQy3z2CP2hCv4v5339FBBMLAu3Rz2pW_twW--WmJaQQd8Q (Untitled)
  Session 4: 2DARvf8VaJEYmR3ZQTg3wNjCYD1qm-UfiS_swV6zw4U-iYdZr3S_tdKIxcYEWQQc20fcu5Z5-wkIJkzYAYE5oA (Untitled)
  Session 5: E7-I8XAnUeMaZODbqD7IbQN-IgAo2buyNzndQ67BHVxOVjry2cEu5Hxb6iUvkluIGg3824LMHQ-lyA-wATFXBA (Untitled)
  Session 6: F17zdv9PM_0zUilrEclCjxCpMgMW_Br9ZvTv1rF0ubKj6iCZ4hoIYDhqXtXhDr0n6lry4K_qu7ZXaOlLyE1dug (Untitled)
  Session 7: eNqHkJ2MZg4gvX96oNbOKGRzQLaFSbJSZ8PZ7cHp4vMBvWDLbAtwAiriVksCmOcMp_ONa-G4nqgfoM6_DBqxcQ (Untitled)
  Session 8: SXNBtMRQLSdg68c2t9Ax20DDO4pv61R9VmnwKNuTOvy50TXA_t33_NI3ybxx6TrsZtkJR56I4As9Klt6rURSmA (Untitled)
  Session 9: LP69FWffXdjHxYSF4SKMr15EV00jLeVBy4w8WMXNdiNbT5UoJpKKrJGO5I_J9f8bGeQKmNXWkiFHXkBiGt8mbg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: SUxJR9RFcEhrS2zeHhqMHpwDEXUTGslqWY74TVRHnxdwmQRxqitsShlmNsr4rtG9CGiYtmQQ7F25zc9JCkTpqw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: QzzSiNDbIEu2jXJ2t7PNB43wGXLvua7a_q8xdZCqGwp8pvmeXLFVuY5SGWLZk5iirBUjjoUxbaxkwl19IhPjOg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, the available product is ekoDB. It's a high-performance database product priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: I'm sorry, but the context provided does not give information on the price of ekoDB. I would recommend checking the product's official website or contacting their sales department for accurate pricing information.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: 1fqqN2U23MjtSgevuv11bvOGkWzny-6xalxPY1WtYmvNExr25XEagxr9Jx62i0MGSfFJfez-F0LWqL5Io8FnXg
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
  1. Score: 0.764
  2. Score: 0.753
  3. Score: 0.736

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.706
  2. Score: 1.501
  3. Score: 0.295

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200

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
  - Bob Smith: Sales
  - Alice Johnson: Engineering

2. Join with filtering:
Found 1 users in Engineering
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data
  - Bob Smith: Sales Manager
  - Alice Johnson: Senior Software Engineer

4. Join orders with user data:
Found 2 completed orders
  - Mouse ($25) by Alice Johnson
  - Laptop ($1200) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
✅ Go client examples complete!
✅ All Go integration tests complete!
📦 Building TypeScript client library...

> @ekodb/ekodb-client@0.2.1 prepare
> npm run build


> @ekodb/ekodb-client@0.2.1 build
> tsc


up to date, audited 7 packages in 1s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.2.1 build
> tsc

✅ TypeScript client built!
🧪 Running TypeScript client library examples...

added 1 package, removed 1 package, and audited 13 packages in 806ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'ekcJvZUU3W7YU1j7U77oHXQY9v1vPlaJ2fiAfvwSdCEQXtVcP23MoVFZMB-YeaYJwr4OAwbR8VcxXx4TZODx_w'
}

=== Find by ID ===
Found: {
  id: 'ekcJvZUU3W7YU1j7U77oHXQY9v1vPlaJ2fiAfvwSdCEQXtVcP23MoVFZMB-YeaYJwr4OAwbR8VcxXx4TZODx_w',
  name: { type: 'String', value: 'Test Record' },
  active: { value: true, type: 'Boolean' },
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  value: { type: 'Integer', value: 100 },
  id: 'ekcJvZUU3W7YU1j7U77oHXQY9v1vPlaJ2fiAfvwSdCEQXtVcP23MoVFZMB-YeaYJwr4OAwbR8VcxXx4TZODx_w',
  active: { value: true, type: 'Boolean' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Batch Insert ===
✓ Batch inserted undefined records
TypeError: inserted.map is not a function
    at main (/Users/tek/Development/ekoDB/ekodb-client/examples/typescript/client_batch_operations.ts:38:24)
    at process.processTicksAndRejections (node:internal/process/task_queues:105:5)
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
Collection created with first record: QY1dEds2cxq-6jULppdgNNHSj1NqSwqqmmf4pksKVd8Jzw7IG5YcBgKrg3EXrcC7OwFZH06pjhqGMv_PijjFzQ

=== List Collections ===
Total collections: 17
Sample collections: schema_users_client_go,ttl_cache,client_collection_management_ts,chat_messages__ek0_bench,batch_users

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: aLcfRTPaVcQj9J35OAoxKp7kjDx_6CcnTjMM9z52u_psq4fudCfSc0gqcnmJpKZqN5l334st2gWX73mR_B9DSw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: a-e90RRojlVJ9hymj-sX7Q_Bc3VS-ntNHG6TRRJP6wKziMUezufyjf0RqRYlDxq_trXFqod-5ccKzAphDCbF6Q

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
✓ Inserted test record: nkyILVyr98ACCPKl59YgF6IexgwRrhoZyIT5ekS04JUfpMKVVKYjDNjtAwSQxfDuczXHtT9cmuATP8N71_rOIw

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
✓ Inserted document with TTL: fyHjdMZ7CF3ICD1HO5_2In1Hhu4VXABGxaoFnyn-fWearGUTiyfPzvb7Ibn7Bn0su5arCspjZkm74euaAst4Xw

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
✓ Created session: HFQXtIvNau7UPM4iaRFItkuFlRUuux8m8ynm30yiYU8mxxPWlTLVKVmgiLGUKelqoB-nchWJ_Gyl62lq9SD_uQ

=== Sending Chat Message ===
Message ID: z-qFWku2uONndPIxJsYKURA5QS8-InoOOquegQ_Ro7smLcS3l825IF_bcjWGq-t8BddXcol7HqiuztMyfG6DrQ

=== AI Response ===
The available products and their prices are as follows:

1. ekoDB Pro: This is an enterprise edition product with advanced features. The price is $299.
2. ekoDB Cloud: This is a fully managed cloud database service product. The price is $499.
3. ekoDB: This is a high-performance database product with AI capabilities. The price is $99.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'dtDl_WJ_aaaYYMrRO4m1Q--aY2AIQRxL6DrrzW6DlO1ILd_z7fzYuo1Eg90vHo7e7p2VXmRFw3uSP6EMnBsGRA',
    name: { value: 'ekoDB Pro', type: 'String' },
    price: { type: 'Integer', value: 299 },
    description: {
      value: 'Enterprise edition product with advanced features',
      type: 'String'
    }
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    price: { type: 'Integer', value: 499 },
    id: 'vInHaRra1Af6UhwuZxmjSQqo5GZk04exqqczCyRt1aQBx8zBHRKkmTzdUn3sTYuiFE1orhhdhXhkiKvfPek_RA',
    name: { value: 'ekoDB Cloud', type: 'String' },
    description: {
      type: 'String',
      value: 'Fully managed cloud database service product'
    }
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: { value: 99, type: 'Integer' },
    id: '78TozcdE95HoH8oZqeDFzgzrcEJG12Em-AkAdZuzDefVnAg0tT3VHshoF932B_T_2ApoHazJ3aDiuijWrKM8GQ',
    name: { value: 'ekoDB', type: 'String' },
    description: {
      type: 'String',
      value: 'A high-performance database product with AI capabilities'
    }
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2655ms

=== Token Usage ===
Prompt tokens: 376
Completion tokens: 79
Total tokens: 455

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: UB6GuGrL-CIN9T-PSEqRth4mhMuwOUefw7ftMPmmQc8caOQVk9NKXFWrNUzI0qHGAFPQ73vd5rXNgpcu5uy2Gw

=== Sending Messages ===
✓ Message 1 sent
  Response: Â The product available is ekoDB, a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: As an assistant, I'm sorry but I can't provide the price since the context doesn't specify any product details. Could you please provide more information about the product you're interested in?

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: iOKC7IJgA7lC5e1hQRleXRXOTKzBPt2Pp76B50cbG_p0rrALp2y2Pp3U8c3D_EvjR_ZbwFeJh-k5ijGxaoWcZQ
  Parent: UB6GuGrL-CIN9T-PSEqRth4mhMuwOUefw7ftMPmmQc8caOQVk9NKXFWrNUzI0qHGAFPQ73vd5rXNgpcu5uy2Gw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: iOKC7IJgA7lC5e1hQRleXRXOTKzBPt2Pp76B50cbG_p0rrALp2y2Pp3U8c3D_EvjR_ZbwFeJh-k5ijGxaoWcZQ (Untitled)
  Session 2: UB6GuGrL-CIN9T-PSEqRth4mhMuwOUefw7ftMPmmQc8caOQVk9NKXFWrNUzI0qHGAFPQ73vd5rXNgpcu5uy2Gw (Untitled)
  Session 3: HFQXtIvNau7UPM4iaRFItkuFlRUuux8m8ynm30yiYU8mxxPWlTLVKVmgiLGUKelqoB-nchWJ_Gyl62lq9SD_uQ (Untitled)
  Session 4: 1fqqN2U23MjtSgevuv11bvOGkWzny-6xalxPY1WtYmvNExr25XEagxr9Jx62i0MGSfFJfez-F0LWqL5Io8FnXg (Untitled)
  Session 5: gj-BOvC1tAHksPg2Zz6zPzcU_em-WhV4Tb-hytcHRPaR_JacjL4KI_k_IO3KHHiCdhxrdks8j14ZqwJDHgObUA (Untitled)
  Session 6: keM1bJy7_oN35pNQCrc1NnPplg9Dti5WmPaLJOm2iQy3z2CP2hCv4v5339FBBMLAu3Rz2pW_twW--WmJaQQd8Q (Untitled)
  Session 7: 2DARvf8VaJEYmR3ZQTg3wNjCYD1qm-UfiS_swV6zw4U-iYdZr3S_tdKIxcYEWQQc20fcu5Z5-wkIJkzYAYE5oA (Untitled)
  Session 8: E7-I8XAnUeMaZODbqD7IbQN-IgAo2buyNzndQ67BHVxOVjry2cEu5Hxb6iUvkluIGg3824LMHQ-lyA-wATFXBA (Untitled)
  Session 9: F17zdv9PM_0zUilrEclCjxCpMgMW_Br9ZvTv1rF0ubKj6iCZ4hoIYDhqXtXhDr0n6lry4K_qu7ZXaOlLyE1dug (Untitled)
  Session 10: eNqHkJ2MZg4gvX96oNbOKGRzQLaFSbJSZ8PZ7cHp4vMBvWDLbAtwAiriVksCmOcMp_ONa-G4nqgfoM6_DBqxcQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: iOKC7IJgA7lC5e1hQRleXRXOTKzBPt2Pp76B50cbG_p0rrALp2y2Pp3U8c3D_EvjR_ZbwFeJh-k5ijGxaoWcZQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: nsKp_jHWjYU4ERBj-UclAhEbn9EvkdmecxnEI0uPviPpFX944899JSm4Ix1WQxvgwnVJv3PP6yZmkqAdKkLumg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the provided context, the available product is ekoDB, a high-performance database product. It costs $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: I'm sorry, but the context provided does not include information on the price of ekoDB. I would recommend checking the official website or contacting the company directly for the most accurate and up-to-date price.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: f1LM19UGDGVoEnWt-TBgGgaueeDQVvaVUptvuFLpbRPhdSmiicHZquiEGq-mo53xF1hD3n83jaH7gQTgNsby6w
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
  1. Score: 25.740, Matched: name.value, email.value, email, name
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio, title, bio.value
  2. Score: 26.400, Matched: title.value, bio.value, bio, title
  3. Score: 26.400, Matched: title, title.value, bio, bio.value
  4. Score: 26.400, Matched: bio, bio.value, title.value, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, title.value, bio, bio.value
  2. Score: 39.600, Matched: bio, title.value, bio.value, title
  3. Score: 39.600, Matched: bio, title, bio.value, title.value
  4. Score: 39.600, Matched: title, title.value, bio.value, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.766, Matched:
  2. Score: 0.750, Matched:
  3. Score: 0.748, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.700, Matched: content, title.value, title, content.value
  2. Score: 1.499, Matched: content.value, title, title.value, content
  3. Score: 0.306, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: bio.value, bio

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
✅ TypeScript client examples complete!
✅ All TypeScript integration tests complete!
🧪 Running JavaScript examples (direct HTTP/WebSocket)...

added 1 package, removed 1 package, and audited 9 packages in 760ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities

╔════════════════════════════════════════╗
║  ekoDB JavaScript Examples Test Suite ║
╚════════════════════════════════════════╝

=== Checking Server Connection ===
(node:44972) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Server is ready

=== Getting Authentication Token ===
✓ Authentication successful

=== Running 7 Examples ===

=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js ===
✓ Authentication successful

=== Insert Document ===
(node:44974) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Inserted: {
  id: 'd0YHynfJcqK291yOc3EAu_ZMtJohinPVnkUD8Jp5Px6yG4HiltjXbRtGgGJpk_72qbTdklTIOfk3-FQOkMiuAw'
}

=== Find by ID ===
Found: {
  active: { value: true, type: 'Boolean' },
  id: 'd0YHynfJcqK291yOc3EAu_ZMtJohinPVnkUD8Jp5Px6yG4HiltjXbRtGgGJpk_72qbTdklTIOfk3-FQOkMiuAw',
  value: { value: 42, type: 'Integer' },
  name: { value: 'Test Record', type: 'String' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { value: 100, type: 'Integer' },
  name: { type: 'String', value: 'Updated Record' },
  id: 'd0YHynfJcqK291yOc3EAu_ZMtJohinPVnkUD8Jp5Px6yG4HiltjXbRtGgGJpk_72qbTdklTIOfk3-FQOkMiuAw',
  active: { value: true, type: 'Boolean' }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.js completed successfully

=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js ===
✓ Authentication successful

=== Inserting Test Data ===
(node:44993) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted test record: 2nkGefmYq4bb86UF2pOB2f9kpwFwomefMzskJuXwdMeUs00NXeW_ru66Zr1MRbTOS51NXwauM4x4TtajqRojQQ

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
        "id": "r18nKHVTruUYT6ullGjZE4TagZiudfGWn3YfI31OQpifIMSGzSHL2c_60kPko2n5j0gtJnQFvsCKyub2E0sb2Q",
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
        "id": "mx1JFaLb4gTtxeTgdH9UaJLpr0vKTa46mpbTkqSNvKDYbl_1USaor2hbrjaYl-4eaYkbo7YpdW6uLPUsRhM62Q",
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
        "id": "u58b8k85ctDidCO_rD1ii3VsNqtNwbmL0RcesNOJIgti_hY65pxOa0KESCYIkOS41DDuIdR-WnnATUrDv0hsbQ",
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
        "id": "2nkGefmYq4bb86UF2pOB2f9kpwFwomefMzskJuXwdMeUs00NXeW_ru66Zr1MRbTOS51NXwauM4x4TtajqRojQQ",
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
✓ simple_websocket.js completed successfully

=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/batch_operations.js ===
✓ Authentication successful

=== Batch Insert ===
(node:45008) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
✓ batch_operations.js completed successfully

=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/kv_operations.js ===
✓ Authentication successful

=== KV Set ===
(node:45028) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {
  loginTime: '2025-11-01T06:42:12.546Z',
  userId: 123,
  username: 'john_doe'
}

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
✓ kv_operations.js completed successfully

=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/collection_management.js ===
✓ Authentication successful

=== Create Collection (via insert) ===
(node:45038) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Collection created with first record: Xt7maffluqaBGlmaCIHwh2p6HNMQmfs1xAz1K-9A6bJE6zZNaEfXnUMYvZl5hRHEx0l1AIWee3YlBKqYDxfXEQ

=== List Collections ===
Total collections: 21
Sample collections: [
  'schema_users_client_go',
  'ttl_cache',
  'chat_messages__ek0_bench',
  'schema_users_client_ts',
  'batch_users'
]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ collection_management.js completed successfully

=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/document_ttl.js ===
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
(node:45080) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document: tSmh6uRJb5Z6fVdcBo3EZEQpw94J-VglFScH2fXmK5tMMZGo4mezlOj05GjyRswa5jCytLApPYwPISI_oSYTcQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: created

=== Query Documents ===
✓ Found 4 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully
✓ document_ttl.js completed successfully

=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js ===
✓ Authentication successful

=== Insert Test Data with TTL ===
(node:45082) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document with TTL: AtjOUlRsXMdHPghUCeACx0Q3m-rJ30Ag1VKBehDJJqyBe9y2LP9B0EiC-QEq1x-vbSCjWwvbI0HW8vXUZPt41Q

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 4 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
✓ websocket_ttl.js completed successfully

╔════════════════════════════════════════╗
║           Test Summary                 ║
╚════════════════════════════════════════╝
Total: 7
Passed: 7
Failed: 0
✅ JavaScript direct examples complete!
🧪 Running JavaScript client library examples...

added 1 package, removed 1 package, and audited 9 packages in 760ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'Gv6wmdlo6BSZUTfp6Fagr6fJFVHSlnTuHzV9MCCKgEcKdeCj_vndJn-JZhIBLkhPqq8kkC6UrPr461ch-q2lVg'
}

=== Find by ID ===
Found: {
  id: 'Gv6wmdlo6BSZUTfp6Fagr6fJFVHSlnTuHzV9MCCKgEcKdeCj_vndJn-JZhIBLkhPqq8kkC6UrPr461ch-q2lVg',
  name: { type: 'String', value: 'Test Record' },
  value: { value: 42, type: 'Integer' },
  active: { value: true, type: 'Boolean' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 100 },
  id: 'Gv6wmdlo6BSZUTfp6Fagr6fJFVHSlnTuHzV9MCCKgEcKdeCj_vndJn-JZhIBLkhPqq8kkC6UrPr461ch-q2lVg'
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Batch Insert ===
✓ Batch inserted undefined records
TypeError: inserted.map is not a function
    at main (/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/client_batch_operations.js:38:24)
    at process.processTicksAndRejections (node:internal/process/task_queues:105:5)
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
Collection created with first record: kYR9JlVuC23tDsxb9XvCmVZSydkGMSmUpxYKSx4I-QlbRtQJUD87QmOgV2gtC1wc5ajOWHwq_2lzA6dIzVacoA

=== List Collections ===
Total collections: 22
Sample collections: schema_users_client_go,ttl_cache,chat_messages__ek0_bench,schema_users_client_ts,client_batch_operations_js

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: nnPgTuFczeuWofRr6nlw1mCjsXFoyKp2OBuCOJA-AqEDM3J_Ncsj-cmzPrwbruDZF06M706killfWQcMmMy04Q

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: mBnDK2H_t4LjYpzWm4EHOCCShKV6rMc8Gxo1yvol6dlAAYJ8tNJdxsQVDzVtdc4sO5lwMkPMdG0t0Y1NUairSQ

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
✓ Inserted test record: uWq5Ke3ggalZ3Fp2vPruJMZ_naH6p_D3ujCgjhCGUdMwywA2mAhUGYEZl0zdOCLwAsZf-MqR4YRQ8czvoW23hw

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
✓ Inserted document with TTL: TIv0RZ05DWT7EQjDBOLMEdqie36Er3TPfI7Px7P0XJJiTNy2DjZDDf3R6Lu84yl5tRgBM9jTOgu5dmjV9i6ejQ

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
✅ JavaScript client examples complete!
✅ All JavaScript integration tests complete!
🟣 Building Kotlin client library...
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build

BUILD SUCCESSFUL in 3s
2 actionable tasks: 2 up-to-date
✅ Kotlin client built!
🧪 Running Kotlin client library examples...
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Simple CRUD Example ===

=== Create ===
✓ Inserted user: Record(fields={id=StringValue(value=JtaiuogLUzhoLGMQg7pNe0I2-OTIeMb67CKKW7VqTiNJCOHcJAuffULxpjjKWdT-FOAny5wYD7Hahs7X24QpeQ)})
  User ID: JtaiuogLUzhoLGMQg7pNe0I2-OTIeMb67CKKW7VqTiNJCOHcJAuffULxpjjKWdT-FOAny5wYD7Hahs7X24QpeQ

=== Read ===
✓ Found user by ID: Record(fields={active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), id=StringValue(value=JtaiuogLUzhoLGMQg7pNe0I2-OTIeMb67CKKW7VqTiNJCOHcJAuffULxpjjKWdT-FOAny5wYD7Hahs7X24QpeQ), age=ObjectValue(value={value=IntegerValue(value=28), type=StringValue(value=Integer)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)})})

=== Update ===
✓ Updated user: Record(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), id=StringValue(value=JtaiuogLUzhoLGMQg7pNe0I2-OTIeMb67CKKW7VqTiNJCOHcJAuffULxpjjKWdT-FOAny5wYD7Hahs7X24QpeQ), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)})})

=== Query ===
✓ Found 1 users matching query
  - Record(fields={id=StringValue(value=JtaiuogLUzhoLGMQg7pNe0I2-OTIeMb67CKKW7VqTiNJCOHcJAuffULxpjjKWdT-FOAny5wYD7Hahs7X24QpeQ), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)})})

=== Delete ===
✓ Deleted user with ID: JtaiuogLUzhoLGMQg7pNe0I2-OTIeMb67CKKW7VqTiNJCOHcJAuffULxpjjKWdT-FOAny5wYD7Hahs7X24QpeQ

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Batch Operations Example ===

=== Batch Insert ===
✓ Inserted 5 records
  IDs: DBjng4kY2siUHrVdP04Imh9nA6Ch1rx_wXVqJk3GOlldN9HWwMDyPONZUj4WKYw8drO50unfJoSbBNGcQX8IrQ, ryx4KnnZzN-OHNHpYM05yvXePHHzHFf-RhfxAO_0aBkXD05IFPL-IEQvqq7AaxJsOrO_rVDMvAC8H5_4nYq0RA, Mwf6ky5x8hVRYd2M1cgQnqQka1WK_hy7GMCK1jLr_C2x6c4kU28ROjqGiIbyXSBwUFM_Y314W9mMdu9fhMSGZg...

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
✓ Session value: {"created_at":1761979363880,"user_id":"123"}
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

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Collection Management Example ===

=== List Collections ===
✓ Found 21 collections
  - schema_users_client_go
  - ttl_cache
  - chat_messages__ek0_bench
  - schema_users_client_ts
  - client_batch_operations_js

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-11-01T06:42:48.331782Z","last_modified":"2025-11-01T06:42:48.331783Z","bypass_ripple":false,"primary_key_alias":"id"}

=== Cleanup ===
✓ Deleted collection: kotlin_collection_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Document TTL Example ===

=== Insert with TTL ===
✓ Inserted document with 10s TTL
  Document ID: 5n7okbraVdUDBIt-117R9LWWUcBnwe2yKCW9SkAEZYt5MzedKcDaMY1ZIwqpRL8cgP-BnfAgZJrwKV07w3jEdQ

=== Verify Document Exists ===
✓ Document found: [created_at, ttl_duration, user_id, id, session_id]

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: 3JYhqtVCdnkdrupQKjpHvB7L13iLMl47tSm0VnLdD5vyP9u3zhXbQUwwSMOs4zhggc4X-TxBOFZ3cNKEfy61sg

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: [value, cache_key, id, ttl_duration]

=== Delete Document ===
✓ Deleted document

=== Cleanup ===
✓ Deleted collection: kotlin_ttl_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build

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
  {"payload":{"data":[{"id":"mOmYxZIpA4HShkqha-ykxSWt8f9UcX9r6PUJVU5MJfHHZCbrqtCwupU-N1L1vhkydMma0csl2_sng_3mdAn_vg","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

=== Close WebSocket ===
✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'kotlin_schema_example' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: StringValue(value=xrQm_aPXx6fD05kw31xZC_VsmHICpYtVEBug_XLEqkr97WyZM9CuOIjuZPBWzVtx80h2B6K5zGNkrxjSMOKSoQ)
✓ Inserted user 2: StringValue(value=I_0dFtcgtL1rms0_QJBExYY-9PaUT23vn2oOphxfizoQw7rEEtCM3cig9_o48AqLNad0kCJQ32UeNP2fJsdcKg)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 22
  Sample: [schema_users_client_go, ttl_cache, chat_messages__ek0_bench, schema_users_client_ts, client_batch_operations_js]

=== Cleanup ===
✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Search results for 'programming':
  {"results":[{"record":{"views":372,"description":"Learn Rust programming language with hands-on examples and best practices.","tags":"programming,rust,tutorial","title":"Rust Programming","id":"y7_R-WeaQzPELTWauFIUNKKoALHbSN4FEPX4fg9BcqCtubfrW2UsUBFtgLdKOL2yBH2BBqBb-8K9wVbK0SOT7Q"},"score":19.8,"matched_fields":["description","title","tags"]},{"record":{"views":420,"tags":"programming,python,data-science","id":"bLot2UI9nbR8oBf0J8s93T-OSXJpkBv3piXN3KjQph7kiqDjfT837XiUoP5vEz0JUtKqmUq3gk_7ah1oPGyWww","title":"Python for Data Science","description":"Master Python for data analysis, machine learning, and visualization."},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"views":284,"description":"Build modern web applications using JavaScript, React, and Node.js.","id":"ln2KHo1oAN2qk5BjQYrP054jrczGWDoZXMGQ24DG-gmtbkL5crShIJiQMManyIKkRNj3WWBi0V82_lKhcwpLmQ","tags":"programming,javascript,web","title":"JavaScript Web Development"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"views":571,"id":"J2tnOkufUJkvU4YvscyivJQdJDyO8KXEUdGQ2f0wosQBZjmVprV9jQ3FSHUMEVMISjBKen6b8UcNfE7ZOSn1kw","tags":"ai,machine-learning,python","title":"Machine Learning Basics","description":"Introduction to machine learning algorithms and neural networks."},"score":2.7,"matched_fields":["description","title","tags"]},{"record":{"title":"Python for Data Science","id":"bLot2UI9nbR8oBf0J8s93T-OSXJpkBv3piXN3KjQph7kiqDjfT837XiUoP5vEz0JUtKqmUq3gk_7ah1oPGyWww","description":"Master Python for data analysis, machine learning, and visualization.","tags":"programming,python,data-science","views":420},"score":1.0,"matched_fields":["description"]},{"record":{"title":"Rust Programming","description":"Learn Rust programming language with hands-on examples and best practices.","views":372,"tags":"programming,rust,tutorial","id":"y7_R-WeaQzPELTWauFIUNKKoALHbSN4FEPX4fg9BcqCtubfrW2UsUBFtgLdKOL2yBH2BBqBb-8K9wVbK0SOT7Q"},"score":0.5,"matched_fields":["description"]},{"record":{"id":"sIRDBLXFo88Bfa_5lE1hJMdyvmSUO95pRcpKsVr9kRAqwfVMIlBG_wOhW4RZEfgYKMs-Wl5OOGquxFvgTiQQ7g","title":"Database Design","description":"Learn database design principles, normalization, and query optimization.","tags":"database,design,sql","views":432},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

=== Cleanup ===
✓ Deleted collection: kotlin_search_example

✓ All search operations completed successfully

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - WebSocket TTL Example ===

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: StringValue(value=JeW6XE_8eX0GyfIppb-pqYX2j-YfHcsxY67NVdp_uWehvVWvfXYdyKsowRuKd5wyYOgFTB9liRZ3e6OfH53PmQ)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1761979389967},"id":"JeW6XE_8eX0GyfIppb-pqYX2j-YfHcsxY67NVdp_uWehvVWvfXYdyKsowRuKd5wyYOgFTB9liRZ3e6OfH53PmQ","name":{"type":"String","value":"WebSocket TTL Test"},"ttl_duration":{"type":"String","value":"1h"},"value":{"type":"Integer","value":42}}]},"type":"Success"}

✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_ttl_example

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build

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
  - Mouse ($25) by Alice Johnson
  - Laptop ($1200) by Alice Johnson

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

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: JwqCvUusAg_I6FImB_ad1rQ2Zn6twknt9ZRCnqvjDH8R2RXfo7r9qUFxsg2eFSOYnh9LKRJAwVZFq_wq8zNhJQ

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "iY90s9eDClMNc11Y9exUYvAfWAM_3IJwayTaEbeEQBWOVgYl3vG30VUKYUMsPB89a_4XXicXjA2NteA7ew-3kg"
  Responses: ["ekoDB is a high-performance database that comes with intelligent caching and real-time capabilities. It also incorporates AI integration for enhanced functionality. Some of the key features of ekoDB include a chat feature that uses AI to respond to database queries in natural language, providing responses with relevant context. Additionally, ekoDB supports comprehensive search capabilities, including full-text search, vector search, and hybrid search. These features also include automatic context retrieval which further refines the search results."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 6s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: rHP-aEVw23Q9bl9wR9q49U_HN3VZmAoWxGd28UEPZd8Z4-rxbYRN-HiqXJr0wf6oxdbEVABViCiNHi5ukUcboA

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["Based on the provided context, the available product is ekoDB. It's a high-performance database product with AI capabilities and is priced at $99."]

✓ Message 2 sent
  Responses: ["As an assistant, I'm sorry, but I can't provide the specific price you're asking for because the context doesn't include information about any particular product. Could you please specify the product you're interested in?"]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"rHP-aEVw23Q9bl9wR9q49U_HN3VZmAoWxGd28UEPZd8Z4-rxbYRN-HiqXJr0wf6oxdbEVABViCiNHi5ukUcboA"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"type":"Object","value":{"collection":{"type":"String","value":"kotlin_chat_sessions_example"},"matched_fields":{"type":"Array","value":[{"type":"String","value":"description"}]},"record":{"type":"Object","value":{"description":{"type":"String","value":"A high-performance database product with AI capabilities"},"id":{"type":"String","value":"XVq7WdDmaTo85OcwOPKSk0OVa99zjIEKsdLqWY2ob-yNnzt1qHBMEXVtCEqGEjZeVolzjd3p2sTjF4q2uJEfIQ"},"price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}}},"score":{"type":"Float","value":0.25}}}]},"created_at":{"type":"DateTime","value":"2025-11-01T06:43:26.991674Z"},"id":"1qywO3zbNPc9E6INBqmljzzxyTLRHJKt91wwGs798_aFwLx5qDXBPbBarEAmRe1OGHBUA6mtSTORO_P6n_p-ig","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":31},"prompt_tokens":{"type":"Integer","value":166},"total_tokens":{"type":"Integer","value":197}}},"updated_at":{"type":"DateTime","value":"2025-11-01T06:43:26.991674Z"}},{"chat_id":{"type":"String","value":"rHP-aEVw23Q9bl9wR9q49U_HN3VZmAoWxGd28UEPZd8Z4-rxbYRN-HiqXJr0wf6oxdbEVABViCiNHi5ukUcboA"},"content":{"type":"String","value":"Based on the provided context, the available product is ekoDB. It's a high-performance database product with AI capabilities and is priced at $99."},"context_snippets":{"type":"Array","value":[{"type":"Object","value":{"collection":{"type":"String","value":"kotlin_chat_sessions_example"},"matched_fields":{"type":"Array","value":[{"type":"String","value":"description"}]},"record":{"type":"Object","value":{"description":{"type":"String","value":"A high-performance database product with AI capabilities"},"id":{"type":"String","value":"XVq7WdDmaTo85OcwOPKSk0OVa99zjIEKsdLqWY2ob-yNnzt1qHBMEXVtCEqGEjZeVolzjd3p2sTjF4q2uJEfIQ"},"price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}}},"score":{"type":"Float","value":0.25}}}]},"created_at":{"type":"DateTime","value":"2025-11-01T06:43:26.992721Z"},"id":"T1RWvf0rB686px0e7Ck86LBUNVgJa1ZvQgC7f5CF4JQD3eaVSvTeWcUEsDakt-URMwdGzmKrBeM23n507wiKmg","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":31},"prompt_tokens":{"type":"Integer","value":166},"total_tokens":{"type":"Integer","value":197}}},"updated_at":{"type":"DateTime","value":"2025-11-01T06:43:26.992721Z"}},{"chat_id":{"type":"String","value":"rHP-aEVw23Q9bl9wR9q49U_HN3VZmAoWxGd28UEPZd8Z4-rxbYRN-HiqXJr0wf6oxdbEVABViCiNHi5ukUcboA"},"content":{"type":"String","value":"What is the price?"},"created_at":{"type":"DateTime","value":"2025-11-01T06:43:28.671176Z"},"id":"-MI9LmaeTp7yJrZI7l56kcqMJCyvSTL2AwzpQT6ujEk8xKY9ae9zXeW1cZ4zC1clYI_VN7qUNtkIraf1WvauUg","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":43},"prompt_tokens":{"type":"Integer","value":63},"total_tokens":{"type":"Integer","value":106}}},"updated_at":{"type":"DateTime","value":"2025-11-01T06:43:28.671176Z"}},{"chat_id":{"type":"String","value":"rHP-aEVw23Q9bl9wR9q49U_HN3VZmAoWxGd28UEPZd8Z4-rxbYRN-HiqXJr0wf6oxdbEVABViCiNHi5ukUcboA"},"content":{"type":"String","value":"As an assistant, I'm sorry, but I can't provide the specific price you're asking for because the context doesn't include information about any particular product. Could you please specify the product you're interested in?"},"created_at":{"type":"DateTime","value":"2025-11-01T06:43:28.672606Z"},"id":"oHw851902VWXtW6MzCXm1dMJI9IOsw55PtjiFbOdS66uLaPIZNNYUPb5DfFep1HBTH_YbziQNcJCDJK1JgylDw","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":43},"prompt_tokens":{"type":"Integer","value":63},"total_tokens":{"type":"Integer","value":106}}},"updated_at":{"type":"DateTime","value":"2025-11-01T06:43:28.672606Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 13

=== Branching Session ===
✓ Created branched session: elXE1KhxCsDfUgdgnl8V-knCJ3XQivIueOYSPKSqcW9ARf4o2Jf7K8My5xX0XVsaMWXjBB_JryVShTtU5h_bKw

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 7s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 814WSGEQAVv6kINN4FIKkb3CpPIPr0F9I1TR2lBLS61PT8ktGMW-WPK5yaj6HhuS2OhiV2YEQImfQxnyrDGy-w

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the context provided, the product available is ekoDB, a high-performance database product priced at $99."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Based on the information provided, the available product is a high-performance database product named ekoDB. It is priced at 99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: OGZ-Ob1WMHWmvo1RGVXl_bgjbLm3HdhvhIW2lJGejVGDyUSUhiNlsMPBknsfsWrqLKT2zjs1szHNeCKUpkoyrw

=== Merging Sessions ===
✓ Merged sessions
  Merged session ID: null

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 8s
2 actionable tasks: 1 executed, 1 up-to-date
✅ Kotlin client examples complete!
✅ All Kotlin integration tests complete!
✅ All integration tests complete!
```
