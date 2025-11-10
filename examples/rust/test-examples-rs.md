make test-examples-rust 🧪 [36mRunning Rust examples (direct
HTTP/WebSocket)...[0m Compiling ekodb_client v0.3.0
(/Users/tek/Development/ekoDB/ekodb-client/ekodb_client) Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.79s Running
`target/debug/examples/simple_crud` ✓ Authentication successful

=== Insert Document === Inserted: Object {"id":
String("NFMp9iSu_P6koq5puhb-wDtYYWTtMw3XbYnVXf-pYvPXkDp9LipSjFuU8ewhzzY038wZT1m_2_wmRyY1iYKptA")}

=== Find by ID === Found: Object {"active": Object {"type": String("Boolean"),
"value": Bool(true)}, "id":
String("NFMp9iSu_P6koq5puhb-wDtYYWTtMw3XbYnVXf-pYvPXkDp9LipSjFuU8ewhzzY038wZT1m_2_wmRyY1iYKptA"),
"name": Object {"type": String("String"), "value": String("Test Record")},
"value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query === Found documents: Array [Object {"active": Object
{"type": String("Boolean"), "value": Bool(true)}, "id":
String("NFMp9iSu_P6koq5puhb-wDtYYWTtMw3XbYnVXf-pYvPXkDp9LipSjFuU8ewhzzY038wZT1m_2_wmRyY1iYKptA"),
"name": Object {"type": String("String"), "value": String("Test Record")},
"value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document === Updated: Object {"active": Object {"type":
String("Boolean"), "value": Bool(true)}, "id":
String("NFMp9iSu_P6koq5puhb-wDtYYWTtMw3XbYnVXf-pYvPXkDp9LipSjFuU8ewhzzY038wZT1m_2_wmRyY1iYKptA"),
"name": Object {"type": String("String"), "value": String("Updated Record")},
"value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.47s Running
`target/debug/examples/simple_websocket` ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
"aRE3XVwlbTj5n7jpL8YazH5Sle1ShUD8o1xXCtJz3n3OMfAvG6SWCG1CUYNC0BVcjv19UXFhDXmpbPhqQ2iYgA"

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"aRE3XVwlbTj5n7jpL8YazH5Sle1ShUD8o1xXCtJz3n3OMfAvG6SWCG1CUYNC0BVcjv19UXFhDXmpbPhqQ2iYgA",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 1
record(s) via WebSocket

✓ WebSocket example completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.33s Running
`target/debug/examples/batch_operations` ✓ Authentication successful

=== Batch Insert === ✓ Batch inserted 5 records ✓ Verified: Found 5 total
records in collection

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === ✓ Batch updated 3 records ✓ Verified: Record updated with
status="active"

=== Batch Delete === ✓ Batch deleted 3 records ✓ Verified: Records successfully
deleted (not found)

✓ All batch operations completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.39s Running
`target/debug/examples/kv_operations` ✓ Authentication successful

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: Object {"userId": Number(123), "username":
String("john_doe")}

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: Object {"name": String("Product 1"),
"price": Number(29.99)} cache:product:2: Object {"name": String("Product 2"),
"price": Number(39.989999999999995)} cache:product:3: Object {"name":
String("Product 3"), "price": Number(49.989999999999995)}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.43s Running
`target/debug/examples/collection_management` ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
"zU0zv3fd_fWY0YGz3czlb5lYke2qBESmmieumtEjHyp1zIJpggPhxmuqGp1pEQwe1ekcijHOV0lAdPBgAgeLnQ"

=== List Collections === Total collections: 4 Sample collections:
["batch_users", "test_collection", "demo_collection", "websocket_test"]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s Running
`target/debug/examples/document_ttl` ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
"CdlBZIVojeyVDshwetZeudOezV0lfLDEFBy4-8VtIRot3tbpEEsPGWTP-0hCSC4VYM3UybA4C4LNWKBh-IRwgA"

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: null

=== Query Documents === ✓ Found 1 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

✓ All document TTL operations completed successfully Compiling ekodb-examples
v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.47s Running
`target/debug/examples/websocket_ttl` ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
String("rKTL9OPwvC-\_TNLgLfSYGPs5ompjO1TkwaLXz9-FBeclDPEcpOK0Ug8VQXK0fmFtoTKOVAspbunJZ4Jm1jmZlA")

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully ✅ [32mRust direct examples
complete![0m 🛠️ [36mBuilding client library...[0m cargo build -p ekodb_client
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s ✅
[32mClient build complete![0m 🧪 [36mRunning Rust client library examples...[0m
Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.88s Running
`target/debug/examples/client_simple_crud` ✓ Client created (token exchange
happens automatically)

=== Insert Document === Inserted: Record { fields: {"id":
String("UHkm71-gQilvxzI80u_mcGzHYms6HYRFrNsRoWRuxRVdANlYKUzPOX4hrD06m9rtDAR68uahS6j3R7NUSIU_0g")}
}

=== Find by ID === Found: Record { fields: {"active": Object({"type":
String("Boolean"), "value": Boolean(true)}), "id":
String("UHkm71-gQilvxzI80u_mcGzHYms6HYRFrNsRoWRuxRVdANlYKUzPOX4hrD06m9rtDAR68uahS6j3R7NUSIU_0g"),
"name": Object({"type": String("String"), "value": String("Test Record")}),
"value": Object({"value": Integer(42), "type": String("Integer")})} }

=== Find with Query === Found documents: [Record { fields: {"active":
Object({"type": String("Boolean"), "value": Boolean(true)}), "value":
Object({"type": String("Integer"), "value": Integer(42)}), "name":
Object({"value": String("Test Record"), "type": String("String")}), "id":
String("UHkm71-gQilvxzI80u_mcGzHYms6HYRFrNsRoWRuxRVdANlYKUzPOX4hrD06m9rtDAR68uahS6j3R7NUSIU_0g")}
}]

=== Update Document === Updated: Record { fields: {"value": Object({"value":
Integer(100), "type": String("Integer")}), "active": Object({"type":
String("Boolean"), "value": Boolean(true)}), "id":
String("UHkm71-gQilvxzI80u_mcGzHYms6HYRFrNsRoWRuxRVdANlYKUzPOX4hrD06m9rtDAR68uahS6j3R7NUSIU_0g"),
"name": Object({"type": String("String"), "value": String("Updated Record")})} }

=== Delete Document === Deleted document

=== Cleanup === ✓ Deleted collection

✓ All CRUD operations completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.72s Running
`target/debug/examples/client_batch_operations` ✓ Client created

=== Batch Insert (via multiple inserts) === ✓ Inserted 5 records ✓ Verified:
Found 5 total records in collection

=== Update Records === ✓ Updated 3 records

=== Delete Records === ✓ Deleted 3 records

=== Cleanup === ✓ Deleted collection

✓ All batch operations completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.59s Running
`target/debug/examples/client_kv_operations` ✓ Client created

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: Object {"userId": Number(123), "username":
String("john_doe")}

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: Object {"name": String("Product 1"),
"price": Number(29.99)} cache:product:2: Object {"name": String("Product 2"),
"price": Number(39.989999999999995)} cache:product:3: Object {"name":
String("Product 3"), "price": Number(49.989999999999995)}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.81s Running
`target/debug/examples/client_collection_management` ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
"ihroYbHhGupJkYygVqL7VKuMpweYrWWyD2C6l_4nsjelLXqho_51HNABATYW3uOvSmon6tcURlAJkMJH4rU4Ow"

=== List Collections === Total collections: 6 Sample collections:
["ws_ttl_test", "batch_users", "test_collection", "ttl_cache", "websocket_test"]

=== Count Documents === Document count: 1

=== Check Collection Exists === Collection exists: true

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.75s Running
`target/debug/examples/client_document_ttl` ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
"hvrZVUpTH0K04-xulND5RXrGvcIxZli01XiGgFcyEkwWd90J_ZPCahJtNZo6TRlkK_q3BaGYbEe234TbIZ2lkQ"

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
Some(String("Df3GoZp-nhOypN0DxrLQOnB7cMKrvpGTZxaohdK5i7cuj4Jyq0nTXdIvgZ9w_t0XMQO67x03maDs5htg1nqg-w"))

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.89s Running
`target/debug/examples/client_simple_websocket` ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
LQbis7gRCKsdbq6Id6ocJEYLukbZtwk5A9F_Mh1NKNi8K-usNVRZqhi350F_08tPQvdeyv_DxcRXdfPKPAn2Xw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 1.11s Running
`target/debug/examples/client_websocket_ttl` ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
Some(String("7oaOvGl9Wh1tD2WU7wyDo_ge-\_VAcYv1COLEabrOuT0dhJa_iF-YTkgLEvbwHJIAcbmAVNaN9vJrG8hG32H_Qw"))

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.80s Running
`target/debug/examples/client_query_builder` === ekoDB Query Builder Example ===

=== Inserting Sample Data === ✓ Inserted 5 users

=== Simple Equality Query === ✓ Found 3 active users

- Some(Object({"type": String("String"), "value": String("David")}))
- Some(Object({"type": String("String"), "value": String("Bob")}))
- Some(Object({"value": String("Alice"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) === ✓ Found 3 users in age range

- Some(Object({"value": String("Eve"), "type": String("String")}))
- Some(Object({"value": String("David"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("Bob")}))

=== IN Operator === ✓ Found 4 users with status active or pending

- Some(Object({"value": String("David"), "type": String("String")}))
- Some(Object({"value": String("Bob"), "type": String("String")}))
- Some(Object({"value": String("Eve"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("Alice")}))

=== NOT IN Operator === ✓ Found 4 users not inactive

=== String Pattern Matching === ✓ Found 5 users with @example.com email

=== Regex Query === ✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) === ✓ Found 2 users
matching all conditions

- Some(Object({"type": String("String"), "value": String("David")}))
- Some(Object({"type": String("String"), "value": String("Bob")}))

=== OR Query === ✓ Found 2 users with age < 28 OR age > 32

- Some(Object({"value": String("Charlie"), "type": String("String")}))
- Some(Object({"value": String("Alice"), "type": String("String")}))

=== Sorted Query (by score descending) === ✓ Top 3 users by score:

1. Some(Object({"type": String("String"), "value": String("Bob")}))
2. Some(Object({"type": String("String"), "value": String("David")}))
3. Some(Object({"type": String("String"), "value": String("Alice")}))

=== Pagination (page 2, size 2) === ✓ Page 2 results:

- Some(Object({"type": String("String"), "value": String("Charlie")}))
- Some(Object({"value": String("David"), "type": String("String")}))

=== NOT Operator === ✓ Found 4 users NOT inactive

=== Complex Chained Query === ✓ Found 3 users with all conditions

=== Cleanup === ✓ Deleted collection

✓ All query builder operations completed successfully Compiling ekodb-examples
v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.81s Running
`target/debug/examples/client_search` === ekoDB Search Example ===

=== Inserting Sample Documents === ✓ Inserted 5 sample documents

=== Basic Text Search === ✓ Found 3 results for 'programming'

1. Score: 19.8000 - Some(String("Rust Programming"))
2. Score: 6.6000 - Some(String("Python for Data Science"))
3. Score: 6.6000 - Some(String("JavaScript Web Development"))

=== Fuzzy Search === ✓ Found 3 results for 'progamming' (typo)

1. Score: 1.6500 - Some(String("Rust Programming"))
2. Score: 0.5500 - Some(String("Python for Data Science"))
3. Score: 0.5500 - Some(String("JavaScript Web Development"))

=== Field-Specific Search === ✓ Found 4 results in title/description

1. Score: 4.0000 Title: Some(String("Machine Learning Basics")) Matched:
   ["description", "title.value", "description.value", "title"]
2. Score: 2.0000 Title: Some(String("Python for Data Science")) Matched:
   ["description", "description.value"]
3. Score: 1.0000 Title: Some(String("Rust Programming")) Matched:
   ["description", "description.value"]
4. Score: 1.0000 Title: Some(String("Database Design")) Matched: ["description",
   "description.value"]

=== Weighted Search === ✓ Found 2 results with field weights

1. Score: 23.1000 - Some(String("Python for Data Science"))
2. Score: 3.3000 - Some(String("Machine Learning Basics"))

=== Advanced Search Options === ✓ Found 1 results with stemming

1. Score: 19.8000 - Some(String("Database Design"))

=== Search with Limit === ✓ Limited to 2 results (requested 2)

1. Score: 19.8000 - Some(String("Rust Programming"))
2. Score: 6.6000 - Some(String("Python for Data Science"))

=== Cleanup === ✓ Deleted collection

✓ All search operations completed successfully Execution time: 2ms Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.74s Running
`target/debug/examples/client_schema_management` === ekoDB Schema Management
Example ===

=== Creating Collection with Schema === ✓ Created collection
'schema_client_rust' with schema

=== Inserting Valid Documents === ✓ Inserted user 1:
Some(String("vSp0QRGKbP7QG0GTqHrMynlAiUoOw2eRem2Yn9f5Qrtzdb9UUKL1gcWIWU6T1ACyDN00PQJdy0dljGeFVZuppw"))
✓ Inserted user 2:
Some(String("ogg99yQdl87KXbJtIB1me4gU7U9PwC2qclT-qXRDEm9JKCwjc7aCvk8-TdFwaRV7st1ukQHpWdckEWPUoJ_HNg"))

=== Getting Schema === ✓ Schema for schema_client_rust:

- title: String (required)
- status: String
- age: Integer
- email: String (required)

=== Listing Collections === ✓ Total collections: 6 Sample: ["ws_ttl_test",
"schema_client_rust", "batch_users", "test_collection", "ttl_cache"]

=== Cleanup === ✓ Deleted collection

✓ All schema management operations completed successfully Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.80s Running
`target/debug/examples/client_joins` === ekoDB Rust Client - Join Operations
Example ===

=== Setting up sample data === ✓ Sample data created

=== Example 1: Single collection join (users with departments) === ✓ Found 2
users with department data

- Alice Johnson: Engineering
- Bob Smith: Sales

=== Example 2: Join with filtering === ✓ Found 1 users in Engineering

- Alice Johnson: Building A

=== Example 3: Join with user profiles === ✓ Found 2 users with profile data

- Alice Johnson: Senior Software Engineer
- Bob Smith: Sales Manager

=== Example 4: Join orders with user data === ✓ Found 2 completed orders

- Mouse ($25) by Alice Johnson
- Laptop ($1200) by Alice Johnson

=== Example 5: Complex join with multiple conditions === ✓ Found 2 users with
example.com emails

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✓ Deleted test collections

✓ Join operations example completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.73s Running
`target/debug/examples/client_chat_basic` === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
5UAdpVm_csoCxOJQxiqdq5hVSVqlgdZq5q3paht_qgWqQJ6DX7lDB_nEP0ehvl1DBU5MEeStrS_FL7mGVCCI_w

=== Sending Chat Message === Message ID:
h2JeqtRTAahM3_a02H34rRT18FBIvKaUNMZPX85J7zN2ic1uXuUKXZ7h9NcjJtp3xRAACczx11RpeHNr_3XhlA

=== AI Response === Response 1: ekoDB is a high-performance database with
intelligent caching and real-time capabilities. It is designed with AI
integration which allows it to offer features such as AI Chat Integration and
advanced search features. The AI Chat Integration feature enables you to query
your database using natural language and receive AI-driven responses that hold
relevant context. Moreover, ekoDB supports various search features like
full-text search, vector search, and hybrid search with automatic context
retrieval.

=== Context Used (3 snippets) ===

Snippet 1: Collection: client_chat_basic_rust Score: 0.6333 Matched Fields:
["title", "category", "content"] Record: Object {"category": Object {"type":
String("String"), "value": String("features")}, "content": Object {"type":
String("String"), "value": String("The chat feature allows you to query your
database using natural language and get AI-powered responses with relevant
context.")}, "id":
String("u-CNHBJSTME7474YxU7o_WWzjNYlT8ghjEygtNgFJVQKHBJAqQ0rlgkVmxyfp-xNy7TTQL3ptgZ3jR_9fHUZlg"),
"title": Object {"type": String("String"), "value": String("AI Chat
Integration")}}

Snippet 2: Collection: client_chat_basic_rust Score: 0.5222 Matched Fields:
["title", "content"] Record: Object {"category": Object {"type":
String("String"), "value": String("documentation")}, "content": Object {"type":
String("String"), "value": String("ekoDB is a high-performance database with
intelligent caching, real-time capabilities, and AI integration.")}, "id":
String("dXs6aCs2iFnZSvbEu6QC4Ve2Sufu8gDAw-YpWA6H4J1bPemGUOOeQjWKMBS02AsbQ7w-uG9e2KJHJ8_3Jl2sjA"),
"title": Object {"type": String("String"), "value": String("Introduction to
ekoDB")}}

Snippet 3: Collection: client_chat_basic_rust Score: 0.5222 Matched Fields:
["title", "content", "category"] Record: Object {"category": Object {"type":
String("String"), "value": String("features")}, "content": Object {"type":
String("String"), "value": String("ekoDB supports full-text search, vector
search, and hybrid search with automatic context retrieval.")}, "id":
String("eeEjZpE8PSZrVCBsxjwxmdwbXSB8qCTS7qylSWXUcNqAqyhkSv1bilupPWkht5WuU5D66RaIgJxM8NfV63VwvA"),
"title": Object {"type": String("String"), "value": String("Search Features")}}

Execution Time: 2984ms

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.86s Running
`target/debug/examples/client_chat_sessions` === ekoDB Chat Session Management
Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
mwJq0JqojQdSve6Yo19pdIZeOdkM4CH0MRXJ04YUrrjq0XCKgvyYyXWU598PsuBif8g5mBc5HttM7WOBlMxywA
=== Sending Messages === ✓ Message 1 sent Response: Based on the provided
information, the available product is ekoDB, which is a high-performance
database product with AI capabilities. It is priced at $99.

✓ Message 2 sent Response: I'm sorry, but there's no information provided about
the product or its price in the context above. Could you please specify the
product you are asking about?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
o6SCBT_n03qa6kIbjvBAqtuisoEekFO4xA_pea7xVSkkD6n-e05eJr005sGJTm4VrqPEIu7vwL2l9EgXKVyYJA
Parent:
mwJq0JqojQdSve6Yo19pdIZeOdkM4CH0MRXJ04YUrrjq0XCKgvyYyXWU598PsuBif8g5mBc5HttM7WOBlMxywA

=== Listing Sessions === ✓ Found 3 sessions Session 1:
o6SCBT_n03qa6kIbjvBAqtuisoEekFO4xA_pea7xVSkkD6n-e05eJr005sGJTm4VrqPEIu7vwL2l9EgXKVyYJA
(Untitled) Session 2:
mwJq0JqojQdSve6Yo19pdIZeOdkM4CH0MRXJ04YUrrjq0XCKgvyYyXWU598PsuBif8g5mBc5HttM7WOBlMxywA
(Untitled) Session 3:
5UAdpVm_csoCxOJQxiqdq5hVSVqlgdZq5q3paht_qgWqQJ6DX7lDB_nEP0ehvl1DBU5MEeStrS_FL7mGVCCI_w
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
o6SCBT_n03qa6kIbjvBAqtuisoEekFO4xA_pea7xVSkkD6n-e05eJr005sGJTm4VrqPEIu7vwL2l9EgXKVyYJA

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.78s Running
`target/debug/examples/client_chat_advanced` === ekoDB Advanced Chat Features
Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
v3eoCmgjmJCV2CsUOYofjvZcEx51slCd04k0X500HY2ZnxsM2ofJUkk8ZfFUPsD3NwElN2ptycy41T1P3n2oCg

=== Sending Initial Message === ✓ Message sent Response: Based on the context
provided, the available product is a high-performance database product named
ekoDB. It is priced at $99.

✓ Second message sent === Feature 1: Regenerate AI Response === ✓ Message
regenerated New response: I'm sorry, but the context provided does not include
information about the price of ekoDB. I recommend checking the official website
or contacting the sales department of ekoDB for the most accurate and current
pricing information.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
SCAx_XbQF_b-0GA8DV0bvD7IiWmVtRNnI8iySeWi8uy956NxdqKfiXXD9TjJ40k2E0xG4N60M_gjtLGOZHDN3Q
✓ Sent message in second session ✓ Sessions merged successfully Total messages
in merged session: 4

=== Feature 5: Delete Message === ✓ Message deleted

✓ Messages remaining: 3

=== Cleanup === ✓ Deleted session ✓ Deleted collection

✓ All advanced chat features demonstrated successfully! Compiling ekodb-examples
v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.85s Running
`target/debug/examples/client_functions` === ekoDB Rust Client - Scripts Example
===

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
4I6m3ND_nN1u38z0kZdw7t14cDKW_jiWElYYSirP_EdV2_ZTTL5oHionZaHN37nrdGpVvWxqCZMI8hqIQvVA7A
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved:
TC_Vk1ep7tRdumKR7w5B4knj-jfR3LSD_zK3bFXkw3Y4VPHoV2qAfNlRZAiwz91euYqiXITEGBKcTig0Lgmk0Q
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
J7bvtAp159V2L3wY2CABjx_z_gPpowcBvQYdaEYYDuIAElQJ3IiQ3rEcTHPx2D5QhjCfVq7SFeQ_QNEMwUjXkA
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 3 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! ✅ [32mRust client examples complete![0m
