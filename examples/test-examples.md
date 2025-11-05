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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("N6kP3TvQ0jXj69IETOMQL7tJmE7IHZs0v3qXN5xKhO-2FRoYQXNcDB5Z8mJRJD86Yo57y4K6eEM7Mu1jOVxAFQ")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("N6kP3TvQ0jXj69IETOMQL7tJmE7IHZs0v3qXN5xKhO-2FRoYQXNcDB5Z8mJRJD86Yo57y4K6eEM7Mu1jOVxAFQ"), "name": Object {"type": String("String"), "value": String("Test Record")}, "public_id": Object {"type": String("String"), "value": String("JN4TtnqG1ZjcaXItO7-MKP6gkHkSO0hUwjNZ_CnRkIdEQLAGkDLWbvpd9x0PpxxMro4kIN8Np4mdtNcRtSlDYAiVKepeOskJZ8ISsOXLcn6kkljZInhc5twAEJ54--M")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("N6kP3TvQ0jXj69IETOMQL7tJmE7IHZs0v3qXN5xKhO-2FRoYQXNcDB5Z8mJRJD86Yo57y4K6eEM7Mu1jOVxAFQ"), "name": Object {"type": String("String"), "value": String("Test Record")}, "public_id": Object {"type": String("String"), "value": String("JN4TtnqG1ZjcaXItO7-MKP6gkHkSO0hUwjNZ_CnRkIdEQLAGkDLWbvpd9x0PpxxMro4kIN8Np4mdtNcRtSlDYAiVKepeOskJZ8ISsOXLcn6kkljZInhc5twAEJ54--M")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("N6kP3TvQ0jXj69IETOMQL7tJmE7IHZs0v3qXN5xKhO-2FRoYQXNcDB5Z8mJRJD86Yo57y4K6eEM7Mu1jOVxAFQ"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "public_id": Object {"type": String("String"), "value": String("JN4TtnqG1ZjcaXItO7-MKP6gkHkSO0hUwjNZ_CnRkIdEQLAGkDLWbvpd9x0PpxxMro4kIN8Np4mdtNcRtSlDYAiVKepeOskJZ8ISsOXLcn6kkljZInhc5twAEJ54--M")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "L9qiURyF7o6aHhFvzVqYmH7ANMPlTiAjsJ55aOlf5OnaWDkeSjR5to-CAGti869BavYDNilknpne5ExsEXouPw"

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
        "id": "L9qiURyF7o6aHhFvzVqYmH7ANMPlTiAjsJ55aOlf5OnaWDkeSjR5to-CAGti869BavYDNilknpne5ExsEXouPw",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "public_id": {
          "type": "String",
          "value": "raFXV70UP3QvcsnmOhiapPJO--cnoukl-aRJmrZLJDAjDyGn6Cp2pUCJNACtYeip7vPYPOUuABrBD8XDZ5cw5WBRDTDnPUiYBD8pRJVpSezhUat-gXO85V-68qGyL0o"
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
Collection created with first record: "rLGtC1EAtDGOGHjym16X9xU5x_ddu3onBEL-OuwqNZY6oTxtjFMkMRrcZtw6v1-_ejz-eSWuym1Y-6vJSoQxEQ"

=== List Collections ===
Total collections: 4
Sample collections: ["test_collection", "batch_users", "demo_collection", "websocket_test"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "-uPJQunbH8CG-lLi653tPLs19vZwHAVLTYKT8LskxGHrr7U2ppx_Pbeirxw7jaH2ibJ5sTKl5_2mTaM0VVmo9Q"

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
✓ Inserted document with TTL: String("WimoTpp1kb8Rx6c3esIt_-GfJqzXAznEGsTfK1dTbMoTmZUJvsatuqn_qyYwGqEOnTks7k9FLuFfUT-rIrEAoQ")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
✅ Rust direct examples complete!
🛠️  Building client library...
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
✅ Client build complete!
🧪 Running Rust client library examples...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record { fields: {"id": String("YRz2kF6ERo5bzHcpDpIr1qqPETo4hfzde67KdoLp-LHi51h9wKMwcXIdGxk8i4JvYoS98H0kR4UGyAZ6nVZfow")} }

=== Find by ID ===
Found: Record { fields: {"public_id": Object({"type": String("String"), "value": String("i5CsY4OpLVPVOEBIMQ88PreUq0k5-qQ-o8rJyUspkZuninh--1NCFJn35csCyZgD6Mfw-ZXSwsnKJP8hqDIf0Y8-h-2eQUjc7zxAakAhcQJz55TuDXv0RONVjY7DH_U")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "id": String("YRz2kF6ERo5bzHcpDpIr1qqPETo4hfzde67KdoLp-LHi51h9wKMwcXIdGxk8i4JvYoS98H0kR4UGyAZ6nVZfow"), "value": Object({"type": String("Integer"), "value": Integer(42)}), "name": Object({"type": String("String"), "value": String("Test Record")})} }

=== Find with Query ===
Found documents: [Record { fields: {"id": String("YRz2kF6ERo5bzHcpDpIr1qqPETo4hfzde67KdoLp-LHi51h9wKMwcXIdGxk8i4JvYoS98H0kR4UGyAZ6nVZfow"), "public_id": Object({"type": String("String"), "value": String("i5CsY4OpLVPVOEBIMQ88PreUq0k5-qQ-o8rJyUspkZuninh--1NCFJn35csCyZgD6Mfw-ZXSwsnKJP8hqDIf0Y8-h-2eQUjc7zxAakAhcQJz55TuDXv0RONVjY7DH_U")}), "name": Object({"value": String("Test Record"), "type": String("String")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "value": Object({"type": String("Integer"), "value": Integer(42)})} }]

=== Update Document ===
Updated: Record { fields: {"id": String("YRz2kF6ERo5bzHcpDpIr1qqPETo4hfzde67KdoLp-LHi51h9wKMwcXIdGxk8i4JvYoS98H0kR4UGyAZ6nVZfow"), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "public_id": Object({"type": String("String"), "value": String("i5CsY4OpLVPVOEBIMQ88PreUq0k5-qQ-o8rJyUspkZuninh--1NCFJn35csCyZgD6Mfw-ZXSwsnKJP8hqDIf0Y8-h-2eQUjc7zxAakAhcQJz55TuDXv0RONVjY7DH_U")}), "name": Object({"type": String("String"), "value": String("Updated Record")}), "value": Object({"value": Integer(100), "type": String("Integer")})} }

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
Collection created with first record: "amX-10pVbpPwFUtykdAY_-bkrU5PPOecKts-h0XuKVYSfjS5kEEfA4j9-J1Ykhb4oPaXIiO87HkRE3fhuruqDw"

=== List Collections ===
Total collections: 6
Sample collections: ["test_collection", "ws_ttl_test", "batch_users", "ttl_cache", "websocket_test"]

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
✓ Inserted document: "-GJ77mdX6xideDBgnwBx3-9QarWi3O72pgWyuew92_WORHXYcEfqNkEciX64CeOPSeIdF3W67Fv8YsMgSd7ALg"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("3iOapMtIaeeqgamUP5N1QJ-L4Je2vjlMOp8fHYeIH4JouQarpuT1LY9YEMvDv1fura20wSix6reMLyR31z-uvw"))

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
✓ Inserted test record: 106G0UvXOIzFI5AS6yQyRdAQN2WWEBUzjTEe1bFaHbExTBipcnN5rrLPFTqYaB5Sh8QZ0x2t2W8RqiFwLRdahQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("wKHG5gtLi9RKjThvhO3MrWq4cmAhTmvGpLelhOVysNEXk-XzosngdOcf-u_PIFEezYqZYrQmzgNkIbub-G8wIA"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

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
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))
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
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"value": String("Bob"), "type": String("String")}))
  2. Some(Object({"type": String("String"), "value": String("David")}))
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
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
     Matched: ["description.value", "title", "description", "title.value"]
  2. Score: 2.0000
     Title: Some(String("Python for Data Science"))
     Matched: ["description", "description.value"]
  3. Score: 1.0000
     Title: Some(String("Rust Programming"))
     Matched: ["description.value", "description"]
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
  2. Score: 6.6000 - Some(String("Python for Data Science"))

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 2ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_schema_management`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("t8LWaUl-Ub6Rurud9Z8_hgiV8pB3Ow0OK0Qsxz7KXWWQ-579AiFRBFndEgBTAfyt8Fh2x76L669kg9wCAPmYvg"))
✓ Inserted user 2: Some(String("i7BChFYKEltaiL0vUJhHgg5RyP1TAOk2hDxWzSoM8K3gbDM6QqKcN86-Smj308PZ_2Q-1Y6NW5cGUCMyxtx_NQ"))

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
  Sample: ["test_collection", "schema_client_rust", "ws_ttl_test", "batch_users", "ttl_cache"]

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
✓ Created session: 9gawzN9bFlAqfh2tPOLAJjkCyDrDQJqD0o3x8flRS81ZG9xpxqK1hEkdp1ScU1xM76lWkpYEGD6AhSQOkDDS8Q

=== Sending Chat Message ===
Message ID: vuC7QrdlferJ22muYS5Noogbksa59d61duiIThspHTITkWnQ8jRUYYzlBosj1sKWZtNe788IdDPRgqWftIQIHQ

=== AI Response ===
Response 1: ekoDB is a high-performance database that encompasses a range of features to enhance its usability and performance. It has intelligent caching and real-time capabilities that allow for faster and more efficient data handling. Additionally, it integrates with AI, enabling you to query your database using natural language and receive AI-powered responses with relevant context.

Among its distinctive features, ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval. This allows for more precise search results and improves the overall user experience.

Furthermore, it has a feature that allows AI chat integration. This means you can interact with your database using a chat feature and receive context-relevant responses powered by AI. This can streamline your workflow and make data retrieval more efficient.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": Object {"type": String("String"), "value": String("features")}, "content": Object {"type": String("String"), "value": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context.")}, "id": String("0f0iIj6b8zrs7u9qYLLLB2q7-7u11BJFpGbSP4dkJN645NZ0entqscUi7tVQ9SRaWYm-kQ4psx1oXiDDAu2CfA"), "public_id": Object {"type": String("String"), "value": String("MTbRsONosdhrzXc_-xVr3GSrKiIJp_d6PxXgoZS07lLdXwhXwBxf7KrEsTEQIU-5Bn2Rmw1vEBWWAQqQmnjjIeXRVFTs1FyxB4GRauyFQeQ6hZH9RMc_d_XgS-qd7AU")}, "title": Object {"type": String("String"), "value": String("AI Chat Integration")}}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title", "category"]
  Record: Object {"category": Object {"type": String("String"), "value": String("features")}, "content": Object {"type": String("String"), "value": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval.")}, "id": String("1MkPTeRV-RLlVlse-sIKFi27LZV_QZDLRxjiUfglAqeZnYEAkYLoWYu_nv5RxRMj8VDQd3O-OHa4hKnzkzuF2g"), "public_id": Object {"type": String("String"), "value": String("aQjvT2FncJMijNuJEt7OCdSUb6WHHBCDMCVEIonRzOf2W0CtKynsafKzoEI4ucSQB-qmrVrGNuohUKcvhS8qv7rlYkDH7IsHxI5pf_lyNZdVwn34dgAXQ0tDWG7W5BY")}, "title": Object {"type": String("String"), "value": String("Search Features")}}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": Object {"type": String("String"), "value": String("documentation")}, "content": Object {"type": String("String"), "value": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration.")}, "id": String("Q4sQHu1P6yEeMc9Gh7I1xtxyInEoEkrOPWzUsmC76bUEReqwuen6fnuqGG-RBBy-MpzifYWVZDcFJ3I0CBYo0w"), "public_id": Object {"type": String("String"), "value": String("7noW_0Ud9Ta7lmYYK7kM8OxmM5qiWvmIPSX46xbsEzjmhZeMFgRcINIigjZdrK6Xv_R7THdT5u2oiAkIDBK5teOHFYr8HugQbd1ZbIwmuyBGJUIP-rkyZ4A0DLD1RsI")}, "title": Object {"type": String("String"), "value": String("Introduction to ekoDB")}}

Execution Time: 4988ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: IDMSkI944tBKr49r145D8XDJtTSoecyXHvJImUzRznTb_Igdh91jEFhW21wNiPfdKqhQKKqDZd4zdNODMF36ig
=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is a high-performance database product with AI capabilities called ekoDB. It is priced at $99.

✓ Message 2 sent
  Response: As an assistant, I'm sorry, but I can't provide the price you're asking for because there's no specific product mentioned in the context. Could you please provide more details about the product you're interested in?

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: xPYDY3WWoWL_tO5KkP-ljE31LgeQ4fRZyZNxL-c5zfOeIrTnVl5-SmcVFk_kBgyICiqGtG-WWBs-IfoqI70e_w
  Parent: IDMSkI944tBKr49r145D8XDJtTSoecyXHvJImUzRznTb_Igdh91jEFhW21wNiPfdKqhQKKqDZd4zdNODMF36ig

=== Listing Sessions ===
✓ Found 3 sessions
  Session 1: xPYDY3WWoWL_tO5KkP-ljE31LgeQ4fRZyZNxL-c5zfOeIrTnVl5-SmcVFk_kBgyICiqGtG-WWBs-IfoqI70e_w (Untitled)
  Session 2: IDMSkI944tBKr49r145D8XDJtTSoecyXHvJImUzRznTb_Igdh91jEFhW21wNiPfdKqhQKKqDZd4zdNODMF36ig (Untitled)
  Session 3: 9gawzN9bFlAqfh2tPOLAJjkCyDrDQJqD0o3x8flRS81ZG9xpxqK1hEkdp1ScU1xM76lWkpYEGD6AhSQOkDDS8Q (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: xPYDY3WWoWL_tO5KkP-ljE31LgeQ4fRZyZNxL-c5zfOeIrTnVl5-SmcVFk_kBgyICiqGtG-WWBs-IfoqI70e_w

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 1BYL-29reRWUo-vqRsIYpoH-bpACUo55PJOQBl_jEXNjanDHKIeTDS8FhuKo-nYEg5-uVGNloKd6ZbUBqjG33Q

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the provided context, the available product is a high-performance database product named ekoDB. It is priced at $99.

✓ Second message sent
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: I'm sorry, but the context provided doesn't include information about the price of ekoDB. I would recommend checking the official website or contacting the sales department of ekoDB for the most accurate and up-to-date pricing information.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: olIA8RwiIJqNaSvm2bo1p83W4hwHhoMqZCTv4fm5QJ-nblThsgoJzym5BfT4D0zmixus1mvrLgSoz2SqWDAyoQ
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
Inserted: {'id': 'Xo44RHyaeI6eKjGfMZ8IvTAmxLGFFciqlW4ClNn5kmyrn9VZ04gtarthSuLVi2aJzxUsS-9Gg6HiP7O1iysYnw'}

=== Find by ID ===
Found: {'public_id': {'type': 'String', 'value': '-9eONn3dzlV7P01DHTkb7N_o-2n7faHbAivK1ukILM2FZdoQS8B4m_it_4ATT-0si-uOI4KDgyEmKrh8H2eIi7vPtnaJEoh9ogXbfcpL_VcM_eDIYwbpJ7X_aTt0fvo'}, 'name': {'type': 'String', 'value': 'Test Record'}, 'id': 'Xo44RHyaeI6eKjGfMZ8IvTAmxLGFFciqlW4ClNn5kmyrn9VZ04gtarthSuLVi2aJzxUsS-9Gg6HiP7O1iysYnw', 'active': {'value': True, 'type': 'Boolean'}, 'value': {'value': 42, 'type': 'Integer'}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'active': {'value': True, 'type': 'Boolean'}, 'value': {'type': 'Integer', 'value': 100}, 'id': 'Xo44RHyaeI6eKjGfMZ8IvTAmxLGFFciqlW4ClNn5kmyrn9VZ04gtarthSuLVi2aJzxUsS-9Gg6HiP7O1iysYnw', 'public_id': {'type': 'String', 'value': '-9eONn3dzlV7P01DHTkb7N_o-2n7faHbAivK1ukILM2FZdoQS8B4m_it_4ATT-0si-uOI4KDgyEmKrh8H2eIi7vPtnaJEoh9ogXbfcpL_VcM_eDIYwbpJ7X_aTt0fvo'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.py completed successfully

=== Running simple_websocket.py ===
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: e0eDbHkjGjG0bUhI65gY5jUq-9xGN7nDhPD-Kh9_JxyegZb0gyeDDnGXNtMKMz8xQqhWNDveLXI3H5vAl6oeAw

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
        "id": "L9qiURyF7o6aHhFvzVqYmH7ANMPlTiAjsJ55aOlf5OnaWDkeSjR5to-CAGti869BavYDNilknpne5ExsEXouPw",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "public_id": {
          "type": "String",
          "value": "raFXV70UP3QvcsnmOhiapPJO--cnoukl-aRJmrZLJDAjDyGn6Cp2pUCJNACtYeip7vPYPOUuABrBD8XDZ5cw5WBRDTDnPUiYBD8pRJVpSezhUat-gXO85V-68qGyL0o"
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
        "id": "e0eDbHkjGjG0bUhI65gY5jUq-9xGN7nDhPD-Kh9_JxyegZb0gyeDDnGXNtMKMz8xQqhWNDveLXI3H5vAl6oeAw",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "public_id": {
          "type": "String",
          "value": "DEJbxxUnRUs5zj_A8fm5ItVtPE8dIFnancuWED0qHEn_bqM4N6G0QN3k8CLVnwWtfftPiIxQOUW2Hi8bUPPx-PYLavw32FNcZytG9cl7WOyYDMjwBsOZ2aCyeAbXb2E"
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
✓ kv_operations.py completed successfully

=== Running collection_management.py ===
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: lgvntvHDGSNJPmS-vUUIDGNwmusBrq7pQxcIxhmJAwLvjNJVHsvFVLkPyOOMfaxqfPW8TQwI1AmqlaDYLoslEw

=== List Collections ===
Total collections: 8
Sample collections: ['test_collection', 'chat_configurations__ek0_bench', 'ws_ttl_test', 'batch_users', 'demo_collection']

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
✓ Inserted document: sxyrRF5naT68Nwks_mrP2AF7sCy2ctVD-OWj2PO44t0x3sR4gwM-Cqyo-QP2-hEm47yzx9J3Z9nrqtlD8ifn6A

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
✓ Inserted document with TTL: stF54bg8hgdtr7N-g85J6IXS5MKWKZeOCZvMCjgyR-NHvhM4TIlgedmed11qOjz03VxixHaRkS-i3pglQ2tRzA

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
    Finished `release` profile [optimized] target(s) in 0.17s
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
Inserted: {'id': 'WG2MKZXvlRokXUP4s6V9B3XWqY0YdTyQ4LULJ3Zy6YYFmVLIAoYMVFq1jG55dps7Wfec3b0lMv7XhQ3EdQXWdg'}

=== Find by ID ===
Found: {'public_id': {'type': 'String', 'value': '2Sgjb0a3IHaeMyKu9njlFSsSR7QrK4-FSc0UmqEggPLgGLmDix9YA6Kjf9zWGGE6ImhYARcXNX3F-4nUwau_UvnUoDBHIxVx_XXAQa6iJ1bqNlhBpjxv3q7UWswae0c'}, 'id': 'WG2MKZXvlRokXUP4s6V9B3XWqY0YdTyQ4LULJ3Zy6YYFmVLIAoYMVFq1jG55dps7Wfec3b0lMv7XhQ3EdQXWdg', 'value': {'value': 42, 'type': 'Integer'}, 'active': {'value': True, 'type': 'Boolean'}, 'name': {'value': 'Test Record', 'type': 'String'}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'value': {'type': 'Integer', 'value': 100}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'id': 'WG2MKZXvlRokXUP4s6V9B3XWqY0YdTyQ4LULJ3Zy6YYFmVLIAoYMVFq1jG55dps7Wfec3b0lMv7XhQ3EdQXWdg', 'active': {'type': 'Boolean', 'value': True}, 'public_id': {'value': '2Sgjb0a3IHaeMyKu9njlFSsSR7QrK4-FSc0UmqEggPLgGLmDix9YA6Kjf9zWGGE6ImhYARcXNX3F-4nUwau_UvnUoDBHIxVx_XXAQa6iJ1bqNlhBpjxv3q7UWswae0c', 'type': 'String'}}

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
Collection created with first record: "ExhUjtnlnj7oq0SX491WIlRl6XhR5C50tG1e6Z1_j1FP7N0JWO5lMHWjxzjcUdOEpPvATWisGX3ArxC7UQ6j0Q"

=== List Collections ===
Total collections: 8
Sample collections: ['test_collection', 'chat_configurations__ek0_bench', 'ws_ttl_test', 'batch_users', 'chat_messages__ek0_bench']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: -Ad08dWzy4oXU581soHl1SuqiwmwK_dDWgkcS82tQFIA3GJ3RgKEn93sqCldTiV2pp5BRdZvc2SCBzgZDpsD7w

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 0dYv6jbRlsZwisyS7sWMjAPUAXyKbomygyGFoQ0nMTwBWhT2IvHMsfXjirgpC4IJor5sA_p0yeLgMMJnKxvflg

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
✓ Inserted test record: tBXRQjdVAbfaKosP86RE1BhgZvlU5yTmiQkYB4ekQgrZMZWhXCP5MA6UWasjLmDvbplbYCkg4GGdPegt9p26Eg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: b4FdK_6fzxORLZG7PtrznPqoC0ZZSxWeq6PMmJxrLjfBWynaq8EoFkIpHVVQ1iycdQrSVKhVvmX6dE4rG05UAw

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 6 fields

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
  1. Score: 25.740, Matched: name, name.value, email.value, email
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, title, title.value, bio.value
  2. Score: 26.400, Matched: bio.value, title.value, bio, title
  3. Score: 26.400, Matched: title, title.value, bio.value, bio
  4. Score: 26.400, Matched: title, bio.value, bio, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title, title.value, bio.value
  2. Score: 39.600, Matched: title.value, bio.value, title, bio
  3. Score: 39.600, Matched: title.value, bio.value, title, bio
  4. Score: 39.600, Matched: bio.value, title, title.value, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 0 semantically similar documents

7. Hybrid search (text + vector):
Found 2 results using hybrid search (text + vector)
  1. Score: 1.200, Matched: title, content, title.value, content.value
  2. Score: 0.600, Matched: title, content, title.value, content.value

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
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 8lQk83lbnOpGVDTwdtggkvsPFnItLeqezhiAdJKQD5rswvUyYh7MPl4vVIRUlES36EYcwRdOtSL_7xMXgeGBpw

=== Sending Chat Message ===
Message ID: jamdHWWyoAlrjp25Z34Fw1HceL6f9KQ8OT5z02G2i6VFee9IlAVW5a8NVS3EEpX0bjZHnGbVRqBAmNDSSki2wg

=== AI Response ===
There are three products available:

1. The ekoDB is a high-performance database product with AI capabilities. It is priced at $99.

2. The ekoDB Cloud is a fully managed cloud database service product. This product is priced at $499.

3. The ekoDB Pro is an enterprise edition product that comes with advanced features. Its price is $299.

Execution Time: 2930ms

=== Token Usage ===
Prompt tokens: 656
Completion tokens: 77
Total tokens: 733

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: b1VaRAu9xyWQneJMvUbjHKn_hC3ajKdMNmM_jtzZGgVw8O8zjW1NiO5kdXzlJK9H7Qh7fR6mHOkWXOUGq-RcCQ

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the provided context, the available product is ekoDB, a high-performance database product. It's priced at $99.

✓ Message 2 sent
  Response: I'm sorry, but there's no information provided about any product in the context. Could you please specify the product you're interested in?

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: aa4XVYy6jw_m_6iUYetNOGVS9D8XENTVFvCOMz4Ss-DUbZImh_M5wnCXIkMCFzdzI0dTnzVjRF2kU8vUyg8YhQ
  Parent: b1VaRAu9xyWQneJMvUbjHKn_hC3ajKdMNmM_jtzZGgVw8O8zjW1NiO5kdXzlJK9H7Qh7fR6mHOkWXOUGq-RcCQ

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: aa4XVYy6jw_m_6iUYetNOGVS9D8XENTVFvCOMz4Ss-DUbZImh_M5wnCXIkMCFzdzI0dTnzVjRF2kU8vUyg8YhQ (Untitled)
  Session 2: b1VaRAu9xyWQneJMvUbjHKn_hC3ajKdMNmM_jtzZGgVw8O8zjW1NiO5kdXzlJK9H7Qh7fR6mHOkWXOUGq-RcCQ (Untitled)
  Session 3: 8lQk83lbnOpGVDTwdtggkvsPFnItLeqezhiAdJKQD5rswvUyYh7MPl4vVIRUlES36EYcwRdOtSL_7xMXgeGBpw (Untitled)
  Session 4: olIA8RwiIJqNaSvm2bo1p83W4hwHhoMqZCTv4fm5QJ-nblThsgoJzym5BfT4D0zmixus1mvrLgSoz2SqWDAyoQ (Untitled)
  Session 5: IDMSkI944tBKr49r145D8XDJtTSoecyXHvJImUzRznTb_Igdh91jEFhW21wNiPfdKqhQKKqDZd4zdNODMF36ig (Untitled)
  Session 6: 9gawzN9bFlAqfh2tPOLAJjkCyDrDQJqD0o3x8flRS81ZG9xpxqK1hEkdp1ScU1xM76lWkpYEGD6AhSQOkDDS8Q (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: aa4XVYy6jw_m_6iUYetNOGVS9D8XENTVFvCOMz4Ss-DUbZImh_M5wnCXIkMCFzdzI0dTnzVjRF2kU8vUyg8YhQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: jRlY0l6tea7j2VM5y05d3azn7gwpH0hi85zYG90QEoMOPHh25hvY6af9KU_FED5pGa1aDqzGILOlCyRtKBH_ng

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the provided context, the available product is a high-performance database product named ekoDB. It's priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['token_usage', 'id', 'created_at', 'role', 'public_id', 'content', 'chat_id', 'updated_at'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: I'm sorry, but the context provided doesn't include information about the price of ekoDB. I recommend checking the product's official website or contacting their sales department for the most accurate and up-to-date pricing information.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: ko6WhUiK7TgzykcNGyGmQju8eAPKuWk4tdQK9o7TsuDTJqt_xWjToUNeT_D3NOl_D35VdfR1SpFueZyG2x8CgA
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
Inserted: map[id:7SAHbrIGBVjWNXag_JvSVUD9mreiH-e85BTrWyJTdane-Itv7AixZdV0k4jSwDLiL0CEB6AbYO8gpsNmPzoCrQ]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:7SAHbrIGBVjWNXag_JvSVUD9mreiH-e85BTrWyJTdane-Itv7AixZdV0k4jSwDLiL0CEB6AbYO8gpsNmPzoCrQ name:map[type:String value:Test Record] public_id:map[type:String value:Dj-swxbaIFECZ4-Z0acKfjgl6N2x8Yvva1jxCNbN5ux1yKiXBSAFdLk-zlaPVQ3lmOPpXQ3Wv-mccJEMcf3A2WmE93HBKrT9aPxySdV_LG_XApCDfozYr2AykTf6jNk] value:map[type:Integer value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:7SAHbrIGBVjWNXag_JvSVUD9mreiH-e85BTrWyJTdane-Itv7AixZdV0k4jSwDLiL0CEB6AbYO8gpsNmPzoCrQ name:map[type:String value:Updated Record] public_id:map[type:String value:Dj-swxbaIFECZ4-Z0acKfjgl6N2x8Yvva1jxCNbN5ux1yKiXBSAFdLk-zlaPVQ3lmOPpXQ3Wv-mccJEMcf3A2WmE93HBKrT9aPxySdV_LG_XApCDfozYr2AykTf6jNk] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.go completed successfully

=== Running simple_websocket.go ===
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: 1e8RWSOu10b_TzdpD5PRMTFNytwt4wGAvdZJm4GEOQLVeYo7xIRcKVZfFb-Wdsx0Tb8S3pH7CcbXidDgHDb-Vg

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
        "id": "L9qiURyF7o6aHhFvzVqYmH7ANMPlTiAjsJ55aOlf5OnaWDkeSjR5to-CAGti869BavYDNilknpne5ExsEXouPw",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "public_id": {
          "type": "String",
          "value": "raFXV70UP3QvcsnmOhiapPJO--cnoukl-aRJmrZLJDAjDyGn6Cp2pUCJNACtYeip7vPYPOUuABrBD8XDZ5cw5WBRDTDnPUiYBD8pRJVpSezhUat-gXO85V-68qGyL0o"
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
        "id": "1e8RWSOu10b_TzdpD5PRMTFNytwt4wGAvdZJm4GEOQLVeYo7xIRcKVZfFb-Wdsx0Tb8S3pH7CcbXidDgHDb-Vg",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "public_id": {
          "type": "String",
          "value": "r6tMEAmRwwUifTAWCm5OBb2lgAKj0iu3YY15t0f0ns_zGtlrJyq2Pt6LI2CvdkQ6OkSqCLns3j-rf4X5SyRjt0ItLRDtUv4SDZz78Ac9zFEIEesJJhMdMj_Bkspolos"
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
        "id": "e0eDbHkjGjG0bUhI65gY5jUq-9xGN7nDhPD-Kh9_JxyegZb0gyeDDnGXNtMKMz8xQqhWNDveLXI3H5vAl6oeAw",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "public_id": {
          "type": "String",
          "value": "DEJbxxUnRUs5zj_A8fm5ItVtPE8dIFnancuWED0qHEn_bqM4N6G0QN3k8CLVnwWtfftPiIxQOUW2Hi8bUPPx-PYLavw32FNcZytG9cl7WOyYDMjwBsOZ2aCyeAbXb2E"
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
Batch insert response: map[failed:[] successful:[b0xJiIVDxJhnhpnijmW4mFcog1zSE3gvdiBZ1fVfpEh474Kuio21mfDCNxtBmFhDgfeprGpHZSTYxpKqxogVdA DQ7gbSTPRgeGIuCJpRCt8YoJRa3Kii4Z--vQGurFaVMpMHZ7LK6YTXT0fGIVGJVsaVRcvTPFTLISyvGsu6xewg h4hXFag34JClpncL3lrWT8Uk9Mu49tTeqAQ1BXb3DVTUUPJ9bVy-F2ZEcue397Xf_IcEpvrwCDGzpTI1QDkRAw V70F_7PVVL2rhvRSY0nlzsXfHA-PgD4DJDoPD10cUNqCWHmSTE2eTCq1VSKdGWz9vdcKXz4VTkOTFV3BMyHvQA m8eGbfnE_itReTZb6Cb3adegHsd4XcSUwsMonU5NschYAM5v_0wFGyvToPw2rGhgKK3xWMpzYuli3eYuRGcgmQ]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[C5d_eNGwhS6HEL2nRbbXu3JLgrv_xqEr8bBcwQn49t_3jw9wpEwYYJfjvz4K1K-b3X4T6UmWY1W11zjHThlV1g elGZl6eKTv8n6KfvZQRekZgJ8DZt6AhE1vAoPPGdZ9ePrfu5H24Qmh1GxI5B0CmmwsHPF8jzGOiWXrBIvJ44_g Mrm8HsOKyKH_lAbMSdetMVPMBR02k5gSld9mYfBDP4LPzuWzsYxvcfslPJlSRBoR-sPuvM6nDd3V9Yk3XOxuuw]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[C5d_eNGwhS6HEL2nRbbXu3JLgrv_xqEr8bBcwQn49t_3jw9wpEwYYJfjvz4K1K-b3X4T6UmWY1W11zjHThlV1g elGZl6eKTv8n6KfvZQRekZgJ8DZt6AhE1vAoPPGdZ9ePrfu5H24Qmh1GxI5B0CmmwsHPF8jzGOiWXrBIvJ44_g Mrm8HsOKyKH_lAbMSdetMVPMBR02k5gSld9mYfBDP4LPzuWzsYxvcfslPJlSRBoR-sPuvM6nDd3V9Yk3XOxuuw]]
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
Collection created with first record: LSrLDPH560cVOx-J6ueyFGY0Lt3dD5PoTZ8SzuUuTDA-oc3kpUC2ya_TGI_7GJXZcT0z15SnUPu8zYByNj5rZw

=== List Collections ===
Total collections: 12
Sample collections: [test_collection chat_configurations__ek0_bench schema_users_client_py ws_ttl_test batch_users demo_collection chat_messages__ek0_bench schema_employees_client_py schema_documents_client_py ttl_cache websocket_test schema_products_client_py]

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
✓ Inserted document: q0NS9VF1ImXHDLlPv0noWN0YJgjINeyBoiIVVeUEdd2vyACjvYIFrpMbMhfAprPMJ8CDBwuuQClEyWjY_WfYxQ

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
✓ Inserted document with TTL: 9xTczlqV2pgMuI1--bUS68y-RmtRgPE7xUd_67d9cKVUVuswV25CsIGgx3GDpS4hLdbmRRZUbhfgI2616ctcAg

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
Inserted: map[id:nmoKsLtvNBE6bIOZccWfZ9aJb9shXa9oLGOSWWEOgdACpppqkP-N7FicPZH7vS-xmzcvWMhmwZY3bQ1eub_pmA]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:nmoKsLtvNBE6bIOZccWfZ9aJb9shXa9oLGOSWWEOgdACpppqkP-N7FicPZH7vS-xmzcvWMhmwZY3bQ1eub_pmA name:map[type:String value:Test Record] public_id:map[type:String value:q6tvg-fkrL_Wsl4vo5xBp8xIKuoTb0aywpO7f2PN6a-IeW-hcOffnS3wo5vnlzAQLDbwStI8qr5s_bPpLc2mqUjAdpt9VqlqNTgnOIIg9dcDaqL9IlqOUW78qSXH6FI] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:nmoKsLtvNBE6bIOZccWfZ9aJb9shXa9oLGOSWWEOgdACpppqkP-N7FicPZH7vS-xmzcvWMhmwZY3bQ1eub_pmA name:map[type:String value:Updated Record] public_id:map[type:String value:q6tvg-fkrL_Wsl4vo5xBp8xIKuoTb0aywpO7f2PN6a-IeW-hcOffnS3wo5vnlzAQLDbwStI8qr5s_bPpLc2mqUjAdpt9VqlqNTgnOIIg9dcDaqL9IlqOUW78qSXH6FI] value:map[type:Integer value:100]]

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
Collection created with first record: Cu0yX-5z82rf9cV8yDQHf7wpOY5jDVX-t7pgEkne1p5kKdOsYUr65kvyPbxtCtZvNoJSbxZcAXKE4cSvwvkBlg

=== List Collections ===
Total collections: 12
Sample collections: [client_collection_management_go test_collection chat_configurations__ek0_bench schema_users_client_py ws_ttl_test]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 3r5ATMM-A3le4wxDxrpCYq7j4y7cFMv85s31ZXQZpiDpcn03Qf8eVbfchyGpa9CuJ6kbNgrUG5R71v0RkPD34Q

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 61qHeZ1xcHAAW9wSwah0fy94FgSPZrTU_nH93CJzhOXSkB82gs5A6OcHB8l5B5zDkQZXI4xK37dCoD4uJtkFQA

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
✓ Inserted test record: ri7_TT9BO2Vd3II3qMoh4DfHg7DfpvszMz1AKyHZ8cW-DW7leRpcVjQDYmqD7K4bhz7R_jSkdsEea4T51XCvWA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: kXvuWej_PP19YIivcLj4Rlfe0_V2qJpD741BVzmdNfWaAHnlC3lVKftPalCePiGaeU9dUO8FpluwpMzj9PfTiQ

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 6 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: GLVDLJTDdxCn876tS5yBRGmedAppdtL_9hct8NSH8uibo1AjiEgnnsvIVyVb9YM6kmiJ69t5Zn5J5V6q_u0ywA

=== Sending Chat Message ===
Message ID: fIfjywqhlY3pMYzDG0E91AfQEPLHotYviYUnRQtiWGw5_jyZJwnpgf-BGGLvQb75cH0U5HE-5Qln1Qiczzlhvw

=== AI Response ===
The available products and their prices are as follows:

1. ekoDB: It is a high-performance database product with AI capabilities. The price of this product is $99.

2. ekoDB Pro: This is the Enterprise edition product with advanced features. It is priced at $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. The cost of this product is $499.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:map[type:String value:A high-performance database product with AI capabilities] id:q_9kqRz_FxUaesQO4lGAHLGAUnidcivY1lUbU52lRoEmyPdZ20u4c8ttFd2aOXQJY0vaKkNO7OWcVIWRku6OKg name:map[type:String value:ekoDB] price:map[type:Integer value:99] public_id:map[type:String value:DxCTE6nMXNl4v87ycpOGMd6QuKHcgHy4IwBcFnKPioAwbpxlVrGlsKVTbTOycPTbDBpaNIRaMHtuQYlMdRTnYlLuVu9Xho1N-qE7itJg_AU-l5rESIp61cRLRsyz-fs]] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:map[type:String value:Enterprise edition product with advanced features] id:tRvFU-z6sAm9azX46bxLdCt0zFnwfU_xi9jVQPFUTkY4VuyaYXj7q4uriIQHwnIanTvPqVo3yrVzO2MdwVvMvg name:map[type:String value:ekoDB Pro] price:map[type:Integer value:299] public_id:map[type:String value:KMghLHy8IWncf_I0bD_p3C5DKUrN0_N6pVLHJMpTAucYo7Dv3KI4rF8gXbx2t6FVdzL_ql2V4SFMjj2GTv8m7iD_Bsh3O8JtmfO84Gx63Pc7Eq7Qp37K0pYNFjvvhKc]] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:map[type:String value:Fully managed cloud database service product] id:OhdslgKVljn21Wn2djPtxv9MD1S9F_bboAnEEQpILR1omVzot7Ic5RCHlvYSFF_6XvlFnBKVEna5eG2YnS6nVw name:map[type:String value:ekoDB Cloud] price:map[type:Integer value:499] public_id:map[type:String value:ES1cET42ibvc2O1u8WaxtAdYbZ7CbY-BNsiYMujrpJcKcl8JAnB7Z5cvP4RPLqXWB60ycNEJf8eIdnL4cP5k4IzRN2IOZ5NdMC8CWSveL7AmDB8e-13TOxWhv-s_pKE]] score:0.1111111111111111]

Execution Time: 2739ms

=== Token Usage ===
Prompt tokens: 667
Completion tokens: 86
Total tokens: 753

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: SdmO1uGgNrQ1_VQUJmH-zrd60qpJsnu5E5bKqzImwkMx_bs-WNmFdpU35DR_EsV1Gs9Todv63ThqnyeGOdpYjw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the product available is ekoDB. It is a high-performance database product priced at $99.

✓ Message 2 sent
  Response: Apologies, but there's no specific product mentioned in the context. Could you please provide more details about the product you're referring to?

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: ciLzJ-gLQbtAqKomErwhtq8q7VzT64KM9qaHS1-Eic2v_cGvr77iMVgS5jYnP_wemrHWAQRbFAbROHCgrTwGxA
  Parent: SdmO1uGgNrQ1_VQUJmH-zrd60qpJsnu5E5bKqzImwkMx_bs-WNmFdpU35DR_EsV1Gs9Todv63ThqnyeGOdpYjw

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: ciLzJ-gLQbtAqKomErwhtq8q7VzT64KM9qaHS1-Eic2v_cGvr77iMVgS5jYnP_wemrHWAQRbFAbROHCgrTwGxA (Untitled)
  Session 2: SdmO1uGgNrQ1_VQUJmH-zrd60qpJsnu5E5bKqzImwkMx_bs-WNmFdpU35DR_EsV1Gs9Todv63ThqnyeGOdpYjw (Untitled)
  Session 3: GLVDLJTDdxCn876tS5yBRGmedAppdtL_9hct8NSH8uibo1AjiEgnnsvIVyVb9YM6kmiJ69t5Zn5J5V6q_u0ywA (Untitled)
  Session 4: ko6WhUiK7TgzykcNGyGmQju8eAPKuWk4tdQK9o7TsuDTJqt_xWjToUNeT_D3NOl_D35VdfR1SpFueZyG2x8CgA (Untitled)
  Session 5: b1VaRAu9xyWQneJMvUbjHKn_hC3ajKdMNmM_jtzZGgVw8O8zjW1NiO5kdXzlJK9H7Qh7fR6mHOkWXOUGq-RcCQ (Untitled)
  Session 6: 8lQk83lbnOpGVDTwdtggkvsPFnItLeqezhiAdJKQD5rswvUyYh7MPl4vVIRUlES36EYcwRdOtSL_7xMXgeGBpw (Untitled)
  Session 7: olIA8RwiIJqNaSvm2bo1p83W4hwHhoMqZCTv4fm5QJ-nblThsgoJzym5BfT4D0zmixus1mvrLgSoz2SqWDAyoQ (Untitled)
  Session 8: IDMSkI944tBKr49r145D8XDJtTSoecyXHvJImUzRznTb_Igdh91jEFhW21wNiPfdKqhQKKqDZd4zdNODMF36ig (Untitled)
  Session 9: 9gawzN9bFlAqfh2tPOLAJjkCyDrDQJqD0o3x8flRS81ZG9xpxqK1hEkdp1ScU1xM76lWkpYEGD6AhSQOkDDS8Q (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: ciLzJ-gLQbtAqKomErwhtq8q7VzT64KM9qaHS1-Eic2v_cGvr77iMVgS5jYnP_wemrHWAQRbFAbROHCgrTwGxA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 0ZNfhsZyywq2ujWT2_AMAMEKAFbutndHBPL9RqoFjykxFbi87Pye5oOj1Ph9lDD3FBglHEviWm2CvIfKOkMPpg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the available product is a high-performance database product named ekoDB. It is priced at $99.

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
✓ Created second session: bNwBa_Y_CihtzK8FFuF5G2b-bBBNS9Y_rf34DouTNyo8zvzeTI-Zg38jz5AjdGYSqze0jkYhzA28PXxnd7xKmg
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
  1. Score: 0.761
  2. Score: 0.750
  3. Score: 0.739

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.700
  2. Score: 1.496
  3. Score: 0.304

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
  - Mouse ($0) by Alice Johnson
  - Laptop ($0) by Alice Johnson

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


up to date, audited 7 packages in 2s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.2.1 build
> tsc

✅ TypeScript client built!
🧪 Running TypeScript client library examples...

added 1 package, removed 1 package, and audited 13 packages in 674ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '9vWA0Lm-Wp_qXmXTEsyPzTkrHGlf8I6TxWdZf2smm8ops07RfrNqhpmEy4Av2ZhrsIZkOZvvKIo2maxvsHrOaw'
}

=== Find by ID ===
Found: {
  public_id: {
    value: 'ziVuU-XMzYI4iJ_3EejRhuQFMOlKj2UQ_zD6g95_LDv-WUNWE5E6IDOQyKic9u35sb4poj3AVOulCvqod0z5arKe6WPARHubTsr0ZweKMR9p06IQb5DmlASo32T4vGw',
    type: 'String'
  },
  id: '9vWA0Lm-Wp_qXmXTEsyPzTkrHGlf8I6TxWdZf2smm8ops07RfrNqhpmEy4Av2ZhrsIZkOZvvKIo2maxvsHrOaw',
  name: { type: 'String', value: 'Test Record' },
  value: { value: 42, type: 'Integer' },
  active: { type: 'Boolean', value: true }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  public_id: {
    value: 'ziVuU-XMzYI4iJ_3EejRhuQFMOlKj2UQ_zD6g95_LDv-WUNWE5E6IDOQyKic9u35sb4poj3AVOulCvqod0z5arKe6WPARHubTsr0ZweKMR9p06IQb5DmlASo32T4vGw',
    type: 'String'
  },
  name: { type: 'String', value: 'Updated Record' },
  value: { value: 100, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  id: '9vWA0Lm-Wp_qXmXTEsyPzTkrHGlf8I6TxWdZf2smm8ops07RfrNqhpmEy4Av2ZhrsIZkOZvvKIo2maxvsHrOaw'
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
Retrieved value: { userId: 123, username: 'john_doe' }

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
Collection created with first record: U-jrfMbU4oiIEgbxfjdU5CIMlYvegk8OqhFNoPh-9rSXx6UJ7eM8DuWgKFmHhE-aNKqoGXc3As78Byjfsa0kaA

=== List Collections ===
Total collections: 17
Sample collections: test_collection,schema_users_client_go,client_batch_operations_ts,chat_configurations__ek0_bench,schema_users_client_py

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 2o25UQdjFmFnrxb204qZmFyuzie-_IpGdcLwOxtTPhTaEPOeq9sv4zWUvYvqR8RC2hQm-Bsyq0iarm82PaUPyw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: P6JNAEoUwHLV0kCTVnPlYEgWFqZ0Nf7pWXE5I2835Es_d3YeTw2eStyVpMlspYiV5gAWEHM9mdFgVnR94NzT-g

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
✓ Inserted test record: ug7328s8Q0IuOqgQbZOOnA3JZYKFv94mDwbEBLBcTWEigc91XDs6PF_MYiE9XdmRr95F4lac4JxkjzA0AO-2fg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: oPQI0YiM8S83CY5x-65-I_s6lNohe8mXW0aBn1AgErRontRrRWxTfx25HNXp90-45oWR5MoKyqhWfytuYigBgQ

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 6 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: LbRnnhVn8vvX8qhVAcSccv6rvfwchy3PWuh_q1nlfAgWjRPtZ4BSZbtv-ssD9LbfAOqegW5RYCiQ34SVRy3XRQ

=== Sending Chat Message ===
Message ID: zvtXEZPX6Sj2NnloG5KFXrzCTt-IFrmw4f6R8ydjjesMetnxJ7Yej5QzJafQX1qBzjV08-T372BkMyZKhsn-Xw

=== AI Response ===
Based on the provided context, the available products and their prices are as follows:

1. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.
2. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.
3. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    price: { value: 299, type: 'Integer' },
    public_id: {
      value: 'Rm2rC4dg8BKw9qWwbQ1WTJsXDhUeHjSLZ3VfNmhlRDpzYuQNkPOEH5kJfqh2AKc8CxMT58eg_T_XshyCLmEOkvun_tAt6hkuDz7KPqqnNJud235N1cDwTUpKP5wwHhY',
      type: 'String'
    },
    description: {
      type: 'String',
      value: 'Enterprise edition product with advanced features'
    },
    name: { value: 'ekoDB Pro', type: 'String' },
    id: 'S3oNXg6vnzw1830jZd7bOd68kpg8aJMdDe5TE3vLJIhmLUbDmHitWQ2knVM5x7mK5loQvSJkZAWbGvzYXo7eHA'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    price: { type: 'Integer', value: 499 },
    name: { type: 'String', value: 'ekoDB Cloud' },
    public_id: {
      type: 'String',
      value: 'wUsbkNv13lKLBVNX2jFSfP8Uec98hXQA8VbZTMP0RzGTdwkQqejJQkiVYU8WAGqiRMJDWCUrxOqBSPWA3HnZEEb4p5QFa-tZIooj7p6YyHqpeKKmbb4nLMoWT_pWipc'
    },
    description: {
      value: 'Fully managed cloud database service product',
      type: 'String'
    },
    id: 'xrHMnqW6WPTHl8DihvWBJA_SknkAFqomkpSuj6WmPV5FmdjENZhlyHnRFxshPKRdAFeyS-L0U4qKMbQ8oSyuwA'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'uOoL-2Xs9A8RxzB-TaZrqYiDO-qIvTAJ7cNJ3mQ1wpAFao_7dmttT5QBcr_kpFgJ1HIXGkwZQCHoM0aC3EKpQg',
    public_id: {
      value: 'iKibPj9i24iThQqu9kQcCEyB7TH3IrzuXV7-Y8ahIVCbeXe-q-lcAXJCLCM62WrII6i-OxPYdmzvUrHVcuLEhzOpapgEi94ASYA-twOtWU0OVo1jY-aUb7wCSFAK5cU',
      type: 'String'
    },
    price: { value: 99, type: 'Integer' },
    name: { value: 'ekoDB', type: 'String' },
    description: {
      type: 'String',
      value: 'A high-performance database product with AI capabilities'
    }
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3293ms

=== Token Usage ===
Prompt tokens: 663
Completion tokens: 94
Total tokens: 757

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 7dIMlBUfXlotsaez_6-rRattUAye4n7vFZVuV5pNMo3kV2Avp4aV7qv3nu51y0Vv7VB6ScbKRsDNFkVKSEiCQA

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the provided context, the available product is ekoDB, a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: I'm sorry, but there seems to be no context provided regarding a specific product. Could you please specify the product you're asking about?

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: N64UMDwtBu5ZF1irYLr7iGyGi_r7pBQUuFby93puMp3HJ4LW9SiLv1JBrbyRnBT8Wgsbhq1EGhHB2i-Wt9WfHg
  Parent: 7dIMlBUfXlotsaez_6-rRattUAye4n7vFZVuV5pNMo3kV2Avp4aV7qv3nu51y0Vv7VB6ScbKRsDNFkVKSEiCQA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: N64UMDwtBu5ZF1irYLr7iGyGi_r7pBQUuFby93puMp3HJ4LW9SiLv1JBrbyRnBT8Wgsbhq1EGhHB2i-Wt9WfHg (Untitled)
  Session 2: 7dIMlBUfXlotsaez_6-rRattUAye4n7vFZVuV5pNMo3kV2Avp4aV7qv3nu51y0Vv7VB6ScbKRsDNFkVKSEiCQA (Untitled)
  Session 3: LbRnnhVn8vvX8qhVAcSccv6rvfwchy3PWuh_q1nlfAgWjRPtZ4BSZbtv-ssD9LbfAOqegW5RYCiQ34SVRy3XRQ (Untitled)
  Session 4: bNwBa_Y_CihtzK8FFuF5G2b-bBBNS9Y_rf34DouTNyo8zvzeTI-Zg38jz5AjdGYSqze0jkYhzA28PXxnd7xKmg (Untitled)
  Session 5: SdmO1uGgNrQ1_VQUJmH-zrd60qpJsnu5E5bKqzImwkMx_bs-WNmFdpU35DR_EsV1Gs9Todv63ThqnyeGOdpYjw (Untitled)
  Session 6: GLVDLJTDdxCn876tS5yBRGmedAppdtL_9hct8NSH8uibo1AjiEgnnsvIVyVb9YM6kmiJ69t5Zn5J5V6q_u0ywA (Untitled)
  Session 7: ko6WhUiK7TgzykcNGyGmQju8eAPKuWk4tdQK9o7TsuDTJqt_xWjToUNeT_D3NOl_D35VdfR1SpFueZyG2x8CgA (Untitled)
  Session 8: b1VaRAu9xyWQneJMvUbjHKn_hC3ajKdMNmM_jtzZGgVw8O8zjW1NiO5kdXzlJK9H7Qh7fR6mHOkWXOUGq-RcCQ (Untitled)
  Session 9: 8lQk83lbnOpGVDTwdtggkvsPFnItLeqezhiAdJKQD5rswvUyYh7MPl4vVIRUlES36EYcwRdOtSL_7xMXgeGBpw (Untitled)
  Session 10: olIA8RwiIJqNaSvm2bo1p83W4hwHhoMqZCTv4fm5QJ-nblThsgoJzym5BfT4D0zmixus1mvrLgSoz2SqWDAyoQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: N64UMDwtBu5ZF1irYLr7iGyGi_r7pBQUuFby93puMp3HJ4LW9SiLv1JBrbyRnBT8Wgsbhq1EGhHB2i-Wt9WfHg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: ohM_2Dje0Qu5oZqu_0rj81nK5vmZXn_0l7QU_V_K6XHM9uU3V9ZVe_Uexb-spLlKmXC2gORfbAx2Mn-FaHwWfw

=== Sending Initial Message ===
✓ Message sent
  Response: The product available according to the provided context is ekoDB. It is a high-performance database product. It is priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: I'm sorry, but there's no context provided regarding the price of ekoDB. I recommend checking the product's official website or contacting the seller directly for the most accurate and up-to-date pricing information.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: n5qw0q1_OPZJfWkVDtp31jtohboW0DlpxGw5aH9EKxDo785uYfuAl5sxCjT0IE-VfL21NyPUbmdeZ9xDqghWxw
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
  1. Score: 25.740, Matched: name, email.value, email, name.value
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio.value, title.value, bio
  2. Score: 26.400, Matched: title.value, bio, title, bio.value
  3. Score: 26.400, Matched: title.value, bio, title, bio.value
  4. Score: 26.400, Matched: title.value, bio.value, title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio, bio.value, title.value
  2. Score: 39.600, Matched: title.value, bio, bio.value, title
  3. Score: 39.600, Matched: title.value, title, bio.value, bio
  4. Score: 39.600, Matched: bio, title.value, title, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.759, Matched:
  2. Score: 0.751, Matched:
  3. Score: 0.727, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.691, Matched: title.value, title, content.value, content
  2. Score: 1.500, Matched: content.value, title.value, content, title
  3. Score: 0.304, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: bio, bio.value

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
✅ TypeScript client examples complete!
✅ All TypeScript integration tests complete!
🧪 Running JavaScript examples (direct HTTP/WebSocket)...

added 1 package, removed 1 package, and audited 9 packages in 731ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities

╔════════════════════════════════════════╗
║  ekoDB JavaScript Examples Test Suite ║
╚════════════════════════════════════════╝

=== Checking Server Connection ===
(node:13767) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Server is ready

=== Getting Authentication Token ===
✓ Authentication successful

=== Running 7 Examples ===

=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js ===
✓ Authentication successful

=== Insert Document ===
(node:13769) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Inserted: {
  id: 'mkqE03UMiiecengGPE2y8fZACeZFkbCdT_-3xJA5ZxhFxrcPyXkhavYqCnqcePoFn1Lo5ESpPT2LFS2c7hbVlQ'
}

=== Find by ID ===
Found: {
  active: { type: 'Boolean', value: true },
  id: 'mkqE03UMiiecengGPE2y8fZACeZFkbCdT_-3xJA5ZxhFxrcPyXkhavYqCnqcePoFn1Lo5ESpPT2LFS2c7hbVlQ',
  value: { type: 'Integer', value: 42 },
  name: { type: 'String', value: 'Test Record' },
  public_id: {
    type: 'String',
    value: 'fsYyWjGMjBy-AFSOg83xbGzNIimVrcHPsUI9eTmIfE07mGgeNqGjBfcDEnCqsVaYwJoWIKExP66Ees8gcoBatgVHku05j5bsNKZM34I8OnogWsFf7FBffgTefTBelqs'
  }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  id: 'mkqE03UMiiecengGPE2y8fZACeZFkbCdT_-3xJA5ZxhFxrcPyXkhavYqCnqcePoFn1Lo5ESpPT2LFS2c7hbVlQ',
  public_id: {
    type: 'String',
    value: 'fsYyWjGMjBy-AFSOg83xbGzNIimVrcHPsUI9eTmIfE07mGgeNqGjBfcDEnCqsVaYwJoWIKExP66Ees8gcoBatgVHku05j5bsNKZM34I8OnogWsFf7FBffgTefTBelqs'
  },
  value: { value: 100, type: 'Integer' },
  active: { type: 'Boolean', value: true }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.js completed successfully

=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js ===
✓ Authentication successful

=== Inserting Test Data ===
(node:13771) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted test record: vxx9S3GKYBkRYwCZaewPGfMX7l31J-Px3-jqORowwWhPEbSnIlglmruyQAPnbeW13zW65ln-vekOx7Q0jmMiEA

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
        "id": "1e8RWSOu10b_TzdpD5PRMTFNytwt4wGAvdZJm4GEOQLVeYo7xIRcKVZfFb-Wdsx0Tb8S3pH7CcbXidDgHDb-Vg",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "public_id": {
          "type": "String",
          "value": "r6tMEAmRwwUifTAWCm5OBb2lgAKj0iu3YY15t0f0ns_zGtlrJyq2Pt6LI2CvdkQ6OkSqCLns3j-rf4X5SyRjt0ItLRDtUv4SDZz78Ac9zFEIEesJJhMdMj_Bkspolos"
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
        "id": "e0eDbHkjGjG0bUhI65gY5jUq-9xGN7nDhPD-Kh9_JxyegZb0gyeDDnGXNtMKMz8xQqhWNDveLXI3H5vAl6oeAw",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "public_id": {
          "type": "String",
          "value": "DEJbxxUnRUs5zj_A8fm5ItVtPE8dIFnancuWED0qHEn_bqM4N6G0QN3k8CLVnwWtfftPiIxQOUW2Hi8bUPPx-PYLavw32FNcZytG9cl7WOyYDMjwBsOZ2aCyeAbXb2E"
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
        "id": "L9qiURyF7o6aHhFvzVqYmH7ANMPlTiAjsJ55aOlf5OnaWDkeSjR5to-CAGti869BavYDNilknpne5ExsEXouPw",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "public_id": {
          "type": "String",
          "value": "raFXV70UP3QvcsnmOhiapPJO--cnoukl-aRJmrZLJDAjDyGn6Cp2pUCJNACtYeip7vPYPOUuABrBD8XDZ5cw5WBRDTDnPUiYBD8pRJVpSezhUat-gXO85V-68qGyL0o"
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
        "id": "vxx9S3GKYBkRYwCZaewPGfMX7l31J-Px3-jqORowwWhPEbSnIlglmruyQAPnbeW13zW65ln-vekOx7Q0jmMiEA",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "public_id": {
          "type": "String",
          "value": "1oNjjp48hlTsMjp9ul5RWpCIsAGis6Z9zGs_TV1b6VRWgbGVbUfMkTMIh4vfqEePnei4cKb4yv0bz8k95qGB8nYx8oUdMqOU3VXFCV6znQo-1TznxRT50scwcYpoIp0"
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
(node:13783) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
(node:13785) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {
  userId: 123,
  loginTime: '2025-11-05T04:08:42.919Z',
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
✓ kv_operations.js completed successfully

=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/collection_management.js ===
✓ Authentication successful

=== Create Collection (via insert) ===
(node:13787) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Collection created with first record: x-Fe24-H_KWw_4uOa0ZKuZInEMuEuSQfbV5m8VX-8lm92fI-GTxkh6vtdJUEqY1hH36lk6qvVigwCZNQYTacoQ

=== List Collections ===
Total collections: 21
Sample collections: [
  'test_collection',
  'schema_users_client_go',
  'client_batch_operations_ts',
  'chat_configurations__ek0_bench',
  'schema_users_client_py'
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
(node:13799) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document: 2nOAnsXDVyYy7_f0pluTDmpenJcCu_h63PNhlTinTbGbeIALo9PyRf6DTeH41E_OMQM-PMUyP7PusCRD_1MK_g

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
(node:13806) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document with TTL: eX0g6vLT9XSTnakY8y4e6PQPYsyAcNnpucN8KwnxBd2q10U51c690eaC4EWGxNZEfpZhByWTIbqUtoSjuM3pSQ

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

added 1 package, removed 1 package, and audited 9 packages in 604ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'ZUQ0Xt_1qwyxpmgAksxRCPwLDW4i1JsU2tuh_cmA-gn8mQpgLSN1dV9jMouOJLnO9LNYoHBGk7y320r3RQTwoA'
}

=== Find by ID ===
Found: {
  public_id: {
    value: 'GjzPaPxvfBomBTCX8pT6AaU6OoghxoulMcxlNYM4UZZjvE2SbaAttR2BMmV19-63xEqsN79udLEQhdiQ89jpg6XnrKkkyUSFtM1-nlKLTe9Cn-EQtKdz2iPXosKoyzA',
    type: 'String'
  },
  active: { value: true, type: 'Boolean' },
  id: 'ZUQ0Xt_1qwyxpmgAksxRCPwLDW4i1JsU2tuh_cmA-gn8mQpgLSN1dV9jMouOJLnO9LNYoHBGk7y320r3RQTwoA',
  name: { type: 'String', value: 'Test Record' },
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  public_id: {
    value: 'GjzPaPxvfBomBTCX8pT6AaU6OoghxoulMcxlNYM4UZZjvE2SbaAttR2BMmV19-63xEqsN79udLEQhdiQ89jpg6XnrKkkyUSFtM1-nlKLTe9Cn-EQtKdz2iPXosKoyzA',
    type: 'String'
  },
  active: { value: true, type: 'Boolean' },
  name: { value: 'Updated Record', type: 'String' },
  id: 'ZUQ0Xt_1qwyxpmgAksxRCPwLDW4i1JsU2tuh_cmA-gn8mQpgLSN1dV9jMouOJLnO9LNYoHBGk7y320r3RQTwoA',
  value: { type: 'Integer', value: 100 }
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
Retrieved value: { userId: 123, username: 'john_doe' }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { name: 'Product 1', price: 29.99 }
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
Collection created with first record: sD2fEcWlXGIyI5amzowT1T7OMixjsnaIj_mQTFxbqXuGmQDSCW827qVNh0f1vMO9LwSDDI8jogFIiP54-GNXmw

=== List Collections ===
Total collections: 22
Sample collections: client_collection_management_js,test_collection,schema_users_client_go,client_batch_operations_ts,chat_configurations__ek0_bench

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: LEVD_o4Eoq65vzJwDNlyO5R22_BM4412J40ApsxWQLMVZcc7cwzIGTLw0JV09AVcZdMMRzhi8GCKF3AteafQOQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 8LkiS-gCu6HyNkfv1aj8Bfs6Jzz4vY3zRoAyv8phEo4wRwAy6JMfdYpHyF0ahekSPZ7HSkqYTaWYoC064YDdRQ

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
✓ Inserted test record: EQB-0MYD-Lgqt4fS-CkliK89fO4bsxhqqFaa9OfyKLCBdjIBnooTPT5fehAvc5p0guV9hSTkSHKgGrB3ziaGJQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: HYCzhUy_9AgUaHc0ldSho_jmbINJLZI4P0kdpjJGFE99K6lzaOKkkh_6BiXkBPN3vQa905rFGvrG0PULYZBAOA

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 6 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
✅ JavaScript client examples complete!
✅ All JavaScript integration tests complete!
🟣 Building Kotlin client library...
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build

BUILD SUCCESSFUL in 4s
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
✓ Inserted user: Record(fields={id=StringValue(value=xnLFMxdyINtKlYn-lJpBp4sQFKipYSbPO-sxUo7S4N77y4Kk6ERBsz83QUddHuvc5QbA9faaSUBg13gvoNKREg)})
  User ID: xnLFMxdyINtKlYn-lJpBp4sQFKipYSbPO-sxUo7S4N77y4Kk6ERBsz83QUddHuvc5QbA9faaSUBg13gvoNKREg

=== Read ===
✓ Found user by ID: Record(fields={age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=28)}), id=StringValue(value=xnLFMxdyINtKlYn-lJpBp4sQFKipYSbPO-sxUo7S4N77y4Kk6ERBsz83QUddHuvc5QbA9faaSUBg13gvoNKREg), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), public_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Z0K2yk0VOu5svwzVtY-3iZzUEG5TeN9qniQ0fP3GhdB6WggqGIn8bpk1tSw_IagpzDF_6YlH40eVU7OFhSF1D0qSo8IC9-laS4nCg7p-LkQRjtTciLEjG67uE8uX_ao)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)})})

=== Update ===
✓ Updated user: Record(fields={active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), public_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Z0K2yk0VOu5svwzVtY-3iZzUEG5TeN9qniQ0fP3GhdB6WggqGIn8bpk1tSw_IagpzDF_6YlH40eVU7OFhSF1D0qSo8IC9-laS4nCg7p-LkQRjtTciLEjG67uE8uX_ao)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), id=StringValue(value=xnLFMxdyINtKlYn-lJpBp4sQFKipYSbPO-sxUo7S4N77y4Kk6ERBsz83QUddHuvc5QbA9faaSUBg13gvoNKREg), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)})})

=== Query ===
✓ Found 1 users matching query
  - Record(fields={public_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Z0K2yk0VOu5svwzVtY-3iZzUEG5TeN9qniQ0fP3GhdB6WggqGIn8bpk1tSw_IagpzDF_6YlH40eVU7OFhSF1D0qSo8IC9-laS4nCg7p-LkQRjtTciLEjG67uE8uX_ao)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), id=StringValue(value=xnLFMxdyINtKlYn-lJpBp4sQFKipYSbPO-sxUo7S4N77y4Kk6ERBsz83QUddHuvc5QbA9faaSUBg13gvoNKREg), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)})})

=== Delete ===
✓ Deleted user with ID: xnLFMxdyINtKlYn-lJpBp4sQFKipYSbPO-sxUo7S4N77y4Kk6ERBsz83QUddHuvc5QbA9faaSUBg13gvoNKREg

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
  IDs: zre3hLTh5SzdlUf3BOxjUMmfc01yjTgU7FK7B4qfqj6TEZ9de1urgHwUupsZkT8z3rcRsiXVjDQHXX7X-zTDng, 8qT-z5C8ehtYA3Bgz1oNe93lPXxKJEciaSXTpDURu6QDQKFtAdmdAni-43hI78BBqmH5HRiZCmVanfoa2pmvkA, RG3SMHf1myEdcBwMTC7whanqUb6D2TfStLJGBiDpROCjmUMtQsy5bqWQzfaty3J-P0jUlVXptgRTBlBoE1K43Q...

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
  - Score: ObjectValue(value={value=IntegerValue(value=88), type=StringValue(value=Integer)})

=== Query 4: Complex (score > 80 AND age >= 25) ===
✓ Found 4 high-scoring adults

=== Query 5: IN (city IN ['NYC', 'LA']) ===
✓ Found 4 records in NYC or LA

=== Query 6: Pagination (skip 2, limit 2) ===
✓ Page 2 (2 records):
  - ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Charlie)})
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
✓ Session value: {"created_at":1762315754153,"user_id":"123"}
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
  - test_collection
  - schema_users_client_go
  - client_batch_operations_ts
  - chat_configurations__ek0_bench
  - schema_users_client_py

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-11-05T04:09:18.395268Z","last_modified":"2025-11-05T04:09:18.395268Z","bypass_ripple":false,"primary_key_alias":"id"}

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
  Document ID: B1hEYexb18tLQRQj0KPUMjpVn7MaB4NuZF_CF8zkVf4VcL5fhNajgOzk4X_U881fqgzLP6gOCVhoooGD6nKIpQ

=== Verify Document Exists ===
✓ Document found: [created_at, public_id, ttl_duration, user_id, id, session_id]

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: 1UdnXW6DDEcW02XVjeZdwqdff9JuJgYLWMA-lIXYH9z0i5q0ANYKCUmaBHLgFCzqPsI1A1xeNRnUDzEcFAk0Lw

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: [id, public_id, value, ttl_duration, cache_key]

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
  {"payload":{"data":[{"id":"nv3UFIajbTH_C0SvAWCkNK-oOIX9zAGh1njyOtCp3NOCAGm6AXv6mrwIoAB_vOfChOwz9C80v0hsKEdfVOgagg","name":{"type":"String","value":"Test User"},"public_id":{"type":"String","value":"rggqv93lNqoeCezvWfA2B5JUcer1FJtIfmrV5ji-n9Jv3XFLHEN5UE_XMWtd36oQtKf-5tHqqaufu2Abg8_6C10E19VU-9fFGsyecI_aUDFiBfOF6DxXim0qMoTauc4"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

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
✓ Inserted user 1: StringValue(value=36Jyo9UESFJ8zYFWuJZWemIJ_wFxDXStrtNrNhzM_RelBgmEzaptKNJBdBig1HQbeKNaiNaYtKlv9vHxSG3z3w)
✓ Inserted user 2: StringValue(value=uqwPkwrvvRcD-K7eeGSkI10vcGX6S-Mm1fxtJj-VAfeIkf8Sr0tAaWnJ39Gh-9BZUfvd-IDod0MTnGs99IeHwQ)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 22
  Sample: [test_collection, schema_users_client_go, kotlin_schema_example, client_batch_operations_ts, chat_configurations__ek0_bench]

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
  {"results":[{"record":{"id":"CcPyo1AwjTP33g79veKF9VZGvLzykV8rAvMqTJJyvXwIydjlFLzs4e0Rnd9m98tt_MOgXGr4ch2f-aZB87WU9Q","public_id":"R9exk0DgZZ_RLaIKUjSvYDLFYB5Xt3BO3JVbMCW5faev2-H8PANzOZvUL-LQq4FukZPym575gA-_offBa8Y2IUoMZxns82m0Y9fNAkNvGE9MD-qyDZuZIJzeD4O4zQ0","tags":"programming,rust,tutorial","description":"Learn Rust programming language with hands-on examples and best practices.","title":"Rust Programming","views":483},"score":19.8,"matched_fields":["description","title","tags"]},{"record":{"id":"LbcznaHjhfB4-L1C19GBL_7VkPT-TNDf8bF1zH0CMVRan6z9Iqh8CqMHT64AbmKFjVYOQUlQHnGCAQ4YTtsWZQ","description":"Master Python for data analysis, machine learning, and visualization.","title":"Python for Data Science","views":738,"public_id":"iUSlqZNuR59cSIFV96F1br8-3xS0YofSuNc8eonLbVblj3rk7i9mpW-qhS4omZrTV8k80QYSicA87z05S1v9c3GuSnSrTO_U_V-VYv2XUYQaNGsDi2npuf-xppQX01s","tags":"programming,python,data-science"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"tags":"programming,javascript,web","id":"v3XoeFiud5BxI79vIgcQT1jHH6l5i6uEK6G7MwJBGtObEusl_f23BNiCnT_RRvBDB_hhpGTDkd-420saFuolFQ","views":985,"description":"Build modern web applications using JavaScript, React, and Node.js.","title":"JavaScript Web Development","public_id":"05Elcktoe3VpsRSEoUcS9w7dlVdld4nQiGpe9q17oReG8WfXGkzWN6Zeq6wMZpMzJ32FsQTMtXFu9Ycuj4ORR_9IegYDIww7jwcQOjNSGJnOGojAEFEOuZHKZxd-5lA"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"tags":"ai,machine-learning,python","views":705,"id":"xGsRYHSlFw22xjEVA7ufd2Fv75SXF4KMb26ONNE7Et4D1h-heSks8ZdXUaInPrx7h4o-8x4noAb9e5VmvYHyAw","title":"Machine Learning Basics","description":"Introduction to machine learning algorithms and neural networks.","public_id":"8FL9s4xtRFakvHRCPPAAfQx1k1ay-aokN7xszu1ccYLQwPKi3UVqxSPh_E5my1rSHVTwxYgiSjJVcGwNy08uiYa55n78FrZ67KOrA1r551KdJTs96Q6VA0uijGIG98o"},"score":2.7,"matched_fields":["tags","title","description"]},{"record":{"public_id":"iUSlqZNuR59cSIFV96F1br8-3xS0YofSuNc8eonLbVblj3rk7i9mpW-qhS4omZrTV8k80QYSicA87z05S1v9c3GuSnSrTO_U_V-VYv2XUYQaNGsDi2npuf-xppQX01s","tags":"programming,python,data-science","id":"LbcznaHjhfB4-L1C19GBL_7VkPT-TNDf8bF1zH0CMVRan6z9Iqh8CqMHT64AbmKFjVYOQUlQHnGCAQ4YTtsWZQ","views":738,"title":"Python for Data Science","description":"Master Python for data analysis, machine learning, and visualization."},"score":1.0,"matched_fields":["description"]},{"record":{"id":"OxQ_oEi2PA6NoxU0Z45hEAzr2PseMcGNS5Ufge-Eb0RHjitKYLPXJmgL5cFuklce-SVOjTrLGYtvGK5lIqyopg","tags":"database,design,sql","views":612,"public_id":"nTH4XPWm-kESP_VgueMbLFCxQH4tpOAV_kIbgva5RyRsP0T9tqaYjOGA3tn-AKBAFut7rhh2dMuTc_WTbecww5tjzzF82SG14V7134OIlmSurD8poGwMFRylvbkilWU","title":"Database Design","description":"Learn database design principles, normalization, and query optimization."},"score":0.5,"matched_fields":["description"]},{"record":{"title":"Rust Programming","views":483,"description":"Learn Rust programming language with hands-on examples and best practices.","tags":"programming,rust,tutorial","id":"CcPyo1AwjTP33g79veKF9VZGvLzykV8rAvMqTJJyvXwIydjlFLzs4e0Rnd9m98tt_MOgXGr4ch2f-aZB87WU9Q","public_id":"R9exk0DgZZ_RLaIKUjSvYDLFYB5Xt3BO3JVbMCW5faev2-H8PANzOZvUL-LQq4FukZPym575gA-_offBa8Y2IUoMZxns82m0Y9fNAkNvGE9MD-qyDZuZIJzeD4O4zQ0"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
✓ Inserted document with TTL: StringValue(value=Gct9Vb-zDGH_APAg6rw02tyJbOdLXaPKgYxYuf1gpCQlWcsIiPQfBrzS42sdktnVyoZQdt0h_JkxTIZ9u_njRQ)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1762315779182},"id":"Gct9Vb-zDGH_APAg6rw02tyJbOdLXaPKgYxYuf1gpCQlWcsIiPQfBrzS42sdktnVyoZQdt0h_JkxTIZ9u_njRQ","name":{"type":"String","value":"WebSocket TTL Test"},"public_id":{"type":"String","value":"we6tbsWMiSTlPdZircnpcVDLXViO5tMIk9AJlbu6ziEcpbFD5E0aY1_Irj-O9tJSbvBo0HqpLxidnFuaK9satbFje0bCogg10l2hHtuDnSsdbEuA1N5gxgBQ94IsPpw"},"ttl_duration":{"type":"String","value":"1h"},"value":{"type":"Integer","value":42}}]},"type":"Success"}

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

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: -mM9chR9Q4xeBFgDjZVfUTfQ8wyJPhQff9vYuTzIMkzfCXDRP-P11Okwh52lCEQIv973hj77THBUfESPmMY9pw

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "FFw4BJBTYPCf3e367bkilgxPZ3o4myxn2czuUioy5_KofcYZ-TXqcqxxSs9rwEa3RJmKXEg-iKZJoxR1kfVJLw"
  Responses: ["ekoDB is a high-performance database that is designed with intelligent caching and real-time capabilities. It also integrates AI into its system, offering a unique feature that enables you to query your database using natural language and get AI-powered responses with relevant context. This is part of ekoDB's AI Chat Integration feature. Furthermore, ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval as part of its Search Features."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 8s
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
✓ Created session: bscAUzQnmEKPfzfCaTR5-PvWSiuhfImJV1byIogcXmOnrRsdYFlbXNY4LOWUDVh_gqp2-E4xFDTX0LhDJLkWiA

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["Based on the information provided, the available product is ekoDB, a high-performance database product with AI capabilities. It is priced at $99."]

✓ Message 2 sent
  Responses: ["Apologies, but there's no context provided regarding the product you're asking about. Could you please specify the product you want to know the price for?"]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"bscAUzQnmEKPfzfCaTR5-PvWSiuhfImJV1byIogcXmOnrRsdYFlbXNY4LOWUDVh_gqp2-E4xFDTX0LhDJLkWiA"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"type":"Object","value":{"collection":{"type":"String","value":"kotlin_chat_sessions_example"},"matched_fields":{"type":"Array","value":[{"type":"String","value":"description"}]},"record":{"type":"Object","value":{"description":{"type":"String","value":"A high-performance database product with AI capabilities"},"id":{"type":"String","value":"kkhIpYoQ8lAOjqAUsFwl1Ninl4qqTL8ZUAY5APrh3qINlk9zk8EPTWhAHNsNcIv3ht63flQxTzEd-eWeXYls3w"},"price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"},"public_id":{"type":"String","value":"nr1TVSjGkLOqVYZPdsWE-J0f1x4NVDI-Zrl_6MxelMcb72JAcm5X8dSX3b61YGgR6WzuUdNJpQ7ohA84_no1aaUwjqZ3WGjtm5GsjCNWJ0LFj04Su7n7dNhM2wJpMLw"}}},"score":{"type":"Float","value":0.25}}}]},"created_at":{"type":"DateTime","value":"2025-11-05T04:09:58.334882Z"},"id":"_GFLVMCdAMxQoc6qQfgiLnRMpwd8WMKyC860iUKRIRF2niiLKP_fhk6IQ9m14MNnvI2NNWKgqpWDzJ0nNpW9SA","public_id":{"type":"String","value":"j1jH7tog88ucM7a_uxe_eZf776J0tjGvtOTTCtihadcNipxHz5-sPZgW2fQmRIfWKk3bW_3GvfQ6Q5Ol3ckM92wqFVU0qe8k_LZdgjLyyN3vWba1RzVE-mWSmYX_Ckk"},"role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":30},"prompt_tokens":{"type":"Integer","value":259},"total_tokens":{"type":"Integer","value":289}}},"updated_at":{"type":"DateTime","value":"2025-11-05T04:09:58.334882Z"}},{"chat_id":{"type":"String","value":"bscAUzQnmEKPfzfCaTR5-PvWSiuhfImJV1byIogcXmOnrRsdYFlbXNY4LOWUDVh_gqp2-E4xFDTX0LhDJLkWiA"},"content":{"type":"String","value":"Based on the information provided, the available product is ekoDB, a high-performance database product with AI capabilities. It is priced at $99."},"context_snippets":{"type":"Array","value":[{"type":"Object","value":{"collection":{"type":"String","value":"kotlin_chat_sessions_example"},"matched_fields":{"type":"Array","value":[{"type":"String","value":"description"}]},"record":{"type":"Object","value":{"description":{"type":"String","value":"A high-performance database product with AI capabilities"},"id":{"type":"String","value":"kkhIpYoQ8lAOjqAUsFwl1Ninl4qqTL8ZUAY5APrh3qINlk9zk8EPTWhAHNsNcIv3ht63flQxTzEd-eWeXYls3w"},"price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"},"public_id":{"type":"String","value":"nr1TVSjGkLOqVYZPdsWE-J0f1x4NVDI-Zrl_6MxelMcb72JAcm5X8dSX3b61YGgR6WzuUdNJpQ7ohA84_no1aaUwjqZ3WGjtm5GsjCNWJ0LFj04Su7n7dNhM2wJpMLw"}}},"score":{"type":"Float","value":0.25}}}]},"created_at":{"type":"DateTime","value":"2025-11-05T04:09:58.337901Z"},"id":"ajz6Vu_wcfYtg3daNUAXFbktFyaWr-_K9zWudfGqhZ00CbO_CwYmLkYLRVo2AAbsgvvbERNiBF9rE76XMZ1OBQ","public_id":{"type":"String","value":"RV4iEKSojtYwx7amygtEpKXM9gGsRB9POZW_7S3hJbPpgNjPUz1rMmdRdRYEPvQU5k4AC-TE723Nkc88EbELFxks5jm2i1PzPQJVIQa1pc65C6JiTpY2LfASK8800R4"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":30},"prompt_tokens":{"type":"Integer","value":259},"total_tokens":{"type":"Integer","value":289}}},"updated_at":{"type":"DateTime","value":"2025-11-05T04:09:58.337901Z"}},{"chat_id":{"type":"String","value":"bscAUzQnmEKPfzfCaTR5-PvWSiuhfImJV1byIogcXmOnrRsdYFlbXNY4LOWUDVh_gqp2-E4xFDTX0LhDJLkWiA"},"content":{"type":"String","value":"What is the price?"},"created_at":{"type":"DateTime","value":"2025-11-05T04:09:59.970679Z"},"id":"e3yPqtQ3B8_C6LWi2dg2oWrVAP1l7O5a2gJ5-C6dNMhque2cnwyNOhoOx9vJRajmBh_215GSPFCpD4lgxGVRsA","public_id":{"type":"String","value":"g79EhOGZJ3qnaRPSvbKenxkFBrpbJiPhCXXjGKezxqkM9YvkooX4uz2cd1qTzWS104R4ENjjZA8RhVjqg2Nt9oy6vISg1fTgLQ0It643r-fWRV3uddAFt0Y52gfiOq8"},"role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":31},"prompt_tokens":{"type":"Integer","value":63},"total_tokens":{"type":"Integer","value":94}}},"updated_at":{"type":"DateTime","value":"2025-11-05T04:09:59.970679Z"}},{"chat_id":{"type":"String","value":"bscAUzQnmEKPfzfCaTR5-PvWSiuhfImJV1byIogcXmOnrRsdYFlbXNY4LOWUDVh_gqp2-E4xFDTX0LhDJLkWiA"},"content":{"type":"String","value":"Apologies, but there's no context provided regarding the product you're asking about. Could you please specify the product you want to know the price for?"},"created_at":{"type":"DateTime","value":"2025-11-05T04:09:59.973339Z"},"id":"y6mw0fX9EDyrTGp6xkiWrZxq9ewCmEkjZRLoT7pd7vI4F_mFWFj4vrovEQOPQ33BoHio8x9n1rbkgDTPTJsOjw","public_id":{"type":"String","value":"ClsJn1MsZp4msY7BygDUUY_lMOKj6OGPm5MLd9kZiMiD-tbQCF0IZcvTXj_C50Izjcp5QkstM0X0K4OPQovdDqe_LLEI76pDe2TnprkwPDxtD-ryI3E9pMuzagjrrjY"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":31},"prompt_tokens":{"type":"Integer","value":63},"total_tokens":{"type":"Integer","value":94}}},"updated_at":{"type":"DateTime","value":"2025-11-05T04:09:59.973339Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 13

=== Branching Session ===
✓ Created branched session: R5WqJkGrydijAeuMAy8OocrzBFF7fv3Q_jted-Jc-LbKtCp4HbFH1E7WEU4pxu6yaHJSbBNlK1ociri0hu42bQ

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 6s
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
✓ Created session: R9Qxh8uzAEv0GSQB8CzwAO7YhhVZHBEJ2UGmzO_d5cgKCVKXEglfc1GQA2FpZL9TT4quc__2hmYfz2ZgzwhltA

=== Sending Initial Message ===
✓ Message sent
  Responses: ["The product available based on the provided context is ekoDB. It is a high-performance database product priced at $99."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Based on the provided information, the available product is ekoDB. It is a high-performance database product priced at $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: pPYg3stJPKxuKzu-BFreMf2NnbxKY-lSFtZ-5pz-fOUZy2zzvZojH4CJiykXhBlNhWuIKWuWYZcFKPe1f5y2bA

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
✅ Kotlin client examples complete!
✅ All Kotlin integration tests complete!
✅ All integration tests complete!
```
