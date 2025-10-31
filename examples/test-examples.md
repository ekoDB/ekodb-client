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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("31JP0rJlkVKZmBbBbNu5JhT0QbqKTwJsE9SGNouqllVYOJvP_RqWcXFHo2TbZrHzG9-Wv9dHBBv8Q5j-JZdDaA")}

=== Find by ID ===
Found: Object {"active": Object {"_field_type": String("Boolean"), "_field_value": Bool(true)}, "id": String("31JP0rJlkVKZmBbBbNu5JhT0QbqKTwJsE9SGNouqllVYOJvP_RqWcXFHo2TbZrHzG9-Wv9dHBBv8Q5j-JZdDaA"), "name": Object {"_field_type": String("String"), "_field_value": String("Test Record")}, "value": Object {"_field_type": String("Integer"), "_field_value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"_field_type": String("Boolean"), "_field_value": Bool(true)}, "id": String("31JP0rJlkVKZmBbBbNu5JhT0QbqKTwJsE9SGNouqllVYOJvP_RqWcXFHo2TbZrHzG9-Wv9dHBBv8Q5j-JZdDaA"), "name": Object {"_field_type": String("String"), "_field_value": String("Test Record")}, "value": Object {"_field_type": String("Integer"), "_field_value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"_field_type": String("Boolean"), "_field_value": Bool(true)}, "id": String("31JP0rJlkVKZmBbBbNu5JhT0QbqKTwJsE9SGNouqllVYOJvP_RqWcXFHo2TbZrHzG9-Wv9dHBBv8Q5j-JZdDaA"), "name": String("Updated Record"), "value": Number(100)}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "LMLOpQxUrO_Mf_hDHzDgWNDgveY_zKo9hXNvcTXfXYIQ5qgjHVnvPoorpd3MBRUIcE_jyUL_GSZSylt7KuaQXg"

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "payload": {
    "data": [
      {
        "active": {
          "_field_type": "Boolean",
          "_field_value": true
        },
        "id": "LMLOpQxUrO_Mf_hDHzDgWNDgveY_zKo9hXNvcTXfXYIQ5qgjHVnvPoorpd3MBRUIcE_jyUL_GSZSylt7KuaQXg",
        "name": {
          "_field_type": "String",
          "_field_value": "WebSocket Test Record"
        },
        "value": {
          "_field_type": "Integer",
          "_field_value": 42
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "KHtrDdmkbPgtBYgQgpbbLme3PShWl01D-R3Le0V4ToukmU8nFhBHPc_MZyU_efw1qycEjVPz6eq4uwagHf9SEg"

=== List Collections ===
Total collections: 4
Sample collections: ["websocket_test", "batch_users", "demo_collection", "test_collection"]

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
✓ Inserted document: "Tr7VD7ei3945_ig_ko8NZC3MUXl2nddjyDD94yv9cgNILkfxVYIO3I2V1xV0eNgyt068tBZHuyrke3vjasiL1Q"

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
✓ Inserted document with TTL: String("q3Ew_frvUQ80RIuV0a9tsru2a0I17sxVrmMWvGJstVYxUcozVBxyWMNuh2qCJDQ34VU7m1KC0hviQMybj-Kgbw")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
✅ Rust direct examples complete!
🛠️  Building client library...
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
✅ Client build complete!
🧪 Running Rust client library examples...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record { fields: {"id": String("p4ItINoOVBv2PddfaQ6teO_uTMSoQW36Sa3nTA10-a3jtm3c4_9R92uTg9w5_0wxxG9LdNYCRrztcg6Shr3W2w")} }

=== Find by ID ===
Found: Record { fields: {"active": Object({"_field_value": Boolean(true), "_field_type": String("Boolean")}), "id": String("p4ItINoOVBv2PddfaQ6teO_uTMSoQW36Sa3nTA10-a3jtm3c4_9R92uTg9w5_0wxxG9LdNYCRrztcg6Shr3W2w"), "name": Object({"_field_value": String("Test Record"), "_field_type": String("String")}), "value": Object({"_field_type": String("Integer"), "_field_value": Integer(42)})} }

=== Find with Query ===
Found documents: [Record { fields: {"active": Object({"_field_value": Boolean(true), "_field_type": String("Boolean")}), "value": Object({"_field_value": Integer(42), "_field_type": String("Integer")}), "id": String("p4ItINoOVBv2PddfaQ6teO_uTMSoQW36Sa3nTA10-a3jtm3c4_9R92uTg9w5_0wxxG9LdNYCRrztcg6Shr3W2w"), "name": Object({"_field_type": String("String"), "_field_value": String("Test Record")})} }]

=== Update Document ===
Updated: Record { fields: {"name": String("Updated Record"), "value": Integer(100), "id": String("p4ItINoOVBv2PddfaQ6teO_uTMSoQW36Sa3nTA10-a3jtm3c4_9R92uTg9w5_0wxxG9LdNYCRrztcg6Shr3W2w"), "active": Object({"_field_type": String("Boolean"), "_field_value": Boolean(true)})} }

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
Collection created with first record: "mLdM9-sjlWcifNH6FIB7vcnKICO7uE3OsMxwblWqb5ckbMGmRfeAEfuk05lczg6zX685n2IOEPSTr7M_4lg8nw"

=== List Collections ===
Total collections: 6
Sample collections: ["websocket_test", "batch_users", "ws_ttl_test", "ttl_cache", "client_collection_management_rust"]

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
✓ Inserted document: "zTyNUhCSyjAkIOXDdK1XRXyjog9nncrXMhecWrA68VOlcLr_gdU8paEgK6wCt-EUq4sPdP1Dfn0q79iqJw7_SA"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("-BDFcLocRgLT46PYRZNuBYZSD_mCSblZebnDDKYOK-wMigsP47EpUci9YRL8ctlzlWCl_ltOUQB0W8-inzubNQ"))

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
✓ Inserted test record: dwKJbF2TsVJ3MQCsLFK1xf43EKGrez6ZRJHlQceMcf-_KNRAxzeDtpVn5MsWhbZ8hZ6vqGBJb7sM33vUD3iQTA

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
✓ Inserted document with TTL: Some(String("Wwh-51OyJ1-tCOBRqBHG-lzUhkFWhrUkoKWqtDm1Tw-Xh7du-vaQv3CpeHpOAHvtYk3X1NU74ES4AR00n0jbQw"))

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
  - Some(Object({"_field_value": String("Alice"), "_field_type": String("String")}))
  - Some(Object({"_field_type": String("String"), "_field_value": String("Bob")}))
  - Some(Object({"_field_type": String("String"), "_field_value": String("David")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"_field_type": String("String"), "_field_value": String("Eve")}))
  - Some(Object({"_field_type": String("String"), "_field_value": String("David")}))
  - Some(Object({"_field_type": String("String"), "_field_value": String("Bob")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"_field_value": String("Eve"), "_field_type": String("String")}))
  - Some(Object({"_field_type": String("String"), "_field_value": String("Alice")}))
  - Some(Object({"_field_type": String("String"), "_field_value": String("Bob")}))
  - Some(Object({"_field_value": String("David"), "_field_type": String("String")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Regex Query ===
✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"_field_type": String("String"), "_field_value": String("David")}))
  - Some(Object({"_field_type": String("String"), "_field_value": String("Bob")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"_field_value": String("Alice"), "_field_type": String("String")}))
  - Some(Object({"_field_value": String("Charlie"), "_field_type": String("String")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"_field_type": String("String"), "_field_value": String("Bob")}))
  2. Some(Object({"_field_value": String("David"), "_field_type": String("String")}))
  3. Some(Object({"_field_type": String("String"), "_field_value": String("Alice")}))

=== Pagination (page 2, size 2) ===
✓ Page 2 results:
  - Some(Object({"_field_type": String("String"), "_field_value": String("Charlie")}))
  - Some(Object({"_field_type": String("String"), "_field_value": String("David")}))

=== NOT Operator ===
✓ Found 4 users NOT inactive

=== Complex Chained Query ===
✓ Found 3 users with all conditions

=== Cleanup ===
✓ Deleted collection

✓ All query builder operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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
  1. Score: 2.0000
     Title: Some(String("Machine Learning Basics"))
     Matched: ["title", "description"]
  2. Score: 1.0000
     Title: Some(String("Python for Data Science"))
     Matched: ["description"]
  3. Score: 0.5000
     Title: Some(String("Database Design"))
     Matched: ["description"]
  4. Score: 0.5000
     Title: Some(String("Rust Programming"))
     Matched: ["description"]

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
Execution time: 3ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_schema_management`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("IXu9X_vn6XcluvyWb-S_4qj-gkvp_PaiC4IzFZsgJgNhwOtvcN3sJcIgk8J23e7WAJNnvwa5mbrhHwvWONWQMA"))
✓ Inserted user 2: Some(String("FPf5N4gEXEF9gmDXohbmyAfcCwFMtYQ5wyPpz0bir2hPUq3JryuX6HoAyqM_yUwLEfKRA1pnPzVbZwPLlaiglA"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - status: String
  - age: Integer
  - title: String
    (required)
  - email: String
    (required)

=== Listing Collections ===
✓ Total collections: 6
  Sample: ["websocket_test", "schema_client_rust", "batch_users", "ws_ttl_test", "ttl_cache"]

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
✓ Created session: VgMK1tt_FLBX2jcxBNHyzwN-zPRyGrRI5_nk-YYkXade23AvDEKU1nQroduLeG6kTemcI9A7CoonuZXKVzcsrQ

=== Sending Chat Message ===
Message ID: QtBF1748edbUACsGg9QL0R0vz1rfHE-9ymUAgrEnORPbKa5vWTY6yr30F0i6XodqP2RpsLtogm32Jw_wYisLIg

=== AI Response ===
Response 1: ekoDB is a high-performance database that supports intelligent caching, real-time capabilities, and AI integration. Some of its unique features include AI Chat Integration and Search features. The AI Chat Integration allows users to query the database using natural language and get AI-powered responses with relevant context. Meanwhile, the search features of ekoDB support full-text search, vector search, and hybrid search with automatic context retrieval.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["category", "title", "content"]
  Record: Object {"category": Object {"_field_type": String("String"), "_field_value": String("features")}, "content": Object {"_field_type": String("String"), "_field_value": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context.")}, "id": String("MjAqeGgM4Oh-IE6Y6FjpIXuXI4UPPhy_qj2s1-f5jrLLTY994s4qAB2NF_k5L9UVOcRkaPx4LT5idLGcQvncrw"), "title": Object {"_field_type": String("String"), "_field_value": String("AI Chat Integration")}}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["category", "title", "content"]
  Record: Object {"category": Object {"_field_type": String("String"), "_field_value": String("features")}, "content": Object {"_field_type": String("String"), "_field_value": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval.")}, "id": String("42X9RfJP_GJ-RE5r4fEMfVbcKJWPlXo2IOzb8i82P6Txjb7q7mtPSf05gTzC452dM3C2Rfc1UUyJp5YCKT4UPg"), "title": Object {"_field_type": String("String"), "_field_value": String("Search Features")}}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": Object {"_field_type": String("String"), "_field_value": String("documentation")}, "content": Object {"_field_type": String("String"), "_field_value": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration.")}, "id": String("6cTHzNwUQhAYCgEdSMHqYzt3UN4cKeINcmEAteAulLEfkm7FrRDQZjby3tWhuvSR5MTtx21dxH8mAFcLIq9dSg"), "title": Object {"_field_type": String("String"), "_field_value": String("Introduction to ekoDB")}}

Execution Time: 4088ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: XhaUYLUXazvgzGE0TM5C_aeORM2RAR3OU5cXw6ru7xfsMx_vtgTdcYmsMAcFG-QpFLeZGtTFKT4YuiZe9O020g
=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the provided context, the available product is ekoDB. It's a high-performance database product with AI capabilities. The price of this product is $99.

✓ Message 2 sent
  Response: Apologies, but there's no context provided regarding a specific product to give the price for. Could you please provide more details or specify the product you're interested in?

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: J2Ccpyphfmks3lYylCReE0NZy83wWS68IlojLpjhyudp4z9KX2dHCPF35YWMFJOVnXfsDsBTmMMgI1neccsXow
  Parent: XhaUYLUXazvgzGE0TM5C_aeORM2RAR3OU5cXw6ru7xfsMx_vtgTdcYmsMAcFG-QpFLeZGtTFKT4YuiZe9O020g

=== Listing Sessions ===
✓ Found 3 sessions
  Session 1: J2Ccpyphfmks3lYylCReE0NZy83wWS68IlojLpjhyudp4z9KX2dHCPF35YWMFJOVnXfsDsBTmMMgI1neccsXow (Untitled)
  Session 2: XhaUYLUXazvgzGE0TM5C_aeORM2RAR3OU5cXw6ru7xfsMx_vtgTdcYmsMAcFG-QpFLeZGtTFKT4YuiZe9O020g (Untitled)
  Session 3: VgMK1tt_FLBX2jcxBNHyzwN-zPRyGrRI5_nk-YYkXade23AvDEKU1nQroduLeG6kTemcI9A7CoonuZXKVzcsrQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: J2Ccpyphfmks3lYylCReE0NZy83wWS68IlojLpjhyudp4z9KX2dHCPF35YWMFJOVnXfsDsBTmMMgI1neccsXow

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: AMvVAB7LeywIiuzxeFDKjNKNqughGlpJ9l8GOy9pOapGjzpqR7E1isH96LfIj0g0u3o-UZty16rXVn1ATb_-VA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, the available product is ekoDB. This is a high-performance database product priced at $99.

✓ Second message sent
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: As a product assistant, I regret to inform you that the context provided doesn't include information on the price of ekoDB. I recommend checking the official website or contacting the sellers directly for the most accurate and current pricing.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: rhVq3FYkqKtdt6SJCnLAJ1ewrKl496usC8BPiWmEv9xml6p2R9Vw6Zq4XgqZY_YhAhTGdOmIvUS5UqEdqIgpRg
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
Inserted: {'id': 'SVgkRaWyykF1e34OObWhA975WkkZA1T89LL4_MXfM_rDoDRYxXO_xxNdcKJgfJYS0ovcfd6FCwjseQdq9ird_Q'}

=== Find by ID ===
Found: {'value': {'_field_value': 42, '_field_type': 'Integer'}, 'active': {'_field_value': True, '_field_type': 'Boolean'}, 'name': {'_field_value': 'Test Record', '_field_type': 'String'}, 'id': 'SVgkRaWyykF1e34OObWhA975WkkZA1T89LL4_MXfM_rDoDRYxXO_xxNdcKJgfJYS0ovcfd6FCwjseQdq9ird_Q'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'active': {'_field_type': 'Boolean', '_field_value': True}, 'name': 'Updated Record', 'value': 100, 'id': 'SVgkRaWyykF1e34OObWhA975WkkZA1T89LL4_MXfM_rDoDRYxXO_xxNdcKJgfJYS0ovcfd6FCwjseQdq9ird_Q'}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.py completed successfully

=== Running simple_websocket.py ===
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: aqw-2UNVOuMQKXNly8MX-KfGQb4PPDcvcfjxdRzmxQOC8NApUkT4VOPo0RzVK7m5PTGKLZ2GvdocXN-zdg80jA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "payload": {
    "data": [
      {
        "active": {
          "_field_type": "Boolean",
          "_field_value": true
        },
        "id": "LMLOpQxUrO_Mf_hDHzDgWNDgveY_zKo9hXNvcTXfXYIQ5qgjHVnvPoorpd3MBRUIcE_jyUL_GSZSylt7KuaQXg",
        "name": {
          "_field_type": "String",
          "_field_value": "WebSocket Test Record"
        },
        "value": {
          "_field_type": "Integer",
          "_field_value": 42
        }
      },
      {
        "active": {
          "_field_type": "Boolean",
          "_field_value": true
        },
        "id": "aqw-2UNVOuMQKXNly8MX-KfGQb4PPDcvcfjxdRzmxQOC8NApUkT4VOPo0RzVK7m5PTGKLZ2GvdocXN-zdg80jA",
        "name": {
          "_field_type": "String",
          "_field_value": "WebSocket Test Record"
        },
        "value": {
          "_field_type": "Integer",
          "_field_value": 42
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
Retrieved value: {'userId': 123, 'username': 'john_doe'}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: {'price': 29.99, 'name': 'Product 1'}
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
Collection created with first record: JpKl3-UqSW4hR95EdwtJrScWyBFL40X9j51qfBJzRPtlv94Tw6rPq6osMaJk3q10o-sdKoeT8qy7opHHBZ2Ttg

=== List Collections ===
Total collections: 8
Sample collections: ['websocket_test', 'chat_configurations__ek0_bench', 'batch_users', 'ws_ttl_test', 'chat_messages__ek0_bench']

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
✓ Inserted document: F1drXXBaGAjQjvgsIfDaQTx-4OgiJLQ3NjCplQeVzGWpoCI_wCwxx6KraZ2mwY9ceIIzSU_ivN6BF2lBXmyung

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
✓ Inserted document with TTL: On10D8ZaZIBdbVpWSwSHpraVlzq2tdcdAHHMBtyIueZGNDoxWZEQu8D22CAAY288kP0zxoLmoHl1pcPtSEfssA

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
Inserted: {'id': 'odHmgHKu2DaX0yHxIxeeIWplcaXklAfa86MryIBewCCdSwztdBNrMtlZGqInXGVYQNNNA2CCdCbXou9Z1S58fg'}

=== Find by ID ===
Found: {'value': {'_field_value': 42, '_field_type': 'Integer'}, 'name': {'_field_value': 'Test Record', '_field_type': 'String'}, 'id': 'odHmgHKu2DaX0yHxIxeeIWplcaXklAfa86MryIBewCCdSwztdBNrMtlZGqInXGVYQNNNA2CCdCbXou9Z1S58fg', 'active': {'_field_value': True, '_field_type': 'Boolean'}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'id': 'odHmgHKu2DaX0yHxIxeeIWplcaXklAfa86MryIBewCCdSwztdBNrMtlZGqInXGVYQNNNA2CCdCbXou9Z1S58fg', 'active': {'_field_type': 'Boolean', '_field_value': True}, 'name': 'Updated Record', 'value': 100}

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
Collection created with first record: "bVeBXVxHMBG6MN316_LfMwLJzhT3zzecVdFfa69gjYW6_UR5KwOflUIPh4U0_oX6eaW8HrzZPPw_Ndovkw_xqA"

=== List Collections ===
Total collections: 8
Sample collections: ['websocket_test', 'chat_configurations__ek0_bench', 'batch_users', 'ws_ttl_test', 'chat_messages__ek0_bench']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: cjuV0eKY7ZWFgO1ohSr1bWLsYYVprgXi1zQHZAmLvLdWBJhAuCMPQOOEnra963RrvWWt5c_z-qsTZKM_saLtWw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 9LwTQrfvWZR5xhdS8k_wjpF0wPjzZ9SNJtCxAhSqH6erO2kQyvpYdva84RddJZ7JSbBwHP3-qGI5uQhARIzEvg

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
✓ Inserted test record: sjBzgwBb4bHtFJlvL315RJTHTEldh0tvwA5L8LqPwxfm27deGZYgx-st3yHnmA4EDrEIFmrP3-20koEuGg1jJA

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
✓ Inserted document with TTL: 6C2dCjDR3KcANNVNNsknkJWBkpGbZI5vfNso20BVeurRKI1SQz4C3ru3e3Sccp3hH-L9mqeomc46RXH_ruu-1Q

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
  1. Score: 12.870, Matched: name, email
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: title, bio
  2. Score: 13.200, Matched: title, bio
  3. Score: 13.200, Matched: bio, title
  4. Score: 13.200, Matched: title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: bio, title
  2. Score: 26.400, Matched: bio, title
  3. Score: 26.400, Matched: bio, title
  4. Score: 26.400, Matched: bio, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

6. Vector search (semantic search):
Found 0 semantically similar documents

7. Hybrid search (text + vector):
Found 2 results using hybrid search (text + vector)
  1. Score: 0.600, Matched: content, title
  2. Score: 0.300, Matched: content, title

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 6.600, Matched: title

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
✓ Created session: aAivvLlnEs6E_TkQdyq7v7loxOTRuqLOWwsVCs9FtaTNS_eFKKByXBL3eOpCK6g8TCSMxN-KDZVl17QCN6fKkA

=== Sending Chat Message ===
Message ID: CVsOQAxeRUcUQQM-2AU5kHI7AsxpQYtlbY3SGNaE7gIpoOSn1UOt4OKHOrw02zLE2oql6R2ynKlWIb8wq1Li5g

=== AI Response ===
There are three products available:

1. ekoDB Pro: This is an enterprise edition product with advanced features. The price for ekoDB Pro is $299.

2. ekoDB: This is a high-performance database product with AI capabilities. The price for ekoDB is $99.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price for ekoDB Cloud is $499.

Execution Time: 3727ms

=== Token Usage ===
Prompt tokens: 385
Completion tokens: 89
Total tokens: 474

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: U9vo2MKpxrvoI4UxXcFJO6czHnQFU5hKQYRy3PhaZV_RR8FW09B1bLuuRXwF6FQI8bNVFJbDclSOre003Q3Byw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the context provided, the available product is a high-performance database product named ekoDB, priced at $99.

✓ Message 2 sent
  Response: I'm sorry, but I can't provide the information you're looking for because there's no specific product mentioned in the context. Could you please provide more details about the product you're interested in?

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: UyB9NNPGWAHDxx0mWtQunc5amdJWqWtASqo-rDhRUHtDviq4O-WUF_bEcaSrLWwennrPtymYaSXnlIlcO4s1Ag
  Parent: U9vo2MKpxrvoI4UxXcFJO6czHnQFU5hKQYRy3PhaZV_RR8FW09B1bLuuRXwF6FQI8bNVFJbDclSOre003Q3Byw

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: UyB9NNPGWAHDxx0mWtQunc5amdJWqWtASqo-rDhRUHtDviq4O-WUF_bEcaSrLWwennrPtymYaSXnlIlcO4s1Ag (Untitled)
  Session 2: U9vo2MKpxrvoI4UxXcFJO6czHnQFU5hKQYRy3PhaZV_RR8FW09B1bLuuRXwF6FQI8bNVFJbDclSOre003Q3Byw (Untitled)
  Session 3: aAivvLlnEs6E_TkQdyq7v7loxOTRuqLOWwsVCs9FtaTNS_eFKKByXBL3eOpCK6g8TCSMxN-KDZVl17QCN6fKkA (Untitled)
  Session 4: rhVq3FYkqKtdt6SJCnLAJ1ewrKl496usC8BPiWmEv9xml6p2R9Vw6Zq4XgqZY_YhAhTGdOmIvUS5UqEdqIgpRg (Untitled)
  Session 5: XhaUYLUXazvgzGE0TM5C_aeORM2RAR3OU5cXw6ru7xfsMx_vtgTdcYmsMAcFG-QpFLeZGtTFKT4YuiZe9O020g (Untitled)
  Session 6: VgMK1tt_FLBX2jcxBNHyzwN-zPRyGrRI5_nk-YYkXade23AvDEKU1nQroduLeG6kTemcI9A7CoonuZXKVzcsrQ (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: UyB9NNPGWAHDxx0mWtQunc5amdJWqWtASqo-rDhRUHtDviq4O-WUF_bEcaSrLWwennrPtymYaSXnlIlcO4s1Ag

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Irmr7XdAuYz4o23O8RIjet1lfGlvg6TEk1Ady_vN9H3n5I7TXfCVyZS4g4XBQvWeh9ZqI2qH8SKgKqTjsqjwxA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context given, the available product is a high-performance database product named ekoDB, priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['created_at', 'chat_id', 'content', 'role', 'updated_at', 'token_usage', 'id'])
Debug: First message role: {'_field_value': 'assistant', '_field_type': 'String'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: I'm sorry, but the context provided doesn't include information about the price of ekoDB. I would recommend checking the official website or contacting the sellers for the most accurate and up-to-date pricing information.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: E4OsS__JNGE-Bm6O1GMZpEsn6s9TqNsUhiRtuP3R6u3aX_soe4je_uyYctUN-W4jlyFr5z9L8T5i_eqwkmv_HA
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
Inserted: map[id:Gq4tMNbbUKKG4sWRnTcJRowYDzbq9Y3bGDg9l9BxgolW9ZJg4UktgzVMySDGwKbM9ZMWl8AEzrRnyK3gbNU9yA]

=== Find by ID ===
Found: map[active:map[_field_type:Boolean _field_value:true] id:Gq4tMNbbUKKG4sWRnTcJRowYDzbq9Y3bGDg9l9BxgolW9ZJg4UktgzVMySDGwKbM9ZMWl8AEzrRnyK3gbNU9yA name:map[_field_type:String _field_value:Test Record] value:map[_field_type:Integer _field_value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[_field_type:Boolean _field_value:true] id:Gq4tMNbbUKKG4sWRnTcJRowYDzbq9Y3bGDg9l9BxgolW9ZJg4UktgzVMySDGwKbM9ZMWl8AEzrRnyK3gbNU9yA name:Updated Record value:100]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.go completed successfully

=== Running simple_websocket.go ===
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: 1m381rz-VlbOvZ0Rlkyq5wDou8ZOGwXxTGY_darU12eAInzTExI7roCUKPg6M8KgImcttlzfSBS50Hf2sky5AQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "payload": {
    "data": [
      {
        "active": {
          "_field_type": "Boolean",
          "_field_value": true
        },
        "id": "LMLOpQxUrO_Mf_hDHzDgWNDgveY_zKo9hXNvcTXfXYIQ5qgjHVnvPoorpd3MBRUIcE_jyUL_GSZSylt7KuaQXg",
        "name": {
          "_field_type": "String",
          "_field_value": "WebSocket Test Record"
        },
        "value": {
          "_field_type": "Integer",
          "_field_value": 42
        }
      },
      {
        "active": {
          "_field_type": "Boolean",
          "_field_value": true
        },
        "id": "1m381rz-VlbOvZ0Rlkyq5wDou8ZOGwXxTGY_darU12eAInzTExI7roCUKPg6M8KgImcttlzfSBS50Hf2sky5AQ",
        "name": {
          "_field_type": "String",
          "_field_value": "WebSocket Test Record"
        },
        "value": {
          "_field_type": "Integer",
          "_field_value": 42
        }
      },
      {
        "active": {
          "_field_type": "Boolean",
          "_field_value": true
        },
        "id": "aqw-2UNVOuMQKXNly8MX-KfGQb4PPDcvcfjxdRzmxQOC8NApUkT4VOPo0RzVK7m5PTGKLZ2GvdocXN-zdg80jA",
        "name": {
          "_field_type": "String",
          "_field_value": "WebSocket Test Record"
        },
        "value": {
          "_field_type": "Integer",
          "_field_value": 42
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
Batch insert response: map[failed:[] successful:[SJfzLVErBnjBInpffhJn-KCWll-20kkOEbOUg0JF4-6XldRpJmP7I_7LBxDCdtHRSDVL4quWRQaWBZYq8brp5g RAvHKXavAEV_dLfSTUlpPTLk3_vsfjuYjsT2U05LCJ_h8awR3MR8dBzdZaY_pp1HJR1indm-TM7t5n16vUQECg fcPv3psoJdrTMbpouv62EAQ5Of5ofZaJGvxSnwAJg_dMfoYK724qgZJwf6nUJVHJZMM0rx4Xu6CTbd2xc-MuWA a7g_9ZDu4kSIeXhxc6EzYiblXoo5EPjCuNeugEeAwMK0rANNLqGnl-h07mkG8qfAfa2KS8l-4TP_7aElanNCug BjP-MCLGdE7BT9ct1kyUyCjTkTdmBL36uBjamxM7RpRBbKKhGIfrQ3YvrrsaFLgwizEwvqoeC-2FgE-hhsrzUQ]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[U3jxBx_cLcEcmu07v5xcc6DX_OOFihowVhf0UF9SZHtvOZtvSHraP0QVysvNm1CCfoK0tAnZNudwuoAq6AlqtA 6vfmKGAxV9awEXNqxgdwlfGg-MFKq1fOnw4M8E0b-HFuSEOlzxjY1OeHnKxeVsvKC-TolI6v5UeOe4ELO4NImg s52B-bRbjVTmhafu2RmNxQq0BfVyWtp_o80aZGV5cDiZeMzMI6n1MBxxrJwFiuBBFF9Q2h1IKJb9o2cudNSdhg]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[U3jxBx_cLcEcmu07v5xcc6DX_OOFihowVhf0UF9SZHtvOZtvSHraP0QVysvNm1CCfoK0tAnZNudwuoAq6AlqtA 6vfmKGAxV9awEXNqxgdwlfGg-MFKq1fOnw4M8E0b-HFuSEOlzxjY1OeHnKxeVsvKC-TolI6v5UeOe4ELO4NImg s52B-bRbjVTmhafu2RmNxQq0BfVyWtp_o80aZGV5cDiZeMzMI6n1MBxxrJwFiuBBFF9Q2h1IKJb9o2cudNSdhg]]
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
Collection created with first record: Ekw4DYij_u2qoyhSTDWrs6EPoOSjjFpU6mz_HQC4cf0BwQGPcmBn1Ot9VdyNM00_zCfgrZjDu0xLy3OdK18nDg

=== List Collections ===
Total collections: 12
Sample collections: [schema_products_client_py schema_employees_client_py websocket_test chat_configurations__ek0_bench batch_users ws_ttl_test chat_messages__ek0_bench schema_users_client_py ttl_cache demo_collection schema_documents_client_py test_collection]

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
✓ Inserted document: 5kfIHoLqsOZUFudktcO_bzAxtwyxuDPYvVwEcYikTJvVhZfnRKb6VfdwFTdJAE6PNfQAqqsh68gw53yy6BIr-g

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
✓ Inserted document with TTL: h9OAGVsSqwm7Z4lXihVJr83NZshLa4sPVQmp7fK1jXWIeDcpDNzH3Ihv-dfX4HUEISoEgU3jBM1cDF9jnKZ4ow

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
Inserted: map[id:o_L6ZXlq4RqUqtbiTclxkBewq_VBJGPptH6vQODSJN7ruGKtTnLxDkFI-yfzfbipewehWSzoAumJU_qkflQjLw]

=== Find by ID ===
Found: map[active:map[_field_type:Boolean _field_value:true] id:o_L6ZXlq4RqUqtbiTclxkBewq_VBJGPptH6vQODSJN7ruGKtTnLxDkFI-yfzfbipewehWSzoAumJU_qkflQjLw name:map[_field_type:String _field_value:Test Record] value:map[_field_type:Integer _field_value:42]]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[_field_type:Boolean _field_value:true] id:o_L6ZXlq4RqUqtbiTclxkBewq_VBJGPptH6vQODSJN7ruGKtTnLxDkFI-yfzfbipewehWSzoAumJU_qkflQjLw name:Updated Record value:100]

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
Collection created with first record: fpBOGi3n3YhQ7m__IIWYO3W6dKx7WzmAGujinE7852wo_LK6TFk7Pn5036CwUOahrEV0PVPyeRrp12p9EINghA

=== List Collections ===
Total collections: 12
Sample collections: [schema_products_client_py schema_employees_client_py websocket_test chat_configurations__ek0_bench batch_users]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: E902KOubojVi3fjA-sd8YmLy55q_CphPRycKLzpQLQcdVW5EMGZzXOeIvOQngoxriOj2c0lV0XX5BcUfpmg_rQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: As_iJ_KgIS6OMKr3WZ6c_tnCT3v744jaqK-F_42j1dfxrmZsN-Imjqcd_dT06w93A0o8IGbCXj3SrG2aEyjjWw

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
✓ Inserted test record: ldvqQ7jfrnxBoW-EVC8YcOPBXBQ3FQZUabHND2GzaqoIV0TxGe9rMy_gy7ZrDG5dep6ws0V1ZExua19QQaRUlQ

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
✓ Inserted document with TTL: Z9F5uiBh1GGvYWMwN4tBk8bzfjJBIN4HBwJEZ80EGuZRriYDY4DzQ1TsBzuGDpx5mCCDwFqt1OPMesYsGmekAQ

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
✓ Created session: XNW3noDe_mUhcveZQDnDC3aa085PhYwTEmyw1S09EMI9PZdnIdSW1l6iI2UQhNu_EAUK4T8bQ-jHAFGcJFej2Q

=== Sending Chat Message ===
Message ID: LdLUf4UcgLIZQC5aFF7Blx8-kcObQ1a-4BBHqY3WL0vysKtHOfydCJtsDttO3Lpd3q-z1aT1nhMBIJIZgpw86w

=== AI Response ===
Based on the provided information, here are the available products and their respective prices:

1. ekoDB Cloud: This is a fully managed cloud database service product priced at $499.
2. ekoDB: This is a high-performance database product with AI capabilities, and it costs $99.
3. ekoDB Pro: This is the enterprise edition product with advanced features, and the price is $299.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:map[_field_type:String _field_value:Fully managed cloud database service product] id:qcu1WUwRyB8YRrIKBXQrLNTSUeqWMMdrtedMQIXG_neXKbmeRC5WRUrLFvu3-mquk3axRe1sA26j7bqADhPJAg name:map[_field_type:String _field_value:ekoDB Cloud] price:map[_field_type:Integer _field_value:499]] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:map[_field_type:String _field_value:A high-performance database product with AI capabilities] id:Kieb5VS-6cZIeG1I66NwQeqx7g-3Td2p_HXNVlBlTPgwIwVHH4bUMKk_lxHodQQXMC7RQJSMIu1td-E9jr-4EQ name:map[_field_type:String _field_value:ekoDB] price:map[_field_type:Integer _field_value:99]] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:map[_field_type:String _field_value:Enterprise edition product with advanced features] id:A38pZ7Jco7dhkjisbIkv0W_XcaO9tjOCNHNBeTfusAInDMDXoLID5WS5Kw96f-_3ZHEe3gfbHZRQrPwnsD9gPg name:map[_field_type:String _field_value:ekoDB Pro] price:map[_field_type:Integer _field_value:299]] score:0.1111111111111111]

Execution Time: 3073ms

=== Token Usage ===
Prompt tokens: 381
Completion tokens: 84
Total tokens: 465

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 0ZTsw12epah1hi-NWkE4D8VN9Su8kCyZXTskdxtaDc0ZHdCfZbaHCIsrEXFJkrqmWAEQZIux6YOHV8jaum_Fdw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the provided context, the available product is a high-performance database product named ekoDB. It is priced at $99.

✓ Message 2 sent
  Response: Apologies, but there's no context provided about a specific product to provide the price information for. Could you please specify which product you're referring to?

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: UKQ5AiAmBOyVHq-u1qGMOIHGnZU3WKIfGDhnYr3W47RFybiYpxqCOAgWTkSoObASxuYyVo_zv5Kgq4a5iZqTSw
  Parent: 0ZTsw12epah1hi-NWkE4D8VN9Su8kCyZXTskdxtaDc0ZHdCfZbaHCIsrEXFJkrqmWAEQZIux6YOHV8jaum_Fdw

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: UKQ5AiAmBOyVHq-u1qGMOIHGnZU3WKIfGDhnYr3W47RFybiYpxqCOAgWTkSoObASxuYyVo_zv5Kgq4a5iZqTSw (Untitled)
  Session 2: 0ZTsw12epah1hi-NWkE4D8VN9Su8kCyZXTskdxtaDc0ZHdCfZbaHCIsrEXFJkrqmWAEQZIux6YOHV8jaum_Fdw (Untitled)
  Session 3: XNW3noDe_mUhcveZQDnDC3aa085PhYwTEmyw1S09EMI9PZdnIdSW1l6iI2UQhNu_EAUK4T8bQ-jHAFGcJFej2Q (Untitled)
  Session 4: E4OsS__JNGE-Bm6O1GMZpEsn6s9TqNsUhiRtuP3R6u3aX_soe4je_uyYctUN-W4jlyFr5z9L8T5i_eqwkmv_HA (Untitled)
  Session 5: U9vo2MKpxrvoI4UxXcFJO6czHnQFU5hKQYRy3PhaZV_RR8FW09B1bLuuRXwF6FQI8bNVFJbDclSOre003Q3Byw (Untitled)
  Session 6: aAivvLlnEs6E_TkQdyq7v7loxOTRuqLOWwsVCs9FtaTNS_eFKKByXBL3eOpCK6g8TCSMxN-KDZVl17QCN6fKkA (Untitled)
  Session 7: rhVq3FYkqKtdt6SJCnLAJ1ewrKl496usC8BPiWmEv9xml6p2R9Vw6Zq4XgqZY_YhAhTGdOmIvUS5UqEdqIgpRg (Untitled)
  Session 8: XhaUYLUXazvgzGE0TM5C_aeORM2RAR3OU5cXw6ru7xfsMx_vtgTdcYmsMAcFG-QpFLeZGtTFKT4YuiZe9O020g (Untitled)
  Session 9: VgMK1tt_FLBX2jcxBNHyzwN-zPRyGrRI5_nk-YYkXade23AvDEKU1nQroduLeG6kTemcI9A7CoonuZXKVzcsrQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: UKQ5AiAmBOyVHq-u1qGMOIHGnZU3WKIfGDhnYr3W47RFybiYpxqCOAgWTkSoObASxuYyVo_zv5Kgq4a5iZqTSw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: v0zBNx4HiZdUNXhzf1QrebFh8PlNMMGrtQb9kyZjwSu6XaIA31oYcod1WTstlAiR2a3UYK2noaVhEyWWXV3rnQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, the available product is ekoDB, a high-performance database product. It is priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: As a product assistant, I'm sorry but I can't provide the current price for ekoDB as the context does not provide information about the price of this product. I would recommend checking the latest price on the official website or contacting the seller directly for the most accurate information.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: TFoKiud61RA__nZ9FaCVFyVXEO3AG2xhxTotjeDA9fXxMXy1mL8skRXlAb2Vh-MNDVp7mSNrFelAWK-V6Fn_kA
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
  1. Score: 12.870
  2. Score: 6.270

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200
  2. Score: 13.200
  3. Score: 13.200
  4. Score: 13.200

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400
  2. Score: 26.400
  3. Score: 26.400
  4. Score: 26.400

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600
  2. Score: 6.600

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 6.600

6. Vector search (semantic search):
Found 0 semantically similar documents

7. Hybrid search (text + vector):
Found 2 results using hybrid search (text + vector)
  1. Score: 1.200
  2. Score: 0.600

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 6.600

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


up to date, audited 6 packages in 2s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.2.1 build
> tsc

✅ TypeScript client built!
🧪 Running TypeScript client library examples...

added 1 package, removed 1 package, and audited 13 packages in 655ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'WmD8CFto5hexpjHzesA1WpzGYY-RjKO3UNZnOZ6_vda97P0iQqaBntZ81rBA7ahaBitH5GgDsDGPn1cJsHXXXA'
}

=== Find by ID ===
Found: {
  name: { _field_type: 'String', _field_value: 'Test Record' },
  id: 'WmD8CFto5hexpjHzesA1WpzGYY-RjKO3UNZnOZ6_vda97P0iQqaBntZ81rBA7ahaBitH5GgDsDGPn1cJsHXXXA',
  active: { _field_type: 'Boolean', _field_value: true },
  value: { _field_type: 'Integer', _field_value: 42 }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: 100,
  id: 'WmD8CFto5hexpjHzesA1WpzGYY-RjKO3UNZnOZ6_vda97P0iQqaBntZ81rBA7ahaBitH5GgDsDGPn1cJsHXXXA',
  name: 'Updated Record',
  active: { _field_value: true, _field_type: 'Boolean' }
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
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: VTtBv_rPLBcfq5yWv3VVztvOZVElEIlSc1FOOPJ34duhKSmErFHDSfRjjivIHdScc0EqlG7ej9ol1TUXRyE4Gg

=== List Collections ===
Total collections: 16
Sample collections: schema_employees_client_go,schema_products_client_py,schema_products_client_go,schema_employees_client_py,websocket_test

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: hhitayVR9-aL7NjqbwzVzTdQdxjDINw96cqxTw6TzFihVJR5Zu_p1cNJKsHvg7iWwy2v6sFu__Y82rwzhgw1Zg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 4EU-wV4_NcMDBSbYC-Es9bNnBo0bohUecAF3hEkTQPYgal1hnU8E2HiCMNKtu1hu6YrGwfLPx3cu-6oeNvUInA

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
✓ Inserted test record: 4gz4RLYAh1hjoUzEXuJI1OqAowOsuHOhq1BVioG_l1THzc67XqS79boabES6Z8NGo1-KqGk4bTKAXcusziub6A

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
✓ Inserted document with TTL: DMhdp97ZiduxSR16AAS9vIbHQq8Be0c3xFLe3CsN1dyB6SQeHOdAVDGmJoJQ0M5xohTA0hMcRxQl3ALa1E5avQ

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
✓ Created session: -bRkfN9d9kkratfDhVNap5rCEPPO5oxYB229-NS6LtDDPSknGA6uK1OJbtKAFDyMfA-Rh-YbBj6XbUzfLo0SGg

=== Sending Chat Message ===
Message ID: 3ZUazI5uq6PyCzyiOLKpFNTYmwG3wsERsBH6MK5-T66sjvRRqEcgzOmops8IY6gfviLNL3PH8yBwpWnRfYOXDg

=== AI Response ===
The available products and their prices are as follows:

1. ekoDB Cloud: This is a fully managed cloud database service product priced at $499.
2. ekoDB Pro: This is an enterprise edition product with advanced features priced at $299.
3. ekoDB: This is a high-performance database product with AI capabilities priced at $99.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'm_Wrw8flGNWte0l1bxSTLruGGmnzoAga1_9zMsgIh9FG_Jo_0a-3A0gTQCul-D2cmR29naV_vVXcOq5QeFPHJg',
    name: { _field_type: 'String', _field_value: 'ekoDB Cloud' },
    price: { _field_type: 'Integer', _field_value: 499 },
    description: {
      _field_value: 'Fully managed cloud database service product',
      _field_type: 'String'
    }
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'DJP4rX_GXbAQGwBpt5xk_P_O9lfZMUWVjIloXnawO72dEISSiTDpoB3CuSCo28YbjJHvaFu35flMHry0Fj_WvA',
    price: { _field_type: 'Integer', _field_value: 299 },
    description: {
      _field_value: 'Enterprise edition product with advanced features',
      _field_type: 'String'
    },
    name: { _field_type: 'String', _field_value: 'ekoDB Pro' }
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'L7TQIRve4uMLbL8ZS_aYO3P8G53-OrlIHVN-Ks-B8Pkk6wZ0PzPWOCgykSbL01NoFi3Dz9klpNHLgp66tXSSHQ',
    name: { _field_value: 'ekoDB', _field_type: 'String' },
    price: { _field_type: 'Integer', _field_value: 99 },
    description: {
      _field_type: 'String',
      _field_value: 'A high-performance database product with AI capabilities'
    }
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2607ms

=== Token Usage ===
Prompt tokens: 378
Completion tokens: 73
Total tokens: 451

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Y9zA2tXB4wlywv5F5Yn8OgnV0TH6dMRKODdleTQummMTroEGy_EECRqZhTnbOepinqDrXNLEyJbjZso9m_3I5g

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the context provided, the available product is ekoDB. It is a high-performance database product priced at $99.

✓ Message 2 sent
  Response: Apologies, but there seems to be no context provided regarding a specific product. Could you please specify the product you're interested in?

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: -TnQjeyeiqs46iNbhEc71Niyv3LXrnrQ3S6ePER9EQtBMNwyBYepHbVHvqwZ78cUASRdOZ5KByHR0g3W3D1NiQ
  Parent: Y9zA2tXB4wlywv5F5Yn8OgnV0TH6dMRKODdleTQummMTroEGy_EECRqZhTnbOepinqDrXNLEyJbjZso9m_3I5g

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: -TnQjeyeiqs46iNbhEc71Niyv3LXrnrQ3S6ePER9EQtBMNwyBYepHbVHvqwZ78cUASRdOZ5KByHR0g3W3D1NiQ (Untitled)
  Session 2: Y9zA2tXB4wlywv5F5Yn8OgnV0TH6dMRKODdleTQummMTroEGy_EECRqZhTnbOepinqDrXNLEyJbjZso9m_3I5g (Untitled)
  Session 3: -bRkfN9d9kkratfDhVNap5rCEPPO5oxYB229-NS6LtDDPSknGA6uK1OJbtKAFDyMfA-Rh-YbBj6XbUzfLo0SGg (Untitled)
  Session 4: TFoKiud61RA__nZ9FaCVFyVXEO3AG2xhxTotjeDA9fXxMXy1mL8skRXlAb2Vh-MNDVp7mSNrFelAWK-V6Fn_kA (Untitled)
  Session 5: 0ZTsw12epah1hi-NWkE4D8VN9Su8kCyZXTskdxtaDc0ZHdCfZbaHCIsrEXFJkrqmWAEQZIux6YOHV8jaum_Fdw (Untitled)
  Session 6: XNW3noDe_mUhcveZQDnDC3aa085PhYwTEmyw1S09EMI9PZdnIdSW1l6iI2UQhNu_EAUK4T8bQ-jHAFGcJFej2Q (Untitled)
  Session 7: E4OsS__JNGE-Bm6O1GMZpEsn6s9TqNsUhiRtuP3R6u3aX_soe4je_uyYctUN-W4jlyFr5z9L8T5i_eqwkmv_HA (Untitled)
  Session 8: U9vo2MKpxrvoI4UxXcFJO6czHnQFU5hKQYRy3PhaZV_RR8FW09B1bLuuRXwF6FQI8bNVFJbDclSOre003Q3Byw (Untitled)
  Session 9: aAivvLlnEs6E_TkQdyq7v7loxOTRuqLOWwsVCs9FtaTNS_eFKKByXBL3eOpCK6g8TCSMxN-KDZVl17QCN6fKkA (Untitled)
  Session 10: rhVq3FYkqKtdt6SJCnLAJ1ewrKl496usC8BPiWmEv9xml6p2R9Vw6Zq4XgqZY_YhAhTGdOmIvUS5UqEdqIgpRg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: -TnQjeyeiqs46iNbhEc71Niyv3LXrnrQ3S6ePER9EQtBMNwyBYepHbVHvqwZ78cUASRdOZ5KByHR0g3W3D1NiQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: B6aY90Pw1zWC9zqcxxSD2aTkfgux63v0BRk4Mfj-7nXB9E8lNlEEZ4b-xye1G_gkFqYl9MMwqSudkVkL3h-4Lw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, the available product is ekoDB, a high-performance database product. It is priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: I'm sorry, but the context provided does not include information on the price of ekoDB. I recommend checking the product's official website or contacting its customer service for the most accurate and up-to-date price.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: f4r7XQCH9ZD1Tc4_Dtm6ZCHriW5uturIok-loK8O2zGH_uiKQmpF0BfOPWW2ZCzeKh_yiiGj9iu4lWuMNXXWCA
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
  1. Score: 12.870, Matched: name, email
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: title, bio
  2. Score: 13.200, Matched: title, bio
  3. Score: 13.200, Matched: title, bio
  4. Score: 13.200, Matched: title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: title, bio
  2. Score: 26.400, Matched: title, bio
  3. Score: 26.400, Matched: bio, title
  4. Score: 26.400, Matched: bio, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 6.600, Matched: bio

6. Vector search (semantic search):
Found 0 semantically similar documents

7. Hybrid search (text + vector):
Found 2 results using hybrid search (text + vector)
  1. Score: 1.200, Matched: content, title
  2. Score: 0.600, Matched: title, content

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 6.600, Matched: bio

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

added 1 package, removed 1 package, and audited 9 packages in 683ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities

╔════════════════════════════════════════╗
║  ekoDB JavaScript Examples Test Suite ║
╚════════════════════════════════════════╝

=== Checking Server Connection ===
(node:20563) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Server is ready

=== Getting Authentication Token ===
✓ Authentication successful

=== Running 7 Examples ===

=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js ===
✓ Authentication successful

=== Insert Document ===
(node:20565) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Inserted: {
  id: 'bLPYSSxRWi733Pg4qMGCSMnD8GzG04PcHvkxw6nFkW-vt2u5vWcm4nbTPH6BxyIjWba9JUnjRFP4-W9i8yYKxw'
}

=== Find by ID ===
Found: {
  active: { _field_type: 'Boolean', _field_value: true },
  id: 'bLPYSSxRWi733Pg4qMGCSMnD8GzG04PcHvkxw6nFkW-vt2u5vWcm4nbTPH6BxyIjWba9JUnjRFP4-W9i8yYKxw',
  value: { _field_value: 42, _field_type: 'Integer' },
  name: { _field_type: 'String', _field_value: 'Test Record' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  active: { _field_value: true, _field_type: 'Boolean' },
  id: 'bLPYSSxRWi733Pg4qMGCSMnD8GzG04PcHvkxw6nFkW-vt2u5vWcm4nbTPH6BxyIjWba9JUnjRFP4-W9i8yYKxw',
  name: 'Updated Record',
  value: 100
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.js completed successfully

=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js ===
✓ Authentication successful

=== Inserting Test Data ===
(node:20573) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted test record: Oj5CMwLm40VuP9QOb0mzzuLLPyFd8w7OMyVRHISjNqZEebOmexTdUBzoRR6ikD5rrctkXQlVCAGG1JppHkg_1A

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "payload": {
    "data": [
      {
        "active": {
          "_field_type": "Boolean",
          "_field_value": true
        },
        "id": "Oj5CMwLm40VuP9QOb0mzzuLLPyFd8w7OMyVRHISjNqZEebOmexTdUBzoRR6ikD5rrctkXQlVCAGG1JppHkg_1A",
        "name": {
          "_field_type": "String",
          "_field_value": "WebSocket Test Record"
        },
        "value": {
          "_field_type": "Integer",
          "_field_value": 42
        }
      },
      {
        "active": {
          "_field_type": "Boolean",
          "_field_value": true
        },
        "id": "1m381rz-VlbOvZ0Rlkyq5wDou8ZOGwXxTGY_darU12eAInzTExI7roCUKPg6M8KgImcttlzfSBS50Hf2sky5AQ",
        "name": {
          "_field_type": "String",
          "_field_value": "WebSocket Test Record"
        },
        "value": {
          "_field_type": "Integer",
          "_field_value": 42
        }
      },
      {
        "active": {
          "_field_type": "Boolean",
          "_field_value": true
        },
        "id": "aqw-2UNVOuMQKXNly8MX-KfGQb4PPDcvcfjxdRzmxQOC8NApUkT4VOPo0RzVK7m5PTGKLZ2GvdocXN-zdg80jA",
        "name": {
          "_field_type": "String",
          "_field_value": "WebSocket Test Record"
        },
        "value": {
          "_field_type": "Integer",
          "_field_value": 42
        }
      },
      {
        "active": {
          "_field_type": "Boolean",
          "_field_value": true
        },
        "id": "LMLOpQxUrO_Mf_hDHzDgWNDgveY_zKo9hXNvcTXfXYIQ5qgjHVnvPoorpd3MBRUIcE_jyUL_GSZSylt7KuaQXg",
        "name": {
          "_field_type": "String",
          "_field_value": "WebSocket Test Record"
        },
        "value": {
          "_field_type": "Integer",
          "_field_value": 42
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
(node:20576) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
(node:20588) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {
  loginTime: '2025-10-29T07:08:58.320Z',
  userId: 123,
  username: 'john_doe'
}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
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
(node:20591) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Collection created with first record: KxsPGhy1_vPXUIZCBZdPSRAEc7bZQyhVeJudvz-1nXoLrLgbc6MIjoS8-XKsSZPeBghvxw8TaF9cJadD-1hyNg

=== List Collections ===
Total collections: 20
Sample collections: [
  'schema_employees_client_go',
  'schema_products_client_py',
  'schema_products_client_go',
  'schema_employees_client_py',
  'schema_products_client_ts'
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
(node:20604) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document: Dsx1eWpT9gxUeVhoFw9HFn5d9IyKEYfHMBcI6WcqhYzyagIbn76c38Z24sKu-immx1l_w6P8ZjqpuQMC9DgtcQ

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
(node:20606) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document with TTL: bhqr6lOW2S7N5YZqlrT7uJXdxyIGC6nEKo8OVqOndYxgOGAbPRw8YsC9tGRr04i5Q5Le04CFgW6bRQ0qX6CmUA

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

added 1 package, removed 1 package, and audited 9 packages in 686ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'Z1nsyXS1VS3zB4I4bMWolXv-7nxpczJyGeChiOri6m4FuPAJzcB9-F9Lyq1IihV5REzwjoQNter-LgkRMOJbnA'
}

=== Find by ID ===
Found: {
  active: { _field_type: 'Boolean', _field_value: true },
  value: { _field_type: 'Integer', _field_value: 42 },
  name: { _field_type: 'String', _field_value: 'Test Record' },
  id: 'Z1nsyXS1VS3zB4I4bMWolXv-7nxpczJyGeChiOri6m4FuPAJzcB9-F9Lyq1IihV5REzwjoQNter-LgkRMOJbnA'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: 100,
  active: { _field_value: true, _field_type: 'Boolean' },
  name: 'Updated Record',
  id: 'Z1nsyXS1VS3zB4I4bMWolXv-7nxpczJyGeChiOri6m4FuPAJzcB9-F9Lyq1IihV5REzwjoQNter-LgkRMOJbnA'
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

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: lEqNvQXh2TQcyGBnRQfHrc_8pYoeKHyPdrQMfWJ4a4PA1hSCIf9p9osp-Blfhj62TKSHTPYvylEMYSocjbLiMg

=== List Collections ===
Total collections: 20
Sample collections: schema_employees_client_go,schema_products_client_py,schema_products_client_go,schema_employees_client_py,schema_products_client_ts

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: M46UN5RdqrA6fN_QIXK8lJfM-yFd_KHpyRyPVmn1OCrEudQHL1WJjJWrGkyL4AD3IIAjy6fgqwDCKRtB7tOi9g

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: fiVBZxl7PH6NuHp6yuqvu60BdpQJi42tYCmQP4nKXXJ-Rgz4315KJiuv2o-DM7S99MHDQupVr7jFSlU-gTF0QQ

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
✓ Inserted test record: Lr_eWfQPIZBqcN7tKWyTxy_5wYNgvtv2_zHcOpADMtqeXNLPU7msZw6qAjTEYo8en1zQbfxKwe2Ub2lWZ0YdzQ

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
✓ Inserted document with TTL: Uo0pFOeYas19K3Y2rpTPP7esKYfICYxWmxq_7uNyIgPYhyG1ieeN-0vHosPVRpLtHqRUoyVBHtntrKfZ2qLHpA

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
✓ Inserted user: Record(fields={id=StringValue(value=40U4DiDWlXHlHO9YTVrgBBzD7mKaaKsWArNG_8WRrvLBl_RNTd4zrFOH58nx8-ER1UOemhWyICI4pP698Box2A)})
  User ID: 40U4DiDWlXHlHO9YTVrgBBzD7mKaaKsWArNG_8WRrvLBl_RNTd4zrFOH58nx8-ER1UOemhWyICI4pP698Box2A

=== Read ===
✓ Found user by ID: Record(fields={email=ObjectValue(value={_field_type=StringValue(value=String), _field_value=StringValue(value=alice@example.com)}), age=ObjectValue(value={_field_type=StringValue(value=Integer), _field_value=IntegerValue(value=28)}), active=ObjectValue(value={_field_value=BooleanValue(value=true), _field_type=StringValue(value=Boolean)}), name=ObjectValue(value={_field_value=StringValue(value=Alice Johnson), _field_type=StringValue(value=String)}), id=StringValue(value=40U4DiDWlXHlHO9YTVrgBBzD7mKaaKsWArNG_8WRrvLBl_RNTd4zrFOH58nx8-ER1UOemhWyICI4pP698Box2A)})

=== Update ===
✓ Updated user: Record(fields={name=ObjectValue(value={_field_value=StringValue(value=Alice Johnson), _field_type=StringValue(value=String)}), email=ObjectValue(value={_field_type=StringValue(value=String), _field_value=StringValue(value=alice@example.com)}), age=IntegerValue(value=29), id=StringValue(value=40U4DiDWlXHlHO9YTVrgBBzD7mKaaKsWArNG_8WRrvLBl_RNTd4zrFOH58nx8-ER1UOemhWyICI4pP698Box2A), active=ObjectValue(value={_field_type=StringValue(value=Boolean), _field_value=BooleanValue(value=true)}), city=StringValue(value=San Francisco)})

=== Query ===
✓ Found 1 users matching query
  - Record(fields={name=ObjectValue(value={_field_value=StringValue(value=Alice Johnson), _field_type=StringValue(value=String)}), email=ObjectValue(value={_field_type=StringValue(value=String), _field_value=StringValue(value=alice@example.com)}), id=StringValue(value=40U4DiDWlXHlHO9YTVrgBBzD7mKaaKsWArNG_8WRrvLBl_RNTd4zrFOH58nx8-ER1UOemhWyICI4pP698Box2A), city=ObjectValue(value={_field_type=StringValue(value=String), _field_value=StringValue(value=San Francisco)}), active=ObjectValue(value={_field_type=StringValue(value=Boolean), _field_value=BooleanValue(value=true)}), age=ObjectValue(value={_field_type=StringValue(value=Integer), _field_value=IntegerValue(value=29)})})

=== Delete ===
✓ Deleted user with ID: 40U4DiDWlXHlHO9YTVrgBBzD7mKaaKsWArNG_8WRrvLBl_RNTd4zrFOH58nx8-ER1UOemhWyICI4pP698Box2A

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
  IDs: FS43PujgLieaRwsDtpc7JLSk7beYEvH9ikGjCbjqtvASfnRDf0TmY3em1LqdTp7ZE-NAXwwyvGaPDwv-Tr5okw, bpLEBso3UvSAQSly1yzKDoKnTJ3t321fO2us9pfSn8VewR1gG2-ZgwhcSMB3Ez8m1A7r3CUcbZR1o6ndmxuzXw, Ugd1RH-iZRY8RWR-EfiAjzLwguU9-fjiY4Vtmiv7a5Ld52R66z7GxFOS-dNaKVsRkLfOm_5xz-Gkba7aawTcqw...

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
  - Score: ObjectValue(value={_field_value=IntegerValue(value=95), _field_type=StringValue(value=Integer)})
  - Score: ObjectValue(value={_field_value=IntegerValue(value=92), _field_type=StringValue(value=Integer)})
  - Score: ObjectValue(value={_field_type=StringValue(value=Integer), _field_value=IntegerValue(value=88)})

=== Query 4: Complex (score > 80 AND age >= 25) ===
✓ Found 4 high-scoring adults

=== Query 5: IN (city IN ['NYC', 'LA']) ===
✓ Found 4 records in NYC or LA

=== Query 6: Pagination (skip 2, limit 2) ===
✓ Page 2 (2 records):
  - ObjectValue(value={_field_type=StringValue(value=String), _field_value=StringValue(value=Charlie)})
  - ObjectValue(value={_field_value=StringValue(value=Diana), _field_type=StringValue(value=String)})

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
✓ Retrieved value: {"email":"alice@example.com","name":"Alice","role":"admin"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"user_id":"123","created_at":1761721769546}
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
✓ Found 19 collections
  - schema_employees_client_go
  - schema_products_client_py
  - schema_products_client_go
  - schema_employees_client_py
  - schema_products_client_ts

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-10-29T07:09:33.914139Z","last_modified":"2025-10-29T07:09:33.914141Z","bypass_ripple":false,"primary_key_alias":"id"}

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
  Document ID: SvWgWNsfQWo46dcC73ainddAxJbWPHu_GYgmX1ArEGtbLHx7h-0k0ZxG4mLEzFKNwOn9YoNgZ_6BOkRnXlqjFw

=== Verify Document Exists ===
✓ Document found: [created_at, id, ttl_duration, user_id, session_id]

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: 5Aj1hkAuNNHXFeKXTbVrH-PVvUaNxx4dFZm-jNEJ6PtfhVjZJK2HAEDIPihyE714cqOB1vt9i__5_drdsvpsNg

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: [id, cache_key, ttl_duration, value]

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
  {"payload":{"data":[{"id":"dPDyFj0XN1Do0klNNfmd7yAXS02th_ZC3TFKFieN5KR8cCyA7C9Qs0u508mfI_f4xTq0F6_F9Ha7i1d7EIj0TQ","name":{"_field_type":"String","_field_value":"Test User"},"status":{"_field_type":"String","_field_value":"active"}}]},"type":"Success"}

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
✓ Inserted user 1: StringValue(value=rzEfLU_68ZCaKASkRtPilyxDO2xhJpnOUz-4Ks0Co8uA0_KfGHhg_pkznO2GSNdPMjluRbLM1ZBAMAPpQ9gRNA)
✓ Inserted user 2: StringValue(value=-hH7UIgyl6IvY_Td4ye3nVk99NNgjGfOgEusiPvTwZJd35chrffFrqyZ-C31h5oNxSxkEvwnIxTpRJ-rFD3yaw)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 20
  Sample: [schema_employees_client_go, schema_products_client_py, schema_products_client_go, schema_employees_client_py, schema_products_client_ts]

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
  {"results":[{"record":{"tags":"programming,rust,tutorial","title":"Rust Programming","description":"Learn Rust programming language with hands-on examples and best practices.","id":"VSdYs9JQn4WP8YgNXLEgYNjWTa3-DyLIDRboWa3hlvAbjOr6iBubdVP8yAy3_ZPJIhV6FUdn8D1SLX2HvBWXQQ","views":18},"score":19.8,"matched_fields":["title","description","tags"]},{"record":{"title":"JavaScript Web Development","views":743,"description":"Build modern web applications using JavaScript, React, and Node.js.","id":"FtqR3WBLzJHH06vNEh7MyGv0A0HRPeYPwTLY5hoI5NyYVXOVs1orp6ZQeqK5qYcGvfDuUC3P4rB9hHbdg_t0Mg","tags":"programming,javascript,web"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"id":"MY7w3jzC1ftyAHSE-9pn_kfNcTyYvef6MjEanpV3hmG-KTuab0_-4sAh8KTyLpkKy9ekSDnOFgFSUwDPdZ67nQ","title":"Python for Data Science","description":"Master Python for data analysis, machine learning, and visualization.","views":735,"tags":"programming,python,data-science"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"title":"Machine Learning Basics","description":"Introduction to machine learning algorithms and neural networks.","id":"4VsWKMmWltrzk8yIJoIj_2n9XInpHS3QuNaXrbjkl5Tox9NEFE-08F1-ot06V4A6Tl4ATTvHdLyJaYSr-1U0vg","tags":"ai,machine-learning,python","views":774},"score":2.7,"matched_fields":["title","description","tags"]},{"record":{"views":735,"title":"Python for Data Science","id":"MY7w3jzC1ftyAHSE-9pn_kfNcTyYvef6MjEanpV3hmG-KTuab0_-4sAh8KTyLpkKy9ekSDnOFgFSUwDPdZ67nQ","tags":"programming,python,data-science","description":"Master Python for data analysis, machine learning, and visualization."},"score":1.0,"matched_fields":["description"]},{"record":{"title":"Rust Programming","id":"VSdYs9JQn4WP8YgNXLEgYNjWTa3-DyLIDRboWa3hlvAbjOr6iBubdVP8yAy3_ZPJIhV6FUdn8D1SLX2HvBWXQQ","tags":"programming,rust,tutorial","views":18,"description":"Learn Rust programming language with hands-on examples and best practices."},"score":0.5,"matched_fields":["description"]},{"record":{"views":511,"id":"-ILpxRmwl5IV8Zb-Wpl31hs2wA2yuNcJtkU_N826ACzNYfO7eRnjDTce_sr-ZXOLrQ9iCbrkn3AiB4-6VL6JBQ","title":"Database Design","description":"Learn database design principles, normalization, and query optimization.","tags":"database,design,sql"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
✓ Inserted document with TTL: StringValue(value=7ajCitb8uKZJ7Cudx5aK8yILhFj9msgqSxkJywPFReBhDKmq079kFFdMcceyke4PW9V3zy4PmEizeUAaW7IcAw)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"_field_type":"Integer","_field_value":1761721795190},"id":"7ajCitb8uKZJ7Cudx5aK8yILhFj9msgqSxkJywPFReBhDKmq079kFFdMcceyke4PW9V3zy4PmEizeUAaW7IcAw","name":{"_field_type":"String","_field_value":"WebSocket TTL Test"},"ttl_duration":{"_field_type":"String","_field_value":"1h"},"value":{"_field_type":"Integer","_field_value":42}}]},"type":"Success"}

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

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 6k_1KE1M357sH0LxZZ8lyFKJKTB5eS90WVioFwvDwijHDtFgw6YgU8mrxFp9LHF-x0VcptfpwByOyi0_DPXVPQ

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "gVopSQzkmxbOpxQ1yIvReM1WhB7PlfVvYT2r87CyO92jaPZTugIgU_LdqqanhFlqwM9e5VyqIQoKmrX7_3Rpzw"
  Responses: ["ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration. It has several standout features. One of these is the AI Chat Integration, which allows you to query your database using natural language and get AI-powered responses with relevant context. ekoDB also boasts robust Search Features, supporting full-text search, vector search, and hybrid search with automatic context retrieval."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 7s
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
✓ Created session: Y7DosxguSrToXO9PxRGXqbHgO45Dmo79LbifKPXNMkch_XfiShydE41c7Eql-12oCdPIVHTjZa7_mW48EKP7Ng

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["Based on the information provided, the available product is ekoDB. It is a high-performance database product with AI capabilities. The price for this product is $99."]

✓ Message 2 sent
  Responses: ["Apologies, but there's no context provided about a specific product. Could you please specify the product you're interested in?"]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"_field_type":"String","_field_value":"Y7DosxguSrToXO9PxRGXqbHgO45Dmo79LbifKPXNMkch_XfiShydE41c7Eql-12oCdPIVHTjZa7_mW48EKP7Ng"},"content":{"_field_type":"String","_field_value":"What products are available?"},"context_snippets":{"_field_type":"Array","_field_value":[{"_field_type":"Object","_field_value":{"collection":{"_field_type":"String","_field_value":"kotlin_chat_sessions_example"},"matched_fields":{"_field_type":"Array","_field_value":[{"_field_type":"String","_field_value":"description"}]},"record":{"_field_type":"Object","_field_value":{"description":{"_field_type":"String","_field_value":"A high-performance database product with AI capabilities"},"id":{"_field_type":"String","_field_value":"TOpMc7mDYLdZfjV7OQNX8HJOcTFBNJUXiRlyY7AdtMgLl9wcQDVUPg8TbkuPMjeNTIHDfq4y3jzbEqvhUkimbA"},"price":{"_field_type":"Integer","_field_value":99},"product":{"_field_type":"String","_field_value":"ekoDB"}}},"score":{"_field_type":"Float","_field_value":0.25}}}]},"created_at":{"_field_type":"DateTime","_field_value":"2025-10-29T07:10:13.163055Z"},"id":"b3PdnLm_tme9uq6fDJV6y0Zq084LTcU3npyDYLCg7gwgGJDOQu7Jv_nLTEARKI8BNHPRcIvLL-XNAGl1-0UdTQ","role":{"_field_type":"String","_field_value":"user"},"token_usage":{"_field_type":"Object","_field_value":{"completion_tokens":{"_field_type":"Integer","_field_value":34},"prompt_tokens":{"_field_type":"Integer","_field_value":165},"total_tokens":{"_field_type":"Integer","_field_value":199}}},"updated_at":{"_field_type":"DateTime","_field_value":"2025-10-29T07:10:13.163055Z"}},{"chat_id":{"_field_type":"String","_field_value":"Y7DosxguSrToXO9PxRGXqbHgO45Dmo79LbifKPXNMkch_XfiShydE41c7Eql-12oCdPIVHTjZa7_mW48EKP7Ng"},"content":{"_field_type":"String","_field_value":"Based on the information provided, the available product is ekoDB. It is a high-performance database product with AI capabilities. The price for this product is $99."},"context_snippets":{"_field_type":"Array","_field_value":[{"_field_type":"Object","_field_value":{"collection":{"_field_type":"String","_field_value":"kotlin_chat_sessions_example"},"matched_fields":{"_field_type":"Array","_field_value":[{"_field_type":"String","_field_value":"description"}]},"record":{"_field_type":"Object","_field_value":{"description":{"_field_type":"String","_field_value":"A high-performance database product with AI capabilities"},"id":{"_field_type":"String","_field_value":"TOpMc7mDYLdZfjV7OQNX8HJOcTFBNJUXiRlyY7AdtMgLl9wcQDVUPg8TbkuPMjeNTIHDfq4y3jzbEqvhUkimbA"},"price":{"_field_type":"Integer","_field_value":99},"product":{"_field_type":"String","_field_value":"ekoDB"}}},"score":{"_field_type":"Float","_field_value":0.25}}}]},"created_at":{"_field_type":"DateTime","_field_value":"2025-10-29T07:10:13.165558Z"},"id":"_FHmMvcBZOYJAaB7_8KagOu5edEPh2JuCnLoY6sqlBF0kHwYwiTCnhx4IlyJ4rCNaOY0_aKjcVE0WhqK48EOiA","role":{"_field_type":"String","_field_value":"assistant"},"token_usage":{"_field_type":"Object","_field_value":{"completion_tokens":{"_field_type":"Integer","_field_value":34},"prompt_tokens":{"_field_type":"Integer","_field_value":165},"total_tokens":{"_field_type":"Integer","_field_value":199}}},"updated_at":{"_field_type":"DateTime","_field_value":"2025-10-29T07:10:13.165558Z"}},{"chat_id":{"_field_type":"String","_field_value":"Y7DosxguSrToXO9PxRGXqbHgO45Dmo79LbifKPXNMkch_XfiShydE41c7Eql-12oCdPIVHTjZa7_mW48EKP7Ng"},"content":{"_field_type":"String","_field_value":"What is the price?"},"created_at":{"_field_type":"DateTime","_field_value":"2025-10-29T07:10:14.698305Z"},"id":"_xfPvPcDMcL9G6q_cloeYRt2JCGQjLMnOv3tChrRphjaFkhc8_QVMmUQK2YQRgtP80rLAaQj-SQVFVwQ8MZIbw","role":{"_field_type":"String","_field_value":"user"},"token_usage":{"_field_type":"Object","_field_value":{"completion_tokens":{"_field_type":"Integer","_field_value":25},"prompt_tokens":{"_field_type":"Integer","_field_value":63},"total_tokens":{"_field_type":"Integer","_field_value":88}}},"updated_at":{"_field_type":"DateTime","_field_value":"2025-10-29T07:10:14.698305Z"}},{"chat_id":{"_field_type":"String","_field_value":"Y7DosxguSrToXO9PxRGXqbHgO45Dmo79LbifKPXNMkch_XfiShydE41c7Eql-12oCdPIVHTjZa7_mW48EKP7Ng"},"content":{"_field_type":"String","_field_value":"Apologies, but there's no context provided about a specific product. Could you please specify the product you're interested in?"},"created_at":{"_field_type":"DateTime","_field_value":"2025-10-29T07:10:14.700008Z"},"id":"2pH52SvLpJg_d_T_tNMlP5gfD3vULSpu2_k4oXMte3dbnJ1ZL_ZGzb_5czuOXjiOkvA3p14Yk0DJqWkj3w4wWA","role":{"_field_type":"String","_field_value":"assistant"},"token_usage":{"_field_type":"Object","_field_value":{"completion_tokens":{"_field_type":"Integer","_field_value":25},"prompt_tokens":{"_field_type":"Integer","_field_value":63},"total_tokens":{"_field_type":"Integer","_field_value":88}}},"updated_at":{"_field_type":"DateTime","_field_value":"2025-10-29T07:10:14.700008Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 13

=== Branching Session ===
✓ Created branched session: glxZbNe-pVrN4xuHCm0kNp2UfflVoJia_os0nfq6mNcANWm66wuVC0qZpfHWPGE_UUhlZR7W-u9v0y5za1Ccdw

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
✓ Created session: aRJrVzz6xzlWMFnJErgHQ2ymUP7ziiv6FKuFWKTwRUy-6XNRIJGFoWIeBHl6D9fKWXo62wlMN58Us3nhxPiadA

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the information provided, the available product is ekoDB. This is a high-performance database product priced at $99."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Based on the provided context, the available product is ekoDB. It is a high-performance database product priced at $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: Sdblq8b1pL2PpbrGHmO59E0yK3vRSdgugeQOdhpkrD6wELHJBaaBEVb4ctJ4pd8yYp9nR1WAikCf3STts7Iv8w

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
