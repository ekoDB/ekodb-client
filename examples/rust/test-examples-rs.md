make test-examples-rust 🧪 [36mRunning Rust examples (direct
HTTP/WebSocket)...[0m Compiling ekodb_client v0.3.0
(/Users/tek/Development/ekoDB/ekodb-client/ekodb_client) Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.89s Running
`target/debug/examples/simple_crud` ✓ Authentication successful

=== Insert Document === Inserted: Object {"id":
String("YcKTEP2nCBSXuJOSArIQRMinazcVcSEQ_ZQVuyIF03lSaQm7RbELzsQs7iHwn-lLG_HEfYcMHnfsbrJ-q_58DQ")}

=== Find by ID === Found: Object {"active": Object {"type": String("Boolean"),
"value": Bool(true)}, "id":
String("YcKTEP2nCBSXuJOSArIQRMinazcVcSEQ_ZQVuyIF03lSaQm7RbELzsQs7iHwn-lLG_HEfYcMHnfsbrJ-q_58DQ"),
"name": Object {"type": String("String"), "value": String("Test Record")},
"value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query === Found documents: Array [Object {"active": Object
{"type": String("Boolean"), "value": Bool(true)}, "id":
String("YcKTEP2nCBSXuJOSArIQRMinazcVcSEQ_ZQVuyIF03lSaQm7RbELzsQs7iHwn-lLG_HEfYcMHnfsbrJ-q_58DQ"),
"name": Object {"type": String("String"), "value": String("Test Record")},
"value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document === Updated: Object {"active": Object {"type":
String("Boolean"), "value": Bool(true)}, "id":
String("YcKTEP2nCBSXuJOSArIQRMinazcVcSEQ_ZQVuyIF03lSaQm7RbELzsQs7iHwn-lLG_HEfYcMHnfsbrJ-q_58DQ"),
"name": Object {"type": String("String"), "value": String("Updated Record")},
"value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.42s Running
`target/debug/examples/simple_websocket` ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
"6zgKLocRk8WYRfFz-J1dF0GPcsGdsemFXr7AyajGSwtUPG0vJlbWvqUCsZlMtjaTUx4KY3S3gw_ebP0Jy8A1kQ"

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"6zgKLocRk8WYRfFz-J1dF0GPcsGdsemFXr7AyajGSwtUPG0vJlbWvqUCsZlMtjaTUx4KY3S3gw_ebP0Jy8A1kQ",
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
[unoptimized + debuginfo] target(s) in 0.32s Running
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
[unoptimized + debuginfo] target(s) in 0.33s Running
`target/debug/examples/collection_management` ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
"dnsqWcK7NsHWxvLOxswwrkZq9tWOugDNY056weCphVlAaH9bcfEBWN3_Lz1VpTQHBMD2_DhAhsJ_wMITDnLlbQ"

=== List Collections === Total collections: 4 Sample collections:
["websocket_test", "test_collection", "batch_users", "demo_collection"]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s Running
`target/debug/examples/document_ttl` ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
"fViTq8-mUrzk1L2y8_IzMvVLagRx2IyDELrEsVjLGAHR11faMr2yz4Af2Lq66gFnytZ7Wxym74RTmQIpIb-IIQ"

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: null

=== Query Documents === ✓ Found 1 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

✓ All document TTL operations completed successfully Compiling ekodb-examples
v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.38s Running
`target/debug/examples/websocket_ttl` ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
String("-rYgA3lsPnUDghh_PrnzL31GqxB2MpF_MUlxWwEI5-D4pRYc2KwaarZVkChScn02s_hoC3Wq2Ar2z1nQMKGZBA")

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully ✅ [32mRust direct examples
complete![0m 🛠️ [36mBuilding client library...[0m cargo build -p ekodb_client
Compiling ekodb_client v0.3.0
(/Users/tek/Development/ekoDB/ekodb-client/ekodb_client) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 1.29s ✅ [32mClient build complete![0m 🧪
[36mRunning Rust client library examples...[0m Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.66s Running
`target/debug/examples/client_simple_crud` ✓ Client created (token exchange
happens automatically)

=== Insert Document === Inserted: Record { fields: {"id":
String("98Riqjla-nvYqghwyGMzX2tRyhk2JbWh_n0X8eWV67DXNLDP0x31utL3grhKR0XCNc5PZxEGM6KM0Xs1w8hhbg")}
}

=== Find by ID === Found: Record { fields: {"value": Object({"value":
Integer(42), "type": String("Integer")}), "id":
String("98Riqjla-nvYqghwyGMzX2tRyhk2JbWh_n0X8eWV67DXNLDP0x31utL3grhKR0XCNc5PZxEGM6KM0Xs1w8hhbg"),
"active": Object({"value": Boolean(true), "type": String("Boolean")}), "name":
Object({"type": String("String"), "value": String("Test Record")})} }

=== Find with Query === Found documents: [Record { fields: {"value":
Object({"type": String("Integer"), "value": Integer(42)}), "id":
String("98Riqjla-nvYqghwyGMzX2tRyhk2JbWh_n0X8eWV67DXNLDP0x31utL3grhKR0XCNc5PZxEGM6KM0Xs1w8hhbg"),
"active": Object({"value": Boolean(true), "type": String("Boolean")}), "name":
Object({"value": String("Test Record"), "type": String("String")})} }]

=== Update Document === Updated: Record { fields: {"value": Object({"value":
Integer(100), "type": String("Integer")}), "name": Object({"type":
String("String"), "value": String("Updated Record")}), "id":
String("98Riqjla-nvYqghwyGMzX2tRyhk2JbWh_n0X8eWV67DXNLDP0x31utL3grhKR0XCNc5PZxEGM6KM0Xs1w8hhbg"),
"active": Object({"value": Boolean(true), "type": String("Boolean")})} }

=== Delete Document === Deleted document

=== Cleanup === ✓ Deleted collection

✓ All CRUD operations completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.49s Running
`target/debug/examples/client_batch_operations` ✓ Client created

=== Batch Insert (via multiple inserts) === ✓ Inserted 5 records ✓ Verified:
Found 5 total records in collection

=== Update Records === ✓ Updated 3 records

=== Delete Records === ✓ Deleted 3 records

=== Cleanup === ✓ Deleted collection

✓ All batch operations completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.41s Running
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
[unoptimized + debuginfo] target(s) in 0.52s Running
`target/debug/examples/client_collection_management` ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
"nYzu8QpfcDQG496rRmJBvhoXqJ7tBYqAyeOjX1n9ua5P8XekJyKHE2frXU8NzaiPTVv6iuL5Os0pGslQ4kxUXg"

=== List Collections === Total collections: 6 Sample collections:
["websocket_test", "test_collection", "ws_ttl_test",
"client_collection_management_rust", "ttl_cache"]

=== Count Documents === Document count: 1

=== Check Collection Exists === Collection exists: true

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.49s Running
`target/debug/examples/client_document_ttl` ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
"qNwGwhxzCDZn8YPf7aPmsP89J9_Zb5eDjQzz459q-XknX8zdvLp3n7HMLbkFbSwCV1EG1LpZLrZ9QOxpgVX5Tg"

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
Some(String("SJk0txLqPpo6riGbe5c6Xodt0ZcZqA20Ehi1UcAXh5bR3GYcBfs6WCSydqzJuQCJl3RTDJZAyQ3WhT-rtOX2fw"))

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.52s Running
`target/debug/examples/client_simple_websocket` ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
p7XoQeBTtYYpRaFBVvQ6eyaHTELfiH9wJmQdN3wXi4BFgmPmmlROhFNJ8FIGWhbnY-02Yg8UEmCIz8d71BVEqQ

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.53s Running
`target/debug/examples/client_websocket_ttl` ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
Some(String("cIHuFfOWvdB_FRh-eZNEOX2LoiHCifLgQE1q61nOLMEynt2lpFH6m8bMOW4Gzmieb-Ue5uFNJKCr7Xk3hQ-jcQ"))

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.51s Running
`target/debug/examples/client_query_builder` === ekoDB Query Builder Example ===

=== Inserting Sample Data === ✓ Inserted 5 users

=== Simple Equality Query === ✓ Found 3 active users

- Some(Object({"value": String("David"), "type": String("String")}))
- Some(Object({"value": String("Bob"), "type": String("String")}))
- Some(Object({"value": String("Alice"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) === ✓ Found 3 users in age range

- Some(Object({"value": String("David"), "type": String("String")}))
- Some(Object({"value": String("Eve"), "type": String("String")}))
- Some(Object({"value": String("Bob"), "type": String("String")}))

=== IN Operator === ✓ Found 4 users with status active or pending

- Some(Object({"type": String("String"), "value": String("David")}))
- Some(Object({"type": String("String"), "value": String("Alice")}))
- Some(Object({"value": String("Bob"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("Eve")}))

=== NOT IN Operator === ✓ Found 4 users not inactive

=== String Pattern Matching === ✓ Found 5 users with @example.com email

=== Regex Query === ✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) === ✓ Found 2 users
matching all conditions

- Some(Object({"value": String("Bob"), "type": String("String")}))
- Some(Object({"value": String("David"), "type": String("String")}))

=== OR Query === ✓ Found 2 users with age < 28 OR age > 32

- Some(Object({"type": String("String"), "value": String("Charlie")}))
- Some(Object({"value": String("Alice"), "type": String("String")}))

=== Sorted Query (by score descending) === ✓ Top 3 users by score:

1. Some(Object({"type": String("String"), "value": String("Bob")}))
2. Some(Object({"type": String("String"), "value": String("David")}))
3. Some(Object({"type": String("String"), "value": String("Alice")}))

=== Pagination (page 2, size 2) === ✓ Page 2 results:

- Some(Object({"type": String("String"), "value": String("Charlie")}))
- Some(Object({"type": String("String"), "value": String("David")}))

=== NOT Operator === ✓ Found 4 users NOT inactive

=== Complex Chained Query === ✓ Found 3 users with all conditions

=== Cleanup === ✓ Deleted collection

✓ All query builder operations completed successfully Compiling ekodb-examples
v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.50s Running
`target/debug/examples/client_search` === ekoDB Search Example ===

=== Inserting Sample Documents === ✓ Inserted 5 sample documents

=== Basic Text Search === ✓ Found 3 results for 'programming'

1. Score: 19.8000 - Some(String("Rust Programming"))
2. Score: 6.6000 - Some(String("JavaScript Web Development"))
3. Score: 6.6000 - Some(String("Python for Data Science"))

=== Fuzzy Search === ✓ Found 3 results for 'progamming' (typo)

1. Score: 1.6500 - Some(String("Rust Programming"))
2. Score: 0.5500 - Some(String("JavaScript Web Development"))
3. Score: 0.5500 - Some(String("Python for Data Science"))

=== Field-Specific Search === ✓ Found 4 results in title/description

1. Score: 4.0000 Title: Some(String("Machine Learning Basics")) Matched:
   ["title.value", "title", "description", "description.value"]
2. Score: 2.0000 Title: Some(String("Python for Data Science")) Matched:
   ["description.value", "description"]
3. Score: 1.0000 Title: Some(String("Rust Programming")) Matched:
   ["description.value", "description"]
4. Score: 1.0000 Title: Some(String("Database Design")) Matched: ["description",
   "description.value"]

=== Weighted Search === ✓ Found 2 results with field weights

1. Score: 23.1000 - Some(String("Python for Data Science"))
2. Score: 3.3000 - Some(String("Machine Learning Basics"))

=== Advanced Search Options === ✓ Found 1 results with stemming

1. Score: 19.8000 - Some(String("Database Design"))

=== Search with Limit === ✓ Limited to 2 results (requested 2)

1. Score: 19.8000 - Some(String("Rust Programming"))
2. Score: 6.6000 - Some(String("JavaScript Web Development"))

=== Cleanup === ✓ Deleted collection

✓ All search operations completed successfully Execution time: 1ms Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.74s Running
`target/debug/examples/client_schema_management` === ekoDB Schema Management
Example ===

=== Creating Collection with Schema === ✓ Created collection
'schema_client_rust' with schema

=== Inserting Valid Documents === ✓ Inserted user 1:
Some(String("vz02BixbmFgtJpoR0QGKgVBLOO-hcj9jZ1Y3r7COcdSqCODAaF4v0vQfeFX-wXOarsE2t96tuY-NljR4FJQvZw"))
✓ Inserted user 2:
Some(String("hDGLlsicGsJzJ5DsTHIjrDFes2Y_NHZv1zrZdkjvwcP3i8QYbNqKPaIHpr-R3rzhEDLjOa9Ko_WcumKwfRSYDw"))

=== Getting Schema === ✓ Schema for schema_client_rust:

- age: Integer
- title: String (required)
- status: String
- email: String (required)

=== Listing Collections === ✓ Total collections: 6 Sample: ["websocket_test",
"test_collection", "ws_ttl_test", "schema_client_rust", "ttl_cache"]

=== Cleanup === ✓ Deleted collection

✓ All schema management operations completed successfully Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.52s Running
`target/debug/examples/client_joins` === ekoDB Rust Client - Join Operations
Example ===

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

✓ Join operations example completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.47s Running
`target/debug/examples/client_chat_basic` === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
loG91Ays_vwuIn8t-PAJR1kUYrlHQrvC6otIhAo144cYL7njJImlr4otR0lH0zbYX88T-9HBdOARxHSBjujdyQ

=== Sending Chat Message === Message ID:
rDT9k4ldgXm5o6SBpB_RFMqY7IgSXyk_OdAxc5Wubp9YLp3FTIN-ayjrsCK2MnY7yfFeIBxMDJRH61Ks8IdNXw

=== AI Response === Response 1: ekoDB is a high-performance database that has
intelligent caching and real-time capabilities. It is characterized by its AI
integration which enhances its functionality. Some of the key features of ekoDB
include a unique AI Chat Integration feature that allows you to query your
database using natural language and receive AI-powered responses with relevant
context. Additionally, ekoDB supports diverse search features including
full-text search, vector search, and hybrid search with automatic context
retrieval.

=== Context Used (3 snippets) ===

Snippet 1: Collection: client_chat_basic_rust Score: 0.6333 Matched Fields:
["title", "category", "content"] Record: Object {"category": Object {"type":
String("String"), "value": String("features")}, "content": Object {"type":
String("String"), "value": String("The chat feature allows you to query your
database using natural language and get AI-powered responses with relevant
context.")}, "id":
String("4V-LA5ajU6TKj0dU_CFUqHV7ZKad3ClB4TPTzFTz_7PZXgPVZJWQmaR0swgqiBzyKC-iqUaAA9RfCuszdmrqfQ"),
"title": Object {"type": String("String"), "value": String("AI Chat
Integration")}}

Snippet 2: Collection: client_chat_basic_rust Score: 0.5222 Matched Fields:
["content", "title", "category"] Record: Object {"category": Object {"type":
String("String"), "value": String("features")}, "content": Object {"type":
String("String"), "value": String("ekoDB supports full-text search, vector
search, and hybrid search with automatic context retrieval.")}, "id":
String("QuuEoUILHg8qcLSGMYTa8PSKGoEvj2yOIO8SP1shNb_Coz1Im9Zsa2w96V1SrxZNG0aHvBv1gHxsxsuEHwaRlw"),
"title": Object {"type": String("String"), "value": String("Search Features")}}

Snippet 3: Collection: client_chat_basic_rust Score: 0.5222 Matched Fields:
["content", "title"] Record: Object {"category": Object {"type":
String("String"), "value": String("documentation")}, "content": Object {"type":
String("String"), "value": String("ekoDB is a high-performance database with
intelligent caching, real-time capabilities, and AI integration.")}, "id":
String("5YHDCpUKjXzUrIcNQEh1r3qGDTPJHTelNCUIrIl5jE1DxVcFMhO70Jepc7-11qVkh0rjMEjPOkOIZgbs_7PUPA"),
"title": Object {"type": String("String"), "value": String("Introduction to
ekoDB")}}

Execution Time: 3024ms

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.50s Running
`target/debug/examples/client_chat_sessions` === ekoDB Chat Session Management
Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
IMktZywTlmF0uIb-kYLw2l5LJ8xdJAS4dx4xY-xhsemWgut5OpK7B58NbOzVhALjnXVTKhRhBdBFn8P3nJkgUw
=== Sending Messages === ✓ Message 1 sent Response: Based on the provided
context, the available product is ekoDB. It's a high-performance database
product with AI capabilities. The price for this product is $99.

✓ Message 2 sent Response: Apologies, but there's no context provided about any
specific product. Could you please specify the product you're asking about?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
dsPYUZ5hNL-nv3g3HUj8wDLPz0RyTwiMLUSk1vNAba1hnK7MPqiJh3d1WaAwq2EvxCxPuRcU9wcfgBMEsgwOUQ
Parent:
IMktZywTlmF0uIb-kYLw2l5LJ8xdJAS4dx4xY-xhsemWgut5OpK7B58NbOzVhALjnXVTKhRhBdBFn8P3nJkgUw

=== Listing Sessions === ✓ Found 3 sessions Session 1:
dsPYUZ5hNL-nv3g3HUj8wDLPz0RyTwiMLUSk1vNAba1hnK7MPqiJh3d1WaAwq2EvxCxPuRcU9wcfgBMEsgwOUQ
(Untitled) Session 2:
IMktZywTlmF0uIb-kYLw2l5LJ8xdJAS4dx4xY-xhsemWgut5OpK7B58NbOzVhALjnXVTKhRhBdBFn8P3nJkgUw
(Untitled) Session 3:
loG91Ays_vwuIn8t-PAJR1kUYrlHQrvC6otIhAo144cYL7njJImlr4otR0lH0zbYX88T-9HBdOARxHSBjujdyQ
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
dsPYUZ5hNL-nv3g3HUj8wDLPz0RyTwiMLUSk1vNAba1hnK7MPqiJh3d1WaAwq2EvxCxPuRcU9wcfgBMEsgwOUQ

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.50s Running
`target/debug/examples/client_chat_advanced` === ekoDB Advanced Chat Features
Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
y5Mpt7SZT7ibSCweqzVvcmOulFjfnPWpoKx0E1GmWPNXev04WYOy3xSoa3bJjRFjOkiM59LRwU7tsQYDeELsTQ

=== Sending Initial Message === ✓ Message sent Response: Based on the context
provided, the available product is a high-performance database product named
ekoDB. It is priced at $99.

✓ Second message sent === Feature 1: Regenerate AI Response === ✓ Message
regenerated New response: As a product assistant, I'm sorry but the provided
context doesn't give information about the price of ekoDB. I recommend checking
the official website or contacting the sales department for the most accurate
and up-to-date price.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
6YIRrX0Nw2WBJcMCmpa--QKGb8NaTSFlXRB270pQqNBrlt02ojr7tXiJzdLdk7ZEnlBZo8f2JPHTeDJF5GkUnw
✓ Sent message in second session ✓ Sessions merged successfully Total messages
in merged session: 4

=== Feature 5: Delete Message === ✓ Message deleted

✓ Messages remaining: 3

=== Cleanup === ✓ Deleted session ✓ Deleted collection

✓ All advanced chat features demonstrated successfully! Compiling ekodb-examples
v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.60s Running
`target/debug/examples/client_functions` === ekoDB Rust Client - Functions
Example ===

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved:
p-l4IseeyTfES6bJYre6iDgLQqM3uYa8nJliQWD0UTnaZZZYJw65NyxGBPbvO9LXs6weawPmZD3EN6hNAzk_GA
📊 Found 10 active users

📝 Example 2: Parameterized Function

✅ Function saved:
3fMjmhb5reUGKG7GpjRGL1WSfqXO86wm\_\_bFRLcndV43eus5CjNI8lmmNaCgH9zE4MV9684SE6oruzp32JNajg
📊 Found 10 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved:
7-ucCr4pNtEC2BiJ6U7gKSsacA-QoK65_YdV4HFSHLaE0xZV6_2i2TPZf3vLpp6mEgJmiy75tFqTPBgGsfKN1A
📊 Statistics: 2 groups

📝 Example 4: Function Management

📋 Total functions: 3 🔍 Retrieved function: Get Active Users ✏️ Function
updated 🗑️ Function deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID returned from
save ℹ️ Only CALL can use either ID or label

✅ All examples completed! ✅ [32mRust client examples complete![0m
