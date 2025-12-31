make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("BE5LW4xt_MY6ttSQq8O3bIZXLu7YethdCiQe8aYIKVO86GvU0lnwagTMGU0GlhWjeV9wzLGf1PFExBp-BoIfYQ")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("BE5LW4xt_MY6ttSQq8O3bIZXLu7YethdCiQe8aYIKVO86GvU0lnwagTMGU0GlhWjeV9wzLGf1PFExBp-BoIfYQ"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("BE5LW4xt_MY6ttSQq8O3bIZXLu7YethdCiQe8aYIKVO86GvU0lnwagTMGU0GlhWjeV9wzLGf1PFExBp-BoIfYQ"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("BE5LW4xt_MY6ttSQq8O3bIZXLu7YethdCiQe8aYIKVO86GvU0lnwagTMGU0GlhWjeV9wzLGf1PFExBp-BoIfYQ"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "fGqA_U1JCaXbYlEiFASIABYGZ-Ckhe9tGhJXqSIoaSMrfQnRLfvqHABUQDbp2Vb2nk6WYTT1jQOIKE8UhS3SGQ"

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
        "id": "fGqA_U1JCaXbYlEiFASIABYGZ-Ckhe9tGhJXqSIoaSMrfQnRLfvqHABUQDbp2Vb2nk6WYTT1jQOIKE8UhS3SGQ",
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
Collection created with first record: "mELODJy5QM0Sgl3-cO9IeIx4ZTjnUMQJOnevW4UbwZRMvExwmcljGUErwq6SYQDNJ-9V0hc_CojEmxXpZ7mxOg"

=== List Collections ===
Total collections: 4
Sample collections: ["websocket_test", "test_collection", "demo_collection", "batch_users"]

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
✓ Inserted document: "uZSslftNADdNNPoV5gRCAANvYhZyJsIxGIF4iMHwc0Vf-fq_lUyG45gdoFpO3rXBO3n5XmtfQcdqNVCk9mrf9A"

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
✓ Inserted document with TTL: String("Lr21rQWHKxrXXDtLD14AyTvjrOmIdxXcYz9tOl-dUbvszElS1Eyr_bWrPMEj6nbCwMfaQF6---0DW_ksu1fNvQ")

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

✅ Script saved: zzyOwGG1Xpe7TT3iG6j1QSlIb_NLKzvKcMSb7uc6JFE0nsst79n99zyAel_ViVRmdBqa_c3ntD5TWxzysi-3nQ
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "bHJ3psbUHPc5oOj8Qo4oG--ORTbNifND5ph8xDHI9rA2yLQS-d40QHa9RUmMtC7-pC7Xr303E7LU_1KEHOt_fA"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: PL8K60KmZZZJlTQVgEe3kC9W5qS_9TUM-GLj3AsQD0VVIlTc3Csz2qnPdnYAkEZluDB96HvsVoGjbAka1Rcz5w
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 6cQUcQOzDFZYc5tVYibPmWypMU1vnKMYkB2Z-nxu3F2N1wcrNRpqO94kCjhWJaHLA_xkcQ7vKuVfZcekjPAOhg
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
Created Alice: $1000 - ID: 9R36PyqSzI-p5q4y114aZrdpVPr2FXT1taD9aUSQYJDUFRGiFO8kOHvuZlaYYyp18i2wxFsAZKAFn2eQYUPRHg
Created Bob: $500 - ID: 77GmdKHCJXdVa2QREJyZKuTJ8teeV8iyMMnCKLnIumBSMxPpGcXvTyEXreM9Cedbehv9Q6OEI9E9oeGi1VqgKQ

=== Example 1: Begin Transaction ===
Transaction ID: c7806726-5946-491f-bb77-a3a6ea848244

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
New transaction: 380e2dee-424f-4ff7-a610-a8280e7413ec
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record { fields: {"id": String("RXDrkwddTCODc1IeUfedXccN2dlNZN3JIYlw6JOhxF6srdCmunSb_-dWZwoGGOzHp4PupFcax7bsziw_Qar7xQ")} }

=== Find by ID ===
Found: Record { fields: {"name": Object({"type": String("String"), "value": String("Test Record")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "id": String("RXDrkwddTCODc1IeUfedXccN2dlNZN3JIYlw6JOhxF6srdCmunSb_-dWZwoGGOzHp4PupFcax7bsziw_Qar7xQ"), "value": Object({"type": String("Integer"), "value": Integer(42)})} }

=== Find with Query ===
Found documents: [Record { fields: {"active": Object({"value": Boolean(true), "type": String("Boolean")}), "id": String("RXDrkwddTCODc1IeUfedXccN2dlNZN3JIYlw6JOhxF6srdCmunSb_-dWZwoGGOzHp4PupFcax7bsziw_Qar7xQ"), "value": Object({"type": String("Integer"), "value": Integer(42)}), "name": Object({"type": String("String"), "value": String("Test Record")})} }]

=== Update Document ===
Updated: Record { fields: {"name": Object({"value": String("Updated Record"), "type": String("String")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "value": Object({"value": Integer(100), "type": String("Integer")}), "id": String("RXDrkwddTCODc1IeUfedXccN2dlNZN3JIYlw6JOhxF6srdCmunSb_-dWZwoGGOzHp4PupFcax7bsziw_Qar7xQ")} }

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "iY_4Ud8qoe5qh3fCCT1YZFdpfAR6ElkqqINFYpUwABaQANLtz59e-iSp9Eh6OR_307rNggBUnBFCjoPjMuHYKQ"

=== List Collections ===
Total collections: 9
Sample collections: ["websocket_test", "ttl_cache", "scripts__ek0_testing", "users", "client_collection_management_rust"]

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
✓ Inserted document: "tTJK-QARo3_jIXTJIsp0nhIHcK97aPHSmxziMBmhLgSBbTtnNx4y-s0Ptb29EOZqTksRlzi-E9rAu73_RonMdQ"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("XXhHm6TwS1uwzuoadAOcLmmfs1xt4H-o0khqxGhR5beRWAAkW87njGIms0Qp8tATwEfIoTsKUPVBpG6iPvd6eg"))

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
✓ Inserted test record: WNoy3IBzN5dHdljlLScnFTwHYRT6IlCXmZ-BuoXngioj-b105rNlenlNpzRHq_bqbWg-NYBQqwkWo96uH8abXQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("yUBE96AyZZ2A8t8KyfHIll9HsjJBe4I3RMSYsxR9R-CCwn20kM35S3pD2kOwLl_4CLUO5P6B--QTuCjaxyrd8A"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

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
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Regex Query ===
✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
  2. Some(Object({"type": String("String"), "value": String("David")}))
  3. Some(Object({"value": String("Alice"), "type": String("String")}))

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
     Title: Some(String("Database Design"))
     Matched: ["description.value", "description"]
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
Execution time: 2ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_schema_management`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("fBPT5mtvVsJ3j0u-2FqwM_0hnOvJ8UDHjjBjWjxa1zpyUWo06xbg7TDFW-NSROIM1nBZFx9MedTnLyJKGjum8Q"))
✓ Inserted user 2: Some(String("swjVqrRO40P8-WdfPfF3HAQfwAsVUeoQ2gwhN5EXzK1FKIWnD55fyIbYsJQhIS-I1rETPJYJ9LT2oOxVC9WkpA"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - status: String
  - age: Integer
  - email: String
    (required)
  - title: String
    (required)

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["websocket_test", "schema_client_rust", "ttl_cache", "scripts__ek0_testing", "users"]

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
✓ Created session: HWO_Ih0OA303BqPhCgWr6vgXp3n0ydpNFOkFouLtHWO6i67yp0lhJOAGkd3BR00nWokmyL7toIpPhMwWY0WFsA

=== Sending Chat Message ===
Message ID: HKz-TOs0JZR7bQglgqWH2gSsdCe3HU-aM5LKz8U3Rv1YRcwEnd2QvSHEWQ6eG3yWnAqGyB1uqViwzggSWIw0Mg

=== AI Response ===
Response 1: ekoDB is a high-performance database that offers intelligent caching, real-time capabilities, and AI integration. One of its notable features is the AI Chat Integration. This feature enables you to query your database using natural language and receive AI-powered responses with relevant context. In addition, ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["title", "content", "category"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("K1J-2gseRTKXZx8-Y8lPEVaSA7EoDZ0GBKMUls2dT0hsKRh1ud8UdJnrR2a6tjrUd51DYzo_Zns05M78fTPV-g"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("-G-VoGLFeg6pNRRMsTANvg2iMQoAPtd1TGFj_GX-n8Hv5sODTJ89TCn_aQ49PRXENxtVsDqE6rKZh0gWI3rt_A"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "category", "title"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("cx3uMl5KJiqVw_rUldvotzjGW6WR8slSrVmVL8JqSXRVpo7us06lIUc8b7C1r8Q8b-A_sYXei-Qukhg3M7a9cg"), "title": String("Search Features")}

Execution Time: 3639ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: JI9ALeP9q7XIpzqiQkD8s_hsGsDcCR2eUhCkRH_5XNItOPrTyc0CeMrkI485fLp7sANlcLMTZeGqjhUlhuyHUw
=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is called ekoDB. It's a high-performance database product with AI capabilities. The price for this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: nZ_a9sOPtVTmctloK9S6p9GELh4sWfMeaXlMPvJka0Q4y8n2BNihi4yS4lWCNGfT0CLWnG62sRjeGi8ZZjsGDg
  Parent: JI9ALeP9q7XIpzqiQkD8s_hsGsDcCR2eUhCkRH_5XNItOPrTyc0CeMrkI485fLp7sANlcLMTZeGqjhUlhuyHUw

=== Listing Sessions ===
✓ Found 3 sessions
  Session 1: nZ_a9sOPtVTmctloK9S6p9GELh4sWfMeaXlMPvJka0Q4y8n2BNihi4yS4lWCNGfT0CLWnG62sRjeGi8ZZjsGDg (Untitled)
  Session 2: JI9ALeP9q7XIpzqiQkD8s_hsGsDcCR2eUhCkRH_5XNItOPrTyc0CeMrkI485fLp7sANlcLMTZeGqjhUlhuyHUw (Untitled)
  Session 3: HWO_Ih0OA303BqPhCgWr6vgXp3n0ydpNFOkFouLtHWO6i67yp0lhJOAGkd3BR00nWokmyL7toIpPhMwWY0WFsA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: nZ_a9sOPtVTmctloK9S6p9GELh4sWfMeaXlMPvJka0Q4y8n2BNihi4yS4lWCNGfT0CLWnG62sRjeGi8ZZjsGDg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Gu1j2Tc-IIp4pe26C6pXsVFBbnTw-JmA72g1oaBnfTcjApklLsfPHyQ7HT6DRIEVzsDnB6c7ESLCPJq9onjB4w

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, the available product is ekoDB. It is a high-performance database product.

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
✓ Created second session: FCexZoTUlOX6KTMmjH8vgzqs3v-JG_bq367FxD8kJTuYbPFS0SQ--AfP1uiO3tCjYL3IFjfW2XJv984i2JMbig
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

✅ Script saved: ECJCHT3xyhXxRd_qe7u6Xxvf7-t3zehReKLFPtsNGpsE2bGI4kJpmg4I-v_hcifq8e6rNMYyLlPpoJel_xmlyw
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: 1MaDeRcjd33pOXQRJkmFTPFb1KdIzwb05eipLjXwFCfolFeK9J6TfkypxZ6XXNc_benEYmsXQF_z7Q_AdtaDTw
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: zbWqlDM0zEFY0Wbx6BCy4HBfmd-mPp5lpOR6ZK658T5Htaavd6QKRbMdOKxTvpezJ3Yrek_wBz0foJm5Yi9jiA
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 6
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
✅ [32mRust client examples complete![0m
