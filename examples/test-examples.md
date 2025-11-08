make test-examples 🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
Compiling ekodb_client v0.3.0
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

✅ All examples completed! ✅ [32mRust client examples complete![0m ✅ [32mAll
Rust integration tests complete![0m 🧪 [36mRunning Python examples (direct
HTTP/WebSocket)...[0m ✓ Authentication successful

=== Insert Document === Inserted: {'id':
'8ZnDx-KsO-PWYgwIgKaoWy8L_F6OY8a0RUFFWHFXjod-iTgVjNtj2uWFGRtDDxgXzRyvMY9KculftjdRhzBdLA'}

=== Find by ID === Found: {'name': {'type': 'String', 'value': 'Test Record'},
'value': {'value': 42, 'type': 'Integer'}, 'active': {'value': True, 'type':
'Boolean'}, 'id':
'8ZnDx-KsO-PWYgwIgKaoWy8L_F6OY8a0RUFFWHFXjod-iTgVjNtj2uWFGRtDDxgXzRyvMY9KculftjdRhzBdLA'}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'value': {'type': 'Integer', 'value': 100},
'name': {'value': 'Updated Record', 'type': 'String'}, 'id':
'8ZnDx-KsO-PWYgwIgKaoWy8L_F6OY8a0RUFFWHFXjod-iTgVjNtj2uWFGRtDDxgXzRyvMY9KculftjdRhzBdLA',
'active': {'value': True, 'type': 'Boolean'}}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
3w36KS3micZ5Sx6JR842Xfz3tMv0TGF8mEiZuZQug0PjIq9F8NIVLPIicjsMLRFBoh5oyoGjV\_-pFNM8Vo2rBw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"3w36KS3micZ5Sx6JR842Xfz3tMv0TGF8mEiZuZQug0PjIq9F8NIVLPIicjsMLRFBoh5oyoGjV_-pFNM8Vo2rBw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"6zgKLocRk8WYRfFz-J1dF0GPcsGdsemFXr7AyajGSwtUPG0vJlbWvqUCsZlMtjaTUx4KY3S3gw_ebP0Jy8A1kQ",
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

=== KV Get === Retrieved value: {'userId': 123, 'username': 'john_doe'}

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: {'name': 'Product 1', 'price': 29.99}
cache:product:2: {'price': 39.989999999999995, 'name': 'Product 2'}
cache:product:3: {'name': 'Product 3', 'price': 49.989999999999995}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
gDW6t3xDFQrKt4FfZ1aTvwIkhN7ozGrVtyl5WEQq-u_apzmM0R4tNUkapDBn-WwUmcSJfxUxMhBM7tb4SpwQ2w

=== List Collections === Total collections: 10 Sample collections:
['functions__ek0_bench', 'websocket_test', 'chat_configurations__ek0_bench',
'users', 'test_collection']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Authentication
successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
IzNoC2nGghI6MDpw2lr2mIyFYVRphLt3FHpqxX5lTAeFhfbNCz1911XqgnasCanwe1Lm5R1rBphsGR2POvX_8w

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
WuF1pblBV917OTzOD0gpSqYrBpX8rBI7BN8kHVsikIr_demWC5RxxFf8_ZCwnNOBN8tkl8msLHIOWMpo-nABWw

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
Compiling ekodb_client v0.3.0
(/Users/tek/Development/ekoDB/ekodb-client/ekodb_client) Compiling
ekodb-client-py v0.3.0
(/Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py) Finished `release`
profile [optimized] target(s) in 10.76s 📦 Built wheel for abi3 Python ≥ 3.8 to
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
'mrFw0bquztGAo65nqe9fUNaaFPwPYNrP1nXP8V_sZhjTNNmNqhSgXWRhy39HC60DEnpHTCx6L95rU2M9ZIiKgQ'}

=== Find by ID === Found: {'id':
'mrFw0bquztGAo65nqe9fUNaaFPwPYNrP1nXP8V_sZhjTNNmNqhSgXWRhy39HC60DEnpHTCx6L95rU2M9ZIiKgQ',
'active': {'type': 'Boolean', 'value': True}, 'name': {'value': 'Test Record',
'type': 'String'}, 'value': {'value': 42, 'type': 'Integer'}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'active': {'type': 'Boolean', 'value': True},
'name': {'type': 'String', 'value': 'Updated Record'}, 'value': {'type':
'Integer', 'value': 100}, 'id':
'mrFw0bquztGAo65nqe9fUNaaFPwPYNrP1nXP8V_sZhjTNNmNqhSgXWRhy39HC60DEnpHTCx6L95rU2M9ZIiKgQ'}

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
"f1knd7NMYsGKKcfjjFGT_Vcn_zDWmYCslR5Vnh05nlQuGpSa_Ye1-jkzPCvg2-\_qkT2FajC_yvmWIJQ90ySN8A"

=== List Collections === Total collections: 10 Sample collections:
['functions__ek0_bench', 'websocket_test', 'chat_configurations__ek0_bench',
'users', 'test_collection']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
-0uadrn_Cphvq6_Tcm_v8f1HfVPr0VwaqNMD-ive02RLh94M-f1bGAb4UpuNl16dSSwppmI8d-9Woh0QFxapXw

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
4ep9d9NK1JvIchU4khaFbFMKQZwuYyai0LDG-oXQxd4VbTVRWzzaThX1zQluI9UHVVMJhLF5vyZwJfe1H7YGuw

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
Ne-kt8KSXEwOaw-TbKeTvP9XU9taDMgsYmZLmfA0k-aVAAaRs6oEwbrcgdLBD7ptWH1ymEvCJY4MKmc5sa2YLw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
Utga3rM8lMBZLxX52bFZoymlLs_z-08jQ0o87zNCBpAqjcB9qkBrIT2xC406peUIf-dWcY_LLo6Dr6vJIcY6qg

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
1. Score: 25.740, Matched: email.value, name, name.value, email
1. Score: 12.540, Matched: name.value, name

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: bio.value, title.value, title, bio
1. Score: 26.400, Matched: bio, title.value, title, bio.value
1. Score: 26.400, Matched: bio.value, title.value, bio, title
1. Score: 26.400, Matched: title, title.value, bio, bio.value

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: title.value, bio.value, bio, title
1. Score: 39.600, Matched: bio, title, bio.value, title.value
1. Score: 39.600, Matched: bio, title.value, title, bio.value
1. Score: 39.600, Matched: title, bio.value, bio, title.value

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio.value, bio

1. Search with stemming and exact match boosting: Found 2 results (matches: run,
   running, runs)
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio, bio.value

1. Vector search (semantic search): Found 0 semantically similar documents

1. Hybrid search (text + vector): Found 2 results using hybrid search (text +
   vector)
1. Score: 1.200, Matched: title.value, content, content.value, title
1. Score: 0.600, Matched: title, title.value, content.value, content

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

- Laptop ($1200) by Alice Johnson
- Mouse ($25) by Alice Johnson

5. Complex join with multiple conditions: Found 2 users with example.com emails:

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✅ Deleted test collections

✅ Join operations examples completed! === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
f-eT1zQ7ClDPwYDBZ992i0813Uo2z174-H8iquiQ8whjYhSChArtxUjmeRrWF8OvsZ378-nfgD74zHcId6rURA

=== Sending Chat Message === Message ID:
lD3TB9y5CpY1ujupQjzRPoEPldO-9xe2Wof6SwcSGdberSEt3FF0US9cfSBQ_U-9Z4MvmK8J6KmxMsBz_SN9qA

=== AI Response === There are three products available:

1. "ekoDB Cloud" is a fully managed cloud database service product, priced at
   $499.
2. "ekoDB Pro" is an enterprise edition product with advanced features, priced
   at $299.
3. "ekoDB" is a high-performance database product with AI capabilities, priced
   at $99.

Execution Time: 2067ms

=== Token Usage === Prompt tokens: 378 Completion tokens: 69 Total tokens: 447

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
21OnPz3iUFCAewjLBksLm7apObROKYm2hbandIHd4apRXOgAqWHWU4LltkLp14mGsGQ6y_YKWiy9CVPC6LCNew

=== Sending Messages === ✓ Message 1 sent Response: Based on the provided
context, the available product is ekoDB, a high-performance database product. It
is priced at $99.

✓ Message 2 sent Response: As the assistant, I'm sorry, but the context provided
does not include information about a specific product, its details, or its
price. Could you please provide more details or specify the product you are
interested in?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
69YSQpc9vu7j9QPQfYTJE-ELGE4ZykbzuChBtvXR2GQiMhD2sCImhTBR0CV0KuGxBf3xT1UV9FonJh3UBlKH-g
Parent:
21OnPz3iUFCAewjLBksLm7apObROKYm2hbandIHd4apRXOgAqWHWU4LltkLp14mGsGQ6y_YKWiy9CVPC6LCNew

=== Listing Sessions === ✓ Found 6 sessions Session 1:
69YSQpc9vu7j9QPQfYTJE-ELGE4ZykbzuChBtvXR2GQiMhD2sCImhTBR0CV0KuGxBf3xT1UV9FonJh3UBlKH-g
(Untitled) Session 2:
21OnPz3iUFCAewjLBksLm7apObROKYm2hbandIHd4apRXOgAqWHWU4LltkLp14mGsGQ6y_YKWiy9CVPC6LCNew
(Untitled) Session 3:
f-eT1zQ7ClDPwYDBZ992i0813Uo2z174-H8iquiQ8whjYhSChArtxUjmeRrWF8OvsZ378-nfgD74zHcId6rURA
(Untitled) Session 4:
6YIRrX0Nw2WBJcMCmpa--QKGb8NaTSFlXRB270pQqNBrlt02ojr7tXiJzdLdk7ZEnlBZo8f2JPHTeDJF5GkUnw
(Untitled) Session 5:
IMktZywTlmF0uIb-kYLw2l5LJ8xdJAS4dx4xY-xhsemWgut5OpK7B58NbOzVhALjnXVTKhRhBdBFn8P3nJkgUw
(Untitled) Session 6:
loG91Ays_vwuIn8t-PAJR1kUYrlHQrvC6otIhAo144cYL7njJImlr4otR0lH0zbYX88T-9HBdOARxHSBjujdyQ
(Untitled)

=== Deleting Branch Session === ✓ Deleted branch session:
69YSQpc9vu7j9QPQfYTJE-ELGE4ZykbzuChBtvXR2GQiMhD2sCImhTBR0CV0KuGxBf3xT1UV9FonJh3UBlKH-g

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
KSvmdtn5PuNPM0TaUxo8GXliG02J_fMCVSEjH6w77zCB1hfHncpKvGbMaJzTfQ6zVSONIRTik20VV5OHuVe_iA

=== Sending Initial Message === ✓ Message sent Response: Based on the provided
context, the available product is a high-performance database product named
ekoDB. It is priced at $99.

✓ Second message sent

Debug: Found 4 messages Debug: First message keys: dict_keys(['id', 'content',
'role', 'chat_id', 'updated_at', 'token_usage', 'created_at']) Debug: First
message role: {'value': 'assistant', 'type': 'String'} === Feature 1: Regenerate
AI Response === ✓ Message regenerated New response: I'm sorry, but the context
provided does not include information about the price of ekoDB. You may need to
check the official website or contact the sellers directly for the most accurate
and current pricing.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
lAHOXEH0Vn_UaU7jgvyRgTYJAwZEfq-86R88IureusiszLg6fj3OIgdsy6hU1Z29liPpbFoRXma4WbfCp7IkDg
✓ Sent message in second session ✓ Sessions merged successfully Total messages
in merged session: 4

=== Feature 5: Delete Message === ✓ Message deleted

✓ Messages remaining: 3

=== Cleanup === ✓ Deleted session ✓ Deleted collection

✓ All advanced chat features demonstrated successfully! 🚀 ekoDB Saved Functions
Example (Python)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved:
sb-J3xcIkp558vZb6qcuViuyUBn-8os2Rd4hlf2RSdDQ2zmSulQuEInAkOnBtNJwS8t9rEVpSYmyfDVozj5r4w
📊 Found 20 active users

📝 Example 2: Parameterized Function

✅ Function saved:
z41iL79nO0ju0AHh1EudWU8bHXqO736fcdfpsjvbd372yBh7vlBwroI3my6Bw_Nl-I-\_yb-3C1JvnhZEKj-jZw
📊 Found 20 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved:
qWOOdRpdFZ8ahC_62yJojaHyj7zjOAMcuZyg_UWjNyK7cZ0M2PoINeCrOAUHcFw0tQUsuMZ55Ji4ngw2ThXUGQ
📊 Statistics: 2 groups {'avg_score': 60.0, 'count': 10, 'status': 'active'}

{'avg_score': 50.0, 'count': 10, 'status': 'inactive'}

📝 Example 4: Function Management

📋 Total functions: 5 🔍 Retrieved function: Get Active Users ✏️ Function
updated 🗑️ Function deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! ✅ [32mPython client examples complete![0m ✅ [32mAll
Python integration tests complete![0m 🧪 [36mRunning Go examples (direct
HTTP/WebSocket)...[0m [34m ╔════════════════════════════════════════╗[0m [34m║
ekoDB Go Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m [34m === Checking Server
Connection ===[0m [32m✓ Server is ready[0m [33m === Running 7 Examples ===[0m
[34m === Running simple_crud.go ===[0m ✓ Authentication successful

=== Insert Document === Inserted:
map[id:t-thWcQA6bTrlzErSqxAyJCjrbwEEy-Il2xpg91iZBGFnfdlyyAqMhyk0bKqETiyfWGft7C9N6zMxUdJTzCR_Q]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:t-thWcQA6bTrlzErSqxAyJCjrbwEEy-Il2xpg91iZBGFnfdlyyAqMhyk0bKqETiyfWGft7C9N6zMxUdJTzCR_Q
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found 1 documents

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:t-thWcQA6bTrlzErSqxAyJCjrbwEEy-Il2xpg91iZBGFnfdlyyAqMhyk0bKqETiyfWGft7C9N6zMxUdJTzCR_Q
name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.go completed
successfully[0m [34m === Running simple_websocket.go ===[0m ✓ Authentication
successful

=== Inserting Test Data === ✓ Inserted test record:
WrJRSnl-xSMYxP45vOAOl7aa7ol-xCmeDuXpZABUjpuYjnidLLF93I7dbbLK_eGfc2WUEPGJESmErjkTweqLRQ

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"3w36KS3micZ5Sx6JR842Xfz3tMv0TGF8mEiZuZQug0PjIq9F8NIVLPIicjsMLRFBoh5oyoGjV_-pFNM8Vo2rBw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"WrJRSnl-xSMYxP45vOAOl7aa7ol-xCmeDuXpZABUjpuYjnidLLF93I7dbbLK_eGfc2WUEPGJESmErjkTweqLRQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"6zgKLocRk8WYRfFz-J1dF0GPcsGdsemFXr7AyajGSwtUPG0vJlbWvqUCsZlMtjaTUx4KY3S3gw_ebP0Jy8A1kQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 3
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.go completed
successfully[0m [34m === Running batch_operations.go ===[0m ✓ Authentication
successful

=== Batch Insert === Batch insert response: map[failed:[]
successful:[0Gb_VoEI2ghxjvqQ60NpRH8L1yZvRRqcDPN5xy8ynA33t5RH7iPb2x6CD9b-6iNtUr1wf4-EQ9Gw9rokb_OJHg
8KEey5wDtJ1Kf22C8aZ2NxmvQRsTion3vVFZ7k6FGpgwIvvvkS_xXVJ1j3ZTAkGYZQJ9om94qcEWfEvCR4q01w
LpKUe954nnwKAN_dvuDHEkw81bhtqfCs111y0-zd9BHAUDX4gEmDRXOzSbZ2THh99w1ULy2uw4DP-R2Ih920ig
g_j-_bulb8H29t6yKAMw5APwvsTc4IYQGwQvNN9fOuISjajbLXhlhzVjuq7HqbTMGryQ1lO0EIl42rRRf7dVUA
iZ9MLbKWNC667yMKmuAkukSf-q4BDReFMV0eWUMggGRvfAKuRDmPyXU0mOet7iIo9g8Kr9mdJBGB1kqEuVGgTw]]
✓ Batch insert completed

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === Batch update response: map[failed:[]
successful:[j4Woj69VPTjnm4nIyIwSkOB9_74aEbHe-iXbFZnRd72cikWcpmv0IQhafCle5fUe7ApNTOS65HjsMLx-MNs5zw
Dx5yV0M_olkwJxbeFuum2T1aPloTOR-aJgfoESJ8NOziruKGoplnITncOAvzg8DxittD7CxO0K7BW4faGJR00w
oT6e2kDlAZqKpnlguXAKHI3H6t81skq6lCZUdBQx4HAxAH_1lmV90Pzkn4ir76orKyakpDUAvpryQ3FlPpI_Hw]]
✓ Batch update completed

=== Batch Delete === Batch delete response: map[failed:[]
successful:[j4Woj69VPTjnm4nIyIwSkOB9_74aEbHe-iXbFZnRd72cikWcpmv0IQhafCle5fUe7ApNTOS65HjsMLx-MNs5zw
Dx5yV0M_olkwJxbeFuum2T1aPloTOR-aJgfoESJ8NOziruKGoplnITncOAvzg8DxittD7CxO0K7BW4faGJR00w
oT6e2kDlAZqKpnlguXAKHI3H6t81skq6lCZUdBQx4HAxAH_1lmV90Pzkn4ir76orKyakpDUAvpryQ3FlPpI_Hw]]
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
FurDgVfuKZJtFcCTUC4uHLtyIapqvLmf0fUkujJYXnJXDcI9t_OSZu2omODJ5_EUqf8FppyFAnJM_IyLCOQgeA

=== List Collections === Total collections: 14 Sample collections:
[functions__ek0_bench websocket_test chat_configurations__ek0_bench users
test_collection ws_ttl_test schema_employees_client_py schema_products_client_py
schema_users_client_py schema_documents_client_py ttl_cache
chat_messages__ek0_bench batch_users demo_collection]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.go completed successfully[0m [34m === Running
document_ttl.go ===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
kNF6hG0U_eip4LT9gaUgy-pUofhDusivI4c3SmpA-4E-3ZF6gJRcBwf0ODz1CkytzxcJwrhzRzY8Y93-w4XznQ

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 3 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.go
completed successfully[0m [34m === Running websocket_ttl.go ===[0m ✓
Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
tB9vlhnbB7AMNDRxtfVOnh9AwpWLD49xyJHJ1KvBgPS-Qj95GTs6WZ51Z5FZE8EKpCwA0_Zrqbkk2P8-S4ptcA

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 3 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.go completed
successfully[0m [34m ╔════════════════════════════════════════╗[0m [34m║ Test
Summary ║[0m [34m╚════════════════════════════════════════╝[0m [34mTotal: 7[0m
[32mPassed: 7[0m [32mFailed: 0[0m ✅ [32mGo direct examples complete![0m 🧪
[36mRunning Go client library examples...[0m ✓ Client created (token exchange
happens automatically)

=== Insert Document === Inserted:
map[id:EBuhb-wdbADhUxVOFXZQgfSXo9F_TZjoO8exhvD9t_2BQa52uw8o4cGM2763YJ0UtZm-fjzG_GsN_BOycXJ0PA]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:EBuhb-wdbADhUxVOFXZQgfSXo9F_TZjoO8exhvD9t_2BQa52uw8o4cGM2763YJ0UtZm-fjzG_GsN_BOycXJ0PA
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found documents: 1

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:EBuhb-wdbADhUxVOFXZQgfSXo9F_TZjoO8exhvD9t_2BQa52uw8o4cGM2763YJ0UtZm-fjzG_GsN_BOycXJ0PA
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
KEgA-wfcdNnUsPoTkdfG-K-19ugZ4HYS4aarzd30_N83iflEo2O-L9csvL0XJ6MQK4wC9-7bFfwVtjIoOu1xHA

=== List Collections === Total collections: 14 Sample collections:
[functions__ek0_bench websocket_test chat_configurations__ek0_bench
client_collection_management_go users]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
Wu-GaaKLhqJTRFXuYUZjOgOr5oYo1SqVdH_PcbN9Q3G8tVOwIF9mzmKjEVMLlCQV1XaoCZrFXR2eZfjSf5e8Ag

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
bBsyLAYGiap_arK6NVFVuvTkzInkGp-LKlAtIkaJmfu6Qfvwf1InEWM5Uh8Ah6bIh317fREKhGzheKmBfNAbsg

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
chZWoD-ZdyVIwFX9JkMUX2XeGGUUFgvmEi7v1lbj3q3xw129PFIVFAXvRF5LlG7yN_WOV23apFN2FLQmbV0P1g

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
BVnrWy6I1gDxUqukPZjYjg7efQfLDix_12jDl3TxnAM-i5q5uakXSVlErCHeVP0pbZ9BRXNup_fgHl7WpkPOZg

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
oPrnM6kivNtaLzP2WoGhJJ_SLoj3tHUyy2jjubt0TOY3M_7sfOSazEKacZYTNtFAvWaXtCZfyhz53pBMMU5ADA

=== Sending Chat Message === Message ID:
YKGYk9QH56HoV4mIzpBIF_0c_e2rVd90wcsu3ZYPWar00VsM6J-qNVj76OeUgREFV5XN-RGk2zjpOpQzHWDcAg

=== AI Response === The available products and their prices are:

1. ekoDB Pro: This is an enterprise edition product with advanced features. The
   price is $299.

2. ekoDB Cloud: This is a fully managed cloud database service product. The
   price is $499.

3. ekoDB: This is a high-performance database product with AI capabilities. The
   price is $99.

=== Context Used (3 snippets) === Snippet 1: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String
value:Enterprise edition product with advanced features]
id:K4iZ6kgbNtPK1h86Z-55ZFBje8Qlo6IjF3kc9sCwm7kPdaP61i9uVD6qB4TQ7L9WMXrwhdgvymnd_83UyeIXbQ
name:map[type:String value:ekoDB Pro] price:map[type:Integer value:299]]
score:0.1111111111111111] Snippet 2: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:Fully
managed cloud database service product]
id:8tGcI4ch3WNqfF4hgSevg7a3ZDiEnaKyh6ScEeQwQYjd7zdMHQ3aXKnVKpftiT7aseeec38EKVTTcMVm5O_BzQ
name:map[type:String value:ekoDB Cloud] price:map[type:Integer value:499]]
score:0.1111111111111111] Snippet 3: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:A
high-performance database product with AI capabilities]
id:hI-bGDg0z4t_tH1qHgor6ypmzXeNq6vf6FVKYlV4orOzWsGhoybQx0OU8S8hvVxcIfJ1lDzEPpRNA4NgnF3kAg
name:map[type:String value:ekoDB] price:map[type:Integer value:99]]
score:0.1111111111111111]

Execution Time: 3240ms

=== Token Usage === Prompt tokens: 388 Completion tokens: 77 Total tokens: 465

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
eITxpP2JZEN8PKiXQdkKnmf6hvhhJGG4kb_gPI0fi4-bk2dFMoB0MMGldYqWbFSohyoEqD83yBsv1dCH9WoWEA

=== Sending Messages === ✓ Message 1 sent Response: Based on the information
provided, the available product is ekoDB, a high-performance database product.
It is priced at $99.

✓ Message 2 sent Response: I'm sorry, but there's no context provided about
which product's price you're asking about. Could you please provide more
details?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
G-\_9VKeAlPIHYBh0HnZBlZJ_GauaARwQgxOvCVLbk4-HsHZCGmuQ2nYfgnoFoKi9CHYMj_dWNBS6pEyPTM7lSQ
Parent:
eITxpP2JZEN8PKiXQdkKnmf6hvhhJGG4kb_gPI0fi4-bk2dFMoB0MMGldYqWbFSohyoEqD83yBsv1dCH9WoWEA

=== Listing Sessions === ✓ Found 9 sessions Session 1:
G-\_9VKeAlPIHYBh0HnZBlZJ_GauaARwQgxOvCVLbk4-HsHZCGmuQ2nYfgnoFoKi9CHYMj_dWNBS6pEyPTM7lSQ
(Untitled) Session 2:
eITxpP2JZEN8PKiXQdkKnmf6hvhhJGG4kb_gPI0fi4-bk2dFMoB0MMGldYqWbFSohyoEqD83yBsv1dCH9WoWEA
(Untitled) Session 3:
oPrnM6kivNtaLzP2WoGhJJ_SLoj3tHUyy2jjubt0TOY3M_7sfOSazEKacZYTNtFAvWaXtCZfyhz53pBMMU5ADA
(Untitled) Session 4:
lAHOXEH0Vn_UaU7jgvyRgTYJAwZEfq-86R88IureusiszLg6fj3OIgdsy6hU1Z29liPpbFoRXma4WbfCp7IkDg
(Untitled) Session 5:
21OnPz3iUFCAewjLBksLm7apObROKYm2hbandIHd4apRXOgAqWHWU4LltkLp14mGsGQ6y_YKWiy9CVPC6LCNew
(Untitled) Session 6:
f-eT1zQ7ClDPwYDBZ992i0813Uo2z174-H8iquiQ8whjYhSChArtxUjmeRrWF8OvsZ378-nfgD74zHcId6rURA
(Untitled) Session 7:
6YIRrX0Nw2WBJcMCmpa--QKGb8NaTSFlXRB270pQqNBrlt02ojr7tXiJzdLdk7ZEnlBZo8f2JPHTeDJF5GkUnw
(Untitled) Session 8:
IMktZywTlmF0uIb-kYLw2l5LJ8xdJAS4dx4xY-xhsemWgut5OpK7B58NbOzVhALjnXVTKhRhBdBFn8P3nJkgUw
(Untitled) Session 9:
loG91Ays_vwuIn8t-PAJR1kUYrlHQrvC6otIhAo144cYL7njJImlr4otR0lH0zbYX88T-9HBdOARxHSBjujdyQ
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
G-\_9VKeAlPIHYBh0HnZBlZJ_GauaARwQgxOvCVLbk4-HsHZCGmuQ2nYfgnoFoKi9CHYMj_dWNBS6pEyPTM7lSQ

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
3C01K3ggtbLimbo4LaLnTaZtr3TE9G_iMaTE9ETWrthlu19xToN-lE6jlDt9kL0WOUGfJbqhQbNnAOWnU8r4hw

=== Sending Initial Message === ✓ Message sent Response: Based on the context
provided, the available product is ekoDB, a high-performance database product.
It is priced at 99.

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
I'm sorry, but the context provided does not include information about the price
of ekoDB. I recommend checking the official website or contacting the company
directly for the most accurate pricing information.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
64PqDH2-9s1ejYU4yAFAXB_BEdFHsAVMRbqhnMnUC9foiNvfmoJkQqgfj56sJy2lfEL4SB5P7oH3LqrbCKHSGA
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
1. Score: 0.781
1. Score: 0.751
1. Score: 0.747

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.700
1. Score: 1.512
1. Score: 0.299

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

- Bob Smith: Sales
- Alice Johnson: Engineering

2. Join with filtering: Found 1 users in Engineering

- Alice Johnson: Building A

3. Join with user profiles: Found 2 users with profile data

- Bob Smith: Sales Manager
- Alice Johnson: Senior Software Engineer

4. Join orders with user data: Found 2 completed orders

- Laptop ($0) by Alice Johnson
- Mouse ($0) by Alice Johnson

5. Complex join with multiple conditions: Found 2 users with example.com emails

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✅ Deleted test collections

✅ Join operations examples completed! ✓ Client created (token exchange happens
automatically) 📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved:
YrvkdiHR4K8vZ_mfUZM4I0uNQfykXS-sPVI0-ha-Xk8GtU8MhD7NDGSpg-nydHFj1RR8W2IC5ebCYHzCoeiHXA
📊 Found 30 active users

📝 Example 2: Parameterized Function

✅ Function saved:
pqoTKLdGDDRaJ-n34-vxT6d4HbkIcr7CpNP2Rl2ziQTH01qHuoFesLjYXA49kPtEf5PCsW1HwG7OGtwdLW5tLg
📊 Found 30 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved:
zpFPSQIQAkluVxlNNwBxZYkntORqh21ylzyUuGgClk1QrIlH1_jDzdzBY8wK6biQgcB8RBKtC5zUF7grOs7MdA
📊 Statistics: 2 groups map[avg_score:50 count:15 status:inactive]
map[avg_score:60 count:15 status:active]

📝 Example 4: Function Management

📋 Total functions: 7 🔍 Retrieved function: Get Active Users ✏️ Function
updated 🗑️ Function deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! ✅ [32mGo client examples complete![0m ✅ [32mAll Go
integration tests complete![0m 📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.3.0 prepare npm run build

> @ekodb/ekodb-client@0.3.0 build tsc

up to date, audited 7 packages in 1s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.3.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning TypeScript client library
examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 664ms

3 packages are looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'FN-yR2DyqXfQx9THq4nqq2EyeHHnQ4Q9F2PQgbg-h6YVBaymjnSDb0G4bp61RdTTDHjUt9go8GtIQ9SByvB8MA'
}

=== Find by ID === Found: { name: { type: 'String', value: 'Test Record' },
active: { type: 'Boolean', value: true }, value: { value: 42, type: 'Integer' },
id:
'FN-yR2DyqXfQx9THq4nqq2EyeHHnQ4Q9F2PQgbg-h6YVBaymjnSDb0G4bp61RdTTDHjUt9go8GtIQ9SByvB8MA'
}

=== Find with Query === Found documents: 1

=== Update Document === Updated: { name: { type: 'String', value: 'Updated
Record' }, id:
'FN-yR2DyqXfQx9THq4nqq2EyeHHnQ4Q9F2PQgbg-h6YVBaymjnSDb0G4bp61RdTTDHjUt9go8GtIQ9SByvB8MA',
value: { type: 'Integer', value: 100 }, active: { type: 'Boolean', value: true }
}

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
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
ijS3h5EUQcfvr-rGbA7IhpEDBWdPUdTYQLW0bBFAxXngBiZUGvdcDy07iV1nRSkN4fqv3bGobrjYHHrr97erjw

=== List Collections === Total collections: 18 Sample collections:
functions**ek0_bench,websocket_test,schema_documents_client_go,chat_configurations**ek0_bench,users

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
eFjRSNYZWn_MCTAgAQXI6hJ5pGwH6V34j2riv40EY6RyS_iK2Dc-aTnD_yTU0-WagCi_kY_3rtuAKYjl53TTyA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
535RSWKAflr7UKXjVrNgOAPnsIrOWcnzFCJSHrVRCNpVgCsX8u0W5YITAaGpb_P7nmwMqfzx3lWyp9wVuNqxbA

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
3SVD8501Zy7kyPA5lMkM_Anbfc871V3QlcSOW1wWftJtYvOR6LmJ4_WGbQqnHljbUT5-b5_LyVHj49O30Bzqjg

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
c0Zf3yavoi9bGt7mWG2-DZppiJwTKJ5i_eft7-ETe3R-AqqpHMPPkzlie6CP9h5DNgdnui1LYDmIjoKZ-dZQhg

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
4QZi*e93dhsh0eI70pnl6p1uM3XXy6gaVIoASTVXKOYxNDUrRZt6PT0N8OgJ03RZg6_X8IgUZy4RPSh4*-2ymg

=== Sending Chat Message === Message ID:
H2lWqr54E8NSfWKiRneDgBRZbJ19HasSd8w_Go_yCv60IK6OPriWH7khIPrUfijUHNZBws9IGGgwhMkkLfVxog

=== AI Response === There are three products available:

1. ekoDB Cloud: This is a fully managed cloud database service product. It's
   priced at $499.

2. ekoDB: This is a high-performance database product with AI capabilities. It's
   priced at $99.

3. ekoDB Pro: This is an enterprise edition product with advanced features. It's
   priced at $299.

=== Context Used (3 snippets) === Snippet 1: { collection:
'client_chat_basic_ts', record: { name: { type: 'String', value: 'ekoDB Cloud'
}, description: { value: 'Fully managed cloud database service product', type:
'String' }, price: { type: 'Integer', value: 499 }, id:
'F-jUz3rA53afXOJkN5C2ugpLP-0y9A9LIeJlkMfYNMZ1wzdA0JhX4Cm1FjdB-oleAQOIrIGPnNwjLaBEo30XhQ'
}, score: 0.1111111111111111, matched_fields: [ 'description' ] } Snippet 2: {
collection: 'client_chat_basic_ts', record: { description: { value: 'A
high-performance database product with AI capabilities', type: 'String' }, id:
'e53hdPphp5Y_bd0yFBNNEQPFdijsrMjCTvbLrXUHavgpdGPHS1vpuUbbLi1nchzqvJMLHv1C2Q8AqMCOg2Fcfg',
name: { type: 'String', value: 'ekoDB' }, price: { type: 'Integer', value: 99 }
}, score: 0.1111111111111111, matched_fields: [ 'description' ] } Snippet 3: {
collection: 'client_chat_basic_ts', record: { price: { type: 'Integer', value:
299 }, name: { type: 'String', value: 'ekoDB Pro' }, id:
'PHZLHbvg3WfYK5xjG8KJ5aSl7r-WYcyqf4QQQO-QorVhX06WQWYnVUUvLYdZRdnX-JG6keI5tVY-htSBhqR_fw',
description: { type: 'String', value: 'Enterprise edition product with advanced
features' } }, score: 0.1111111111111111, matched_fields: [ 'description' ] }

Execution Time: 2252ms

=== Token Usage === Prompt tokens: 386 Completion tokens: 78 Total tokens: 464

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
58N69ci6R0DjA4d3YVjcvHSpZhx65SmyadGVnWzfynELu3R1VVgd01F-cekrhY4m6V0RqYFq9fIDHnSVrVk9Qw

=== Sending Messages === ✓ Message 1 sent Response: Based on the provided
context, the available product is a high-performance database product named
ekoDB. It is priced at $99.

✓ Message 2 sent Response: Apologies, but there's no context provided about a
specific product to determine its price. Could you please specify which product
you're referring to?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
to4YhbLL1ypv9Wwl3GpGvTAwsFLTW0gU-zuf_CYjzCXadIm6s9_38-M3u7hXRXyG9tU-SGMXwgJEVxLknyYVew
Parent:
58N69ci6R0DjA4d3YVjcvHSpZhx65SmyadGVnWzfynELu3R1VVgd01F-cekrhY4m6V0RqYFq9fIDHnSVrVk9Qw

=== Listing Sessions === ✓ Found 10 sessions Session 1:
to4YhbLL1ypv9Wwl3GpGvTAwsFLTW0gU-zuf*CYjzCXadIm6s9_38-M3u7hXRXyG9tU-SGMXwgJEVxLknyYVew
(Untitled) Session 2:
58N69ci6R0DjA4d3YVjcvHSpZhx65SmyadGVnWzfynELu3R1VVgd01F-cekrhY4m6V0RqYFq9fIDHnSVrVk9Qw
(Untitled) Session 3:
4QZi_e93dhsh0eI70pnl6p1uM3XXy6gaVIoASTVXKOYxNDUrRZt6PT0N8OgJ03RZg6_X8IgUZy4RPSh4*-2ymg
(Untitled) Session 4:
64PqDH2-9s1ejYU4yAFAXB_BEdFHsAVMRbqhnMnUC9foiNvfmoJkQqgfj56sJy2lfEL4SB5P7oH3LqrbCKHSGA
(Untitled) Session 5:
eITxpP2JZEN8PKiXQdkKnmf6hvhhJGG4kb_gPI0fi4-bk2dFMoB0MMGldYqWbFSohyoEqD83yBsv1dCH9WoWEA
(Untitled) Session 6:
oPrnM6kivNtaLzP2WoGhJJ_SLoj3tHUyy2jjubt0TOY3M_7sfOSazEKacZYTNtFAvWaXtCZfyhz53pBMMU5ADA
(Untitled) Session 7:
lAHOXEH0Vn_UaU7jgvyRgTYJAwZEfq-86R88IureusiszLg6fj3OIgdsy6hU1Z29liPpbFoRXma4WbfCp7IkDg
(Untitled) Session 8:
21OnPz3iUFCAewjLBksLm7apObROKYm2hbandIHd4apRXOgAqWHWU4LltkLp14mGsGQ6y_YKWiy9CVPC6LCNew
(Untitled) Session 9:
f-eT1zQ7ClDPwYDBZ992i0813Uo2z174-H8iquiQ8whjYhSChArtxUjmeRrWF8OvsZ378-nfgD74zHcId6rURA
(Untitled) Session 10:
6YIRrX0Nw2WBJcMCmpa--QKGb8NaTSFlXRB270pQqNBrlt02ojr7tXiJzdLdk7ZEnlBZo8f2JPHTeDJF5GkUnw
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
to4YhbLL1ypv9Wwl3GpGvTAwsFLTW0gU-zuf_CYjzCXadIm6s9_38-M3u7hXRXyG9tU-SGMXwgJEVxLknyYVew

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
5ZWDXaqks_WRjzgSC_NShlE94p43Wj17yM0rcmhDBVU32ZDqzNk6qt6VZyybqVIpGAZRfELSdCg3ZVSYIeJMTw

=== Sending Initial Message === ✓ Message sent Response: Based on the
information provided, the available product is a high-performance database
product named ekoDB, priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
I'm sorry, but the context provided doesn't include information about the price
of ekoDB. I'd recommend checking the official website or contacting the sales
department of ekoDB for the most accurate and up-to-date pricing information.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
lIP5KOzTPzvMR1uaQ4cbkgoc18Vy2CqKeO9F0gAYVN0ZXDQ-yyNZdcOCjNCHLUJ0-3GfLFo0wIYD_oB537TgqQ
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
1. Score: 25.740, Matched: email, email.value, name, name.value
1. Score: 12.540, Matched: name, name.value

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: bio.value, title, title.value, bio
1. Score: 26.400, Matched: bio, bio.value, title, title.value
1. Score: 26.400, Matched: bio.value, title, title.value, bio
1. Score: 26.400, Matched: title, bio.value, bio, title.value

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: bio.value, title, title.value, bio
1. Score: 39.600, Matched: title.value, bio, title, bio.value
1. Score: 39.600, Matched: bio, bio.value, title.value, title
1. Score: 39.600, Matched: bio.value, bio, title, title.value

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio.value, bio

1. Search with stemming and exact match boosting: Found 1 results (matches:
   work, working, worked)
1. Score: 13.200, Matched: bio.value, bio

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.744, Matched:
1. Score: 0.735, Matched:
1. Score: 0.727, Matched:

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.691, Matched: content.value, title, title.value, content
1. Score: 1.494, Matched: title.value, content, content.value, title
1. Score: 0.298, Matched:

1. Case-sensitive search: Found 1 results (case-sensitive)
1. Score: 13.200, Matched: bio, bio.value

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

✅ Join operations examples completed! 🚀 ekoDB Saved Functions Example
(TypeScript)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved:
nwaUpSdPgajavxCaXeCJ82MvooKYl8jJqBrH3rttCh6Yt8F5cs_MNsTyCJD9CRmrWe7H-xsd7EybeKShFXWHLw
📊 Found 40 active users

📝 Example 2: Parameterized Function

✅ Function saved:
K1LauOyTGQitW-plvawVxOJNaC-KSy6BIGfF2E5xC52IvvrZBG6uFi9gqWMvMa7Q05oMoHcbU7gJMySzCzvKxA
📊 Found 40 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved:
myvR1eFs1Ooq40dnOmH6PEtqbRXD8cvL2fsbB6RO3egnPFULpteJA30t4826QkTYdXiEqkA2eKLR5s5T5YHYug
📊 Statistics: 2 groups {"avg_score":50,"count":20,"status":"inactive"}
{"status":"active","count":20,"avg_score":60}

📝 Example 4: Function Management

📋 Total functions: 9 🔍 Retrieved function: Get Active Users ✏️ Function
updated 🗑️ Function deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! ✅ [32mTypeScript client examples complete![0m ✅
[32mAll TypeScript integration tests complete![0m 🧪 [36mRunning JavaScript
examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 643ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities [34m ╔════════════════════════════════════════╗[0m [34m║
ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m [34m === Checking Server
Connection ===[0m (node:4040) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) [32m✓
Server is ready[0m [34m === Getting Authentication Token ===[0m [32m✓
Authentication successful[0m [33m === Running 7 Examples ===[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js
===[0m ✓ Authentication successful

=== Insert Document === (node:4042) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) Inserted:
{ id:
'MPf0hPUWhISXnFyaMEMN3hdLZ5oBnC2JVa8wFylXaEbPDbuEykz2yhKRo-tYD-aJJ82VmYPm0UI8iJciEvMLJQ'
}

=== Find by ID === Found: { name: { type: 'String', value: 'Test Record' }, id:
'MPf0hPUWhISXnFyaMEMN3hdLZ5oBnC2JVa8wFylXaEbPDbuEykz2yhKRo-tYD-aJJ82VmYPm0UI8iJciEvMLJQ',
value: { type: 'Integer', value: 42 }, active: { value: true, type: 'Boolean' }
}

=== Find with Query === Found documents: 1

=== Update Document === Updated: { id:
'MPf0hPUWhISXnFyaMEMN3hdLZ5oBnC2JVa8wFylXaEbPDbuEykz2yhKRo-tYD-aJJ82VmYPm0UI8iJciEvMLJQ',
value: { type: 'Integer', value: 100 }, name: { value: 'Updated Record', type:
'String' }, active: { value: true, type: 'Boolean' } }

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js
===[0m ✓ Authentication successful

=== Inserting Test Data === (node:4054) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
test record:
Se3UvJ0XI7OyDDEKDxZ7JtsiEFdNXRxU-xtiLI-koXCy0hnZvBSXDQ5Mb1TPSSJkfldUF9qBrW24AdUTvJWZ_w

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"3w36KS3micZ5Sx6JR842Xfz3tMv0TGF8mEiZuZQug0PjIq9F8NIVLPIicjsMLRFBoh5oyoGjV_-pFNM8Vo2rBw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"Se3UvJ0XI7OyDDEKDxZ7JtsiEFdNXRxU-xtiLI-koXCy0hnZvBSXDQ5Mb1TPSSJkfldUF9qBrW24AdUTvJWZ_w",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"WrJRSnl-xSMYxP45vOAOl7aa7ol-xCmeDuXpZABUjpuYjnidLLF93I7dbbLK_eGfc2WUEPGJESmErjkTweqLRQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"6zgKLocRk8WYRfFz-J1dF0GPcsGdsemFXr7AyajGSwtUPG0vJlbWvqUCsZlMtjaTUx4KY3S3gw_ebP0Jy8A1kQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 4
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/batch_operations.js
===[0m ✓ Authentication successful

=== Batch Insert === (node:4057) [DEP0040] DeprecationWarning: The `punycode`
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

=== KV Set === (node:4070) [DEP0040] DeprecationWarning: The `punycode` module
is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Set key:
session:user123

=== KV Get === Retrieved value: { username: 'john_doe', loginTime:
'2025-11-08T06:24:04.122Z', userId: 123 }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully [32m✓ kv_operations.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/collection_management.js
===[0m ✓ Authentication successful

=== Create Collection (via insert) === (node:4079) [DEP0040] DeprecationWarning:
The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Collection created with first record:
fcFCBy3UuibObZImRPYAm_0k5EgvWJh3at4XzIxDAXiu2Iq77qtUS4GKyXz2L1E_JWTNOiBmgyZbA0VxXwyPVA

=== List Collections === Total collections: 22 Sample collections: [
'functions__ek0_bench', 'schema_products_client_ts', 'websocket_test',
'schema_documents_client_go', 'chat_configurations__ek0_bench' ]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.js completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/document_ttl.js
===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === (node:4081) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) ✓ Inserted document:
8UKOAxku9qyZi-JxAgz1SVEstNE9QG-4y1K-erSe6ZyBRzW52PgzOYcJnNvVARxW8MSy81a\_-iB_cP2UyA_k1Q

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 4 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.js
completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js
===[0m ✓ Authentication successful

=== Insert Test Data with TTL === (node:4101) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
document with TTL:
bGcqdaDurAkfD6aFYHK9uj3OUSVtJYSQxfjA9AjbbgJJQYj_XuDkw0f9U_qYX40OFms8tVmpwmXyVoq-v828Dg

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 4 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.js completed
successfully[0m [34m ╔════════════════════════════════════════╗[0m [34m║ Test
Summary ║[0m [34m╚════════════════════════════════════════╝[0m [34mTotal: 7[0m
[32mPassed: 7[0m [32mFailed: 0[0m ✅ [32mJavaScript direct examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.3.0 prepare npm run build

> @ekodb/ekodb-client@0.3.0 build tsc

up to date, audited 7 packages in 900ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.3.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning JavaScript client library
examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 964ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'XD9LloQ-pu2va6jg0Pr0ocgU8vget5Wn-VBJbshmxsaLv6RPpyQ2rj0JsENDiDjhONlsCmFe7i53NOKh85Bshg'
}

=== Find by ID === Found: { id:
'XD9LloQ-pu2va6jg0Pr0ocgU8vget5Wn-VBJbshmxsaLv6RPpyQ2rj0JsENDiDjhONlsCmFe7i53NOKh85Bshg',
name: { value: 'Test Record', type: 'String' }, active: { value: true, type:
'Boolean' }, value: { type: 'Integer', value: 42 } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { id:
'XD9LloQ-pu2va6jg0Pr0ocgU8vget5Wn-VBJbshmxsaLv6RPpyQ2rj0JsENDiDjhONlsCmFe7i53NOKh85Bshg',
active: { value: true, type: 'Boolean' }, name: { type: 'String', value:
'Updated Record' }, value: { type: 'Integer', value: 100 } }

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
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
yR862PjnlNCsIFGFnNB8oQDKBMOEowpOj0wjMCit40b7SklyLiwPjNEUyfL8jubM9dJ-M3n9ucaoufEx5x1EJw

=== List Collections === Total collections: 22 Sample collections:
functions**ek0_bench,schema_products_client_ts,websocket_test,schema_documents_client_go,chat_configurations**ek0_bench

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
E-GrBapZEh7Wu4QbeAk7TY40SxbURQftVcF5PnteAO5tsBwH-y-fZwa9SfMwIi35Whtl9bdkBv-0FJ-QWgxQAg

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
D28QdDPfJ3a5H4pdFZYF5JSPOCeOm09IiCDaXhogQ6JV0TrWyb-fvmZiA3Rytle5F2UjoVCZkgDrYbp1pbSDdA

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
JcXmfSppsZs4j6StB4H4RO6eEzXGt-AksJkBSz3SfcxZt3V07Yo2hPnudn6u42YKKi0HQKya91D1bMJQo5aYTw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
IHyIyhz2KzDs7qXQ3Iu-7TxGG15l6y1EQarA1qVFM51fcXuLMUoY9oLFhWwqS0z4EaAlSLegerKwgiixzzIdxg

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration 🚀 ekoDB Saved Functions Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved:
uP3DfUegqTlipQMnBmXjjQDGeV4_ms2U8Knt3mY6CUmULZSrbbtKcLWi_1PJweH2QO-Y9iZTjFIbu7rDLw8-LQ
📊 Found 50 active users

📝 Example 2: Parameterized Function

✅ Function saved:
hrTJEjQS61FbvXchoRp4NQU40e3Z2C_9Cwefx7q7LPNNVz_ikoobWNdbHDZQiGKG9rQbm7hvyjsjKPecDnFUSQ
📊 Found 50 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved:
XGm82zQ_KAdz-zGba6hejkgu7qdCOhMAafPmAmwQb8JAiIuxJBG91WDqsTsoz6lOrALJ0kyGxrcdM2JpCvKOGw
📊 Statistics: 2 groups {"avg_score":60,"count":25,"status":"active"}
{"count":25,"avg_score":50,"status":"inactive"}

📝 Example 4: Function Management

📋 Total functions: 11 🔍 Retrieved function: Get Active Users ✏️ Function
updated 🗑️ Function deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! ✅ [32mJavaScript client examples complete![0m ✅
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

BUILD SUCCESSFUL in 4s 2 actionable tasks: 2 up-to-date ✅ [32mKotlin client
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
Record(fields={id=StringValue(value=HjnhWEtt_Ul5aLcxoT9KPgfuxfqswoHcBjYKem2TAbP0uGlVylwlsIX1gj_1nQZxhOdYFFMnEsJOYBGzCU4N2g)})
User ID:
HjnhWEtt_Ul5aLcxoT9KPgfuxfqswoHcBjYKem2TAbP0uGlVylwlsIX1gj_1nQZxhOdYFFMnEsJOYBGzCU4N2g

=== Read === ✓ Found user by ID:
Record(fields={email=ObjectValue(value={value=StringValue(value=alice@example.com),
type=StringValue(value=String)}),
id=StringValue(value=HjnhWEtt_Ul5aLcxoT9KPgfuxfqswoHcBjYKem2TAbP0uGlVylwlsIX1gj_1nQZxhOdYFFMnEsJOYBGzCU4N2g),
active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)}),
age=ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=28)}),
name=ObjectValue(value={value=StringValue(value=Alice Johnson),
type=StringValue(value=String)})})

=== Update === ✓ Updated user:
Record(fields={active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)}),
city=ObjectValue(value={value=StringValue(value=San Francisco),
type=StringValue(value=String)}),
email=ObjectValue(value={value=StringValue(value=alice@example.com),
type=StringValue(value=String)}),
age=ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=29)}),
name=ObjectValue(value={value=StringValue(value=Alice Johnson),
type=StringValue(value=String)}),
id=StringValue(value=HjnhWEtt_Ul5aLcxoT9KPgfuxfqswoHcBjYKem2TAbP0uGlVylwlsIX1gj_1nQZxhOdYFFMnEsJOYBGzCU4N2g)})

=== Query === ✓ Found 1 users matching query

- Record(fields={active=ObjectValue(value={type=StringValue(value=Boolean),
  value=BooleanValue(value=true)}),
  city=ObjectValue(value={value=StringValue(value=San Francisco),
  type=StringValue(value=String)}),
  email=ObjectValue(value={value=StringValue(value=alice@example.com),
  type=StringValue(value=String)}),
  age=ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=29)}),
  id=StringValue(value=HjnhWEtt_Ul5aLcxoT9KPgfuxfqswoHcBjYKem2TAbP0uGlVylwlsIX1gj_1nQZxhOdYFFMnEsJOYBGzCU4N2g),
  name=ObjectValue(value={value=StringValue(value=Alice Johnson),
  type=StringValue(value=String)})})

=== Delete === ✓ Deleted user with ID:
HjnhWEtt_Ul5aLcxoT9KPgfuxfqswoHcBjYKem2TAbP0uGlVylwlsIX1gj_1nQZxhOdYFFMnEsJOYBGzCU4N2g

✓ Confirmed user was deleted

=== Cleanup === ✓ Deleted collection: kotlin_users_example

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
> Kotlin Client - Batch Operations Example ===

=== Batch Insert === ✓ Inserted 5 records IDs:
hjxLreb8sWuqo-0WLB9jnn6a4c1m5mby5yEEXc0ea7y24rptt6omwvyDjptCzp5pJISt8PSV4ByHweR7ETSUqg,
bIC5LwGHqcfe5d8WqP0n0m3Qy0AsrDpPEQmxkc1gyZVb94KJ8dhwWR7Ojh4ORhDoybOevYkGCbfx1UZ-lDVbWg,
QBmeeR1MwrFzGLf8GVqFr_yclSBQ8Bq6WDvkCl7EhJfvR0HcKNq3v3izKTWz5K8oU15dP_8uk99Lm6Yg1ANeTQ...

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
- Score: ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=88)})

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
{"role":"admin","name":"Alice","email":"alice@example.com"}

=== KV Set with TTL === ✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key === ✓ Session value:
{"user_id":"123","created_at":1762583077792} (Will expire in 10 seconds)

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

=== List Collections === ✓ Found 21 collections

- functions\_\_ek0_bench
- schema_products_client_ts
- websocket_test
- schema_documents_client_go
- chat_configurations\_\_ek0_bench

=== Check Collection Existence === Collection 'kotlin_collection_example'
exists: false

=== Create Collection with Schema === ✓ Created collection with schema:
kotlin_collection_example

=== Get Collection Schema === ✓ Schema:
{"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-11-08T06:24:42.114137Z","last_modified":"2025-11-08T06:24:42.114137Z","bypass_ripple":false,"primary_key_alias":"id"}

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
7-ZYqvm75nvtDfyjjP-QyKzz1xs7yPfy-3mhCcGvbBTNBpmtHwV-K1YnkCHWNRNvcmndL1xmNK5kJgzFmq4bAA

=== Verify Document Exists === ✓ Document found: [user_id, id, created_at,
session_id, ttl_duration]

=== Insert with Longer TTL === ✓ Inserted document with 1h TTL Document ID:
pVzy43Z9UHo5P5sdcBgP2Zuhe7pJX0ZoGcVxwypiRczwMkVads17MrrOzQcYXnsjjxysd9IPxMx3FNnm9BYETw

=== TTL Expiration === ✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document === ✓ Long TTL document still exists: [value,
ttl_duration, cache_key, id]

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
{"payload":{"data":[{"id":"vBzvBWD1CdVzVKIu6CFj5mIO_tLd0J53f5m5iv16btAEfFGLDWlwB3DRLTQBj56W4yAkXxTCpituk6qS9_vgXA","name":{"type":"String","value":"Test
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
StringValue(value=dBeaTThzvsT9EtqbfjEvAhVk0yNaXcA61aa4np2ZRynzd1mV9S9vmNGX-t4u6NkQH_wypsEVHoZmlVIAxShSRA)
✓ Inserted user 2:
StringValue(value=2YDeUTf4fiTa4-yKt5xrVP4NMMyifIhQY_JG9tlrK28_wHhUWeKZLQXghJy4IYxiPbHNSTlJm7kDdOtCLJBgXA)

=== Getting Schema === ✓ Schema for kotlin_schema_example: Fields:
{"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections === ✓ Total collections: 22 Sample:
[functions__ek0_bench, schema_products_client_ts, websocket_test,
schema_documents_client_go, chat_configurations__ek0_bench]

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
{"results":[{"record":{"tags":"programming,rust,tutorial","views":68,"title":"Rust
Programming","description":"Learn Rust programming language with hands-on
examples and best
practices.","id":"MAUnVuRSmpb-iQUXt5AoUxuyIotB12UeIYbIczS7uLKBgyBaXtSneNgAGtqwKLPsjcw1g0MqiI*dgRQRSI9yEQ"},"score":19.8,"matched_fields":["title","tags","description"]},{"record":{"id":"KGSA_WQuWmpt11iEPo2cAUS54oYmPtITFcYNCJlsIDIM7IlacYGlfSQ6pdvys5aBSKosvt3DDJ1wwP*-znFSyA","description":"Master
Python for data analysis, machine learning, and
visualization.","views":62,"tags":"programming,python,data-science","title":"Python
for Data
Science"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"views":803,"tags":"programming,javascript,web","title":"JavaScript
Web Development","description":"Build modern web applications using JavaScript,
React, and
Node.js.","id":"hRCPQRGqcyFIOHJEh8Pb0iczW2DPvqNIXIYe8Ruw_1vVYkvOIl-oZDlGxjwpebgLuAM-fffy-GfwGtlYe46dlA"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' === ✓ Found results for 'machine learning'
{"results":[{"record":{"views":603,"id":"GBf-mbrN24H9ofpT4fyD7Maq1juHNfE4U5IVjtcybdFtNZF0M2v0hGaAJx*wAYDOcue2vAkQK0L2WHyVY2W63Q","title":"Machine
Learning Basics","tags":"ai,machine-learning,python","description":"Introduction
to machine learning algorithms and neural
networks."},"score":2.7,"matched_fields":["tags","title","description"]},{"record":{"id":"KGSA_WQuWmpt11iEPo2cAUS54oYmPtITFcYNCJlsIDIM7IlacYGlfSQ6pdvys5aBSKosvt3DDJ1wwP*-znFSyA","title":"Python
for Data Science","description":"Master Python for data analysis, machine
learning, and
visualization.","tags":"programming,python,data-science","views":62},"score":1.0,"matched_fields":["description"]},{"record":{"tags":"database,design,sql","description":"Learn
database design principles, normalization, and query
optimization.","id":"fD71w4uc-YSmvExbfLD5k4GPWwVsHMiOVgO_ubV9Lpss2D1blkzWw8uaAY60T97pvwNg-Qh0c8zgA3v_7g-mew","views":616,"title":"Database
Design"},"score":0.5,"matched_fields":["description"]},{"record":{"description":"Learn
Rust programming language with hands-on examples and best
practices.","id":"MAUnVuRSmpb-iQUXt5AoUxuyIotB12UeIYbIczS7uLKBgyBaXtSneNgAGtqwKLPsjcw1g0MqiI_dgRQRSI9yEQ","title":"Rust
Programming","tags":"programming,rust,tutorial","views":68},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
StringValue(value=lOQ7-FBhkD5sAWMigDNYbQKgV6PtjUVpeiMa9npCBNPcrk8E1xyn7I3dmJK4scBwwaGGZY8Pv-6PSLEHYZUW_g)

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved data via
WebSocket:
{"payload":{"data":[{"created_at":{"type":"Integer","value":1762583103436},"id":"lOQ7-FBhkD5sAWMigDNYbQKgV6PtjUVpeiMa9npCBNPcrk8E1xyn7I3dmJK4scBwwaGGZY8Pv-6PSLEHYZUW_g","name":{"type":"String","value":"WebSocket
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

- Mouse ($25) by Alice Johnson
- Laptop ($1200) by Alice Johnson

=== Example 5: Complex join with multiple conditions === ✓ Found 2 users with
example.com emails

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✓ Deleted test collections

✓ Join operations example completed successfully

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
> Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
vs2rNwL4LwmWADfQU_ZS3SMn1JMIOxdhUysNVLum1KXaspW3Pilkdr8bYFBT23IrRzJibRjUK359vr5RI3QqVQ

=== Sending Chat Message === ✓ Chat response: Message ID:
"T0BUqUDd2CfeBmsHzgdviClWd37UOB-EWFagShv2PQ9ioKU1f2_MNCIF_yYu3oErInZTftyCWJ0PVS4EI6UlFw"
Responses: ["ekoDB is a high-performance database that offers intelligent
caching, real-time capabilities, and AI integration. Some of its key features
include a chat feature that allows you to query your database using natural
language and receive AI-powered responses with relevant context. Additionally,
ekoDB supports various search features such as full-text search, vector search,
and hybrid search with automatic context retrieval."]

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
A4IucbrO-WA6AQYcUmUS4fLK0mE0INknVS4gVg-FRIDS5zHUmUO9rj0XVEZN9kC4PWl9aTpXjolu6TcqyO1ZEA

=== Sending Messages === ✓ Message 1 sent Responses: ["Based on the context
provided, the product available is ekoDB. It is a high-performance database
product with AI capabilities. The price for this product is $99."]

✓ Message 2 sent Responses: ["Apologies, but there's no context provided about a
specific product. Could you please specify the product you're interested in?"]

=== Getting Message History === ✓ Retrieved message history Total messages:
[{"chat_id":{"type":"String","value":"A4IucbrO-WA6AQYcUmUS4fLK0mE0INknVS4gVg-FRIDS5zHUmUO9rj0XVEZN9kC4PWl9aTpXjolu6TcqyO1ZEA"},"content":{"type":"String","value":"What
products are
available?"},"context_snippets":{"type":"Array","value":[{"type":"Object","value":{"collection":{"type":"String","value":"kotlin_chat_sessions_example"},"matched_fields":{"type":"Array","value":[{"type":"String","value":"description"}]},"record":{"type":"Object","value":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":{"type":"String","value":"dr6b8FKvpePvfdYgLRChVH0R4d8S5WyG-oFs1nRml5CWeQs_nPcMb0ceDw7Nqdq7m5ofVZcp_BGbUR4zbZCZJw"},"price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}}},"score":{"type":"Float","value":0.25}}}]},"created_at":{"type":"DateTime","value":"2025-11-08T06:25:20.812233Z"},"id":"pJAFAyX-EvKdnpYZ8bEjIq6YNO0qPsKEBMaXkhkBd4IR4Nuy6Bj-fmLB-75DJRYbkfIvyGV4AcZIc-KovZ8_qg","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":34},"prompt_tokens":{"type":"Integer","value":169},"total_tokens":{"type":"Integer","value":203}}},"updated_at":{"type":"DateTime","value":"2025-11-08T06:25:20.812233Z"}},{"chat_id":{"type":"String","value":"A4IucbrO-WA6AQYcUmUS4fLK0mE0INknVS4gVg-FRIDS5zHUmUO9rj0XVEZN9kC4PWl9aTpXjolu6TcqyO1ZEA"},"content":{"type":"String","value":"Based
on the context provided, the product available is ekoDB. It is a
high-performance database product with AI capabilities. The price for this
product is
$99."},"context_snippets":{"type":"Array","value":[{"type":"Object","value":{"collection":{"type":"String","value":"kotlin_chat_sessions_example"},"matched_fields":{"type":"Array","value":[{"type":"String","value":"description"}]},"record":{"type":"Object","value":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":{"type":"String","value":"dr6b8FKvpePvfdYgLRChVH0R4d8S5WyG-oFs1nRml5CWeQs_nPcMb0ceDw7Nqdq7m5ofVZcp_BGbUR4zbZCZJw"},"price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}}},"score":{"type":"Float","value":0.25}}}]},"created_at":{"type":"DateTime","value":"2025-11-08T06:25:20.813658Z"},"id":"p0OE1qcP6fu7S4kM-cvd3rmjtmAQgl71A2WfNAQDgrSXna8nijq5W2qji1ZQY4c_LpGjMtHl8f-l2o5wGTMd-A","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":34},"prompt_tokens":{"type":"Integer","value":169},"total_tokens":{"type":"Integer","value":203}}},"updated_at":{"type":"DateTime","value":"2025-11-08T06:25:20.813658Z"}},{"chat_id":{"type":"String","value":"A4IucbrO-WA6AQYcUmUS4fLK0mE0INknVS4gVg-FRIDS5zHUmUO9rj0XVEZN9kC4PWl9aTpXjolu6TcqyO1ZEA"},"content":{"type":"String","value":"What
is the
price?"},"created_at":{"type":"DateTime","value":"2025-11-08T06:25:21.938387Z"},"id":"9L-xFa84WvyNEWeAuoGXDLUjsGHust1Xg6pe5665AlydlrxeYWU52E5zb2GZnEddS0JqutqrKmSpDNZ6TJf08A","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":25},"prompt_tokens":{"type":"Integer","value":63},"total_tokens":{"type":"Integer","value":88}}},"updated_at":{"type":"DateTime","value":"2025-11-08T06:25:21.938387Z"}},{"chat_id":{"type":"String","value":"A4IucbrO-WA6AQYcUmUS4fLK0mE0INknVS4gVg-FRIDS5zHUmUO9rj0XVEZN9kC4PWl9aTpXjolu6TcqyO1ZEA"},"content":{"type":"String","value":"Apologies,
but there's no context provided about a specific product. Could you please
specify the product you're interested
in?"},"created_at":{"type":"DateTime","value":"2025-11-08T06:25:21.941836Z"},"id":"IV-rP-jxfU7py0VMGHhxdL3Is6-LLbG1CW5n3C5hDsxUZeFYPwQ9iEaDQv2hU7uH8_4cfQjPHsQoJxa-2y6raQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":25},"prompt_tokens":{"type":"Integer","value":63},"total_tokens":{"type":"Integer","value":88}}},"updated_at":{"type":"DateTime","value":"2025-11-08T06:25:21.941836Z"}}]

=== Updating Session === ✓ Updated session system prompt

=== Listing Sessions === ✓ Total sessions: 13

=== Branching Session === ✓ Created branched session:
neII3GnWU0P87cMIrMFNDZ4iDEyDR7Pv5mpPW5ns3rs_jl22KPLDC25eppS_RCZ7XPD8ZeC2gVpI3bv6bbhvdQ

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_sessions_example

✓ Chat session management example completed successfully

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
> Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
wWgO7DZ_pO-7NgSDEssXgCU_xJ9OxVITzyzbJOJH7_29_R-yWAxkNmUMr7JtlzXXHU2QHOoFPe2ywU2zhu3stw

=== Sending Initial Message === ✓ Message sent Responses: ["Based on the
information provided, the product available is ekoDB. It is a high-performance
database product priced at $99."]

✓ Second message sent

=== Regenerating AI Response === ✓ AI response regenerated New responses:
["Based on the context provided, the available product is ekoDB, which is a
high-performance database product. It is priced at $99."]

=== Updating Message === ✓ Updated message content

=== Toggling Forgotten Status === ✓ Marked message as forgotten (excluded from
context)

=== Creating Second Session for Merge === ✓ Created second session:
9V7AhAzoUS1qC9TcOJxTpQAJ-7qyb8PZF93Tw7K9elMtp8RCZwaVnxbQJpxq5acgzoPlqZExDPoRhfqKVzFOOg

=== Merging Sessions === ✓ Merged sessions Merged session ID: null

=== Deleting Message === ✓ Deleted message

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 8s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run 🚀 ekoDB Saved Functions Example (Kotlin)

SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to no-operation
(NOP) logger implementation SLF4J(W): See
https://www.slf4j.org/codes.html#noProviders for further details. 📋 Setting up
test data... ✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved:
-BrAGpfdVJpLIBJuKyXrcXwyHQ3E4jvsSgoDDZn9T0XYbT9rGTDYNApRi4YfW5R7Juv9UHvvFaSo5VwjYtzDqw
📊 Found 60 active users

📝 Example 2: Parameterized Function

✅ Function saved:
N83_YnfI91uephgKCt_oyoVOkZpsLDFvGL346hEA_PJCT9JusbCyYYbZgBJIyZ94A5QXA0UC1cLfTg3Lx5oS7A
📊 Found 60 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved:
0aenFYkYf-J8jXkmyNO8RAdRNxf0wI3pWBBFEip5d0cvmLELFTJPy5M1BzC8mx_Mdt0fMvXgUxRdtqO3Cxtx7w
📊 Statistics: 2 groups {"status":"active","count":30,"avg_score":60.0}
{"avg_score":50.0,"count":30,"status":"inactive"}

📝 Example 4: Function Management

📋 Total functions: 13 🔍 Retrieved function: Get Active Users ✏️ Function
updated 🗑️ Function deleted

✅ All examples completed!

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date ✅
[32mKotlin client examples complete![0m ✅ [32mAll Kotlin integration tests
complete![0m
