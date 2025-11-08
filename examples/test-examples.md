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
  rust: 15 client examples
  python: 15 client examples
  go: 15 client examples
  typescript: 15 client examples
  javascript: 8 client examples
  kotlin: 15 client examples
✅ Examples list generated!
  Total: 111 examples
  Direct: 28 | Client: 83
  Files: examples_list.txt.tmp, examples_list.json.tmp
✅ Examples inventory is up to date!
🧪 Running Rust examples (direct HTTP/WebSocket)...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("msleTXd2OLMY-gD62TrThCKiWQbGe-8TZvG21QWse0XiQzSAygfv_Ksrn07YVorMHynk6O0d3UKCmJHMFyhM_Q")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("msleTXd2OLMY-gD62TrThCKiWQbGe-8TZvG21QWse0XiQzSAygfv_Ksrn07YVorMHynk6O0d3UKCmJHMFyhM_Q"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("msleTXd2OLMY-gD62TrThCKiWQbGe-8TZvG21QWse0XiQzSAygfv_Ksrn07YVorMHynk6O0d3UKCmJHMFyhM_Q"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("msleTXd2OLMY-gD62TrThCKiWQbGe-8TZvG21QWse0XiQzSAygfv_Ksrn07YVorMHynk6O0d3UKCmJHMFyhM_Q"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "b3qKJBsYGoEqxYexL1BWzxzSF326PmhRdXS93j5DqzjBhJCGsP0LBUPPqvQKPBASWK66AXL464rSMI7vSZ8vRA"

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
        "id": "b3qKJBsYGoEqxYexL1BWzxzSF326PmhRdXS93j5DqzjBhJCGsP0LBUPPqvQKPBASWK66AXL464rSMI7vSZ8vRA",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "6gtCCqKXDMBLwIb1PF-oXkggyy7IhVXY9-xuipO8gxlmd6Re5U6N87C7wPZbQIOH-bxJKcpfb6HmnNDSC7gHeg"

=== List Collections ===
Total collections: 4
Sample collections: ["demo_collection", "batch_users", "websocket_test", "test_collection"]

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
✓ Inserted document: "G9Ln4_ehsGBzN7YOJ747Ny6VANPtwcBst-4WULEK1ekuG0SDZN6TDNVVtH_9iIyUoM1LMJwswJBycC4zJRK-aQ"

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
✓ Inserted document with TTL: String("S2VY1-vW7q3UDpWh2SluA5XDKo7dwlHufimOV_re9vhUKUh2pvQs6yynM7PuynVHaPBz306ROifEq3Vp05FMuQ")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
✅ Rust direct examples complete!
🛠️  Building client library...
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
✅ Client build complete!
🧪 Running Rust client library examples...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record { fields: {"id": String("bI0PtYLZn8dTXgxffUzPajq8-5W2Xnw50R7bRvVcdhe82KrJJrrp_a0rAosbb1tNx8xJiAD5ODXDkGoKp8XnBw")} }

=== Find by ID ===
Found: Record { fields: {"name": Object({"value": String("Test Record"), "type": String("String")}), "id": String("bI0PtYLZn8dTXgxffUzPajq8-5W2Xnw50R7bRvVcdhe82KrJJrrp_a0rAosbb1tNx8xJiAD5ODXDkGoKp8XnBw"), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "value": Object({"type": String("Integer"), "value": Integer(42)})} }

=== Find with Query ===
Found documents: [Record { fields: {"name": Object({"type": String("String"), "value": String("Test Record")}), "id": String("bI0PtYLZn8dTXgxffUzPajq8-5W2Xnw50R7bRvVcdhe82KrJJrrp_a0rAosbb1tNx8xJiAD5ODXDkGoKp8XnBw"), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "value": Object({"value": Integer(42), "type": String("Integer")})} }]

=== Update Document ===
Updated: Record { fields: {"name": Object({"type": String("String"), "value": String("Updated Record")}), "id": String("bI0PtYLZn8dTXgxffUzPajq8-5W2Xnw50R7bRvVcdhe82KrJJrrp_a0rAosbb1tNx8xJiAD5ODXDkGoKp8XnBw"), "value": Object({"value": Integer(100), "type": String("Integer")}), "active": Object({"value": Boolean(true), "type": String("Boolean")})} }

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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
Collection created with first record: "5bTXo8EAnOqcgZTa4ctW-Gl1HYcHVFxFc2wXDsRqtWrOAOgu4Va9hE5Esy4ZtkPfF0Yn0UjI8MYAYHpk7uygzQ"

=== List Collections ===
Total collections: 6
Sample collections: ["batch_users", "ws_ttl_test", "ttl_cache", "websocket_test", "client_collection_management_rust"]

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
✓ Inserted document: "9Nt1RK244bhwoCpHYZFhiLi2Q2AKp8iBLfvmCo8dbfN2wHOc4-0kgfaJdZHf8aIika_Ef30o01Lp-YGmDYK2EQ"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("gu7QEqQzZ8YPtkIFdqLHJkrI60i-GeIzm8fuaXYUknejdzoMma1Xc3jr1Wa3fFP1eSF7eVN237kC-jTOmRbUuw"))

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: tN6mJZsmD3NHhP60_i0at1R9tVZokXQqKwPaicnAHbtTRfxHjUT7tXnKBycq1E78kxmC92SCuLXCpMSCzE2fvQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("6FX1E_Pnl97LoxXF5Vl0YPcBZNRI6DyYg5oh6pw9FAI5MdGcuDhd03-2mfYkUT8_ZhvQysV40bEIydu2aamDug"))

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
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Regex Query ===
✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Charlie")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
  2. Some(Object({"value": String("David"), "type": String("String")}))
  3. Some(Object({"type": String("String"), "value": String("Alice")}))

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
     Matched: ["title.value", "description", "description.value", "title"]
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
Execution time: 1ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_schema_management`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("nhzofFpreInBYTYqEGk6U8Fv0k9omGrP6HYI_-0l_WVf0excBcjya30qPFdkaIPgRGkuEYGAInTelNiZwnziEQ"))
✓ Inserted user 2: Some(String("oUgOE-AeS4PBIjFCwZ9D1zrVgxwuEZHdKVva_KU_za2DuNUiJmclwg8exnzRFcQ8hBgMq7onUCPYC30xm6mIUg"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - email: String
    (required)
  - title: String
    (required)
  - status: String
  - age: Integer

=== Listing Collections ===
✓ Total collections: 6
  Sample: ["batch_users", "ws_ttl_test", "ttl_cache", "websocket_test", "schema_client_rust"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: BqjVpTOe2YDFBTUp5LYCj-asQ8Kq6eWYAG8VauI2-32YV4PygmpRpubrtxvfNNK0wzN5htcVRhyYPeiMKKGDIA

=== Sending Chat Message ===
Message ID: pp1SoANGvQNU1F7r6tYtnOrbFIU9b5bZnA7U4umDKXpVlqJZRPnDGVs2ShYmetdolFTkROV-277xU-O2ycTFJA

=== AI Response ===
Response 1: ekoDB is a high-performance database with intelligent caching and real-time capabilities. It also features AI integration, which can provide AI-powered responses to natural language queries. The unique features of ekoDB include a chat feature that allows you to query your database using natural language. Additionally, it supports various types of search including full-text search, vector search, and hybrid search with automatic context retrieval.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["title", "content", "category"]
  Record: Object {"category": Object {"type": String("String"), "value": String("features")}, "content": Object {"type": String("String"), "value": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context.")}, "id": String("NuqC5wxo14Me5ZyhrjDLLM3U31nvebigMtbcN3k_xNa_rQRXHEm-4lahb7F-i4TUBhglREa-FBew0Hv4-uGU9w"), "title": Object {"type": String("String"), "value": String("AI Chat Integration")}}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "category", "title"]
  Record: Object {"category": Object {"type": String("String"), "value": String("features")}, "content": Object {"type": String("String"), "value": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval.")}, "id": String("pfxDYTCCqiT71l6OZyGoB9bXgwUz_ezHum0Fx2t2QAExZP_6Qa8cN_DBwOrsdjNoWcYWTWAPMden5t9GOMHweg"), "title": Object {"type": String("String"), "value": String("Search Features")}}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": Object {"type": String("String"), "value": String("documentation")}, "content": Object {"type": String("String"), "value": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration.")}, "id": String("mdcyZndpX6O9yJIVuvb_5Un5RumznDwkpXyl6I7FnOs62F2HLNOKsh26rlpFk5AxDgwwepSETchv9wJSQFw3xQ"), "title": Object {"type": String("String"), "value": String("Introduction to ekoDB")}}

Execution Time: 2906ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: E4-BhRR90TkWbmBAsMzY96ozEcDu26Mp-ubST8m6lBcBPpLkIkKRkBp7_LWRZsrG8lhEtjOLQs9gk-j5TTtMDw
=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information given, the available product is ekoDB. It is a high-performance database product with AI capabilities, priced at $99.

✓ Message 2 sent
  Response: Apologies, but there seems to be no context provided about a specific product. Could you please provide more details about the product you are inquiring about?

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: ZagzVP3EWE2tM9sdfXJSafEaG1ia41z_MYZd8SGvJdMrx1HC9zlSnNPttKiecgS1_HHdSB3aJgK0xOeYjDJP1A
  Parent: E4-BhRR90TkWbmBAsMzY96ozEcDu26Mp-ubST8m6lBcBPpLkIkKRkBp7_LWRZsrG8lhEtjOLQs9gk-j5TTtMDw

=== Listing Sessions ===
✓ Found 3 sessions
  Session 1: ZagzVP3EWE2tM9sdfXJSafEaG1ia41z_MYZd8SGvJdMrx1HC9zlSnNPttKiecgS1_HHdSB3aJgK0xOeYjDJP1A (Untitled)
  Session 2: E4-BhRR90TkWbmBAsMzY96ozEcDu26Mp-ubST8m6lBcBPpLkIkKRkBp7_LWRZsrG8lhEtjOLQs9gk-j5TTtMDw (Untitled)
  Session 3: BqjVpTOe2YDFBTUp5LYCj-asQ8Kq6eWYAG8VauI2-32YV4PygmpRpubrtxvfNNK0wzN5htcVRhyYPeiMKKGDIA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: ZagzVP3EWE2tM9sdfXJSafEaG1ia41z_MYZd8SGvJdMrx1HC9zlSnNPttKiecgS1_HHdSB3aJgK0xOeYjDJP1A

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: dpgzErDlHxFuY84pGX3gK3LtQCDcWSE5I3g-nd1GXTfh1o7Y6y0v0bhpSiowsumB4pHi9MFFNhM-geSWD_DgBw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the provided context, the available product is ekoDB, a high-performance database product priced at $99.

✓ Second message sent
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: I'm sorry, but the context provided does not include information on the price of ekoDB. I would recommend checking the official website or contacting the selling company for the most accurate and current price.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: V9pnex-gR-kOF--avO0vL4KqKqtDXPgIo-f47IsI1SL7RYDmSnQcSfPjIf1G4CZlv17xmYvsfE0wSzyhmmlYwQ
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
=== ekoDB Rust Client - Functions Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: LxCZCjHsLT_NtJUPasTBTXVXTkWI87BT8RNMLQZk5d8xz4zig0louxKuR6OH8_2ocmLZqZxqw5BejZXi_qUSsA
📊 Found 10 active users

📝 Example 2: Parameterized Function

✅ Function saved: WTav9mWzMpV0sK7xp0yjzzUKSDGvC-7Eov3igAdNdxZInBo1V8hm0j8fFr7ij6tfW4yG2pssSlFEki7VZregGg
📊 Found 10 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: CrpEH2fmiodiCHsBxq-DkSP7nPMaZr-lGFrpmmH0-lAvtxt9hJsftICg4idHVykuxHlQ7Oi1MVjOaUcjKA_x_w
📊 Statistics: 2 groups

📝 Example 4: Function Management

📋 Total functions: 3
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID returned from save
ℹ️  Only CALL can use either ID or label


✅ All examples completed!
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
Inserted: {'id': 'akbGMGRQW03wcGFoMIVYFJbkShDLEkbVoxT5refuGnZEwwEHRi63U5y74UmdZKPHH7AHxiHxiZnat8obksVMZg'}

=== Find by ID ===
Found: {'value': {'value': 42, 'type': 'Integer'}, 'id': 'akbGMGRQW03wcGFoMIVYFJbkShDLEkbVoxT5refuGnZEwwEHRi63U5y74UmdZKPHH7AHxiHxiZnat8obksVMZg', 'active': {'value': True, 'type': 'Boolean'}, 'name': {'type': 'String', 'value': 'Test Record'}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'id': 'akbGMGRQW03wcGFoMIVYFJbkShDLEkbVoxT5refuGnZEwwEHRi63U5y74UmdZKPHH7AHxiHxiZnat8obksVMZg', 'value': {'value': 100, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'active': {'value': True, 'type': 'Boolean'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.py completed successfully

=== Running simple_websocket.py ===
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: UBNtJCjhlVriAbaDRg0xZ26HeOtKUwX-dlpUqRGL5z11UKSJp-Pkc5Z__XaEWqxdXMapBKLOeA9ZiwG2hisiFw

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
        "id": "UBNtJCjhlVriAbaDRg0xZ26HeOtKUwX-dlpUqRGL5z11UKSJp-Pkc5Z__XaEWqxdXMapBKLOeA9ZiwG2hisiFw",
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
        "id": "b3qKJBsYGoEqxYexL1BWzxzSF326PmhRdXS93j5DqzjBhJCGsP0LBUPPqvQKPBASWK66AXL464rSMI7vSZ8vRA",
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
cache:product:1: {'name': 'Product 1', 'price': 29.99}
cache:product:2: {'name': 'Product 2', 'price': 39.989999999999995}
cache:product:3: {'price': 49.989999999999995, 'name': 'Product 3'}

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
Collection created with first record: 5q30is7czRbnfO4tOCqKX5P40jI7r-7hPTrvBXA738nCNb7SBXXtgSpOokNRkq1My5DI3zoHijAa05WyZFStpQ

=== List Collections ===
Total collections: 10
Sample collections: ['demo_collection', 'batch_users', 'users', 'ws_ttl_test', 'ttl_cache']

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
✓ Inserted document: tDht5a8t9cbiQtbMcRhbA1ztCcaO-XCN969z-6clDZlcFgq9KMwqfCRVq3piL6JY2Ab-7INnNFIAioP5pNynaw

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
✓ Inserted document with TTL: JTaIcV2vJAfv7ht1dtysgRYW9p_0U0fGK_7WZX9ux-1n2iXDIBCjBxsmhPQYxCFLll_KC9WNNx6fY6BkDmV92w

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
    Finished `release` profile [optimized] target(s) in 0.14s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.3.0-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel for current platform...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.3.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.3.0
    Uninstalling ekodb_client-0.3.0:
      Successfully uninstalled ekodb_client-0.3.0
Successfully installed ekodb-client-0.3.0
✅ Python client package built and installed!
🧪 Running Python client library examples...
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'uocRXl5DanFoJadZKPiEjVB-Y0mb_5hMzEA6SYZkSx83n1_cIxRmWEDNjZQ7etuv_PqlxYrt7mo0NHibl_BH9g'}

=== Find by ID ===
Found: {'name': {'type': 'String', 'value': 'Test Record'}, 'id': 'uocRXl5DanFoJadZKPiEjVB-Y0mb_5hMzEA6SYZkSx83n1_cIxRmWEDNjZQ7etuv_PqlxYrt7mo0NHibl_BH9g', 'value': {'value': 42, 'type': 'Integer'}, 'active': {'value': True, 'type': 'Boolean'}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'active': {'value': True, 'type': 'Boolean'}, 'value': {'value': 100, 'type': 'Integer'}, 'id': 'uocRXl5DanFoJadZKPiEjVB-Y0mb_5hMzEA6SYZkSx83n1_cIxRmWEDNjZQ7etuv_PqlxYrt7mo0NHibl_BH9g', 'name': {'value': 'Updated Record', 'type': 'String'}}

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
Collection created with first record: "S46_IALEVlebI3vfP0ujR_bW1uOmNlghvkUWI1-JxXw9VxORUzDd8TVMze6StCCW85EhWGxI1Apjrkk6DW4lyQ"

=== List Collections ===
Total collections: 10
Sample collections: ['batch_users', 'users', 'ws_ttl_test', 'ttl_cache', 'websocket_test']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 11Y9QwFkS3Id28MtFuekKkMYvp_XYzKYTA9kKHMf-LHO8VDTCKxBE_AVxs6vIkqF1_Godquzmz4e92wNRJKfAQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: O66RRGw-PIU-BOPTVfBIKyujoXOKuCeA1YqZzusMElv_UWKQwU9EDkzdK6QhZin6wGsqZk-S1mqq9iS9gvWGmw

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
✓ Inserted test record: nllAtgzIUlia7fbXKVMn9s7S6NvFWOysShpr5cGgPSA1vikWFmby-0Oxu5upw4W42RynO7sdncicgtvllKMLLA

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
✓ Inserted document with TTL: HUukNHH4tRoqwXw_jAA3_VAqzY9VgGPdef1FqcdpLYouLALehmsWye0P5KNaAymd9HuC5s5MJ-8zmtKCbafbRA

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
  1. Score: 25.740, Matched: name.value, email, name, email.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, bio, title.value, title
  2. Score: 26.400, Matched: title, bio.value, bio, title.value
  3. Score: 26.400, Matched: bio, title, title.value, bio.value
  4. Score: 26.400, Matched: title.value, title, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title, title.value, bio
  2. Score: 39.600, Matched: bio, bio.value, title, title.value
  3. Score: 39.600, Matched: bio.value, title, title.value, bio
  4. Score: 39.600, Matched: bio.value, bio, title, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 0 semantically similar documents

7. Hybrid search (text + vector):
Found 2 results using hybrid search (text + vector)
  1. Score: 1.200, Matched: title.value, content.value, content, title
  2. Score: 0.600, Matched: title.value, title, content.value, content

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
✓ Created session: o6ROh93Coqn-_gnqNWC59GOxmLgM1k7HYlO1HayVf5bzNMqaOs-bFztu7geJqlYJ5NcCFeqje3ev2imk479btw

=== Sending Chat Message ===
Message ID: m5KAhb6M5FxZ6xZORQgQz_wAMEuM7CrywP-Jhq2nw9RJdTZU46yyOa46yvCpzkxN2PqO9xjT-lf_GrsKYsa_pg

=== AI Response ===
The available products and their prices are as follows:

1. ekoDB Cloud is a fully managed cloud database service product. Its price is $499.
2. ekoDB Pro is an enterprise edition product with advanced features. Its price is $299.
3. ekoDB is a high-performance database product with AI capabilities. Its price is $99.

Execution Time: 2209ms

=== Token Usage ===
Prompt tokens: 385
Completion tokens: 73
Total tokens: 458

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: KlR3nSGO8VdrtMOT8OJUboVMkLthxz7BZwBbW-bwVTT8XG_hFF3E-jcpF6BByX4Vtlu07DUCL3tS_R2A68rOmA

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is ekoDB. It is a high-performance database product priced at $99.

✓ Message 2 sent
  Response: I'm sorry, but there's no context provided about a specific product for which you're asking the price. Could you please specify the product?

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 91uaMt4240W1NtUlzNUkyApahglzbw7dFlWvOpMJx4bbktJ1WqEzymwg9cysNxY5mA62YGb1dxh6jDgNUh6gfA
  Parent: KlR3nSGO8VdrtMOT8OJUboVMkLthxz7BZwBbW-bwVTT8XG_hFF3E-jcpF6BByX4Vtlu07DUCL3tS_R2A68rOmA

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: 91uaMt4240W1NtUlzNUkyApahglzbw7dFlWvOpMJx4bbktJ1WqEzymwg9cysNxY5mA62YGb1dxh6jDgNUh6gfA (Untitled)
  Session 2: KlR3nSGO8VdrtMOT8OJUboVMkLthxz7BZwBbW-bwVTT8XG_hFF3E-jcpF6BByX4Vtlu07DUCL3tS_R2A68rOmA (Untitled)
  Session 3: o6ROh93Coqn-_gnqNWC59GOxmLgM1k7HYlO1HayVf5bzNMqaOs-bFztu7geJqlYJ5NcCFeqje3ev2imk479btw (Untitled)
  Session 4: V9pnex-gR-kOF--avO0vL4KqKqtDXPgIo-f47IsI1SL7RYDmSnQcSfPjIf1G4CZlv17xmYvsfE0wSzyhmmlYwQ (Untitled)
  Session 5: E4-BhRR90TkWbmBAsMzY96ozEcDu26Mp-ubST8m6lBcBPpLkIkKRkBp7_LWRZsrG8lhEtjOLQs9gk-j5TTtMDw (Untitled)
  Session 6: BqjVpTOe2YDFBTUp5LYCj-asQ8Kq6eWYAG8VauI2-32YV4PygmpRpubrtxvfNNK0wzN5htcVRhyYPeiMKKGDIA (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: 91uaMt4240W1NtUlzNUkyApahglzbw7dFlWvOpMJx4bbktJ1WqEzymwg9cysNxY5mA62YGb1dxh6jDgNUh6gfA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: AL7ZYbDsuTZJuezbQapJLT8QQx3vp036BV54DWCpuwgckEKJVWk2ywe2SjjRIEV89w8rMMoVrrBoxqLrENln-Q

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, the available product is ekoDB. It is a high-performance database product priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['created_at', 'id', 'content', 'role', 'token_usage', 'chat_id', 'updated_at'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: I'm sorry, but the context provided does not include information on the price of ekoDB. It would be best to check the official website or contact the sellers for the most accurate and up-to-date pricing information.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: 6lnBwbGqJSHmbnN8MDHbIbRkskrSs5JZrochXNhE0YblFk955S1Lp0Ym7M8J1yppEpK5VFc1LcnBLODJD_9ePw
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
🚀 ekoDB Saved Functions Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: jTF7pyKf_nBaT0gp9WLh8iY6Fd5nXmrIjVNAwg0hSOj-Q1Ayu5DaUF4t-h853_TXkLq-btUI6A4z3rSRlT9XJg
📊 Found 20 active users

📝 Example 2: Parameterized Function

✅ Function saved: JcyCMyTe3QieSPha7R2J0qN7-uN3RI-0i7n8aDUmg11P1EKOvq95AlYkp0K-pVogDlY-J_M2VNNdaE9uSss3kw
📊 Found 20 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: -gNSziraBj6p7-E3Ju0b1RoVPseL1sjDulIHD1h84-y88aA11pRwQ01r02zLGb-TckcLB7IXyxWdpFP8ndMk1g
📊 Statistics: 2 groups
   {'avg_score': 50.0, 'count': 10, 'status': 'inactive'}

   {'avg_score': 60.0, 'count': 10, 'status': 'active'}

📝 Example 4: Function Management

📋 Total functions: 5
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
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
Inserted: map[id:1XPYS-AIRlSGwaJ2y-_0yNWbDSRC6ki6e-wooV4cJuRJyVYoFPTjL45s2KfGEXf7QKfFbXwkev9rzKHcTD4pRA]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:1XPYS-AIRlSGwaJ2y-_0yNWbDSRC6ki6e-wooV4cJuRJyVYoFPTjL45s2KfGEXf7QKfFbXwkev9rzKHcTD4pRA name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:1XPYS-AIRlSGwaJ2y-_0yNWbDSRC6ki6e-wooV4cJuRJyVYoFPTjL45s2KfGEXf7QKfFbXwkev9rzKHcTD4pRA name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.go completed successfully

=== Running simple_websocket.go ===
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: pqBwEL7A2CiVvNs-XM43W6OUPcm0MKJgbyPWJuh0EGrxQgcE35QKx0Dba5TNPhy3nrFk1pDR9AcQotmoYV-B0w

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
        "id": "pqBwEL7A2CiVvNs-XM43W6OUPcm0MKJgbyPWJuh0EGrxQgcE35QKx0Dba5TNPhy3nrFk1pDR9AcQotmoYV-B0w",
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
        "id": "UBNtJCjhlVriAbaDRg0xZ26HeOtKUwX-dlpUqRGL5z11UKSJp-Pkc5Z__XaEWqxdXMapBKLOeA9ZiwG2hisiFw",
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
        "id": "b3qKJBsYGoEqxYexL1BWzxzSF326PmhRdXS93j5DqzjBhJCGsP0LBUPPqvQKPBASWK66AXL464rSMI7vSZ8vRA",
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
Batch insert response: map[failed:[] successful:[w5rdV7eeDmIX9ne06gFTCfNihtnAc0wz-UPX8jZauHNYB3uqkMsnBa5S9YKoEuk4PfTBTM36BCJkSKSuiaLxTQ ZCy2qBsBiqTnE-Yv2zbp_PSaFEnMRvDRsAXzBfVTmPXIMWM8g_Lf-3gGVu1TWHfbStl9eHdFnkellhd6ir6c1g QXUhOOGRyptQQTti-9a-G86JUW0XkHSlT9VMFf_eRqH1-5Q8UHLaOh9IxCISrlzPSzrdHrcFYQuGn8g_zl0u2Q _S31Rt8QuPsZhRAnL60KxXzFtpLEyZvwsPjblYnTwqlVsMwTBO0OMWGkktZhfdpHanMwHVO5Ux7-6G8pZygc2Q hQeukLdNz_qMz_BM7XTBKJE9uWgiCDzTT9Wr2XXm2R583dSyKRItrVLjTt-CVRzXLATVATMMTmpWORxGtzILDg]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[lCN3JcvMwx_MwpNrAGBXRYOsZCcT_zKJlvBVrhuzrMp1puefzeb06CUQbDY388zyOAC5ATmWfSU6qTa6XfESDg jAIUEiUS2QVb_CvdyihBJFF2VaMCq-qWwXvX4aAmOLEqR796we58Kyf5vc221WtYoL0eQMWCAtDBdYzEZfhrYQ GLPoycIc2Ujct-cJLZkP-A4hLEH1ip_Oiqmfi0AiwY0NOT8Knr3-i3rJGs7nm4GMzmQDV-qM0Q1pDlZ4-fRswg]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[lCN3JcvMwx_MwpNrAGBXRYOsZCcT_zKJlvBVrhuzrMp1puefzeb06CUQbDY388zyOAC5ATmWfSU6qTa6XfESDg jAIUEiUS2QVb_CvdyihBJFF2VaMCq-qWwXvX4aAmOLEqR796we58Kyf5vc221WtYoL0eQMWCAtDBdYzEZfhrYQ GLPoycIc2Ujct-cJLZkP-A4hLEH1ip_Oiqmfi0AiwY0NOT8Knr3-i3rJGs7nm4GMzmQDV-qM0Q1pDlZ4-fRswg]]
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
Collection created with first record: n_DC5aWaNp5X7olI3wQrYLBHMxhJuv9Uy_0TVMiIypIIFqQ_4adcILIZDLWTo4WT8B_3D7eX8sHSnod8aNBVdw

=== List Collections ===
Total collections: 14
Sample collections: [demo_collection schema_users_client_py schema_documents_client_py batch_users users schema_products_client_py ws_ttl_test ttl_cache schema_employees_client_py websocket_test chat_configurations__ek0_bench test_collection chat_messages__ek0_bench functions__ek0_bench]

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
✓ Inserted document: jFMcV7jf2W2awN8ATg5sefqtidKZvHLYYt49NW66UwnqBfhYu6VuyYPefAfZLjjb562jm8Sb3TTWVRw93Zg9Kw

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
✓ Inserted document with TTL: cl1zOFysoxiajSwQoRNyvfs5Y8yL01OIIVXUTg93rhIMeV61REkXTkrakrl0WnPmdbweolecmTPZnU8PPtQ9nw

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
Inserted: map[id:qrm687p-mkEE8FzCbRD7qY3o4FcZmr9lZZ_b1wDnNVOxnOhwoBolJSW78__v1YzkEbTCaEI9pQ2FwfT938DenQ]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:qrm687p-mkEE8FzCbRD7qY3o4FcZmr9lZZ_b1wDnNVOxnOhwoBolJSW78__v1YzkEbTCaEI9pQ2FwfT938DenQ name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:qrm687p-mkEE8FzCbRD7qY3o4FcZmr9lZZ_b1wDnNVOxnOhwoBolJSW78__v1YzkEbTCaEI9pQ2FwfT938DenQ name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

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
Collection created with first record: 5nA9Sns89QPglzYpT9kBBj_GKJdYlIZkTJyMxCYjRhKuLnh0RzVcTAiir9SMR8ZhPkT8dtwtusU8F8IaQLCmtA

=== List Collections ===
Total collections: 14
Sample collections: [schema_users_client_py schema_documents_client_py batch_users users schema_products_client_py]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: ba1eP0Q7LV1naGVaarHde7ipKMkzk1MChgFlDz6C749cdf9JZrp3jX59guBRX0t0pIAuwewesAmcnWe8-81mQQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: JnWXF4gZQp4G6n0nchODILbGaRyan-ifZf28oMrF-2WNXkI8OoXcIry3xCtAnIdq49hY_UmUdKaUcJrZO0oKvw

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
✓ Inserted test record: lUbEt6eCi59dBRneqVeWg-h8-LWFA-hOu0Vna6UaY_DVlLzRrulCXg88HhT56643pqi96Req9RNvBxj8ocWKhA

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
✓ Inserted document with TTL: _odrmjZpoNL-zHNaequ_D32Pg0Ip_bEPiBlCcYfRDBkjR87LEPM-FGxtogLEW3tl8BS7TGsg0eeaBAHZY5fIAw

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
✓ Created session: us_n0VnlsoxqSC6UEuFFyLx-6Fxry5ktFNQBw3e1peR1AopWUhz9_tlK5qXGA8SCRF3fnNoyb_jwHynRlC3J5g

=== Sending Chat Message ===
Message ID: DoVZgZ5lZI2XqSgy6c1qkVY7A57RESXe26_pjebxHzqZ1-90tuUsFCXB61mYw3f1EqxV6aFyUEbPe5lDs2CbBg

=== AI Response ===
The available products are:

1. ekoDB, a high-performance database product with AI capabilities, priced at $99.
2. ekoDB Pro, the enterprise edition product with advanced features, priced at $299.
3. ekoDB Cloud, a fully managed cloud database service product, priced at $499.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:map[type:String value:A high-performance database product with AI capabilities] id:RAuvHUwgrJyTbdmGqruXmU495bI0UQVyt2Tm6QjPm4a4k2Av50GhBkqs6Eicd2-w5ynGjk-CEH_SI0HDCYQHOA name:map[type:String value:ekoDB] price:map[type:Integer value:99]] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:map[type:String value:Enterprise edition product with advanced features] id:LIejv1L92I4P3KXMkpOBCq2q37qLRJ1V4fVctwrgFgoIyvt-nl4HUWK2ce7Y-goGS41bPcn98y8KdmPPR5NgaQ name:map[type:String value:ekoDB Pro] price:map[type:Integer value:299]] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:map[type:String value:Fully managed cloud database service product] id:ixfOpKgqI7WguC0Vw2yKd-jbKmPW2tYu0zkpHQkYuP0w6IWPdb0WTnj9l_u84CBf8dtLXZ67RX-DoG8trQEhYg name:map[type:String value:ekoDB Cloud] price:map[type:Integer value:499]] score:0.1111111111111111]

Execution Time: 2033ms

=== Token Usage ===
Prompt tokens: 384
Completion tokens: 65
Total tokens: 449

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 9SnPq_SF2tV-60161FF5L1rSo88Zb5Uiq_DT-jLAu8GQR0EwWulJTytCXqRJv9IF7TSxYF9AeCMWfFW2QSCWuQ

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the context provided, the available product is a high-performance database product named ekoDB, priced at $99. Please note that the availability may vary.

✓ Message 2 sent
  Response: As an assistant, I'm sorry but I can't provide the information you're asking for because the context doesn't mention any specific product. Could you please provide more details?

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 7NrwLrrQtPJXYEOJD64_ANWh0BMyoC1KpmyCDUW63nY5dTYz9VW7te5bS-95KuaM-sbXEvHjkc3TFu3nUhkPFw
  Parent: 9SnPq_SF2tV-60161FF5L1rSo88Zb5Uiq_DT-jLAu8GQR0EwWulJTytCXqRJv9IF7TSxYF9AeCMWfFW2QSCWuQ

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: 7NrwLrrQtPJXYEOJD64_ANWh0BMyoC1KpmyCDUW63nY5dTYz9VW7te5bS-95KuaM-sbXEvHjkc3TFu3nUhkPFw (Untitled)
  Session 2: 9SnPq_SF2tV-60161FF5L1rSo88Zb5Uiq_DT-jLAu8GQR0EwWulJTytCXqRJv9IF7TSxYF9AeCMWfFW2QSCWuQ (Untitled)
  Session 3: us_n0VnlsoxqSC6UEuFFyLx-6Fxry5ktFNQBw3e1peR1AopWUhz9_tlK5qXGA8SCRF3fnNoyb_jwHynRlC3J5g (Untitled)
  Session 4: 6lnBwbGqJSHmbnN8MDHbIbRkskrSs5JZrochXNhE0YblFk955S1Lp0Ym7M8J1yppEpK5VFc1LcnBLODJD_9ePw (Untitled)
  Session 5: KlR3nSGO8VdrtMOT8OJUboVMkLthxz7BZwBbW-bwVTT8XG_hFF3E-jcpF6BByX4Vtlu07DUCL3tS_R2A68rOmA (Untitled)
  Session 6: o6ROh93Coqn-_gnqNWC59GOxmLgM1k7HYlO1HayVf5bzNMqaOs-bFztu7geJqlYJ5NcCFeqje3ev2imk479btw (Untitled)
  Session 7: V9pnex-gR-kOF--avO0vL4KqKqtDXPgIo-f47IsI1SL7RYDmSnQcSfPjIf1G4CZlv17xmYvsfE0wSzyhmmlYwQ (Untitled)
  Session 8: E4-BhRR90TkWbmBAsMzY96ozEcDu26Mp-ubST8m6lBcBPpLkIkKRkBp7_LWRZsrG8lhEtjOLQs9gk-j5TTtMDw (Untitled)
  Session 9: BqjVpTOe2YDFBTUp5LYCj-asQ8Kq6eWYAG8VauI2-32YV4PygmpRpubrtxvfNNK0wzN5htcVRhyYPeiMKKGDIA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 7NrwLrrQtPJXYEOJD64_ANWh0BMyoC1KpmyCDUW63nY5dTYz9VW7te5bS-95KuaM-sbXEvHjkc3TFu3nUhkPFw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: P7jaOXriwl8ijaI8_FQuaiXDFvMxE0qSfnNcD8VDDqTuqxtnB8XhhwqwPipsX5Ws2KkXrhXPSONqWe-bpKn7lw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, the available product is a high-performance database product named ekoDB. It is priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: As a product assistant, I'm sorry but I can't provide the price for ekoDB as the context doesn't include any information about the pricing of this product. I would recommend checking the official website or contacting the sales team for the most accurate and up-to-date pricing.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: NZuo9e9uJzy4hUmAPsDkFdXQ_9eCTIEVYkVTyDd2DEsfDfHTToq4Stw_zMWum2CtK4nR1kuBNmz2s9YyguZbjw
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
  2. Score: 0.750
  3. Score: 0.748

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.699
  2. Score: 1.500
  3. Score: 0.308

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
✓ Client created (token exchange happens automatically)
📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: WHqpR2oZyn64SkRGtdOmTyTFUxL8aa3H7Gzw4xpKS-AU_lgasy-E-IHpGEXbW6dAw8GSkJ5ftZSuCFbL7JrOWg
📊 Found 30 active users

📝 Example 2: Parameterized Function

✅ Function saved: gKQUqOAJNSnpC_uYrV-eXDmnkCQtXG5xzqChzYz5HL61p2SpePdKIcGfvABP6hFzJXLjTRWwO0NzaBnJ9r7idA
📊 Found 30 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: sNjCEgB2ryjv9QLQAuJ3fyftZU-q4i3MtUdkGZOTeK_jZHhBRiiyok1xLH68wvEQMZ9xqJV72C99pMU0TcBqeA
📊 Statistics: 2 groups
   map[avg_score:60 count:15 status:active]
   map[avg_score:50 count:15 status:inactive]

📝 Example 4: Function Management

📋 Total functions: 7
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
✅ Go client examples complete!
✅ All Go integration tests complete!
📦 Building TypeScript client library...

> @ekodb/ekodb-client@0.3.0 prepare
> npm run build


> @ekodb/ekodb-client@0.3.0 build
> tsc


up to date, audited 7 packages in 2s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.3.0 build
> tsc

✅ TypeScript client built!
🧪 Running TypeScript client library examples...

added 1 package, removed 1 package, and audited 13 packages in 920ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'fS5nEGCTKFtvmBFstHaC0qPEihPYajZ-4W2qRskmVcAWjdx5uDkei_9ka5OCb1xGEeCwiW0EWclnc19Y9oYsnA'
}

=== Find by ID ===
Found: {
  name: { value: 'Test Record', type: 'String' },
  active: { type: 'Boolean', value: true },
  id: 'fS5nEGCTKFtvmBFstHaC0qPEihPYajZ-4W2qRskmVcAWjdx5uDkei_9ka5OCb1xGEeCwiW0EWclnc19Y9oYsnA',
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'fS5nEGCTKFtvmBFstHaC0qPEihPYajZ-4W2qRskmVcAWjdx5uDkei_9ka5OCb1xGEeCwiW0EWclnc19Y9oYsnA',
  value: { value: 100, type: 'Integer' },
  name: { type: 'String', value: 'Updated Record' },
  active: { type: 'Boolean', value: true }
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
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: Atjiy7PC4GwR99UdjicSNLC1jrHrN4zfVpuECdsDcGq411-hZGNHHwFh26m9zG9m_PjPMhAkDbaPJtRHxk8cBA

=== List Collections ===
Total collections: 18
Sample collections: schema_users_client_py,schema_documents_client_py,batch_users,users,schema_products_client_py

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 8EjxRA90OyI4UW0ywqBqhAPtuF3joPYBxSx_M0mr38lYTA9bBhTs2-lOQ_u63Pfhgx0qrTNYwnS8H1cNPrvt6A

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: HIxKaApndpKtK4gPBQKYHmtarH5f2Aw3cv-24UB-Cykmn101KxkD81djmJy9m0GzZoemfmGicr9ZyZ3S2u7SQg

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
✓ Inserted test record: FM9D-5kY6Gx7gDkorZPQGVXhzb92f6H-Ss5UtisZcEdy6gOnDO_lWG_K8_tPWXrUqAWotxtkoTw_l2nykz4uNw

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
✓ Inserted document with TTL: s9NcmIG_J86I-eNtTygqcSKbJhDy3Mg4J3TDAR5-lR5pSuj8Mh7IR_KP5t4B8dS53_sCT2ENNySnhb1r-JoxMQ

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
✓ Created session: DxsUY4AzSrx56VM2oOZ-0IHX09n12Vd4fMiGTtV9TMcHDA_mqq5alCM5J2Be1zTsx76o4mpblEkUXH5ihzqsuA

=== Sending Chat Message ===
Message ID: ipAoi_ORs8z4eR0pRlJ27ST28vKunKMtTpFlrAyZP0Qy7OmsetjKnWff9PjK_pCcneiAeymHCQo8wJxdRvlXkw

=== AI Response ===
The available products and their prices are as follows:

1. ekoDB Pro: This is an Enterprise edition product with advanced features. It is priced at $299.
2. ekoDB Cloud: This is a fully managed cloud database service product and its price is $499.
3. ekoDB: This is a high-performance database product with AI capabilities. You can get it for $99.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'sIu_Ti6QIt3yTLlbvHDsRs-x91bwuiJSpUFdEiGY_G4TAXJoyDFvjMY-rISzGKd9_4gzA2488YJ10udQPL9LWg',
    price: { type: 'Integer', value: 299 },
    name: { type: 'String', value: 'ekoDB Pro' },
    description: {
      type: 'String',
      value: 'Enterprise edition product with advanced features'
    }
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'hRWEuXAtUJ8xACt4E8_bOyzT-xnePq-cTCxZ-_rhNgvi-EXwOeCrClf7nfbmhFhc0iU71aKNbaeBab8jXnnv-Q',
    name: { value: 'ekoDB Cloud', type: 'String' },
    description: {
      value: 'Fully managed cloud database service product',
      type: 'String'
    },
    price: { value: 499, type: 'Integer' }
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    id: '7jO8YLQbYzTiceVGKC-xSROSNXiXd2Zm3BhgEcdfc4LIiipHFsJ2N5SuBx80vJWnGQYuQf0VL3Yg2Va61m8MbA',
    description: {
      type: 'String',
      value: 'A high-performance database product with AI capabilities'
    },
    name: { type: 'String', value: 'ekoDB' },
    price: { type: 'Integer', value: 99 }
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3586ms

=== Token Usage ===
Prompt tokens: 376
Completion tokens: 85
Total tokens: 461

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: cbYZre4NNHAaYa5kBvDP8NhKuIkafUh_IgtTDh6-v0u6TlSH4vAI9H7PDGsqm1CF78Z6dKhQgfcgL0VF02wuPg

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the provided context, the available product is a high-performance database product named ekoDB, priced at $99.

✓ Message 2 sent
  Response: I'm sorry, but there's no information provided about the product's price in the context given. Could you please specify which product you're interested in?

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: _wK9X45kgn7ob2pfCa1qnsyhRhUgrW5hW-zLk8OZbKFaZITTVE4mCAvUeB5AJS2-r5dsF2-wCnev0i53NjfgaQ
  Parent: cbYZre4NNHAaYa5kBvDP8NhKuIkafUh_IgtTDh6-v0u6TlSH4vAI9H7PDGsqm1CF78Z6dKhQgfcgL0VF02wuPg

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: _wK9X45kgn7ob2pfCa1qnsyhRhUgrW5hW-zLk8OZbKFaZITTVE4mCAvUeB5AJS2-r5dsF2-wCnev0i53NjfgaQ (Untitled)
  Session 2: cbYZre4NNHAaYa5kBvDP8NhKuIkafUh_IgtTDh6-v0u6TlSH4vAI9H7PDGsqm1CF78Z6dKhQgfcgL0VF02wuPg (Untitled)
  Session 3: DxsUY4AzSrx56VM2oOZ-0IHX09n12Vd4fMiGTtV9TMcHDA_mqq5alCM5J2Be1zTsx76o4mpblEkUXH5ihzqsuA (Untitled)
  Session 4: NZuo9e9uJzy4hUmAPsDkFdXQ_9eCTIEVYkVTyDd2DEsfDfHTToq4Stw_zMWum2CtK4nR1kuBNmz2s9YyguZbjw (Untitled)
  Session 5: 9SnPq_SF2tV-60161FF5L1rSo88Zb5Uiq_DT-jLAu8GQR0EwWulJTytCXqRJv9IF7TSxYF9AeCMWfFW2QSCWuQ (Untitled)
  Session 6: us_n0VnlsoxqSC6UEuFFyLx-6Fxry5ktFNQBw3e1peR1AopWUhz9_tlK5qXGA8SCRF3fnNoyb_jwHynRlC3J5g (Untitled)
  Session 7: 6lnBwbGqJSHmbnN8MDHbIbRkskrSs5JZrochXNhE0YblFk955S1Lp0Ym7M8J1yppEpK5VFc1LcnBLODJD_9ePw (Untitled)
  Session 8: KlR3nSGO8VdrtMOT8OJUboVMkLthxz7BZwBbW-bwVTT8XG_hFF3E-jcpF6BByX4Vtlu07DUCL3tS_R2A68rOmA (Untitled)
  Session 9: o6ROh93Coqn-_gnqNWC59GOxmLgM1k7HYlO1HayVf5bzNMqaOs-bFztu7geJqlYJ5NcCFeqje3ev2imk479btw (Untitled)
  Session 10: V9pnex-gR-kOF--avO0vL4KqKqtDXPgIo-f47IsI1SL7RYDmSnQcSfPjIf1G4CZlv17xmYvsfE0wSzyhmmlYwQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: _wK9X45kgn7ob2pfCa1qnsyhRhUgrW5hW-zLk8OZbKFaZITTVE4mCAvUeB5AJS2-r5dsF2-wCnev0i53NjfgaQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: F_C1qSGI_Lb5PITuNvT8bojBktK5xXtuoNqyJAExkei0PDxc3FSajYBODc5bVElBD4gmHTZDFGQzl6KDpFyoqQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, the product that is available is ekoDB. This is a high-performance database product priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: I'm sorry, but the context provided does not include information on the price of ekoDB. I recommend checking the product's official website or contacting its customer service for the most accurate information.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: m5nlza3UxYpTNJLd94Me_7w_YyYkxLFWog0SKwS2-e8C1V3AxRVbDqLEvAryy4dhy5nuys41AN3aMnH-K-lanA
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
  1. Score: 25.740, Matched: email, name, name.value, email.value
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, title, title.value, bio.value
  2. Score: 26.400, Matched: title, bio.value, bio, title.value
  3. Score: 26.400, Matched: title.value, bio, bio.value, title
  4. Score: 26.400, Matched: bio.value, title, title.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title, title.value, bio
  2. Score: 39.600, Matched: title, title.value, bio, bio.value
  3. Score: 39.600, Matched: title.value, bio, bio.value, title
  4. Score: 39.600, Matched: bio.value, title, title.value, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.752, Matched:
  2. Score: 0.746, Matched:
  3. Score: 0.734, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.694, Matched: content, title, content.value, title.value
  2. Score: 1.501, Matched: title.value, content, content.value, title
  3. Score: 0.298, Matched:

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
🚀 ekoDB Saved Functions Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: TYn4JmLp9kVYR_Hco5OwmU5Vm21wGAd4cfUwMyjbcsW8oa0vzxHXg0cq-9cutfYxGB8XQZ33OduVsBLg3Zpv_Q
📊 Found 40 active users

📝 Example 2: Parameterized Function

✅ Function saved: ChofXO6DNFc9AzeXTCkZuZo30QNe2GJMH_yhEiaSs8dcQT74FnCSZ0Z8IrMHMSeTp83pXKKHkGqein6EGY15Vw
📊 Found 40 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: amiHq5iD5aF9eNzRnk2B1AyBVl8KASr3Lt8lMPGVLo7akdOuNuRdcZPUeLXKyQIeg3pNjZDyqQ7ZzlkmquMEww
📊 Statistics: 2 groups
   {"count":20,"status":"inactive","avg_score":50}
   {"status":"active","count":20,"avg_score":60}

📝 Example 4: Function Management

📋 Total functions: 9
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
✅ TypeScript client examples complete!
✅ All TypeScript integration tests complete!
🧪 Running JavaScript examples (direct HTTP/WebSocket)...

added 1 package, removed 1 package, and audited 9 packages in 699ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities

╔════════════════════════════════════════╗
║  ekoDB JavaScript Examples Test Suite ║
╚════════════════════════════════════════╝

=== Checking Server Connection ===
(node:92768) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Server is ready

=== Getting Authentication Token ===
✓ Authentication successful

=== Running 7 Examples ===

=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js ===
✓ Authentication successful

=== Insert Document ===
(node:92775) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Inserted: {
  id: 'NVgooFSlhc5N-RC2oD807as3c2Jeb-mvOwY0ssFegeifgixC8vxm6yjh37FBKKGtxqj7i2725qP5gFskPr46nA'
}

=== Find by ID ===
Found: {
  value: { type: 'Integer', value: 42 },
  name: { value: 'Test Record', type: 'String' },
  active: { value: true, type: 'Boolean' },
  id: 'NVgooFSlhc5N-RC2oD807as3c2Jeb-mvOwY0ssFegeifgixC8vxm6yjh37FBKKGtxqj7i2725qP5gFskPr46nA'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  id: 'NVgooFSlhc5N-RC2oD807as3c2Jeb-mvOwY0ssFegeifgixC8vxm6yjh37FBKKGtxqj7i2725qP5gFskPr46nA',
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 100 }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.js completed successfully

=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js ===
✓ Authentication successful

=== Inserting Test Data ===
(node:92786) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted test record: EtxkM7LkhqeqBv62ZCX31loTTt3g2xJezS1oErZCKeuFSjQUtBQ8y1yTLis0xpfIBSV6kRHxnrndMEsFS8FivQ

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
        "id": "pqBwEL7A2CiVvNs-XM43W6OUPcm0MKJgbyPWJuh0EGrxQgcE35QKx0Dba5TNPhy3nrFk1pDR9AcQotmoYV-B0w",
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
        "id": "UBNtJCjhlVriAbaDRg0xZ26HeOtKUwX-dlpUqRGL5z11UKSJp-Pkc5Z__XaEWqxdXMapBKLOeA9ZiwG2hisiFw",
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
        "id": "b3qKJBsYGoEqxYexL1BWzxzSF326PmhRdXS93j5DqzjBhJCGsP0LBUPPqvQKPBASWK66AXL464rSMI7vSZ8vRA",
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
        "id": "EtxkM7LkhqeqBv62ZCX31loTTt3g2xJezS1oErZCKeuFSjQUtBQ8y1yTLis0xpfIBSV6kRHxnrndMEsFS8FivQ",
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
(node:92789) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
(node:92801) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {
  userId: 123,
  username: 'john_doe',
  loginTime: '2025-11-08T05:37:44.233Z'
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
✓ kv_operations.js completed successfully

=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/collection_management.js ===
✓ Authentication successful

=== Create Collection (via insert) ===
(node:92803) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Collection created with first record: woA7bZuIbTwkKkQ85aSJlm0tYDeIyQ0nmJ6b0oCC1BuPpgXovbFuJ_NmiEQWeZ8joH3XLfwWO-X8fCwJID8GpA

=== List Collections ===
Total collections: 22
Sample collections: [
  'schema_products_client_ts',
  'demo_collection',
  'schema_users_client_py',
  'schema_documents_client_py',
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
(node:92815) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document: e6MNbb_J55UV5QXD0cMoGyupVnpCilSEEBdVCRb9wsB0M3KJcgHeMXkFHY_hwR5KCIwPSyB5_-F5qPq7sdw15g

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
(node:92817) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document with TTL: 2wHnk7L7G7pMpZrdpjUTTsS6D5qMv39ciXWQBFyG46TR-ZxjmtJaL5cAyYPaXWnKz4_iknfMQTPV3DBNzy87HQ

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

added 1 package, removed 1 package, and audited 9 packages in 817ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'f826jzOrpWXoFT0LpprduFBs4S6gw4-YSp_Gx2oCTn_AbUfJHHkz8hz7ENZ-bH6ongyZKOkMyda8BMnePoM86A'
}

=== Find by ID ===
Found: {
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 42 },
  name: { type: 'String', value: 'Test Record' },
  id: 'f826jzOrpWXoFT0LpprduFBs4S6gw4-YSp_Gx2oCTn_AbUfJHHkz8hz7ENZ-bH6ongyZKOkMyda8BMnePoM86A'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 100 },
  name: { value: 'Updated Record', type: 'String' },
  id: 'f826jzOrpWXoFT0LpprduFBs4S6gw4-YSp_Gx2oCTn_AbUfJHHkz8hz7ENZ-bH6ongyZKOkMyda8BMnePoM86A'
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
Collection created with first record: Eg2uS7-mzaK-Cy-CDtML-tV-vrr0nOt2KAML6_vXeFEsm_yUb_IIMt0KkKk-adAJDe62q6pjShs460tL9_N1WA

=== List Collections ===
Total collections: 22
Sample collections: schema_products_client_ts,schema_users_client_py,schema_documents_client_py,batch_users,users

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: TBdWChCaszWgrqgrsmgdJpxCU48o8OHmUdY1Ie91HcLuL6XZseRTyZKY6sTG0hXh_PG0SIPo0sUp94Fnr9-Tkg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Vg0nRM4j6RYZnifR0hQC_3g8zrPNa-pWX5xqNJHPel7oCjSKr8O5XO4AWWwKC1N01J7UbCePNzYhwpkzU4_szw

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
✓ Inserted test record: _o3jm5LlfbmuNHhF9HtgrYc3QCc1YwhsARDjtAzfUrcNjFLPIs38-YY5ll6xKN1KyuldZgaYu-KPXxISdv4c-A

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
✓ Inserted document with TTL: rjEH3G1a5w8U5fL1I2sGRUamK0X_ETsQuXfJw5XBZVD3knz8XLzK2-cMNepszTB5VYo1E_LBWFZQ86tlu6HS_Q

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
🚀 ekoDB Saved Functions Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: 9pIGrnFBR9DlahjMnjs4RhKzZlrzgtp2bc4FNVqV99mWjRrdN_n6O4OtzmWl29szvBjArY-GdN_qqn5esQnGbw
📊 Found 50 active users

📝 Example 2: Parameterized Function

✅ Function saved: DMSQV757_ZtPQ8ewIYI8vCEvgzQ822URh_tJsAzLkaB7g4HvhdapyHWUZpWH650qSsBorq2kEVilk1oiOGJt9w
📊 Found 50 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: AWAiEehuLr62Hmu4SnOUqd25j_BNPnp7DsSH1pEq-Vcmk_sxKFfgHQrqWl0MyQug4zXfAmA9n9J4AJe4KP42pg
📊 Statistics: 2 groups
   {"status":"inactive","count":25,"avg_score":50}
   {"count":25,"avg_score":60,"status":"active"}

📝 Example 4: Function Management

📋 Total functions: 11
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
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
✓ Inserted user: Record(fields={id=StringValue(value=jlO_T0HknIHtpkNWHK81C7g5lkSX5E3QZRuntscW539_GgrYsH_rKMeC-0vvHBrACif9lNAAV8xL5XWxe11zqA)})
  User ID: jlO_T0HknIHtpkNWHK81C7g5lkSX5E3QZRuntscW539_GgrYsH_rKMeC-0vvHBrACif9lNAAV8xL5XWxe11zqA

=== Read ===
✓ Found user by ID: Record(fields={name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), id=StringValue(value=jlO_T0HknIHtpkNWHK81C7g5lkSX5E3QZRuntscW539_GgrYsH_rKMeC-0vvHBrACif9lNAAV8xL5XWxe11zqA), age=ObjectValue(value={value=IntegerValue(value=28), type=StringValue(value=Integer)})})

=== Update ===
✓ Updated user: Record(fields={name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), id=StringValue(value=jlO_T0HknIHtpkNWHK81C7g5lkSX5E3QZRuntscW539_GgrYsH_rKMeC-0vvHBrACif9lNAAV8xL5XWxe11zqA), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)})})

=== Query ===
✓ Found 1 users matching query
  - Record(fields={active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), id=StringValue(value=jlO_T0HknIHtpkNWHK81C7g5lkSX5E3QZRuntscW539_GgrYsH_rKMeC-0vvHBrACif9lNAAV8xL5XWxe11zqA), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)})})

=== Delete ===
✓ Deleted user with ID: jlO_T0HknIHtpkNWHK81C7g5lkSX5E3QZRuntscW539_GgrYsH_rKMeC-0vvHBrACif9lNAAV8xL5XWxe11zqA

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
  IDs: E6k_cou6SSsfG-XNU5XquE8b9bh_xPORQM4LXkxV_3E8eHGhWt44uZGBd4GZ-mVUaONUObrZmJaVrLpxQmEnLQ, nbhYEQQgikkblkeDH8t6Gcd4f_jnXEli3NrfRtk7AgXChTTrC0jGjTnFNJQ4D8T3xig3dLTeKXzy89P0OyXaEA, 6WQZxoDuHsKK7X1xVaFOiIverEnrP9i2oiKA_ShZiUkiKyLWRmje0mgSE3KO9VkqMvGkal0OEoiKSxOyE8gcDg...

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
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=95)})
  - Score: ObjectValue(value={value=IntegerValue(value=92), type=StringValue(value=Integer)})
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
✓ Session value: {"user_id":"123","created_at":1762580295631}
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

BUILD SUCCESSFUL in 4s
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
  - schema_products_client_ts
  - schema_users_client_py
  - schema_documents_client_py
  - batch_users
  - users

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-11-08T05:38:20.267100Z","last_modified":"2025-11-08T05:38:20.267101Z","bypass_ripple":false,"primary_key_alias":"id"}

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
  Document ID: R3fG4lBp4Y7BiL0yAJ8MaKS4_WMeES04ZhpEGjALRxQ2YBMsL057waFdV1KC0_mU0Yp_RxIsOdaN_mGctS3Kvw

=== Verify Document Exists ===
✓ Document found: [user_id, ttl_duration, created_at, id, session_id]

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: JIJhlihch_Xk9dmm18uYmRQFiu7JHjrooQG2DkzUJ1YZRDIRE61q9mf498YU7OfpqT-RgayOY-SH-mbJQVzgDQ

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: [ttl_duration, id, cache_key, value]

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
  {"payload":{"data":[{"id":"95XWIIlfYj0Mw01GLaM21aGzPH9J3MJOUubw_E8mWcEUbV6EeUg2j_2lnfhZR1Jtvctb15aOlNVdbYDmtM_WeA","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

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
✓ Inserted user 1: StringValue(value=V832vj5aeODwMLAFCxmSoyM9QNEUUsTz-A-XwKUQ9-A295iH6chNcWq_2rNTnbwT9IdAbYfu5qS8249h20wEFw)
✓ Inserted user 2: StringValue(value=gTkJ38aVDbweqkJPhOEO1ZQHrG-gpOQO7-EQxdHaRVf0IxiV7U-lrSO-QN2opwIuPYXmeoZInMb7N1ShLPAG_Q)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 22
  Sample: [kotlin_schema_example, schema_products_client_ts, schema_users_client_py, schema_documents_client_py, batch_users]

=== Cleanup ===
✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

BUILD SUCCESSFUL in 4s
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
  {"results":[{"record":{"views":68,"description":"Learn Rust programming language with hands-on examples and best practices.","id":"iFy3PNYdCyHqswgWkcd-8nB_EFtZtbQcdxI4Cq9unVZXwZEhFlMvyqpy7FLDsR3J5Ua3VFc7o9srhiTm9qmA2Q","title":"Rust Programming","tags":"programming,rust,tutorial"},"score":19.8,"matched_fields":["title","tags","description"]},{"record":{"tags":"programming,python,data-science","title":"Python for Data Science","views":36,"id":"X06GXLLV3LjWZKEexOh3Wd2VEkEiRqyiYAwkLTUscG8jh3Dy7NBk-7ulY4uh4wq5BwBiwWz6LPW6hAfYoApI6w","description":"Master Python for data analysis, machine learning, and visualization."},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"views":139,"description":"Build modern web applications using JavaScript, React, and Node.js.","title":"JavaScript Web Development","id":"TnP8cI6aKVW1FwYLS90oUMBHYRutH7rgnnFOtcu0UPF8wkF_2rQVWGaBFje-vd39G66VOm6ROUWLod6TrrvydA","tags":"programming,javascript,web"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":2}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"title":"Machine Learning Basics","description":"Introduction to machine learning algorithms and neural networks.","views":38,"id":"AC0Nl2LoDw017xOcRpzHMn0gGSM8ZbQQclWNUJvrohTzigiB8qFApNaCAYXxNpasr-ShAQLKi6A-0MS6ydbkTg","tags":"ai,machine-learning,python"},"score":2.7,"matched_fields":["description","title","tags"]},{"record":{"id":"X06GXLLV3LjWZKEexOh3Wd2VEkEiRqyiYAwkLTUscG8jh3Dy7NBk-7ulY4uh4wq5BwBiwWz6LPW6hAfYoApI6w","title":"Python for Data Science","views":36,"tags":"programming,python,data-science","description":"Master Python for data analysis, machine learning, and visualization."},"score":1.0,"matched_fields":["description"]},{"record":{"description":"Learn database design principles, normalization, and query optimization.","views":433,"tags":"database,design,sql","title":"Database Design","id":"W9zR6JwbC06uxRZP92bKTtZNtC2hrJrGVHQmM8q-WB3W3obPVsP83pMNH6j0-epAG_-GlifJfkS1ieiduN4sPQ"},"score":0.5,"matched_fields":["description"]},{"record":{"description":"Learn Rust programming language with hands-on examples and best practices.","views":68,"title":"Rust Programming","id":"iFy3PNYdCyHqswgWkcd-8nB_EFtZtbQcdxI4Cq9unVZXwZEhFlMvyqpy7FLDsR3J5Ua3VFc7o9srhiTm9qmA2Q","tags":"programming,rust,tutorial"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

=== Cleanup ===
✓ Deleted collection: kotlin_search_example

✓ All search operations completed successfully

BUILD SUCCESSFUL in 4s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - WebSocket TTL Example ===

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: StringValue(value=usyBlTjFQ9LBOkki5C9uRQs8FCFGafYdihLApxMCSXFBYnyuvF_YJQnDadSbM5b62nfEOeGSB3s2_opvXIhibw)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1762580322171},"id":"usyBlTjFQ9LBOkki5C9uRQs8FCFGafYdihLApxMCSXFBYnyuvF_YJQnDadSbM5b62nfEOeGSB3s2_opvXIhibw","name":{"type":"String","value":"WebSocket TTL Test"},"ttl_duration":{"type":"String","value":"1h"},"value":{"type":"Integer","value":42}}]},"type":"Success"}

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

BUILD SUCCESSFUL in 4s
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
✓ Created session: wTBKV9H7h3O1sVAfiC1pTNBJr4ffkrZmTPmJHNhV7dNKCn5PtXoDRdTTLsS3X4CJB3BG41sm1Cc5ifCJbkUbgg

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "qlfSCUq0t1BqaWCViLJrNyFEvHM9d0u2r9hk2zd0cN2kPzoVYh4VNyuiODoaPRSzNN6paKJsLWTma9YEpKyu0w"
  Responses: ["ekoDB is a high-performance database that comes with intelligent caching, real-time capabilities, and AI integration. Some of its notable features include a unique AI Chat Integration which permits you to query your database using natural language, and in return, you get AI-powered responses with relevant context. Moreover, ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."]

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
✓ Created session: 6t-ar8JvxHnUI-8WjHX4Wgcu1EVc5P5KPT4KGw_3VyMKxJtYqfXFvDKZePf7INx6uHj7s7FDLItrtn1MONn1CQ

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["Based on the information provided, the available product is ekoDB, a high-performance database product with AI capabilities. The price for this product is 99."]

✓ Message 2 sent
  Responses: ["I'm sorry, but there's no context provided about a specific product. Could you please specify which product's price you're inquiring about?"]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"6t-ar8JvxHnUI-8WjHX4Wgcu1EVc5P5KPT4KGw_3VyMKxJtYqfXFvDKZePf7INx6uHj7s7FDLItrtn1MONn1CQ"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"type":"Object","value":{"collection":{"type":"String","value":"kotlin_chat_sessions_example"},"matched_fields":{"type":"Array","value":[{"type":"String","value":"description"}]},"record":{"type":"Object","value":{"description":{"type":"String","value":"A high-performance database product with AI capabilities"},"id":{"type":"String","value":"k3GImnvuIBgq0Sm1-fnbGs22WxrELXirrxe4l0kwm4fvVA3KOJ3IHXJ6rZsa_bQfLFbrsJPNRH77HuXAEbFYXA"},"price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}}},"score":{"type":"Float","value":0.25}}}]},"created_at":{"type":"DateTime","value":"2025-11-08T05:39:00.398062Z"},"id":"GCHuWY4WlMjqfYkb8o4REQIumbZ4mV5ZeBQjWwV3iP40cvA-upYo6IqU5XnGxfCMsJaiJV-JYtOLc0Zt4FDD4w","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":32},"prompt_tokens":{"type":"Integer","value":162},"total_tokens":{"type":"Integer","value":194}}},"updated_at":{"type":"DateTime","value":"2025-11-08T05:39:00.398062Z"}},{"chat_id":{"type":"String","value":"6t-ar8JvxHnUI-8WjHX4Wgcu1EVc5P5KPT4KGw_3VyMKxJtYqfXFvDKZePf7INx6uHj7s7FDLItrtn1MONn1CQ"},"content":{"type":"String","value":"Based on the information provided, the available product is ekoDB, a high-performance database product with AI capabilities. The price for this product is 99."},"context_snippets":{"type":"Array","value":[{"type":"Object","value":{"collection":{"type":"String","value":"kotlin_chat_sessions_example"},"matched_fields":{"type":"Array","value":[{"type":"String","value":"description"}]},"record":{"type":"Object","value":{"description":{"type":"String","value":"A high-performance database product with AI capabilities"},"id":{"type":"String","value":"k3GImnvuIBgq0Sm1-fnbGs22WxrELXirrxe4l0kwm4fvVA3KOJ3IHXJ6rZsa_bQfLFbrsJPNRH77HuXAEbFYXA"},"price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}}},"score":{"type":"Float","value":0.25}}}]},"created_at":{"type":"DateTime","value":"2025-11-08T05:39:00.402515Z"},"id":"0HwiYQ-LY14Xif0f7jCgGWPAfgApT4JrDEf-_UUDBf7v9j7HOmVstITzcnjkF1IsjFnZ_UXXkOH5VOy7UBYa1w","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":32},"prompt_tokens":{"type":"Integer","value":162},"total_tokens":{"type":"Integer","value":194}}},"updated_at":{"type":"DateTime","value":"2025-11-08T05:39:00.402515Z"}},{"chat_id":{"type":"String","value":"6t-ar8JvxHnUI-8WjHX4Wgcu1EVc5P5KPT4KGw_3VyMKxJtYqfXFvDKZePf7INx6uHj7s7FDLItrtn1MONn1CQ"},"content":{"type":"String","value":"What is the price?"},"created_at":{"type":"DateTime","value":"2025-11-08T05:39:01.743932Z"},"id":"aWXhRVTnbWmvNMBgaTWZkok8rUYulobr53p_j03gPLwN8-hsvMFGK8qB7ZYlAzAKz7iHZqe0thD8mWQTehrmsg","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":29},"prompt_tokens":{"type":"Integer","value":63},"total_tokens":{"type":"Integer","value":92}}},"updated_at":{"type":"DateTime","value":"2025-11-08T05:39:01.743932Z"}},{"chat_id":{"type":"String","value":"6t-ar8JvxHnUI-8WjHX4Wgcu1EVc5P5KPT4KGw_3VyMKxJtYqfXFvDKZePf7INx6uHj7s7FDLItrtn1MONn1CQ"},"content":{"type":"String","value":"I'm sorry, but there's no context provided about a specific product. Could you please specify which product's price you're inquiring about?"},"created_at":{"type":"DateTime","value":"2025-11-08T05:39:01.748002Z"},"id":"XLtcsH1iFynCvvG5bxttiRX7-fqJFj8Xh3e7tnZ2ARCL6GqWHP5Al4VjfMI-fnyZC3cOkLiYVM4P0LIU9buWPA","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":29},"prompt_tokens":{"type":"Integer","value":63},"total_tokens":{"type":"Integer","value":92}}},"updated_at":{"type":"DateTime","value":"2025-11-08T05:39:01.748002Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 13

=== Branching Session ===
✓ Created branched session: 5j1g7J7MnftA9yrAvvM3jhRtJQy96-9OMfR12S-ZIVd64X4bwxrixmn5LTdASY5573dBdA7baBQajE0YvKH2nA

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
✓ Created session: pESUDajoru5PofgbeDlU0qz0UXz4IZjAolccNoEMmFswrCpri5z1IdpqF29bFpWRxTNDmcG4JS6V8Sx18Wb7KQ

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the information provided, the available product is ekoDB, a high-performance database product priced at $99."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Based on the information provided, the available product is ekoDB, a high-performance database product priced at 99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: gFicrexUmgXmvLFjuTRsGNbvYn_bfybZm1_B2T9G68VLC1JLUez5VU0wSKNMmEk6u1H-FwL_awQrxYRXeAzGWw

=== Merging Sessions ===
✓ Merged sessions
  Merged session ID: null

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 9s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build

> Task :run
🚀 ekoDB Saved Functions Example (Kotlin)

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: M1JWubXxw6JQf1gi82VxGDQ40NcX6-vhz6BzlzzhlwgUmlEMHQzKsbklVkAKy99d93m3tWIkxTihK5JL7MOzmg
📊 Found 60 active users

📝 Example 2: Parameterized Function

✅ Function saved: f9uMpseLIgLvuwuWUq5gIRQKRTJoG-A-jIUqJbIh0iKvVgTEW9_7fsgdlvG6UL7941eSZcDbh51wgn5Ku8lECA
📊 Found 60 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: p8EWrFHlPOeu-Ea2FYzDQm-NKVN9xaMJq5kpbCtEpP_ssPSaeDxVr9Fmog-vzmBuJytbnXki93IPznMfO4ZZlQ
📊 Statistics: 2 groups
   {"status":"active","count":30,"avg_score":60.0}
   {"avg_score":50.0,"status":"inactive","count":30}

📝 Example 4: Function Management

📋 Total functions: 13
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

✅ All examples completed!

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
✅ Kotlin client examples complete!
✅ All Kotlin integration tests complete!
✅ All integration tests complete!
```
