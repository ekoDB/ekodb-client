make test-examples 🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
Compiling ekodb_client v0.3.0
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

✅ All examples completed! ✅ [32mRust client examples complete![0m ✅ [32mAll
Rust integration tests complete![0m 🧪 [36mRunning Python examples (direct
HTTP/WebSocket)...[0m ✓ Authentication successful

=== Insert Document === Inserted: {'id':
'zT8v5xzyqPEJBqaCpao4u37UTs1Y4ENEswI238pmsZivhUtCm9HnFRzYA1BWmNd-3prKmSfRLuVrwNfIYqgEhw'}

=== Find by ID === Found: {'value': {'type': 'Integer', 'value': 42}, 'active':
{'value': True, 'type': 'Boolean'}, 'name': {'value': 'Test Record', 'type':
'String'}, 'id':
'zT8v5xzyqPEJBqaCpao4u37UTs1Y4ENEswI238pmsZivhUtCm9HnFRzYA1BWmNd-3prKmSfRLuVrwNfIYqgEhw'}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'name': {'value': 'Updated Record', 'type':
'String'}, 'active': {'value': True, 'type': 'Boolean'}, 'id':
'zT8v5xzyqPEJBqaCpao4u37UTs1Y4ENEswI238pmsZivhUtCm9HnFRzYA1BWmNd-3prKmSfRLuVrwNfIYqgEhw',
'value': {'value': 100, 'type': 'Integer'}}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
PhhdxEYbQszJEMgXIonEwzmyOwVtvGc1Yocd1cTDxLWhJrpoLdEWJOyKQ864_IEWJUlyIlVlbZH6TB-II9J5bQ

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"aRE3XVwlbTj5n7jpL8YazH5Sle1ShUD8o1xXCtJz3n3OMfAvG6SWCG1CUYNC0BVcjv19UXFhDXmpbPhqQ2iYgA",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"PhhdxEYbQszJEMgXIonEwzmyOwVtvGc1Yocd1cTDxLWhJrpoLdEWJOyKQ864_IEWJUlyIlVlbZH6TB-II9J5bQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 2
record(s) via WebSocket

✓ WebSocket example completed successfully ✓ Authentication successful

=== Batch Insert === ✓ Batch inserted 5 records ✓ Verified: Found 10 total
records in collection

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === ✓ Batch updated 3 records ✓ Verified: Record updated with
status="None"

=== Batch Delete === ✓ Batch deleted 3 records ✓ Verified: Records successfully
deleted (not found)

✓ All batch operations completed successfully ✓ Authentication successful

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: {'username': 'john_doe', 'userId': 123}

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: {'name': 'Product 1', 'price': 29.99}
cache:product:2: {'name': 'Product 2', 'price': 39.989999999999995}
cache:product:3: {'name': 'Product 3', 'price': 49.989999999999995}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
rdzlEhcDkXNzF97wnbKkEI4B3vG5hh8cc*NedXO*-p12DzBHA8MjA1f0JnPK1XI9P2jJ_QLYaZ_b1UYahhjQhA

=== List Collections === Total collections: 10 Sample collections:
['ws_ttl_test', 'batch_users', 'test_collection', 'chat_messages__ek0_bench',
'ttl_cache']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Authentication
successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
ULyxLR-y2KBjhZy0oiBgEKit42qQoMiNwX9tAHVApPLUw_36R6pX3fQZe68hqYWyITjTS5QJPwl7pe18ptRNcQ

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
TlpRywEVJnBddQrJpYG8FaYA-YYrlXyQl72b_ERunbjxBLB5yhTKiXs0eul3PiiFOfm42Bp2z4CVn6itBkM9yQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 2 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [34m
╔════════════════════════════════════════╗[0m [34m║ ekoDB Python Examples Test
Suite ║[0m [34m╚════════════════════════════════════════╝[0m [34m === Checking
Server Connection ===[0m [32m✓ Server is ready[0m [34m === Getting
Authentication Token ===[0m [32m✓ Authentication successful[0m [33m === Running
7 Examples ===[0m [34m === Running simple_crud.py ===[0m [32m✓ simple_crud.py
completed successfully[0m [34m === Running simple_websocket.py ===[0m [32m✓
simple_websocket.py completed successfully[0m [34m === Running
batch_operations.py ===[0m [32m✓ batch_operations.py completed successfully[0m
[34m === Running kv_operations.py ===[0m [32m✓ kv_operations.py completed
successfully[0m [34m === Running collection_management.py ===[0m [32m✓
collection_management.py completed successfully[0m [34m === Running
document_ttl.py ===[0m [32m✓ document_ttl.py completed successfully[0m [34m ===
Running websocket_ttl.py ===[0m [32m✓ websocket_ttl.py completed successfully[0m
[34m ╔════════════════════════════════════════╗[0m [34m║ Test Summary ║[0m
[34m╚════════════════════════════════════════╝[0m [34mTotal: 7[0m [32mPassed:
7[0m [32mFailed: 0[0m ✅ [32mPython direct examples complete![0m 🐍 [36mBuilding
Python client package...[0m 🍹 Building a mixed python/rust project 🔗 Found
pyo3 bindings with abi3 support 🐍 Not using a specific python interpreter 💻
Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
Finished `release` profile [optimized] target(s) in 0.15s 📦 Built wheel for
abi3 Python ≥ 3.8 to
/Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.3.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m Processing
./ekodb-client-py/target/wheels/ekodb_client-0.3.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client Attempting uninstall: ekodb-client
Found existing installation: ekodb_client 0.3.0 Uninstalling ekodb_client-0.3.0:
Successfully uninstalled ekodb_client-0.3.0 Successfully installed
ekodb-client-0.3.0 ✅ [32mPython client package built and installed![0m 🧪
[36mRunning Python client library examples...[0m ✓ Client created (token
exchange happens automatically)

=== Insert Document === Inserted: {'id':
'Hk93chYxPBAlSk6FPX8TWs9zgUcfqpUWzQh9KA9H6_dlmM9oFeg34qLdD0PWeOww00YHm7eTYsBcu7V_WA0QJw'}

=== Find by ID === Found: {'value': {'value': 42, 'type': 'Integer'}, 'name':
{'type': 'String', 'value': 'Test Record'}, 'id':
'Hk93chYxPBAlSk6FPX8TWs9zgUcfqpUWzQh9KA9H6_dlmM9oFeg34qLdD0PWeOww00YHm7eTYsBcu7V_WA0QJw',
'active': {'type': 'Boolean', 'value': True}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'active': {'value': True, 'type': 'Boolean'},
'value': {'value': 100, 'type': 'Integer'}, 'name': {'value': 'Updated Record',
'type': 'String'}, 'id':
'Hk93chYxPBAlSk6FPX8TWs9zgUcfqpUWzQh9KA9H6_dlmM9oFeg34qLdD0PWeOww00YHm7eTYsBcu7V_WA0QJw'}

=== Delete Document === Deleted document

=== Cleanup === ✓ Deleted collection

✓ All CRUD operations completed successfully ✓ Client created

=== Batch Insert === ✓ Batch inserted 5 records ✓ Verified: Found 5 total
records in collection

=== Batch Update === ✓ Batch updated 3 records

=== Batch Delete === ✓ Batch deleted 3 records

=== Cleanup === ✓ Deleted collection

✓ All batch operations completed successfully ✓ Client created

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: {'value':
'{"userId":123,"username":"john_doe"}'}

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: {'value': '{"name":"Product
1","price":29.99}'} cache:product:2: {'value': '{"name":"Product
2","price":39.989999999999995}'} cache:product:3: {'value': '{"name":"Product
3","price":49.989999999999995}'}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
"AGYBzxThU_p_TWg3ZE3GSPPWIUtAQ8ws8RwGV4Fh9XMRN3KiaF75v6WkpQ43CGe-iCIhTUvTLqo7auU25ROgOw"

=== List Collections === Total collections: 10 Sample collections:
['ws_ttl_test', 'batch_users', 'test_collection', 'chat_messages__ek0_bench',
'ttl_cache']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
oVbjT-AHPNNvtjh7WxQcGEEWGS2dIkETNwHRfC3XdzLjJoRTqBwJBMR24GJtBTbqqzkbaSXhw8zWpHucYRKaJA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
8ltuVMIAKJZ8NTsZhdiaH15iBB10NrpKnZsoIic-27qQkp8XWAYwNzl7Ho-p9hvnCF3LFOMFy5XTdlrDhiwKwg

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
NPtMovMMrWKB7mzaulDV-EB0Sjtp9Hyl4OmAmjX33BeRzN9OgGYfQC0uORNqBRe0I_uwxKJa1WNB6RQ-M1cjfA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
OulqWOsU3C-6YkyaYWX026qLx6nZLBGcy5pICRvWFv-\_h3mPUNmH2aCEc7GME9vUkoq2I98TY_8CRUMJgq1hyw

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === Query Builder Examples ===

Setting up test data... ✅ Test data created

1. Simple equality query: Found 2 active users

2. Range query with sorting: Found 3 users aged 18-65

3. String operations: Found 2 users with @example.com emails

4. IN operator: Found 2 privileged users

5. Complex query with multiple conditions: Found 1 active US users over 21

6. Pagination: Page 1: 2 users

7. NOT IN operator: Found 2 valid users

8. Using bypass flags: Found 2 users (bypassed cache)

=== Cleanup === ✅ Deleted test collection

✅ Query Builder examples completed! === Search Examples ===

Setting up test data... ✅ Test data created

1. Basic full-text search: Found 2 results
1. Score: 25.740, Matched: name.value, email, name, email.value
1. Score: 12.540, Matched: name, name.value

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: title, bio, title.value, bio.value
1. Score: 26.400, Matched: title, title.value, bio, bio.value
1. Score: 26.400, Matched: title, bio.value, bio, title.value
1. Score: 26.400, Matched: bio.value, bio, title.value, title

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: bio, bio.value, title, title.value
1. Score: 39.600, Matched: title, title.value, bio.value, bio
1. Score: 39.600, Matched: title.value, title, bio, bio.value
1. Score: 39.600, Matched: bio, bio.value, title.value, title

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio, bio.value
1. Score: 13.200, Matched: bio.value, bio

1. Search with stemming and exact match boosting: Found 2 results (matches: run,
   running, runs)
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio.value, bio

1. Vector search (semantic search): Found 0 semantically similar documents

1. Hybrid search (text + vector): Found 2 results using hybrid search (text +
   vector)
1. Score: 1.200, Matched: content, title, title.value, content.value
1. Score: 0.600, Matched: title.value, title, content.value, content

1. Case-sensitive search: Found 1 results (case-sensitive)
1. Score: 13.200, Matched: title, title.value

=== Cleanup === ✅ Deleted test collections

✅ Search examples completed! === Schema Management Examples ===

1. Creating user schema with basic fields: ✅ User schema created

2. Creating product schema with text index: ✅ Product schema with indexes
   created

3. Creating document schema with vector index: ✅ Document schema with vector
   index created

4. Retrieving collection schema: Schema fields: ['age', 'email', 'name',
   'status'] Schema version: 1

5. Retrieving collection metadata: Collection has 4 fields

6. Creating employee schema with all constraint types: ✅ Employee schema with
   all constraints created

✅ Schema management examples completed! === Join Operations Examples ===

Setting up sample data... ✅ Sample data created

1. Single collection join (users with departments): Found 2 users with
   department data:

- Alice Johnson: Engineering
- Bob Smith: Sales

2. Join with filtering: Found 1 users in Engineering:

- Alice Johnson: Building A

3. Join with user profiles: Found 2 users with profile data:

- Alice Johnson: Senior Software Engineer
- Bob Smith: Sales Manager

4. Join orders with user data: Found 2 completed orders:

- Mouse ($25) by Alice Johnson
- Laptop ($1200) by Alice Johnson

5. Complex join with multiple conditions: Found 2 users with example.com emails:

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✅ Deleted test collections

✅ Join operations examples completed! === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
06JVJeje00VfyFekc9GCgSS8QpyxR8sK66prVhP6gAv2K7ckAfjRtW-bACzxAnBwbgqh6MK9wxo7TFbN7sHeMw

=== Sending Chat Message === Message ID:
1MQbeyZkxuyX5OgPwc_wqhIKpZviBAammVlIgSMuIVmX1f9l5UDb_K71MTpl2BDepG_uookCzkhqvdWsYTxKAg

=== AI Response === There are three products available:

1. ekoDB Cloud: This is a fully managed cloud database service product. It is
   priced at $499.

2. ekoDB Pro: This is an enterprise edition product with advanced features. It
   is priced at $299.

3. ekoDB: This is a high-performance database product with AI capabilities. It
   is priced at $99.

Execution Time: 3241ms

=== Token Usage === Prompt tokens: 369 Completion tokens: 78 Total tokens: 447

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
sfm7SuCU_7I60z0VFUIdufcM4PTTusaCwHVTUjlfO03ayFDkrlNa0-3aQx99iAqXzQotv1ywUpfN073Gq_6RvA

=== Sending Messages === ✓ Message 1 sent Response: Based on the provided
context, the available product is a high-performance database product named
ekoDB. It is priced at $99.

✓ Message 2 sent Response: As an assistant, I'm sorry, but there's no context
provided above regarding a specific product. Could you please provide more
information or specify the product you're asking about?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
NeTq7n0Y-Lqk3Eb-eWuffPPciZ8AdXGSNPbe0HbEqj7hy1D-PM_kpFyW3bIEtZr6IzmT21yIcvpNX0lORLHofg
Parent:
sfm7SuCU_7I60z0VFUIdufcM4PTTusaCwHVTUjlfO03ayFDkrlNa0-3aQx99iAqXzQotv1ywUpfN073Gq_6RvA

=== Listing Sessions === ✓ Found 6 sessions Session 1:
NeTq7n0Y-Lqk3Eb-eWuffPPciZ8AdXGSNPbe0HbEqj7hy1D-PM_kpFyW3bIEtZr6IzmT21yIcvpNX0lORLHofg
(Untitled) Session 2:
sfm7SuCU_7I60z0VFUIdufcM4PTTusaCwHVTUjlfO03ayFDkrlNa0-3aQx99iAqXzQotv1ywUpfN073Gq_6RvA
(Untitled) Session 3:
06JVJeje00VfyFekc9GCgSS8QpyxR8sK66prVhP6gAv2K7ckAfjRtW-bACzxAnBwbgqh6MK9wxo7TFbN7sHeMw
(Untitled) Session 4:
SCAx_XbQF_b-0GA8DV0bvD7IiWmVtRNnI8iySeWi8uy956NxdqKfiXXD9TjJ40k2E0xG4N60M_gjtLGOZHDN3Q
(Untitled) Session 5:
mwJq0JqojQdSve6Yo19pdIZeOdkM4CH0MRXJ04YUrrjq0XCKgvyYyXWU598PsuBif8g5mBc5HttM7WOBlMxywA
(Untitled) Session 6:
5UAdpVm_csoCxOJQxiqdq5hVSVqlgdZq5q3paht_qgWqQJ6DX7lDB_nEP0ehvl1DBU5MEeStrS_FL7mGVCCI_w
(Untitled)

=== Deleting Branch Session === ✓ Deleted branch session:
NeTq7n0Y-Lqk3Eb-eWuffPPciZ8AdXGSNPbe0HbEqj7hy1D-PM_kpFyW3bIEtZr6IzmT21yIcvpNX0lORLHofg

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
8X7U3Wq7XO40iFIvLATr5G6JuLPsDM18cjKI8vzpslW3BSTk-uTFyc9uJJmo8ToaMa7qeM7BPAwzzQBFns27Kw

=== Sending Initial Message === ✓ Message sent Response: Based on the context
provided, the available product is a high-performance database product named
ekoDB. It is priced at $99.

✓ Second message sent

Debug: Found 4 messages Debug: First message keys: dict_keys(['content',
'chat_id', 'id', 'token_usage', 'updated_at', 'role', 'created_at']) Debug:
First message role: {'value': 'assistant', 'type': 'String'} === Feature 1:
Regenerate AI Response === ✓ Message regenerated New response: I'm sorry, but
the context provided does not include information about the price of ekoDB. I
recommend checking the product's official website or contacting the seller for
the most accurate and up-to-date pricing information.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
JTaC0beEkTbQdlCZePnWpSgtzxgkeSFO6MBawhoKUreXdifOnFbcRmCZ7m-9shlCBFXP2NkUL1SMmviUhEGXKA
✓ Sent message in second session ✓ Sessions merged successfully Total messages
in merged session: 4

=== Feature 5: Delete Message === ✓ Message deleted

✓ Messages remaining: 3

=== Cleanup === ✓ Deleted session ✓ Deleted collection

✓ All advanced chat features demonstrated successfully! 🚀 ekoDB Scripts Example
(Python)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
KqZE8baWe3sXES47JbDn7fBbQHj3CUm7U7wIWu_BZYTn-\_jmXu0rNatsElppto2CKOibRr-y_octMd0GHWVfGQ
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved:
pqv-QCbvTr-AOJhNCKF8HjS3DTMCNigfl12K59fFdHxYMe3FCzfuQQYTSmuJafa_cT5KzXKlHBSThJzdtWidEw
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
fxN8APrp5_oao3fTHcC_tUJe6pwq7v5N1FRQo_OmPosdGwpdbJATCzoHv2xcUVrbnCcD5jHWSuiOF819a_xB5w
📊 Statistics: 2 groups {'avg_score': 60.0, 'count': 10, 'status': 'active'}

{'avg_score': 50.0, 'count': 10, 'status': 'inactive'}

📝 Example 4: Script Management

📋 Total scripts: 5 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! 🚀 ekoDB Python Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data... ✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved:
ZUCRvZp4hpgRsCrEpLSCmR3z1YgXsIQQ4ZeOLnSg2obHiG_7Bki-9xSCIffOC1-1DckZUVGEsVeSK8KwWMndBg
📊 Found 2 product groups {'avg_price': 575.6666666666666, 'category':
'Electronics', 'count': 3} {'avg_price': 474.0, 'category': 'Furniture',
'count': 2} ⏱️ Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved 📊 Found 5 products ⏱️ Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved 📊 Found 2 categories {'category': 'Electronics', 'count': 3}
{'category': 'Furniture', 'count': 2} ⏱️ Execution time: 0ms

📝 Example 4: High Rating Products

✅ Script saved 📊 Found 5 products ⏱️ Execution time: 0ms

📝 Example 5: Script with Parameter Definition

✅ Script saved 📊 Found 5 products ⏱️ Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Script saved 📊 Pipeline executed 3 stages ⏱️ Total execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's Script system: ✅ FindAll operations ✅
Group aggregations (Count, Average) ✅ Multi-stage pipelines (FindAll → Group →
Count) ✅ Parameter definitions ✅ Script management (save, call, delete) ✅
[32mPython client examples complete![0m ✅ [32mAll Python integration tests
complete![0m 🧪 [36mRunning Go examples (direct HTTP/WebSocket)...[0m [34m
╔════════════════════════════════════════╗[0m [34m║ ekoDB Go Examples Test Suite
║[0m [34m╚════════════════════════════════════════╝[0m [34m === Checking Server
Connection ===[0m [32m✓ Server is ready[0m [33m === Running 7 Examples ===[0m
[34m === Running simple_crud.go ===[0m ✓ Authentication successful

=== Insert Document === Inserted:
map[id:xL9VoIeEzeuPhjGeuIU-deJeDkbU7zZ6dBND-w5z3cCVWUWp5o5U3Wd6wDET-fIKkCbj4t9giNNXOmjr_ytijg]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:xL9VoIeEzeuPhjGeuIU-deJeDkbU7zZ6dBND-w5z3cCVWUWp5o5U3Wd6wDET-fIKkCbj4t9giNNXOmjr_ytijg
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found 1 documents

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:xL9VoIeEzeuPhjGeuIU-deJeDkbU7zZ6dBND-w5z3cCVWUWp5o5U3Wd6wDET-fIKkCbj4t9giNNXOmjr_ytijg
name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.go completed
successfully[0m [34m === Running simple_websocket.go ===[0m ✓ Authentication
successful

=== Inserting Test Data === ✓ Inserted test record:
P7tzKk-WVv2ZyOr-TimEHkFoy3v_DOuYNWiVtY4ylJowBxvP5zi3OBl9U3QFj4kRZo5NQGmwIdpc3gvO_se9AQ

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"aRE3XVwlbTj5n7jpL8YazH5Sle1ShUD8o1xXCtJz3n3OMfAvG6SWCG1CUYNC0BVcjv19UXFhDXmpbPhqQ2iYgA",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"PhhdxEYbQszJEMgXIonEwzmyOwVtvGc1Yocd1cTDxLWhJrpoLdEWJOyKQ864_IEWJUlyIlVlbZH6TB-II9J5bQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"P7tzKk-WVv2ZyOr-TimEHkFoy3v_DOuYNWiVtY4ylJowBxvP5zi3OBl9U3QFj4kRZo5NQGmwIdpc3gvO_se9AQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 3
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.go completed
successfully[0m [34m === Running batch_operations.go ===[0m ✓ Authentication
successful

=== Batch Insert === Batch insert response: map[failed:[]
successful:[LkzEWpPN1nnu2nmyaJOm_svgnEpe85KYCXPZDu2hKBiOYmANvW4wDqieCJFtsSUiJ1QCGFZKK6wDie7H1L7z2Q
M464w3woQPhOGXzwduYc7lfglQ6Kgzo5Lvu1Td73AilaG_BZhiFPjPIPJP05MxeAKXrCunFvISeOu23prspWaQ
XmS7P1UIu8I-Eb4Vd9cfGuc31x0ISGdPlwDGf7z6wLK__345OXakUr5JciGyAXl0mTTXnmvmvYAJnhhDsnnb3A
NqUeAMFJvbJqyWSu7pVRUmPHt9jLKj9wifKd0AMdELa2ykAYy6cCx4d2Vht-ajWrhCpK5PwhwwHocpyc-IIGqg
29yaQemSoVnapdQLUq2pDhKejyhOrkXCk2ggMOL5-2SiOKk7DWrO_CXrMOtJhQPSGas-8gDXUHFhx99EZ92Acw]]
✓ Batch insert completed

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === Batch update response: map[failed:[]
successful:[3DzWVNRqFkPORgXjkuf2X-u7rpWwuEyOHwOdxeA9OPYMsnyeJ1OXaz6mUQ-vK_83fSxpIuW7ncP-DRoCgyemnA
RQufRFCeKKB41Rtj4P4Rd_xz65MhqdOSUf6AJ8Ha0VQ_QnxWHG83Q4Y_SIPompdpKf1FiLdDbM44HhrPUb3Q9w
dymsKlG23gr4VC5U9rCF4g3VhL2s3tfRC9RqVE2YebL576gQkWlSSlQlgCb_RerUPFKnj6ICf7sHTgw9nMjZgg]]
✓ Batch update completed

=== Batch Delete === Batch delete response: map[failed:[]
successful:[3DzWVNRqFkPORgXjkuf2X-u7rpWwuEyOHwOdxeA9OPYMsnyeJ1OXaz6mUQ-vK_83fSxpIuW7ncP-DRoCgyemnA
RQufRFCeKKB41Rtj4P4Rd_xz65MhqdOSUf6AJ8Ha0VQ_QnxWHG83Q4Y_SIPompdpKf1FiLdDbM44HhrPUb3Q9w
dymsKlG23gr4VC5U9rCF4g3VhL2s3tfRC9RqVE2YebL576gQkWlSSlQlgCb_RerUPFKnj6ICf7sHTgw9nMjZgg]]
✓ Batch delete completed

✓ All batch operations completed successfully [32m✓ batch_operations.go
completed successfully[0m [34m === Running kv_operations.go ===[0m ✓
Authentication successful

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: map[userId:123 username:john_doe]

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: map[name:Product 1 price:29.99]
cache:product:2: map[name:Product 2 price:39.989999999999995] cache:product:3:
map[name:Product 3 price:49.989999999999995]

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully [32m✓ kv_operations.go completed
successfully[0m [34m === Running collection_management.go ===[0m ✓
Authentication successful

=== Create Collection (via insert) === Collection created with first record:
ntDg9s4QzVIf-OHCRh6vZwFZzC2fgZCEpa3PGUdE_HuJIReyxcaageFEEutGCmh9YCHCwNfoSrmfVE-Q4UZUyg

=== List Collections === Total collections: 14 Sample collections: [ws_ttl_test
schema_documents_client_py batch_users test_collection schema_users_client_py
chat_messages__ek0_bench schema_employees_client_py ttl_cache
chat_configurations__ek0_bench demo_collection scripts__ek0_bench websocket_test
users schema_products_client_py]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.go completed successfully[0m [34m === Running
document_ttl.go ===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
oKIDQyWOb-QWaZL7gBNQGoD7PMj8LGwrFB_TSOX_wrqgvncCOIMveRHE8wycLnNaNRrTpXDzMGIQPNz3MT_P7Q

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 3 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.go
completed successfully[0m [34m === Running websocket_ttl.go ===[0m ✓
Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
WxBVfF41sdg9AYHQMukkIw6cqe5Gog9hy9OdFBrj-PASaw5gn80UrZsnQbsIeDd8EBcWMUh8HmxCRn1ODa_16w

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 3 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.go completed
successfully[0m [34m ╔════════════════════════════════════════╗[0m [34m║ Test
Summary ║[0m [34m╚════════════════════════════════════════╝[0m [34mTotal: 7[0m
[32mPassed: 7[0m [32mFailed: 0[0m ✅ [32mGo direct examples complete![0m 🧪
[36mRunning Go client library examples...[0m ✓ Client created (token exchange
happens automatically)

=== Insert Document === Inserted:
map[id:mokOPRPuTTCCRCIxkC2czVJa53SWkM2hPnEuH-N7iErkiZtg7skqRR3DmmlXKgUNTHzFCeRZv_qTyYf73M_KqQ]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:mokOPRPuTTCCRCIxkC2czVJa53SWkM2hPnEuH-N7iErkiZtg7skqRR3DmmlXKgUNTHzFCeRZv_qTyYf73M_KqQ
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found documents: 1

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:mokOPRPuTTCCRCIxkC2czVJa53SWkM2hPnEuH-N7iErkiZtg7skqRR3DmmlXKgUNTHzFCeRZv_qTyYf73M_KqQ
name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document === Deleted document

=== Cleanup === ✓ Deleted collection

✓ All CRUD operations completed successfully ✓ Client created

=== Batch Insert === ✓ Batch inserted 5 records ✓ Verified: Found 5 total
records in collection

=== Batch Update === ✓ Batch updated 3 records

=== Batch Delete === ✓ Batch deleted 3 records

=== Cleanup === ✓ Deleted collection

✓ All batch operations completed successfully ✓ Client created

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: map[userId:123 username:john_doe]

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: map[name:Product 1 price:29.99]
cache:product:2: map[name:Product 2 price:39.989999999999995] cache:product:3:
map[name:Product 3 price:49.989999999999995]

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
-7gA-nKPUJbqs-neyYmpmQpok7LJir-sTnEUsjab8B-Pb-CHFcqdBr5eJKPAB20NrNwOl-Yx8mRQ5zILXC3mjg

=== List Collections === Total collections: 14 Sample collections: [ws_ttl_test
schema_documents_client_py batch_users test_collection schema_users_client_py]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
4oYQ9gECiQOW_26czrECwhWbXuACJUsWH-NBMQ-KJtT2GqvvEcue9B3zOVgn594-YGBkHOreKi51bOk4LZawQw

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
Lx1epMoQJ8oJt87rdOU_SNyawSbRn9kpUscPdjQo0-fn7RVjVVFGII9Jr1htQ9ZHkoegB0rZURxbluYT7dZi7A

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
Bp6X3IbeEeq0YUD2sbHF_72lw2_w22HSsMuO8gXNpOVX3cyzioKHnfwBcj04CmuJpalpu_fMevbrpZ5mii8_Mw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
E7m9VvjnYo4wSVxxg9fZ-vsAy_s4JKa5oDV72M_iz6Y84BPXsgj0ib2Lxkh7d8EgSa06e0SmKXzupS4DyS2bMA

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
CTrY3GBCQt9S1dSjN1AQzq45w2RWlDwtLOEahLxRN3-VXij8qrGSROWP3Bwh5UVtcyyhtOv9C3pAMWfGOnyo1g

=== Sending Chat Message === Message ID:
jE7_BqjWbt-axWXkUN34rIVz2cquJGJ7m845xQp8N7CiMwU41oouJbYHnlIOWkQSdw8zRMEtyvYlfYNGFIlw9A

=== AI Response === The available products and their prices are:

1. ekoDB Pro, which is an Enterprise edition product with advanced features. It
   is priced at $299.
2. ekoDB Cloud, a fully managed cloud database service product. It costs $499.
3. ekoDB, a high-performance database product with AI capabilities. This is
   priced at $99.

=== Context Used (3 snippets) === Snippet 1: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String
value:Enterprise edition product with advanced features]
id:QIYvBGHyfkJ6dxWYQQ7Ps23v-rvWaHdtGQH3A-uXeKrPO5JyQYcEvoh\_\_thFGaiwymJKyaUxv8tADYHXfAw-6Q
name:map[type:String value:ekoDB Pro] price:map[type:Integer value:299]]
score:0.1111111111111111] Snippet 2: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:Fully
managed cloud database service product]
id:Xa8JdHx5kWMyprndXkHTrFagk9DukHr6JYgBYJeMY2D8EtizbXrg0Oi_caBrCigTvH6WAmh-ncdCIPypZ5ggVg
name:map[type:String value:ekoDB Cloud] price:map[type:Integer value:499]]
score:0.1111111111111111] Snippet 3: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:A
high-performance database product with AI capabilities]
id:vZ97NbEr37ew506M-6Je645YSN7TVXCcFKV1xrAWDsTSG0H9bpRV2CqfUhfqTFFXiT0Fnhqxf01WnMuFsxivjw
name:map[type:String value:ekoDB] price:map[type:Integer value:99]]
score:0.1111111111111111]

Execution Time: 2242ms

=== Token Usage === Prompt tokens: 374 Completion tokens: 74 Total tokens: 448

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
nakIYBT3rWKd68bf5RpvzhCjU6tL1JL9DRRkScueN5VZrqdqKfqkKgfEkzpsyb0P7ryPhN5CPRs6ZpsHIShBiw

=== Sending Messages === ✓ Message 1 sent Response: Based on the provided
context, the available product is a high-performance database product named
ekoDB. It is priced at 99.

✓ Message 2 sent Response: I'm sorry, but there's no context provided about a
specific product to provide the price details. Could you please specify the
product you're referring to?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
Ey8KUECC2wmvj4izvG9Ugta1_KP1xFMTs27RvEHwAF5JZzU6B_MPoc832465Ym_1memMRQHckGZbcN3Z6K-IoA
Parent:
nakIYBT3rWKd68bf5RpvzhCjU6tL1JL9DRRkScueN5VZrqdqKfqkKgfEkzpsyb0P7ryPhN5CPRs6ZpsHIShBiw

=== Listing Sessions === ✓ Found 9 sessions Session 1:
Ey8KUECC2wmvj4izvG9Ugta1_KP1xFMTs27RvEHwAF5JZzU6B_MPoc832465Ym_1memMRQHckGZbcN3Z6K-IoA
(Untitled) Session 2:
nakIYBT3rWKd68bf5RpvzhCjU6tL1JL9DRRkScueN5VZrqdqKfqkKgfEkzpsyb0P7ryPhN5CPRs6ZpsHIShBiw
(Untitled) Session 3:
CTrY3GBCQt9S1dSjN1AQzq45w2RWlDwtLOEahLxRN3-VXij8qrGSROWP3Bwh5UVtcyyhtOv9C3pAMWfGOnyo1g
(Untitled) Session 4:
JTaC0beEkTbQdlCZePnWpSgtzxgkeSFO6MBawhoKUreXdifOnFbcRmCZ7m-9shlCBFXP2NkUL1SMmviUhEGXKA
(Untitled) Session 5:
sfm7SuCU_7I60z0VFUIdufcM4PTTusaCwHVTUjlfO03ayFDkrlNa0-3aQx99iAqXzQotv1ywUpfN073Gq_6RvA
(Untitled) Session 6:
06JVJeje00VfyFekc9GCgSS8QpyxR8sK66prVhP6gAv2K7ckAfjRtW-bACzxAnBwbgqh6MK9wxo7TFbN7sHeMw
(Untitled) Session 7:
SCAx_XbQF_b-0GA8DV0bvD7IiWmVtRNnI8iySeWi8uy956NxdqKfiXXD9TjJ40k2E0xG4N60M_gjtLGOZHDN3Q
(Untitled) Session 8:
mwJq0JqojQdSve6Yo19pdIZeOdkM4CH0MRXJ04YUrrjq0XCKgvyYyXWU598PsuBif8g5mBc5HttM7WOBlMxywA
(Untitled) Session 9:
5UAdpVm_csoCxOJQxiqdq5hVSVqlgdZq5q3paht_qgWqQJ6DX7lDB_nEP0ehvl1DBU5MEeStrS_FL7mGVCCI_w
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
Ey8KUECC2wmvj4izvG9Ugta1_KP1xFMTs27RvEHwAF5JZzU6B_MPoc832465Ym_1memMRQHckGZbcN3Z6K-IoA

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
zn4lmRInAe4kTibPhMRDOqwehfFerXiW9NqZZodsu4fkTxiGNoUJj-Ydhg_HKh7RmxGNpe_eUunedQZrmFI42w

=== Sending Initial Message === ✓ Message sent Response: Based on the
information provided, the available product is ekoDB. This is a high-performance
database product, priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
I'm sorry, but the context provided does not contain information about the price
of ekoDB. I recommend checking the official website or contacting the seller for
the most accurate and up-to-date price.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
WNX0cfFqcU4g0_lJy2VwuNtbNvsdBhkH7LfXfh-KxYKps2Ru3iK23m9v3Su2SWggJcbxoWw8xK9g6PVCtWZiAQ
✓ Sent message in second session ✓ Sessions merged successfully Total messages
in merged session: 4

=== Feature 5: Delete Message === ✓ Message deleted

✓ Messages remaining: 3

=== Cleanup === ✓ Deleted session ✓ Deleted collection

✓ All advanced chat features demonstrated successfully! === Query Builder
Examples ===

Setting up test data... ✅ Test data created

1. Simple equality query: Found 2 active users

2. Range query with sorting: Found 3 users aged 18-65

3. String operations: Found 2 users with @example.com emails

4. IN operator: Found 2 privileged users

5. Complex query with multiple conditions: Found 1 active US users over 21

6. Pagination: Page 1: 2 users

7. NOT IN operator: Found 3 valid users

8. Using bypass flags: Found 2 users (bypassed cache)

=== Cleanup === ✅ Deleted test collection

✅ Query Builder examples completed! === Search Examples ===

Setting up test data... ✅ Test data created

1. Basic full-text search: Found 2 results
1. Score: 25.740
1. Score: 12.540

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400
1. Score: 26.400
1. Score: 26.400
1. Score: 26.400

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600
1. Score: 39.600
1. Score: 39.600
1. Score: 39.600

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200
1. Score: 13.200

1. Search with stemming and exact match boosting: Found 1 results (matches:
   work, working, worked)
1. Score: 13.200

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.752
1. Score: 0.752
1. Score: 0.736

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.694
1. Score: 1.501
1. Score: 0.301

1. Case-sensitive search: Found 1 results (case-sensitive)
1. Score: 13.200

=== Cleanup === ✅ Deleted test collections

✅ Search examples completed! === Schema Management Examples ===

1. Creating user schema with basic fields: ✅ User schema created

2. Creating product schema with text index: ✅ Product schema with indexes
   created

3. Creating document schema with vector index: ✅ Document schema with vector
   index created

4. Retrieving collection schema: Schema fields: 4 fields Schema version: 1

5. Retrieving collection metadata: Collection has 4 fields

6. Creating employee schema with all constraint types: ✅ Employee schema with
   all constraints created

✅ Schema management examples completed! === Join Operations Examples ===

Setting up sample data... ✅ Sample data created

1. Single collection join (users with departments): Found 2 users with
   department data

- Alice Johnson: Engineering
- Bob Smith: Sales

2. Join with filtering: Found 1 users in Engineering

- Alice Johnson: Building A

3. Join with user profiles: Found 2 users with profile data

- Alice Johnson: Senior Software Engineer
- Bob Smith: Sales Manager

4. Join orders with user data: Found 2 completed orders

- Laptop ($0) by Alice Johnson
- Mouse ($0) by Alice Johnson

5. Complex join with multiple conditions: Found 2 users with example.com emails

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✅ Deleted test collections

✅ Join operations examples completed! 🚀 ekoDB Scripts Example (Go Client)

✅ Client initialized

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
TvSD0Vt64bessRsKSqaDnvgYPVjXYPiVHNEk_Qtk-XG4Qassz3ueZYkI6aII1Nn-tRxBMxfXhNoROIDovixRIg
📊 Found 30 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 30 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups map[avg_score:50 count:15
status:inactive] map[avg_score:60 count:15 status:active] ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 7 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved 📊 Pipeline executed 2 stages ⏱️ Total execution
time: 0ms 📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved 📊 Total user count: 30 ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Deleted collection ✅ Deleted test scripts

✅ All examples completed successfully!

💡 Key Advantages of Using the Client: • Automatic token management • Type-safe
Stage builders • Built-in error handling ✅ [32mGo client examples complete![0m
✅ [32mAll Go integration tests complete![0m 📦 [36mBuilding TypeScript client
library...[0m

> @ekodb/ekodb-client@0.3.0 prepare npm run build

> @ekodb/ekodb-client@0.3.0 build tsc

up to date, audited 7 packages in 1s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.3.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning TypeScript client library
examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 645ms

3 packages are looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'lePNYFndT6p9dWXx_X74mAbhJ9NqwaAxjvWPTIvL8swwpyt5O-gCq8tVZiVtGYL4MMBNAbo5X5dW1hHCbFTJcw'
}

=== Find by ID === Found: { name: { type: 'String', value: 'Test Record' },
active: { type: 'Boolean', value: true }, id:
'lePNYFndT6p9dWXx_X74mAbhJ9NqwaAxjvWPTIvL8swwpyt5O-gCq8tVZiVtGYL4MMBNAbo5X5dW1hHCbFTJcw',
value: { value: 42, type: 'Integer' } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { name: { type: 'String', value: 'Updated
Record' }, active: { type: 'Boolean', value: true }, id:
'lePNYFndT6p9dWXx_X74mAbhJ9NqwaAxjvWPTIvL8swwpyt5O-gCq8tVZiVtGYL4MMBNAbo5X5dW1hHCbFTJcw',
value: { type: 'Integer', value: 100 } }

=== Delete Document === Deleted document

=== Cleanup === ✓ Deleted collection

✓ All CRUD operations completed successfully ✓ Client created

=== Batch Insert === ✓ Batch inserted 5 records ✓ Verified: Found 5 total
records in collection

=== Batch Update === ✓ Batch updated 3 records

=== Batch Delete === ✓ Batch deleted 3 records

=== Cleanup === ✓ Deleted collection

✓ All batch operations completed successfully ✓ Client created

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: { username: 'john_doe', userId: 123 }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
iKn92oowZ8_p3MfSTTizln9dU5FjMJgYmk0VhSAh0DNjl35y5RgcZSpvyk8Y52CSmMFa7Ds9R7hEucGGDij_jw

=== List Collections === Total collections: 17 Sample collections:
ws_ttl_test,schema_documents_client_py,batch_users,test_collection,schema_users_client_py

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
Wou9kccTlznrByryXHG2fXfQXI4PLe_y5Sj7ccMiaff14Wm6LS8pzNTETVMN0tAVyK5RmdvuNCMLUTn2b81MQQ

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
jJHcesiEpByLloyQyqyyUxCeEaQmddrm5ImJDfqihgOeDLFNzcHgRR0eBrZS4iRva2k\_\_M-AnCNcT4dZNueiFQ

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
hx_z1W2RUQBvGkzFq0DIBdQ5CpggfQcIEmAGsqbJJHvX50TIPk_BSqZO4a1joh6RVf2CST-wljTnReDLnOC3kg

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
u-yKZQ8zS_Zk5VW-hnC3ITMYUPc67mnTbeXEG7dJPhc-xMfG7P_xWt5BioK1X8swjyPOJPadLAzi3667v9JLYQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
B9DJFdfaZrl4B87BX7PwzHPZOlPbtDOSv9H1P7sKXia3LyLyanQIusKR1dov12NY77OgxlGhQ8RJg3j0MlngSQ

=== Sending Chat Message === Message ID:
Cq2_E6wZU5jmBmOn0NW81Tiu5ykFPmYjqhAOjYmQ12UkeDOQ2QlUC3wc_PL97FJa_5FCLfy1QdPFNdUNhewe7g

=== AI Response === There are three products available:

1. ekoDB Cloud: This is a fully managed cloud database service product. The
   price for this product is $499.

2. ekoDB Pro: This is an enterprise edition product with advanced features. The
   price for this product is $299.

3. ekoDB: This is a high-performance database product with AI capabilities. The
   price for this product is $99.

=== Context Used (3 snippets) === Snippet 1: { collection:
'client_chat_basic_ts', record: { name: { type: 'String', value: 'ekoDB Cloud'
}, id:
'H4iNHWBxv84h3Cuo0LTS6vcKYXHhmQWTY8a7ld8YfvnHIcNc4X8yzlsblNpLMyV53vmec1mUHAjYgvemCt6shA',
price: { value: 499, type: 'Integer' }, description: { value: 'Fully managed
cloud database service product', type: 'String' } }, score: 0.1111111111111111,
matched_fields: [ 'description' ] } Snippet 2: { collection:
'client_chat_basic_ts', record: { description: { value: 'Enterprise edition
product with advanced features', type: 'String' }, name: { value: 'ekoDB Pro',
type: 'String' }, price: { type: 'Integer', value: 299 }, id:
'3CWl9krXbv3uxH74e6fs83VWWpc_Vl-0zEUyFso-EfNtWwDovAI142GGybXT5YqoKGcB5uy3EmmlHAXk6Ksggw'
}, score: 0.1111111111111111, matched_fields: [ 'description' ] } Snippet 3: {
collection: 'client_chat_basic_ts', record: { id:
'ry1jTRRyUMUmMX6kzWiNIp3r4OZ4hgrx9_bZSUldCWAb9UCy8Kk8-rkgn-hs4R5JVFo_5-KYUdTMOPTtPN8JQA',
name: { type: 'String', value: 'ekoDB' }, description: { value: 'A
high-performance database product with AI capabilities', type: 'String' },
price: { value: 99, type: 'Integer' } }, score: 0.1111111111111111,
matched_fields: [ 'description' ] }

Execution Time: 3448ms

=== Token Usage === Prompt tokens: 380 Completion tokens: 84 Total tokens: 464

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
A5zA9RJKpJ8sV3YuW9oJfD20oPAEiMohYlJlkWrYd0uI_Dxbb_WUGrMcEZHSs5Ki1B3Elu2rCJX0VJ7HXXTHpg

=== Sending Messages === ✓ Message 1 sent Response: Based on the context
provided, the available product is ekoDB, a high-performance database product
that is priced at $99.

✓ Message 2 sent Response: I'm sorry, but there's no information provided about
any product in the context. Could you please specify the product you're
interested in?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
2kIRHUW4UYmzbsORkds-t2Zo2gfUqZr3IvEvI6QfmqM8oNYw4huwf3_sSJkWEWgvLR57ak6abnFtsiFgIITzTQ
Parent:
A5zA9RJKpJ8sV3YuW9oJfD20oPAEiMohYlJlkWrYd0uI_Dxbb_WUGrMcEZHSs5Ki1B3Elu2rCJX0VJ7HXXTHpg

=== Listing Sessions === ✓ Found 10 sessions Session 1:
2kIRHUW4UYmzbsORkds-t2Zo2gfUqZr3IvEvI6QfmqM8oNYw4huwf3_sSJkWEWgvLR57ak6abnFtsiFgIITzTQ
(Untitled) Session 2:
A5zA9RJKpJ8sV3YuW9oJfD20oPAEiMohYlJlkWrYd0uI_Dxbb_WUGrMcEZHSs5Ki1B3Elu2rCJX0VJ7HXXTHpg
(Untitled) Session 3:
B9DJFdfaZrl4B87BX7PwzHPZOlPbtDOSv9H1P7sKXia3LyLyanQIusKR1dov12NY77OgxlGhQ8RJg3j0MlngSQ
(Untitled) Session 4:
WNX0cfFqcU4g0_lJy2VwuNtbNvsdBhkH7LfXfh-KxYKps2Ru3iK23m9v3Su2SWggJcbxoWw8xK9g6PVCtWZiAQ
(Untitled) Session 5:
nakIYBT3rWKd68bf5RpvzhCjU6tL1JL9DRRkScueN5VZrqdqKfqkKgfEkzpsyb0P7ryPhN5CPRs6ZpsHIShBiw
(Untitled) Session 6:
CTrY3GBCQt9S1dSjN1AQzq45w2RWlDwtLOEahLxRN3-VXij8qrGSROWP3Bwh5UVtcyyhtOv9C3pAMWfGOnyo1g
(Untitled) Session 7:
JTaC0beEkTbQdlCZePnWpSgtzxgkeSFO6MBawhoKUreXdifOnFbcRmCZ7m-9shlCBFXP2NkUL1SMmviUhEGXKA
(Untitled) Session 8:
sfm7SuCU_7I60z0VFUIdufcM4PTTusaCwHVTUjlfO03ayFDkrlNa0-3aQx99iAqXzQotv1ywUpfN073Gq_6RvA
(Untitled) Session 9:
06JVJeje00VfyFekc9GCgSS8QpyxR8sK66prVhP6gAv2K7ckAfjRtW-bACzxAnBwbgqh6MK9wxo7TFbN7sHeMw
(Untitled) Session 10:
SCAx_XbQF_b-0GA8DV0bvD7IiWmVtRNnI8iySeWi8uy956NxdqKfiXXD9TjJ40k2E0xG4N60M_gjtLGOZHDN3Q
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
2kIRHUW4UYmzbsORkds-t2Zo2gfUqZr3IvEvI6QfmqM8oNYw4huwf3_sSJkWEWgvLR57ak6abnFtsiFgIITzTQ

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
7pgPTzuQyxsDgrzeKCUUC7b6CFPy6sJa0933e97AxW0TUCfLucrhpUztezm8bMquHzz64psqW2_0c2CdQIg-SA

=== Sending Initial Message === ✓ Message sent Response: Based on the provided
context, the available product is a high-performance database product named
ekoDB priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
I'm sorry, but the context provided does not include information on the price of
ekoDB. I recommend checking the official website or contacting the sales
department of ekoDB for accurate pricing information.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
Z4vPg23lp1jGgctQJ-P5LyxUwqYiTNOxN_1cL9svdG-c55ge_sthmVhtystS0Fv3OY-83TAqmcgi1_f_Qr-WdQ
✓ Sent message in second session ✓ Sessions merged successfully Total messages
in merged session: 4

=== Feature 5: Delete Message === ✓ Message deleted

✓ Messages remaining: 3

=== Cleanup === ✓ Deleted session ✓ Deleted collection

✓ All advanced chat features demonstrated successfully! === Query Builder
Examples ===

Setting up test data... ✅ Test data created

1. Simple equality query: Found 2 active users

2. Range query with sorting: Found 3 users aged 18-65

3. String operations: Found 2 users with @example.com emails

4. IN operator: Found 2 privileged users

5. Complex query with multiple conditions: Found 1 active US users over 21

6. Pagination: Page 1: 2 users

7. NOT IN operator: Found 3 valid users

8. Using bypass flags: Found 2 users (bypassed cache)

=== Cleanup === ✅ Deleted test collection

✅ Query Builder examples completed! === Search Examples ===

Setting up test data... ✅ Test data created

1. Basic full-text search: Found 2 results
1. Score: 25.740, Matched: name, name.value, email, email.value
1. Score: 12.540, Matched: name.value, name

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: title, bio.value, bio, title.value
1. Score: 26.400, Matched: title, bio, title.value, bio.value
1. Score: 26.400, Matched: bio, title, bio.value, title.value
1. Score: 26.400, Matched: title, title.value, bio, bio.value

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: bio, title.value, bio.value, title
1. Score: 39.600, Matched: bio, title, title.value, bio.value
1. Score: 39.600, Matched: bio, bio.value, title, title.value
1. Score: 39.600, Matched: bio, title, title.value, bio.value

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio, bio.value
1. Score: 13.200, Matched: bio.value, bio

1. Search with stemming and exact match boosting: Found 1 results (matches:
   work, working, worked)
1. Score: 13.200, Matched: bio.value, bio

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.763, Matched:
1. Score: 0.748, Matched:
1. Score: 0.747, Matched:

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.699, Matched: content, title, content.value, title.value
1. Score: 1.499, Matched: title.value, title, content, content.value
1. Score: 0.305, Matched:

1. Case-sensitive search: Found 1 results (case-sensitive)
1. Score: 13.200, Matched: bio.value, bio

=== Cleanup === ✅ Deleted test collections

✅ Search examples completed! === Schema Management Examples ===

1. Creating user schema with basic fields: ✅ User schema created

2. Creating product schema with text index: ✅ Product schema with indexes
   created

3. Creating document schema with vector index: ✅ Document schema with vector
   index created

4. Retrieving collection schema: Schema fields: [ 'age', 'email', 'name',
   'status' ] Schema version: 1

5. Retrieving collection metadata: Collection has 4 fields

6. Creating employee schema with all constraint types: ✅ Employee schema with
   all constraints created

✅ Schema management examples completed! === Join Operations Examples ===

Setting up sample data... ✅ Sample data created

1. Single collection join (users with departments): Found 2 users with
   department data:

- Bob Smith: Sales
- Alice Johnson: Engineering

2. Join with filtering: Found 1 users in Engineering:

- Alice Johnson: Building A

3. Join with user profiles: Found 2 users with profile data:

- Bob Smith: Sales Manager
- Alice Johnson: Senior Software Engineer

4. Join orders with user data: Found 2 completed orders:

- Laptop ($1200) by Alice Johnson
- Mouse ($25) by Alice Johnson

5. Complex join with multiple conditions: Found 2 users with example.com emails:

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✅ Deleted test collections

✅ Join operations examples completed! 🚀 ekoDB Scripts Example (TypeScript)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
WOi85_pkRgCz8hNZytxqk7PikQaCltAudBcx8MQDb8-K3--SN6IVHWJbV2mb8awSC3P-9ZmbIBmek2HUdB2zIw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved:
IdRnP-o0QZEJ1qWvALST2LlMf9J2v4vtNw6Aqv9gU1zMZT3Xc_yMsjaky_lvzVAWSuCgsQNUFn27mj7fVspSmA
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
19YcDvYAQgdrMQD90AP5cA_h1ke9AKtnylVKzi_pZ7mhIo4M-\_T_TiURSk2eE2pL6_CJA3fyHfv9VBxhYGXu4w
📊 Statistics: 2 groups {"avg_score":50,"count":5,"status":"inactive"}
{"count":5,"avg_score":60,"status":"active"}

📝 Example 4: Script Management

📋 Total scripts: 11 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! 🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data... ✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved:
TPXtLc2XNRgqcnZEm6EY-xzCz51roGhwOmDUKSZbuen3o7iOv_cETXc6_vfRuqdQBAsSNZ8nM5q-8D3M-PKTQw
📊 Found 2 product groups
{"avg_price":575.6666666666666,"category":"Electronics","count":3}
{"count":2,"avg_price":474,"category":"Furniture"} ⏱️ Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved 📊 Found 5 products ⏱️ Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved 📊 Found 2 categories {"count":2,"category":"Furniture"}
{"category":"Electronics","count":3} ⏱️ Execution time: 0ms

📝 Example 4: High Rating Products

✅ Script saved 📊 Found 5 products ⏱️ Execution time: 0ms

📝 Example 5: Script with Parameter Definition

✅ Script saved 📊 Found 5 products ⏱️ Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Script saved 📊 Pipeline executed 3 stages ⏱️ Total execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's Script system: ✅ FindAll operations ✅
Group aggregations (Count, Average) ✅ Multi-stage pipelines (FindAll → Group →
Count) ✅ Parameter definitions ✅ Script management (save, call, delete) ✅
[32mTypeScript client examples complete![0m ✅ [32mAll TypeScript integration
tests complete![0m 🧪 [36mRunning JavaScript examples (direct
HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 657ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities [34m ╔════════════════════════════════════════╗[0m [34m║
ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m [34m === Checking Server
Connection ===[0m (node:17313) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) [32m✓
Server is ready[0m [34m === Getting Authentication Token ===[0m [32m✓
Authentication successful[0m [33m === Running 7 Examples ===[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js
===[0m ✓ Authentication successful

=== Insert Document === (node:17315) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) Inserted:
{ id:
'GM2z3W4zBlm-4BGEUyRKDlahIFEnbQTdwGOXhh5Mk66ouFk1HPuF7AaEq1p96XJO29ijpChKviphq_J8rU0qrw'
}

=== Find by ID === Found: { id:
'GM2z3W4zBlm-4BGEUyRKDlahIFEnbQTdwGOXhh5Mk66ouFk1HPuF7AaEq1p96XJO29ijpChKviphq_J8rU0qrw',
active: { value: true, type: 'Boolean' }, value: { type: 'Integer', value: 42 },
name: { type: 'String', value: 'Test Record' } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { value: { value: 100, type: 'Integer' }, name:
{ value: 'Updated Record', type: 'String' }, active: { value: true, type:
'Boolean' }, id:
'GM2z3W4zBlm-4BGEUyRKDlahIFEnbQTdwGOXhh5Mk66ouFk1HPuF7AaEq1p96XJO29ijpChKviphq_J8rU0qrw'
}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js
===[0m ✓ Authentication successful

=== Inserting Test Data === (node:17318) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
test record:
hVpqSXhpvTz7V3vDIscxjh4qx4qCstQGEAHrHjcRrRk3w9pt92Kqnh17YE9tHveGctaPKBqhOduLbW9zMIFNRA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"aRE3XVwlbTj5n7jpL8YazH5Sle1ShUD8o1xXCtJz3n3OMfAvG6SWCG1CUYNC0BVcjv19UXFhDXmpbPhqQ2iYgA",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"hVpqSXhpvTz7V3vDIscxjh4qx4qCstQGEAHrHjcRrRk3w9pt92Kqnh17YE9tHveGctaPKBqhOduLbW9zMIFNRA",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"PhhdxEYbQszJEMgXIonEwzmyOwVtvGc1Yocd1cTDxLWhJrpoLdEWJOyKQ864_IEWJUlyIlVlbZH6TB-II9J5bQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"P7tzKk-WVv2ZyOr-TimEHkFoy3v_DOuYNWiVtY4ylJowBxvP5zi3OBl9U3QFj4kRZo5NQGmwIdpc3gvO_se9AQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 4
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/batch_operations.js
===[0m ✓ Authentication successful

=== Batch Insert === (node:17331) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Batch
inserted 5 records ✓ Verified: Found 20 total records in collection

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === ✓ Batch updated 3 records ✓ Verified: Record updated with
status="active"

=== Batch Delete === ✓ Batch deleted 3 records ✓ Verified: Records successfully
deleted (not found)

✓ All batch operations completed successfully [32m✓ batch_operations.js
completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/kv_operations.js
===[0m ✓ Authentication successful

=== KV Set === (node:17333) [DEP0040] DeprecationWarning: The `punycode` module
is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Set key:
session:user123

=== KV Get === Retrieved value: { userId: 123, username: 'john_doe', loginTime:
'2025-11-10T04:25:17.228Z' }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully [32m✓ kv_operations.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/collection_management.js
===[0m ✓ Authentication successful

=== Create Collection (via insert) === (node:17340) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) Collection created with first record:
BCsO-rKuX8SsKq-eO102pDt9Zes5UAWP4I4dDaw42thov1rvW8inTyYoHChUE0gqeJX3em3h4VyiTeGRevnQ0w

=== List Collections === Total collections: 22 Sample collections: [
'ws_ttl_test', 'schema_documents_client_ts', 'schema_products_client_ts',
'schema_documents_client_py', 'batch_users' ]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.js completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/document_ttl.js
===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === (node:17347) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) ✓ Inserted document:
CDlGnXTHuLgUya7RN8t5QLd4g3aJBhR5KA6xxarCDRbIj5RBpIvzzYyPhRoYiuDKSOym5ht7-10VgZAN6yMICw

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 4 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.js
completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js
===[0m ✓ Authentication successful

=== Insert Test Data with TTL === (node:17349) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
document with TTL:
N8rB-o_XdPtKYE84_97JVDou5OEwYbuKeLzq_52EYstxCaCCxnLDd22KxJ_5VSFLjtg8KUfRBHB09tHrsIyPfA

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 4 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.js completed
successfully[0m [34m ╔════════════════════════════════════════╗[0m [34m║ Test
Summary ║[0m [34m╚════════════════════════════════════════╝[0m [34mTotal: 7[0m
[32mPassed: 7[0m [32mFailed: 0[0m ✅ [32mJavaScript direct examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.3.0 prepare npm run build

> @ekodb/ekodb-client@0.3.0 build tsc

up to date, audited 7 packages in 870ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.3.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning JavaScript client library
examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 744ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'-h6jiQJPYr5KcnJp8mjfRlGi10qMuh92P2z4wng4aM_oFOJAARtnRhSKVaJOeCIG9ZS57pJw6a2VZop9S0f7qw'
}

=== Find by ID === Found: { id:
'-h6jiQJPYr5KcnJp8mjfRlGi10qMuh92P2z4wng4aM_oFOJAARtnRhSKVaJOeCIG9ZS57pJw6a2VZop9S0f7qw',
value: { type: 'Integer', value: 42 }, name: { type: 'String', value: 'Test
Record' }, active: { type: 'Boolean', value: true } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { active: { type: 'Boolean', value: true },
name: { value: 'Updated Record', type: 'String' }, id:
'-h6jiQJPYr5KcnJp8mjfRlGi10qMuh92P2z4wng4aM_oFOJAARtnRhSKVaJOeCIG9ZS57pJw6a2VZop9S0f7qw',
value: { value: 100, type: 'Integer' } }

=== Delete Document === Deleted document

=== Cleanup === ✓ Deleted collection

✓ All CRUD operations completed successfully ✓ Client created

=== Batch Insert === ✓ Batch inserted 5 records ✓ Verified: Found 5 total
records in collection

=== Batch Update === ✓ Batch updated 3 records

=== Batch Delete === ✓ Batch deleted 3 records

=== Cleanup === ✓ Deleted collection

✓ All batch operations completed successfully ✓ Client created

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: { userId: 123, username: 'john_doe' }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
Vxl2x5nmzyv50YMJD7DQVtJHrRfaFaDawoSXx3x7lKhCqtaLKYYFlvZkMWL2sdGottqSKOXO8bGTLgqXn9Of1g

=== List Collections === Total collections: 22 Sample collections:
ws_ttl_test,schema_documents_client_ts,schema_products_client_ts,schema_documents_client_py,batch_users

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
y6luYg11pmmpDEcT-8KYqigIA1934WHUNJ51xhDCdkFR_HPQXSHiQ-UeffauXQIBQEiDuXdJXXREHbGfXqh2xg

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
eitRi_ZZTHQQqEh2K1hDtPZMSHMDjdJmsPopAdo37LMG-9B2J1A_2AJPf5OG3vWhGvp4G5JGAhme8vXR9HgM8A

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
brqmMdBqrJaejqgCyonw4Nl-iLKzOZdW0KHAJoBYfyEaT9QiSrSUkIbeCVGuAja_MH4LLy6eeu7mXozeBmWC9Q

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
34lr6nNUwcrJCbr6kkCp7US3ocXNQ5pfJHIPKmIav68xhWWyn4GAtnkfJUka7_GcifJSf8pYvLESdOsr0h-JhQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration 🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function

✅ Script saved:
dQA8PAbgHy1JtrTScJpWdDloJWCT2W5KGJbRH61E56D1feoXpOOmYWbCPW-85H7oT6LMeOHEEhNKFcUd324AMw
📊 Found 20 active users

📝 Example 2: Parameterized Function

✅ Script saved:
i0hPOH_5KXS37AnIJMU0mowfKk_P07FCAj057iJy63V2nEAnUJTxoLKXdc0_qGzYp_lgzifjR5REXvnO9iPg9g
📊 Found 20 users (limited)

📝 Example 3: Aggregation Function

✅ Script saved:
SStmYK6UbYkWuMzujuHB7ezPmCgJl8xit1JGWTCTs-ETW5Z8UTD-ngFbl6piRiNj41rCk_ylOIZnzKDh2eRwlQ
📊 Statistics: 2 groups {"avg_score":50,"count":10,"status":"inactive"}
{"avg_score":60,"status":"active","count":10}

📝 Example 4: Function Management

📋 Total scripts: 13 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! 🚀 ekoDB Scripts Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
KpNpC_kP7h3OameA8aqm7TQpq8ID0OSBuZwH--gN79-sLxs3dUp16M2jerRzfTJtCvb3-4FFSnr9E5sW2P-mtg
📊 Found 30 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 30 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"count":15,"status":"active","avg_score":60}
{"avg_score":50,"status":"inactive","count":15} ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 15 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved 📊 Pipeline executed 2 stages ⏱️ Total execution
time: 0ms 📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved 📊 Total user count: 30 ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Deleted collection ✅ Deleted test scripts

✅ All examples completed successfully!

💡 Key Advantages of Using the Client: • Automatic token management • Type-safe
Stage builders • ChatMessage helpers • Cleaner, more maintainable code •
Built-in error handling 🚀 ekoDB Advanced Scripts Example

📋 Setting up test data... ✅ Created 10 products

📝 Example 1: List All Products

✅ Script saved 📊 Found 10 products ⏱️ Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Script saved 📊 Found 2 categories Electronics: 6 items (avg $325.67)
Furniture: 4 items (avg $294.00) ⏱️ Execution time: 0ms

📝 Example 3: Count All Products

✅ Script saved 📊 Total products: 10 ⏱️ Execution time: 0ms

📝 Example 4: Multi-Stage Aggregation

✅ Script saved 📊 Category analysis (2 categories): Furniture: Products: 4 |
Stock: 43 | Avg Rating: ⭐4.26 Electronics: Products: 6 | Stock: 232 | Avg
Rating: ⭐4.52

⏱️ Total execution time: 0ms 📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved 📊 Product summaries (10 items, showing first 3):

1.  Keyboard - $89 (⭐4.4)
2.  Laptop Pro - $1299 (⭐4.8)
3.  Monitor 27" - $399 (⭐4.6) ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All advanced script examples completed! 🚀 ekoDB CRUD Scripts Example

📋 Setting up test data... ✅ Created 10 test users

📝 Example 1: List All Users

✅ Script saved 📊 Found 10 users ⏱️ Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Script saved 📊 User counts by status: inactive: 3 users active: 7 users ⏱️
Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved 📊 Average scores by role: admin: 20.0 (3 users) user: 70.0 (7
users) ⏱️ Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved 📊 Users (showing first 5 of 0): ⏱️ Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved 📊 User summary (2 groups): active undefineds: 7 users, Total
Score: 370 inactive undefineds: 3 users, Total Score: 180 ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All CRUD script examples completed! 🚀 ekoDB Search Scripts Example

📋 Setting up test data... ✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved 📊 Found 5 documents

1.  Introduction to Machine Learning (AI)
2.  Natural Language Processing (AI)
3.  Vector Databases Explained (Database)
4.  Database Design Principles (Database)
5.  Getting Started with ekoDB (Database) ⏱️ Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved 📊 Documents by category: AI: 2 documents Database: 3 documents
⏱️ Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Script saved 📊 Document titles (5 docs):

1.  Introduction to Machine Learning
2.  Natural Language Processing
3.  Vector Databases Explained
4.  Database Design Principles
5.  Getting Started with ekoDB ⏱️ Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved 📊 Projected documents (showing first 3):

1.  Introduction to Machine Learning
2.  Natural Language Processing
3.  Vector Databases Explained ⏱️ Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved 📊 All documents (5 total, showing first 2):

1.  Introduction to Machine Learning (AI)
2.  Natural Language Processing (AI) ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All search script examples completed! 🚀 ekoDB AI Scripts Example

⚠️ Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data... ✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved 🤖 AI Response:

1. Efficiency: Vector databases allow for efficient querying and retrieval of
   spatial data.
2. Precision: They provide higher precision for geographic data, ensuring
   accurate representations.
3. Flexibility: Vector data can be scaled up or down without losing resolution
   or detail.
4. Complexity: They can represent complex geographic features such as points,
   lines, and polygons.
5. Attributes: Vector databases allow for easy attachment of attributes to
   geographic features.
6. Analysis: They support various spatial analyses like network analysis,
   proximity analysis, and overlay analysis. ⏱️ Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved 📊 Generated embeddings for 3 articles

1.  "Draft Article" - 1536D vector
2.  "Getting Started with ekoDB" - 1536D vector
3.  "Advanced Query Patterns" - 1536D vector ⏱️ Execution time: 0ms

📝 Example 3: List All Articles

✅ Script saved 📊 Found 3 articles ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All AI examples completed! ✅ [32mJavaScript client examples complete![0m ✅
[32mAll JavaScript integration tests complete![0m 🟣 [36mBuilding Kotlin client
library...[0m To honour the JVM settings for this build a single-use Daemon
process will be forked. For more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE Task :jar UP-TO-DATE Task :assemble UP-TO-DATE Task
> :compileTestKotlin NO-SOURCE Task :compileTestJava NO-SOURCE Task
> :processTestResources NO-SOURCE Task :testClasses UP-TO-DATE Task :test
> NO-SOURCE Task :check UP-TO-DATE Task :build UP-TO-DATE

BUILD SUCCESSFUL in 3s 2 actionable tasks: 2 up-to-date ✅ [32mKotlin client
built![0m 🧪 [36mRunning Kotlin client library examples...[0m To honour the JVM
settings for this build a single-use Daemon process will be forked. For more on
this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Simple CRUD Example ===

=== Create === ✓ Inserted user:
Record(fields={id=StringValue(value=\_tEBrM_1i_6sscs3DmX1zftvuiGQLkRn7ACiU1R-e-\_aLoNA_Tr_xEHT8HNFPFHtMki7voUz-SZHG5dixvdFaA)})
User ID:
\_tEBrM_1i_6sscs3DmX1zftvuiGQLkRn7ACiU1R-e-\_aLoNA_Tr_xEHT8HNFPFHtMki7voUz-SZHG5dixvdFaA

=== Read === ✓ Found user by ID:
Record(fields={name=ObjectValue(value={value=StringValue(value=Alice Johnson),
type=StringValue(value=String)}),
email=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=alice@example.com)}),
id=StringValue(value=\_tEBrM_1i_6sscs3DmX1zftvuiGQLkRn7ACiU1R-e-\_aLoNA_Tr_xEHT8HNFPFHtMki7voUz-SZHG5dixvdFaA),
age=ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=28)}),
active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)})})

=== Update === ✓ Updated user:
Record(fields={city=ObjectValue(value={value=StringValue(value=San Francisco),
type=StringValue(value=String)}),
name=ObjectValue(value={value=StringValue(value=Alice Johnson),
type=StringValue(value=String)}),
id=StringValue(value=\_tEBrM_1i_6sscs3DmX1zftvuiGQLkRn7ACiU1R-e-\_aLoNA_Tr_xEHT8HNFPFHtMki7voUz-SZHG5dixvdFaA),
age=ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=29)}),
email=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=alice@example.com)}),
active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)})})

=== Query === ✓ Found 1 users matching query

- Record(fields={name=ObjectValue(value={value=StringValue(value=Alice Johnson),
  type=StringValue(value=String)}),
  active=ObjectValue(value={type=StringValue(value=Boolean),
  value=BooleanValue(value=true)}),
  id=StringValue(value=\_tEBrM_1i_6sscs3DmX1zftvuiGQLkRn7ACiU1R-e-\_aLoNA_Tr_xEHT8HNFPFHtMki7voUz-SZHG5dixvdFaA),
  email=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=alice@example.com)}),
  city=ObjectValue(value={value=StringValue(value=San Francisco),
  type=StringValue(value=String)}),
  age=ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=29)})})

=== Delete === ✓ Deleted user with ID:
\_tEBrM_1i_6sscs3DmX1zftvuiGQLkRn7ACiU1R-e-\_aLoNA_Tr_xEHT8HNFPFHtMki7voUz-SZHG5dixvdFaA

✓ Confirmed user was deleted

=== Cleanup === ✓ Deleted collection: kotlin_users_example

=== Example Complete ===

BUILD SUCCESSFUL in 4s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Batch Operations Example ===

=== Batch Insert === ✓ Inserted 5 records IDs:
HZhnXHhnxh_OAEia19ipf-fmQnxCG537U_s96EqHebf-2T6Qd7G1eo1J2N2wNUOuVVGN03VrLLZ9FYd1nxLt_w,
XSCAtJXyF5-cASbcKU1WgEoN5P76J9Ut5sX4UH3QTg6MqWVVrCNhfg4Cy5DkO3XRWrRXrShPzhCnRo5R7EG2nA,
WtNVmIrnRcpAJUM515JhB7IKb3SEWK4wkc24OVrHvEKVhuMvcORp6NOstSCixQhwi8JTbp4aZgeD6clRrci3gw...

=== Batch Update === ✓ Updated 3 records

=== Batch Delete === ✓ Deleted 2 records

=== Cleanup === ✓ Deleted collection: kotlin_batch_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Query Builder Example ===

=== Inserting Sample Data === ✓ Inserted 5 records

=== Query 1: Equality (city = 'NYC') === ✓ Found 2 records in NYC

=== Query 2: Range (age >= 25 AND age < 32) === ✓ Found 3 records with age 25-31

=== Query 3: Sort by score (descending) === ✓ Top 3 scores:

- Score: ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=95)})
- Score: ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=92)})
- Score: ObjectValue(value={value=IntegerValue(value=88),
  type=StringValue(value=Integer)})

=== Query 4: Complex (score > 80 AND age >= 25) === ✓ Found 4 high-scoring
adults

=== Query 5: IN (city IN ['NYC', 'LA']) === ✓ Found 4 records in NYC or LA

=== Query 6: Pagination (skip 2, limit 2) === ✓ Page 2 (2 records):

- ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=Charlie)})
- ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=Diana)})

=== Query 7: Contains (name contains 'a') === ✓ Found 2 names containing 'a'

=== Cleanup === ✓ Deleted collection: kotlin_query_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - KV Operations Example ===

=== KV Set === ✓ Set key: user:123

=== KV Get === ✓ Retrieved value:
{"name":"Alice","email":"alice@example.com","role":"admin"}

=== KV Set with TTL === ✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key === ✓ Session value:
{"created_at":1762748756711,"user_id":"123"} (Will expire in 10 seconds)

=== Set Multiple Keys === ✓ Set 3 configuration keys

=== KV Delete === ✓ Deleted key: user:123

=== Verify Deletion === ✓ Confirmed key was deleted

=== Cleanup === ✓ Cleaned up test keys

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Collection Management Example ===

=== List Collections === ✓ Found 20 collections

- ws_ttl_test
- schema_documents_client_ts
- schema_products_client_ts
- schema_documents_client_py
- batch_users

=== Check Collection Existence === Collection 'kotlin_collection_example'
exists: false

=== Create Collection with Schema === ✓ Created collection with schema:
kotlin_collection_example

=== Get Collection Schema === ✓ Schema:
{"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-11-10T04:26:01.051124Z","last_modified":"2025-11-10T04:26:01.051124Z","bypass_ripple":false,"primary_key_alias":"id"}

=== Cleanup === ✓ Deleted collection: kotlin_collection_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Document TTL Example ===

=== Insert with TTL === ✓ Inserted document with 10s TTL Document ID:
f6ch19BesiNFelUC7Ky_OKz2YzlK7atDp1LcdCnSCphIVWeI2GlKK2cV2WHkS5EmCjlr56t4pjf7xUMMxwR_Kw

=== Verify Document Exists === ✓ Document found: [session_id, user_id,
created_at, ttl_duration, id]

=== Insert with Longer TTL === ✓ Inserted document with 1h TTL Document ID:
C2E8E_tCmYbfxFpJMkndllemj8BYguDxSzjV4azYtg_rU1tVLqbMgy7Mb_VVCPUTMjtKh1JPunwxHhP9UptvZw

=== TTL Expiration === ✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document === ✓ Long TTL document still exists: [cache_key,
ttl_duration, id, value]

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection: kotlin_ttl_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client === ✓ WebSocket client created

=== Connect to WebSocket === ✓ Connected to WebSocket

=== Insert Test Record === ✓ Inserted test record

=== Find All via WebSocket === ✓ WebSocket findAll result:
{"payload":{"data":[{"id":"kGfCF9pQ7ePUqUPkbKznH87-4H-QamAVXOqKVu4c8U548DEe0uh0cij6-HXze_XFE9jQPTTdwKzLFT4eQWjEcQ","name":{"type":"String","value":"Test
User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

=== Close WebSocket === ✓ WebSocket closed

=== Cleanup === ✓ Deleted collection: kotlin_websocket_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Schema Management Example ===

=== Creating Collection with Schema === ✓ Created collection
'kotlin_schema_example' with schema

=== Inserting Valid Documents === ✓ Inserted user 1:
StringValue(value=jIvZ3Q6JRue2BWimxk2CRugNR406VeMU6i4hQPBY5c8ncWd8FiCTuumwKCigA6yltldiOpraEweqyl2lTj3yQg)
✓ Inserted user 2:
StringValue(value=D8FS8yH38jK1-kJRpiM0w4DgybsE8vzPllnGoZTLPNTrHVn0vwN8GoGXtp6iTnYCXPAX5pVoRtg8LbQ9-mE3Ww)

=== Getting Schema === ✓ Schema for kotlin_schema_example: Fields:
{"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections === ✓ Total collections: 21 Sample: [ws_ttl_test,
schema_documents_client_ts, schema_products_client_ts,
schema_documents_client_py, batch_users]

=== Cleanup === ✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Search Example ===

=== Inserting Sample Documents === ✓ Inserted 5 sample documents

=== Basic Text Search === ✓ Search results for 'programming':
{"results":[{"record":{"id":"aFOkfAhhtlrZXm7WSuVIJj39lbFuFIk-ET2H1m0yxICNSUke4K6StkT1ra1m9cia-\_nDbMNwl_wUVf8m29YCbw","views":365,"tags":"programming,rust,tutorial","title":"Rust
Programming","description":"Learn Rust programming language with hands-on
examples and best
practices."},"score":19.8,"matched_fields":["tags","description","title"]},{"record":{"id":"3wxQ7W9RmrIoNLN_EyVcTMFZhrL6XpAsilfDhFjrZNOKKihgfmFSCxNFWb45F6wP8BI5KBCjOLRcqjITuGC8dQ","tags":"programming,javascript,web","title":"JavaScript
Web Development","description":"Build modern web applications using JavaScript,
React, and
Node.js.","views":185},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"tags":"programming,python,data-science","id":"ju7d0tPIB1H9bgE1JvWlV6Gl1QKxUHG0DEkX2WUO46B07iG_tbpwaZ6RND-CUYn4opyhUDaHh2eIP_gQuOkTLA","views":717,"title":"Python
for Data Science","description":"Master Python for data analysis, machine
learning, and
visualization."},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":2}

=== Search for 'machine learning' === ✓ Found results for 'machine learning'
{"results":[{"record":{"tags":"ai,machine-learning,python","id":"X1YkXcI8fk3mFfUkSYZh9ZDCmswFh0VhTzzmmCA3CFoLGaI-6urx8GWQLWQgUCsC4tEEbBjJbTn-d2I6IYbs2w","description":"Introduction
to machine learning algorithms and neural networks.","views":88,"title":"Machine
Learning
Basics"},"score":2.7,"matched_fields":["tags","title","description"]},{"record":{"description":"Master
Python for data analysis, machine learning, and
visualization.","id":"ju7d0tPIB1H9bgE1JvWlV6Gl1QKxUHG0DEkX2WUO46B07iG_tbpwaZ6RND-CUYn4opyhUDaHh2eIP_gQuOkTLA","title":"Python
for Data
Science","views":717,"tags":"programming,python,data-science"},"score":1.0,"matched_fields":["description"]},{"record":{"id":"aFOkfAhhtlrZXm7WSuVIJj39lbFuFIk-ET2H1m0yxICNSUke4K6StkT1ra1m9cia-\_nDbMNwl_wUVf8m29YCbw","views":365,"description":"Learn
Rust programming language with hands-on examples and best
practices.","tags":"programming,rust,tutorial","title":"Rust
Programming"},"score":0.5,"matched_fields":["description"]},{"record":{"id":"ax4rSpkbv99SH7pwmjsbyC4cg770jnUmG8OOujO88r78E4rR1l2vb_JJynT4Ift2V4MYG5V5o6PLjtvbiB5e7g","views":337,"description":"Learn
database design principles, normalization, and query
optimization.","tags":"database,design,sql","title":"Database
Design"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

=== Cleanup === ✓ Deleted collection: kotlin_search_example

✓ All search operations completed successfully

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - WebSocket TTL Example ===

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
StringValue(value=3Gbn1Evjg7_FSiVxo0_YLU9GxVy_GE2wBayWLO9jh3hbqu4qjMsF8k_vjtrqpyreBo6P4FU8iWlE9ocETh81Vw)

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved data via
WebSocket:
{"payload":{"data":[{"created_at":{"type":"Integer","value":1762748782352},"id":"3Gbn1Evjg7_FSiVxo0_YLU9GxVy_GE2wBayWLO9jh3hbqu4qjMsF8k_vjtrqpyreBo6P4FU8iWlE9ocETh81Vw","name":{"type":"String","value":"WebSocket
TTL
Test"},"ttl_duration":{"type":"String","value":"1h"},"value":{"type":"Integer","value":42}}]},"type":"Success"}

✓ WebSocket closed

=== Cleanup === ✓ Deleted collection: kotlin_websocket_ttl_example

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Join Operations Example ===

=== Setting up sample data === ✓ Sample data created

=== Example 1: Single collection join (users with departments) === ✓ Found 2
users with department data

- Bob Smith: Sales
- Alice Johnson: Engineering

=== Example 2: Join with filtering === ✓ Found 1 users in Engineering

- Alice Johnson: Building A

=== Example 3: Join with user profiles === ✓ Found 2 users with profile data

- Bob Smith: Sales Manager
- Alice Johnson: Senior Software Engineer

=== Example 4: Join orders with user data === ✓ Found 2 completed orders

- Laptop ($1200) by Alice Johnson
- Mouse ($25) by Alice Johnson

=== Example 5: Complex join with multiple conditions === ✓ Found 2 users with
example.com emails

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✓ Deleted test collections

✓ Join operations example completed successfully

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
aoqsNvhg3f7WEys2YitiiZh3ebBnczrfuMzHAIs79arXBZcprkdwrCs34DZlXx04MITED4pLjOzyj-aVDPZWKQ

=== Sending Chat Message === ✓ Chat response: Message ID:
"\_txrJR2ElUYqZPK8NBXHzSXwgbeWE6teB4euUGmYlQfaXFRjW0Jox8pywu42S805DC3oeVfvt3uHcMRdJ1AV1w"
Responses: ["ekoDB is a high-performance database that comes with intelligent
caching, real-time capabilities, and AI integration. It offers several key
features to enhance your data management experience. One of these features is an
AI Chat Integration, which allows you to query your database using natural
language and receive AI-powered responses with relevant context. Additionally,
ekoDB also supports different types of search features like full-text search,
vector search, and hybrid search with automatic context retrieval."]

=== Cleanup === ✓ Deleted chat session ✓ Deleted collection:
kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 6s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
0cLXBjBnOjbnQ5t2Vf2UYgQqvA0ieGKY6UfJju5hwURin-weSG0gUHgwyq2D1zb4Qfw1xaSW8dhEliFPwLj-Sg

=== Sending Messages === ✓ Message 1 sent Responses: ["Based on the information
provided, the available product is ekoDB, a high-performance database product
with AI capabilities. It is priced at $99."]

✓ Message 2 sent Responses: ["As an assistant, I'm sorry but I can't provide the
price information because there's no context or product specified in your
question. Could you please specify the product you're interested in?"]

=== Getting Message History === ✓ Retrieved message history Total messages:
[{"chat_id":{"type":"String","value":"0cLXBjBnOjbnQ5t2Vf2UYgQqvA0ieGKY6UfJju5hwURin-weSG0gUHgwyq2D1zb4Qfw1xaSW8dhEliFPwLj-Sg"},"content":{"type":"String","value":"What
products are
available?"},"context_snippets":{"type":"Array","value":[{"type":"Object","value":{"collection":{"type":"String","value":"kotlin_chat_sessions_example"},"matched_fields":{"type":"Array","value":[{"type":"String","value":"description"}]},"record":{"type":"Object","value":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":{"type":"String","value":"RQgknh2edyapWD8bXbAVMwwEZrcrvbDZBCEhljy23QSsCryCrIMR0dwaf3nayt87XbL45pIFqzSQi0PX8yjtRw"},"price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}}},"score":{"type":"Float","value":0.25}}}]},"created_at":{"type":"DateTime","value":"2025-11-10T04:26:40.672365Z"},"id":"TzjOMsb_BUYsEYEAplSgl49z9QtgweK1MbWSqGdZVVHLW1yZHwCOfN_nqygM8uiV2HAy33UzNbjlMsLixXq41g","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":30},"prompt_tokens":{"type":"Integer","value":164},"total_tokens":{"type":"Integer","value":194}}},"updated_at":{"type":"DateTime","value":"2025-11-10T04:26:40.672365Z"}},{"chat_id":{"type":"String","value":"0cLXBjBnOjbnQ5t2Vf2UYgQqvA0ieGKY6UfJju5hwURin-weSG0gUHgwyq2D1zb4Qfw1xaSW8dhEliFPwLj-Sg"},"content":{"type":"String","value":"Based
on the information provided, the available product is ekoDB, a high-performance
database product with AI capabilities. It is priced at
$99."},"context_snippets":{"type":"Array","value":[{"type":"Object","value":{"collection":{"type":"String","value":"kotlin_chat_sessions_example"},"matched_fields":{"type":"Array","value":[{"type":"String","value":"description"}]},"record":{"type":"Object","value":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":{"type":"String","value":"RQgknh2edyapWD8bXbAVMwwEZrcrvbDZBCEhljy23QSsCryCrIMR0dwaf3nayt87XbL45pIFqzSQi0PX8yjtRw"},"price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}}},"score":{"type":"Float","value":0.25}}}]},"created_at":{"type":"DateTime","value":"2025-11-10T04:26:40.674766Z"},"id":"5cWe_z4taVbebVWOsHR_OmwMPiqOxAgiUaiTuidtuJD5vqILArjG4g4fma6n5Lg2N61uzyDdYzpSaV38QlCVPg","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":30},"prompt_tokens":{"type":"Integer","value":164},"total_tokens":{"type":"Integer","value":194}}},"updated_at":{"type":"DateTime","value":"2025-11-10T04:26:40.674766Z"}},{"chat_id":{"type":"String","value":"0cLXBjBnOjbnQ5t2Vf2UYgQqvA0ieGKY6UfJju5hwURin-weSG0gUHgwyq2D1zb4Qfw1xaSW8dhEliFPwLj-Sg"},"content":{"type":"String","value":"What
is the
price?"},"created_at":{"type":"DateTime","value":"2025-11-10T04:26:42.422992Z"},"id":"UVoFcsQwWEKUQ52WIl31fi1FoQ2xVb6H9Wz9f7uJ3Ah4Ino3rp8XYGhXqXPa-HlHPuTEtwsxJJntHx_UEj-pSA","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":38},"prompt_tokens":{"type":"Integer","value":63},"total_tokens":{"type":"Integer","value":101}}},"updated_at":{"type":"DateTime","value":"2025-11-10T04:26:42.422992Z"}},{"chat_id":{"type":"String","value":"0cLXBjBnOjbnQ5t2Vf2UYgQqvA0ieGKY6UfJju5hwURin-weSG0gUHgwyq2D1zb4Qfw1xaSW8dhEliFPwLj-Sg"},"content":{"type":"String","value":"As
an assistant, I'm sorry but I can't provide the price information because
there's no context or product specified in your question. Could you please
specify the product you're interested
in?"},"created_at":{"type":"DateTime","value":"2025-11-10T04:26:42.427073Z"},"id":"8Kwve4Y21UY23tWRRWtZPz-0YdxEJKCPofXXYzXTpB8qrk9u-6y6ms96L-kw-sncTnW5h4ggdd2B2TQ8tf2ZFQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":38},"prompt_tokens":{"type":"Integer","value":63},"total_tokens":{"type":"Integer","value":101}}},"updated_at":{"type":"DateTime","value":"2025-11-10T04:26:42.427073Z"}}]

=== Updating Session === ✓ Updated session system prompt

=== Listing Sessions === ✓ Total sessions: 13

=== Branching Session === ✓ Created branched session:
runvjCWvRnzRkT1_Dh7xfjBMATfqe_dMXkCPLdPKJU6EgstALh3gUe_kng9j5wGaN9H7a3xR5nhBIZGkfqTXDA

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 7s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
WmDjPlt7iquWWyh_K1H_nujT3jSsE6O9yq-wqyoo1B3pH3kVhHHn4qLURWB4_WhkTv3pTmA1HWl4JKbjEDnx1Q

=== Sending Initial Message === ✓ Message sent Responses: ["Based on the context
provided, the available product is ekoDB. It is a high-performance database
product priced at $99."]

✓ Second message sent

=== Regenerating AI Response === ✓ AI response regenerated New responses:
["Based on the provided context, there is one product available which is ekoDB.
This is a high-performance database product priced at $99."]

=== Updating Message === ✓ Updated message content

=== Toggling Forgotten Status === ✓ Marked message as forgotten (excluded from
context)

=== Creating Second Session for Merge === ✓ Created second session:
JpvN6hSpKUZ_0tUtNRFHDbX9a0VLs4-9DIIH0KsyoRzLKqux-XG05v93orWHyM9yAMnLGc5S9Z3I85t0bnsqQQ

=== Merging Sessions === ✓ Merged sessions Merged session ID: null

=== Deleting Message === ✓ Deleted message

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 9s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run 🚀 ekoDB Scripts Example (Kotlin Client)

SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to no-operation
(NOP) logger implementation SLF4J(W): See
https://www.slf4j.org/codes.html#noProviders for further details. ✅ Client
initialized

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
bwTa8Hx3oleryse0fgxNE-4Lt1lsLYzhxB3IWjS2gTOD559WU6IXsu-MLDKf_1I-HT-RipN--pY6OooW57r9Qg
📊 Found 10 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 10 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"status":"active","count":5,"avg_score":60.0}
{"status":"inactive","count":5,"avg_score":50.0} ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 19 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved 📊 Pipeline executed 2 stages ⏱️ Total execution
time: 0ms 📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved 📊 Total user count: 10 ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Deleted collection ✅ Deleted test scripts

✅ All examples completed successfully!

💡 Key Advantages of Using the Client: • Automatic token management • Type-safe
Stage builders • Built-in error handling

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date ✅
[32mKotlin client examples complete![0m ✅ [32mAll Kotlin integration tests
complete![0m
