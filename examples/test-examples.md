make test-examples

🔍 Checking examples inventory against committed snapshot... 🔍 Generating
comprehensive examples list... 📋 Counting Direct API examples... javascript: 7
direct examples python: 7 direct examples go: 7 direct examples rust: 7 direct
examples 📚 Counting Client Library examples... rust: 14 client examples python:
14 client examples go: 14 client examples typescript: 14 client examples
javascript: 7 client examples kotlin: 14 client examples ✅ Examples list
generated! Total: 105 examples Direct: 28 | Client: 77 Files:
examples_list.txt.tmp, examples_list.json.tmp ✅ Examples inventory is up to
date! 🧪 Running Rust examples (direct HTTP/WebSocket)... Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.11s Running
`target/debug/examples/simple_crud` ✓ Authentication successful

=== Insert Document === Inserted: Object {"id":
String("FYQ2d6Vn9jIhCryRFFcD0yx4O5VecZQWUzSRuHUDY8shafOQLXx4CzLXQUX9N3liOJ7Qet4PzW5Rx9T0pAjedw")}

=== Find by ID === Found: Object {"active": Object {"type": String("Boolean"),
"value": Bool(true)}, "id":
String("FYQ2d6Vn9jIhCryRFFcD0yx4O5VecZQWUzSRuHUDY8shafOQLXx4CzLXQUX9N3liOJ7Qet4PzW5Rx9T0pAjedw"),
"name": Object {"type": String("String"), "value": String("Test Record")},
"value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query === Found documents: Array [Object {"active": Object
{"type": String("Boolean"), "value": Bool(true)}, "id":
String("FYQ2d6Vn9jIhCryRFFcD0yx4O5VecZQWUzSRuHUDY8shafOQLXx4CzLXQUX9N3liOJ7Qet4PzW5Rx9T0pAjedw"),
"name": Object {"type": String("String"), "value": String("Test Record")},
"value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document === Updated: Object {"active": Object {"type":
String("Boolean"), "value": Bool(true)}, "id":
String("FYQ2d6Vn9jIhCryRFFcD0yx4O5VecZQWUzSRuHUDY8shafOQLXx4CzLXQUX9N3liOJ7Qet4PzW5Rx9T0pAjedw"),
"name": String("Updated Record"), "value": Number(100)}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully Finished `dev` profile [unoptimized

- debuginfo] target(s) in 0.09s Running `target/debug/examples/simple_websocket`
  ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
"8U3djf9soDBOt9Ukki_aeb5abDxCt-CCiku0KZpBkWaBhFqQCRmBBv6v2-\_lm61KF2WgBxA1C5bSyjyoAb-UJQ"

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"8U3djf9soDBOt9Ukki_aeb5abDxCt-CCiku0KZpBkWaBhFqQCRmBBv6v2-_lm61KF2WgBxA1C5bSyjyoAb-UJQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 1
record(s) via WebSocket

✓ WebSocket example completed successfully Finished `dev` profile [unoptimized +
debuginfo] target(s) in 0.08s Running `target/debug/examples/batch_operations` ✓
Authentication successful

=== Batch Insert === ✓ Batch inserted 5 records ✓ Verified: Found 5 total
records in collection

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === ✓ Batch updated 3 records ✓ Verified: Record updated with
status="active"

=== Batch Delete === ✓ Batch deleted 3 records ✓ Verified: Records successfully
deleted (not found)

✓ All batch operations completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.08s Running
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

✓ All KV operations completed successfully Finished `dev` profile [unoptimized +
debuginfo] target(s) in 0.08s Running
`target/debug/examples/collection_management` ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
"U5NCWlNwIU982KUvAbMatCpbyEHPEXIywrSJXVuvKXflKlRbjrws9mq2hMoTg94RJPpXIJOiD28lywhxeUerig"

=== List Collections === Total collections: 4 Sample collections:
["batch_users", "test_collection", "demo_collection", "websocket_test"]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/document_ttl` ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
"gxb7ygiuuy3jBaUvLD5upmapgJtDk-Y5Oc1KX7MHFOsf_CuuePHlKmhCB7345qalEmD5o4PXQvsjRD2mzBgPyQ"

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: null

=== Query Documents === ✓ Found 1 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

✓ All document TTL operations completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.08s Running
`target/debug/examples/websocket_ttl` ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
String("XsNKyc_xiK-8sO3FpXzO0R48dB7ZsRwfqC02pZF0DNIa5cU3WwdmXKiBEE6v2Xm9mJUZp1NraT1SsfLgRrtxCg")

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully ✅ Rust direct examples complete!
🛠️ Building client library... cargo build -p ekodb_client Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.14s ✅ Client build complete! 🧪
Running Rust client library examples... Finished `dev` profile [unoptimized +
debuginfo] target(s) in 0.08s Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: Record { fields: {"id":
String("DWnO7suFkC6tFX8BbuXv0fxzzNYfwxnwUQog1m1xW9sZhu1aUFlOdmyas9UU0Oke3HkXcEEsGaEVHnPa6_VXfQ")}
}

=== Find by ID === Found: Record { fields: {"id":
String("DWnO7suFkC6tFX8BbuXv0fxzzNYfwxnwUQog1m1xW9sZhu1aUFlOdmyas9UU0Oke3HkXcEEsGaEVHnPa6_VXfQ"),
"value": Object({"value": Integer(42), "type": String("Integer")}), "active":
Object({"value": Boolean(true), "type": String("Boolean")}), "name":
Object({"value": String("Test Record"), "type": String("String")})} }

=== Find with Query === Found documents: [Record { fields: {"value":
Object({"type": String("Integer"), "value": Integer(42)}), "name":
Object({"type": String("String"), "value": String("Test Record")}), "active":
Object({"type": String("Boolean"), "value": Boolean(true)}), "id":
String("DWnO7suFkC6tFX8BbuXv0fxzzNYfwxnwUQog1m1xW9sZhu1aUFlOdmyas9UU0Oke3HkXcEEsGaEVHnPa6_VXfQ")}
}]

=== Update Document === Updated: Record { fields: {"id":
String("DWnO7suFkC6tFX8BbuXv0fxzzNYfwxnwUQog1m1xW9sZhu1aUFlOdmyas9UU0Oke3HkXcEEsGaEVHnPa6_VXfQ"),
"value": Integer(100), "active": Object({"value": Boolean(true), "type":
String("Boolean")}), "name": String("Updated Record")} }

=== Delete Document === Deleted document

=== Cleanup === ✓ Deleted collection

✓ All CRUD operations completed successfully Finished `dev` profile [unoptimized

- debuginfo] target(s) in 0.09s Running
  `target/debug/examples/client_batch_operations` ✓ Client created

=== Batch Insert (via multiple inserts) === ✓ Inserted 5 records ✓ Verified:
Found 5 total records in collection

=== Update Records === ✓ Updated 3 records

=== Delete Records === ✓ Deleted 3 records

=== Cleanup === ✓ Deleted collection

✓ All batch operations completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.09s Running
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

✓ All KV operations completed successfully Finished `dev` profile [unoptimized +
debuginfo] target(s) in 0.08s Running
`target/debug/examples/client_collection_management` ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
"6tKRKOL9hzeznIGgDSzZK1NafQK3Q9wtRc6EGDpPJ1qUecPFj2g5gtib2mRVQ8eYzv3VDs6nfKSkB5Bj395oSQ"

=== List Collections === Total collections: 6 Sample collections:
["batch_users", "test_collection", "ws_ttl_test", "ttl_cache",
"client_collection_management_rust"]

=== Count Documents === Document count: 1

=== Check Collection Exists === Collection exists: true

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_document_ttl` ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
"S5IuiJoa0CHZy4kGjb3LtvDEPuHkvkWPzZ8yNSfGvgqVzPFC7zp_mUJyJTNyPIySbRq22OBrjaw16KY4kTVf_A"

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
Some(String("w1cewbM3-Sq4tw7rKyWbk76tjd9XNh_SCRfQJ0m2tdUHW1q2s9UvrfA4jKj63JQe218MumONMCjiWnz-UJr0Rw"))

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
Running `target/debug/examples/client_simple_websocket` ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
oJVv_curwbqtfkAOKxaXBW5h_iyp532Ei8p2rsTSXQPx5yV8iwcT0nXQOw3tvvTWkSKewM0KGfLL9DArkYjMtQ

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully Finished `dev` profile [unoptimized +
debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_websocket_ttl` ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
Some(String("7eNgeGqFEI8IEiPS6YavWmAvvFPo7bg-DnzIwA8iF7FSOhitqXrJd92ujfGe400Jjs8FkhrKUDEPxLkygxJtgw"))

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
Running `target/debug/examples/client_query_builder` === ekoDB Query Builder
Example ===

=== Inserting Sample Data === ✓ Inserted 5 users

=== Simple Equality Query === ✓ Found 3 active users

- Some(Object({"type": String("String"), "value": String("David")}))
- Some(Object({"type": String("String"), "value": String("Bob")}))
- Some(Object({"value": String("Alice"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) === ✓ Found 3 users in age range

- Some(Object({"value": String("David"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("Eve")}))
- Some(Object({"value": String("Bob"), "type": String("String")}))

=== IN Operator === ✓ Found 4 users with status active or pending

- Some(Object({"value": String("David"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("Eve")}))
- Some(Object({"type": String("String"), "value": String("Bob")}))
- Some(Object({"value": String("Alice"), "type": String("String")}))

=== NOT IN Operator === ✓ Found 4 users not inactive

=== String Pattern Matching === ✓ Found 5 users with @example.com email

=== Regex Query === ✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) === ✓ Found 2 users
matching all conditions

- Some(Object({"type": String("String"), "value": String("Bob")}))
- Some(Object({"type": String("String"), "value": String("David")}))

=== OR Query === ✓ Found 2 users with age < 28 OR age > 32

- Some(Object({"value": String("Charlie"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("Alice")}))

=== Sorted Query (by score descending) === ✓ Top 3 users by score:

1. Some(Object({"value": String("Bob"), "type": String("String")}))
2. Some(Object({"value": String("David"), "type": String("String")}))
3. Some(Object({"value": String("Alice"), "type": String("String")}))

=== Pagination (page 2, size 2) === ✓ Page 2 results:

- Some(Object({"type": String("String"), "value": String("Charlie")}))
- Some(Object({"value": String("David"), "type": String("String")}))

=== NOT Operator === ✓ Found 4 users NOT inactive

=== Complex Chained Query === ✓ Found 3 users with all conditions

=== Cleanup === ✓ Deleted collection

✓ All query builder operations completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.08s Running
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
   ["description.value", "title.value", "title", "description"]
2. Score: 2.0000 Title: Some(String("Python for Data Science")) Matched:
   ["description", "description.value"]
3. Score: 1.0000 Title: Some(String("Database Design")) Matched: ["description",
   "description.value"]
4. Score: 1.0000 Title: Some(String("Rust Programming")) Matched:
   ["description.value", "description"]

=== Weighted Search === ✓ Found 2 results with field weights

1. Score: 23.1000 - Some(String("Python for Data Science"))
2. Score: 3.3000 - Some(String("Machine Learning Basics"))

=== Advanced Search Options === ✓ Found 1 results with stemming

1. Score: 19.8000 - Some(String("Database Design"))

=== Search with Limit === ✓ Limited to 2 results (requested 2)

1. Score: 19.8000 - Some(String("Rust Programming"))
2. Score: 6.6000 - Some(String("JavaScript Web Development"))

=== Cleanup === ✓ Deleted collection

✓ All search operations completed successfully Execution time: 3ms Finished
`dev` profile [unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_schema_management` === ekoDB Schema Management
Example ===

=== Creating Collection with Schema === ✓ Created collection
'schema_client_rust' with schema

=== Inserting Valid Documents === ✓ Inserted user 1:
Some(String("1wOQuUxzj9jjU7Y0NbFPyC8QSE32VizZmt6I4uAEKcKgMQLNkxEMwl1evZHnHjltOor7qYUYvgB8Axkrt0bJtA"))
✓ Inserted user 2:
Some(String("HeTBdj-fERd935DBBMcviQZxErxOUg6hcU5zk3j1EtS5PG6P-Ig2nvKVryo90RbQnEWCLPPFgh15E2Hv09J3kA"))

=== Getting Schema === ✓ Schema for schema_client_rust:

- title: String (required)
- status: String
- age: Integer
- email: String (required)

=== Listing Collections === ✓ Total collections: 6 Sample: ["batch_users",
"test_collection", "ws_ttl_test", "ttl_cache", "schema_client_rust"]

=== Cleanup === ✓ Deleted collection

✓ All schema management operations completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_joins` === ekoDB Rust Client - Join Operations
Example ===

=== Setting up sample data === ✓ Sample data created

=== Example 1: Single collection join (users with departments) === ✓ Found 2
users with department data

- Alice Johnson
- Bob Smith

=== Example 2: Join with filtering === ✓ Found 2 users in Engineering

- Alice Johnson
- Bob Smith

=== Example 3: Join with user profiles === ✓ Found 2 users with department join

- Alice Johnson
- Bob Smith

=== Example 4: Join orders with user data === ✓ Found 3 completed orders

- Laptop ($1200)
- Keyboard ($75)
- Mouse ($25)

=== Example 5: Complex join with multiple conditions === ✓ Found 2 users with
example.com emails

- Alice Johnson (alice@example.com)
- Bob Smith (bob@example.com)

=== Cleanup === ✓ Deleted test collections

✓ Join operations example completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.10s Running
`target/debug/examples/client_chat_basic` === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
ZURF2ib3SHaypBxnBrqL2naVxuCnl23gEz-QQRuu-1IuGSqjsj96IXO-7eXCKjQB0YqRQKufcOMqmuf5Kz2ZTw

=== Sending Chat Message === Message ID:
osAaS2jMcritqFcf8kSNKXUZkr_iIKx-VK0ekXmk5jWjLUMXcDY_T2GZa0ewGJlbOISph4nlvpe_xw089gaHUg

=== AI Response === Response 1: Answer: ekoDB is a high-performance database
that provides intelligent caching and real-time capabilities. It also features
AI integration. Some of the other notable features include a chat feature that
allows you to query your database using natural language and get AI-powered
responses with relevant context. Additionally, it supports full-text search,
vector search, and hybrid search with automatic context retrieval.

=== Context Used (3 snippets) ===

Snippet 1: Collection: client_chat_basic_rust Score: 0.6333 Matched Fields:
["title", "category", "content"] Record: Object {"category": Object {"type":
String("String"), "value": String("features")}, "content": Object {"type":
String("String"), "value": String("The chat feature allows you to query your
database using natural language and get AI-powered responses with relevant
context.")}, "id":
String("9DZ8SxldSyHOX-rk6BK8PGcj7oLisCZS4zM3fIJksSIv8dCswILrxuLRJ2fiwpgZVyeDDf6Fyvr9jI0QeEXy1g"),
"title": Object {"type": String("String"), "value": String("AI Chat
Integration")}}

Snippet 2: Collection: client_chat_basic_rust Score: 0.5222 Matched Fields:
["content", "category", "title"] Record: Object {"category": Object {"type":
String("String"), "value": String("features")}, "content": Object {"type":
String("String"), "value": String("ekoDB supports full-text search, vector
search, and hybrid search with automatic context retrieval.")}, "id":
String("wsDGColWwb0Z2kfBSspE_Nfl6m3vNUQFLjitJNGZRZBNER7dzWgvn7vPhyztPcb2M1635qMtIucDt9KOHhtevg"),
"title": Object {"type": String("String"), "value": String("Search Features")}}

Snippet 3: Collection: client_chat_basic_rust Score: 0.5222 Matched Fields:
["title", "content"] Record: Object {"category": Object {"type":
String("String"), "value": String("documentation")}, "content": Object {"type":
String("String"), "value": String("ekoDB is a high-performance database with
intelligent caching, real-time capabilities, and AI integration.")}, "id":
String("WZOI4XmmXYcF9hy-VzH0joFyau_YPJRS8Akv9Co9_XQk0QnBl0Vx7JVnTfWwMC-YAuts-ZN371xOtGU2Aplihw"),
"title": Object {"type": String("String"), "value": String("Introduction to
ekoDB")}}

Execution Time: 3213ms

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully Finished `dev` profile [unoptimized + debuginfo]
target(s) in 0.09s Running `target/debug/examples/client_chat_sessions` ===
ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
HBxCxS4SggFkCb0KDnhHmx0K5E-UyoAonpKvBBkCSrsriWxIyyjwMs6rlkbUA\_\_N965YGH-8IGT-XGUoAgpnrA
=== Sending Messages === ✓ Message 1 sent Response: Based on the information
provided, the product available is ekoDB, a high-performance database product
with AI capabilities. It is priced at $99.

✓ Message 2 sent Response: Apologies, but there's no context provided about a
specific product. Could you please specify which product you're referring to?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
92fCa1jk9n_a6YVUw1ufdeNoQa3UwItGJC6jGhQHB-tYWxPbkE7uKN0lo1AtvUrE8XwDd1jD72ilglXooqfKHA
Parent:
HBxCxS4SggFkCb0KDnhHmx0K5E-UyoAonpKvBBkCSrsriWxIyyjwMs6rlkbUA\_\_N965YGH-8IGT-XGUoAgpnrA

=== Listing Sessions === ✓ Found 3 sessions Session 1:
92fCa1jk9n_a6YVUw1ufdeNoQa3UwItGJC6jGhQHB-tYWxPbkE7uKN0lo1AtvUrE8XwDd1jD72ilglXooqfKHA
(Untitled) Session 2:
HBxCxS4SggFkCb0KDnhHmx0K5E-UyoAonpKvBBkCSrsriWxIyyjwMs6rlkbUA\_\_N965YGH-8IGT-XGUoAgpnrA
(Untitled) Session 3:
ZURF2ib3SHaypBxnBrqL2naVxuCnl23gEz-QQRuu-1IuGSqjsj96IXO-7eXCKjQB0YqRQKufcOMqmuf5Kz2ZTw
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
92fCa1jk9n_a6YVUw1ufdeNoQa3UwItGJC6jGhQHB-tYWxPbkE7uKN0lo1AtvUrE8XwDd1jD72ilglXooqfKHA

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.08s Running
`target/debug/examples/client_chat_advanced` === ekoDB Advanced Chat Features
Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
KER9RVds0bFTv_JU-n2ofY-Em9r6xSsdZQeGmPYxYdTDSv7fMt67aMHjecQBT5ViPZudi8RXRF-3D1K-vRF3LA

=== Sending Initial Message === ✓ Message sent Response: Based on the context
provided, the available product is ekoDB. It is a high-performance database
product priced at $99.

✓ Second message sent === Feature 1: Regenerate AI Response === ✓ Message
regenerated New response: I'm sorry, but the context provided does not include
information on the price of ekoDB. I recommend checking the official website or
contacting the sales department of ekoDB for the most accurate and up-to-date
pricing information.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
DXKs6VxkJxJmJTavQYM3q_3MsGmZ4bb9tAl_ILih_VZ5d2qcQHGYuWnmWJuMtNLroXVVeZOwT4H8eKwSkowJ-A
✓ Sent message in second session ✓ Sessions merged successfully Total messages
in merged session: 4

=== Feature 5: Delete Message === ✓ Message deleted

✓ Messages remaining: 3

=== Cleanup === ✓ Deleted session ✓ Deleted collection

✓ All advanced chat features demonstrated successfully! ✅ Rust client examples
complete! ✅ All Rust integration tests complete! 🧪 Running Python examples
(direct HTTP/WebSocket)...

╔════════════════════════════════════════╗ ║ ekoDB Python Examples Test Suite ║
╚════════════════════════════════════════╝

=== Checking Server Connection === ✓ Server is ready

=== Getting Authentication Token === ✓ Authentication successful

=== Running 7 Examples ===

=== Running simple_crud.py === ✓ Authentication successful

=== Insert Document === Inserted: {'id':
'\_HN9HFpmVtnJcSZohbkiI4ZvaDCVYX6LhLlnQ5ChhoeXkkEznPAVIrR094kpwsTg_Ty08jslsEPxJvQ2oYZHjA'}

=== Find by ID === Found: {'active': {'value': True, 'type': 'Boolean'}, 'id':
'\_HN9HFpmVtnJcSZohbkiI4ZvaDCVYX6LhLlnQ5ChhoeXkkEznPAVIrR094kpwsTg_Ty08jslsEPxJvQ2oYZHjA',
'value': {'type': 'Integer', 'value': 42}, 'name': {'type': 'String', 'value':
'Test Record'}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'active': {'type': 'Boolean', 'value': True},
'name': 'Updated Record', 'value': 100, 'id':
'\_HN9HFpmVtnJcSZohbkiI4ZvaDCVYX6LhLlnQ5ChhoeXkkEznPAVIrR094kpwsTg_Ty08jslsEPxJvQ2oYZHjA'}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully ✓ simple_crud.py completed
successfully

=== Running simple_websocket.py === ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
WEGxpeB4YnE2t1l48ZiBo0r0i-0OapZon3QrGoRFCcA703ICWHmvRMcARry3o1kBEGbIlovbbn-bIJ-oLLmGbg

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"8U3djf9soDBOt9Ukki_aeb5abDxCt-CCiku0KZpBkWaBhFqQCRmBBv6v2-_lm61KF2WgBxA1C5bSyjyoAb-UJQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"WEGxpeB4YnE2t1l48ZiBo0r0i-0OapZon3QrGoRFCcA703ICWHmvRMcARry3o1kBEGbIlovbbn-bIJ-oLLmGbg",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 2
record(s) via WebSocket

✓ WebSocket example completed successfully ✓ simple_websocket.py completed
successfully

=== Running batch_operations.py === ✓ Authentication successful

=== Batch Insert === ✓ Batch inserted 5 records ✓ Verified: Found 10 total
records in collection

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === ✓ Batch updated 3 records ✓ Verified: Record updated with
status="None"

=== Batch Delete === ✓ Batch deleted 3 records ✓ Verified: Records successfully
deleted (not found)

✓ All batch operations completed successfully ✓ batch_operations.py completed
successfully

=== Running kv_operations.py === ✓ Authentication successful

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: {'userId': 123, 'username': 'john_doe'}

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: {'name': 'Product 1', 'price': 29.99}
cache:product:2: {'price': 39.989999999999995, 'name': 'Product 2'}
cache:product:3: {'name': 'Product 3', 'price': 49.989999999999995}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ kv_operations.py completed
successfully

=== Running collection_management.py === ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
Fv3tOAlJUjsjB5C--NY1KOhAgHoC_yYTjOAu4Iq2Wih9v6aQmcrobBzuWqEj3XC3LmFwT-uIC0A_QrT_t71yCQ

=== List Collections === Total collections: 8 Sample collections:
['batch_users', 'test_collection', 'ws_ttl_test', 'ttl_cache',
'demo_collection']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓
collection_management.py completed successfully

=== Running document_ttl.py === ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
BkN9BBBlVNCcGh_iCmXPaF4kU0pehO-x6v6wlwGHfd8i3Vh-vMc2968P18AOLSzIyriszXVXEXdWY8UdOaAwpg

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully ✓ document_ttl.py completed
successfully

=== Running websocket_ttl.py === ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
c2MCGpL659V-cnjOCW24fsOXUY1N6TTEMWbhZMWCgeAIow00jbtjKY3fpXJKAZBmo5ow3mCJO9XRNtl4Y4jrFQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 2 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully ✓ websocket_ttl.py completed
successfully

╔════════════════════════════════════════╗ ║ Test Summary ║
╚════════════════════════════════════════╝ Total: 7 Passed: 7 Failed: 0 ✅
Python direct examples complete! 🐍 Building Python client package... 🍹
Building a mixed python/rust project 🔗 Found pyo3 bindings with abi3 support 🐍
Not using a specific python interpreter 💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0`
for aarch64-apple-darwin by default Finished `release` profile [optimized]
target(s) in 0.14s 📦 Built wheel for abi3 Python ≥ 3.8 to
/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.2.1-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel for current platform... Processing
./ekodb-client-py/target/wheels/ekodb_client-0.2.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client Attempting uninstall: ekodb-client
Found existing installation: ekodb_client 0.2.1 Uninstalling ekodb_client-0.2.1:
Successfully uninstalled ekodb_client-0.2.1 Successfully installed
ekodb-client-0.2.1 ✅ Python client package built and installed! 🧪 Running
Python client library examples... ✓ Client created (token exchange happens
automatically)

=== Insert Document === Inserted: {'id':
'lE6B3hg1YrEWQjBz6i9f_gwlO6HGFGqxVDjZeNWtYhVPgGobnyM0aytmH2Fpv6f9WH96iBi0Fwo3YKnuWapvbA'}

=== Find by ID === Found: {'name': {'value': 'Test Record', 'type': 'String'},
'id':
'lE6B3hg1YrEWQjBz6i9f_gwlO6HGFGqxVDjZeNWtYhVPgGobnyM0aytmH2Fpv6f9WH96iBi0Fwo3YKnuWapvbA',
'value': {'value': 42, 'type': 'Integer'}, 'active': {'value': True, 'type':
'Boolean'}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'value': 100, 'active': {'type': 'Boolean',
'value': True}, 'name': 'Updated Record', 'id':
'lE6B3hg1YrEWQjBz6i9f_gwlO6HGFGqxVDjZeNWtYhVPgGobnyM0aytmH2Fpv6f9WH96iBi0Fwo3YKnuWapvbA'}

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
"td9VnxOvs5ADMbMXUkroVDRfbuU1Dr5TwCO-JZrESUrX1uY3-ZsV8Y6TGvteBK_D_2FMNf7MXQ9k_xBioetGFQ"

=== List Collections === Total collections: 8 Sample collections:
['batch_users', 'test_collection', 'ws_ttl_test', 'ttl_cache', 'websocket_test']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
WYsPiSITCJ2ioT8y8ATp6vhI2yvWiBPlnnoVa1qIeL2QdvePLlrxoe3W9jqCKFKevkikL4ov7ifAE7yCcWg_sA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
aItz5UxgOpdgzgrGG9F3Dlj7sC1ygJq8-XrWBm_plJuUF_A62Jd1MpuDvxj0df_iqY40UfDUQq-EfTeOu7GorA

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
qPhlexcct_o_KR72QRtmx4jJXD94Mm8tOIz03zGxMl3sVfgxo6Np8Sn5YS27Cl56Zmruay5mP4LPK4dA8gV2Pw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
\_rFwxl1L13do_pMIY4VeIn69E5gf85ngqxYRPcVUMs0CIrSvwun22N9EPc69_SOihApFN_g8CBjRAP2nfjzySQ

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
1. Score: 25.740, Matched: email, name.value, name, email.value
1. Score: 12.540, Matched: name.value, name

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: title, bio, title.value, bio.value
1. Score: 26.400, Matched: title, bio, bio.value, title.value
1. Score: 26.400, Matched: bio, title.value, bio.value, title
1. Score: 26.400, Matched: bio.value, title, bio, title.value

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: bio, bio.value, title, title.value
1. Score: 39.600, Matched: bio.value, bio, title.value, title
1. Score: 39.600, Matched: bio, bio.value, title, title.value
1. Score: 39.600, Matched: title, bio.value, title.value, bio

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio, bio.value
1. Score: 13.200, Matched: bio, bio.value

1. Search with stemming and exact match boosting: Found 2 results (matches: run,
   running, runs)
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio, bio.value

1. Vector search (semantic search): Found 0 semantically similar documents

1. Hybrid search (text + vector): Found 2 results using hybrid search (text +
   vector)
1. Score: 1.200, Matched: title.value, content, content.value, title
1. Score: 0.600, Matched: content, title, title.value, content.value

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

- Alice Johnson: No department
- Bob Smith: No department

2. Join with filtering: Found 2 users in Engineering:

- Alice Johnson: Unknown
- Bob Smith: Unknown

3. Multi-collection join (users with departments and profiles): Found 2 users
   with multiple joins:

- Alice Johnson: Department: N/A Bio: N/A
- Bob Smith: Department: N/A Bio: N/A

4. Join orders with user data: Found 3 completed orders:

- Keyboard ($75) by Unknown
- Laptop ($1200) by Unknown
- Mouse ($25) by Unknown

5. Complex join with multiple conditions: Found 2 users with example.com emails:

- Alice Johnson (alice@example.com): N/A
- Bob Smith (bob@example.com): N/A

=== Cleanup === ✅ Deleted test collections

✅ Join operations examples completed! === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
GpJGyY4TwEnpt94EiY1-eMbr4fU7hhB3XOzUvfWBO8XEEDS_zisfZKCzKou-zE_b4zHYl5Vwh3mZVckj1JUgvw

=== Sending Chat Message === Message ID:
HvXqw0eTCwHgf7fiwh3KOpG2kfrzR7_XWxtywEhf_hkbQLfaNxNuEryNH9FUANYycXcCno3loyjjS-RVr5zA7Q

=== AI Response === The available products and their prices are as follows:

1. ekoDB: A high-performance database product with AI capabilities, priced at
   $99.
2. ekoDB Pro: An enterprise edition product with advanced features, priced at
   $299.
3. ekoDB Cloud: A fully managed cloud database service product, priced at $499.

Execution Time: 3094ms

=== Token Usage === Prompt tokens: 375 Completion tokens: 70 Total tokens: 445

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
iQoohxHDhoCePst9Ntir0pRa5_Wkn8ll8z5WjMZF_bJ-V93TcOhO6xDkK0hZpHPQ5BQPcqibZlVEgd-R9bh23g

=== Sending Messages === ✓ Message 1 sent Response: Based on the provided
context, the available product is a high-performance database product named
ekoDB. It is priced at $99.

✓ Message 2 sent Response: I'm sorry, but there's no information provided about
which product you're asking for. Could you please specify the product?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
uE42O1eeT_zDuvxNtBwr5V34tVOwu_01DTvzirL72sYBzm9CCx7juYfXFVOB3zyvK5ukmeamSTK8LpYE1Jl-dA
Parent:
iQoohxHDhoCePst9Ntir0pRa5_Wkn8ll8z5WjMZF_bJ-V93TcOhO6xDkK0hZpHPQ5BQPcqibZlVEgd-R9bh23g

=== Listing Sessions === ✓ Found 6 sessions Session 1:
uE42O1eeT_zDuvxNtBwr5V34tVOwu_01DTvzirL72sYBzm9CCx7juYfXFVOB3zyvK5ukmeamSTK8LpYE1Jl-dA
(Untitled) Session 2:
iQoohxHDhoCePst9Ntir0pRa5_Wkn8ll8z5WjMZF_bJ-V93TcOhO6xDkK0hZpHPQ5BQPcqibZlVEgd-R9bh23g
(Untitled) Session 3:
GpJGyY4TwEnpt94EiY1-eMbr4fU7hhB3XOzUvfWBO8XEEDS_zisfZKCzKou-zE_b4zHYl5Vwh3mZVckj1JUgvw
(Untitled) Session 4:
DXKs6VxkJxJmJTavQYM3q_3MsGmZ4bb9tAl_ILih_VZ5d2qcQHGYuWnmWJuMtNLroXVVeZOwT4H8eKwSkowJ-A
(Untitled) Session 5:
HBxCxS4SggFkCb0KDnhHmx0K5E-UyoAonpKvBBkCSrsriWxIyyjwMs6rlkbUA\_\_N965YGH-8IGT-XGUoAgpnrA
(Untitled) Session 6:
ZURF2ib3SHaypBxnBrqL2naVxuCnl23gEz-QQRuu-1IuGSqjsj96IXO-7eXCKjQB0YqRQKufcOMqmuf5Kz2ZTw
(Untitled)

=== Deleting Branch Session === ✓ Deleted branch session:
uE42O1eeT_zDuvxNtBwr5V34tVOwu_01DTvzirL72sYBzm9CCx7juYfXFVOB3zyvK5ukmeamSTK8LpYE1Jl-dA

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
GgEtZ3OX7WsDzGyu2yCrPqqazxn5c6kat9iMTltgcd4-lWDZFEJ3uCxKHFyxptkjiEY_ioAyTQ7iXy0Tg5YHfg

=== Sending Initial Message === ✓ Message sent Response: Based on the context
provided, the available product is ekoDB. It is a high-performance database
product priced at $99.

✓ Second message sent

Debug: Found 4 messages Debug: First message keys: dict_keys(['content', 'id',
'role', 'updated_at', 'chat_id', 'created_at', 'token_usage']) Debug: First
message role: {'value': 'assistant', 'type': 'String'} === Feature 1: Regenerate
AI Response === ✓ Message regenerated New response: I'm sorry, but the context
provided does not include information on the price of ekoDB. You may need to
check the official website or contact the vendor for the most accurate and
current pricing.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
1ttWfumAX4sysCJO_c_HkD1p5NgOpYCon-2-Hj6mJWd5X7ALW2F59iTaGFokRTi_GJmHG3PBbDgZdXhqN1oFXA
✓ Sent message in second session ✓ Sessions merged successfully Total messages
in merged session: 4

=== Feature 5: Delete Message === ✓ Message deleted

✓ Messages remaining: 3

=== Cleanup === ✓ Deleted session ✓ Deleted collection

✓ All advanced chat features demonstrated successfully! ✅ Python client
examples complete! ✅ All Python integration tests complete! 🧪 Running Go
examples (direct HTTP/WebSocket)...

╔════════════════════════════════════════╗ ║ ekoDB Go Examples Test Suite ║
╚════════════════════════════════════════╝

=== Checking Server Connection === ✓ Server is ready

=== Running 7 Examples ===

=== Running simple_crud.go === ✓ Authentication successful

=== Insert Document === Inserted:
map[id:gLABISjMgrDTEkByHwilgVXm3KIyunsARdWasJ29L6niN3vCYh_-i14yw8krwjTCClwjI5SKL6FJUU5tLKMtqg]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:gLABISjMgrDTEkByHwilgVXm3KIyunsARdWasJ29L6niN3vCYh\_-i14yw8krwjTCClwjI5SKL6FJUU5tLKMtqg
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found 1 documents

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:gLABISjMgrDTEkByHwilgVXm3KIyunsARdWasJ29L6niN3vCYh\_-i14yw8krwjTCClwjI5SKL6FJUU5tLKMtqg
name:Updated Record value:100]

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully ✓ simple_crud.go completed
successfully

=== Running simple_websocket.go === ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
2bEt2dz6vweiMDpuewUo_4SvAC0luxh7rUxc0l2bMF3J9ZHmw7Q7OynpyxnVUf-0oXkEH3sQYnkWcobtrWN6ig

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"2bEt2dz6vweiMDpuewUo_4SvAC0luxh7rUxc0l2bMF3J9ZHmw7Q7OynpyxnVUf-0oXkEH3sQYnkWcobtrWN6ig",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"8U3djf9soDBOt9Ukki_aeb5abDxCt-CCiku0KZpBkWaBhFqQCRmBBv6v2-_lm61KF2WgBxA1C5bSyjyoAb-UJQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"WEGxpeB4YnE2t1l48ZiBo0r0i-0OapZon3QrGoRFCcA703ICWHmvRMcARry3o1kBEGbIlovbbn-bIJ-oLLmGbg",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 3
record(s) via WebSocket

✓ WebSocket example completed successfully ✓ simple_websocket.go completed
successfully

=== Running batch_operations.go === ✓ Authentication successful

=== Batch Insert === Batch insert response: map[failed:[]
successful:[E_0FVwOlVYkXJdbAx9lB8R9Pc_a0wRhop5X6_4JPrFcFsrjt6XPS8LwErM4r-z9OO_nnORIL_FgbQYuemIy0CQ
P4JQxX8DQGabj-YQpyg4kYbZW4d3YXC_kkLyykJy2RGB4oTrWQtxGYHG3Kax5k3Pn6hLyzDhP-n7Q6CS7MnDbw
mkeCPmdHd-zaAXdLOdkQ2qTtJvJxYffDPGqvMS4UbGinQ_92xzp_cjAjnJ3N9hHJUdRwxehKDHm0aN9kTDdViA
WvsoXCv--HCqPVOxnNZDOYA74PgtMQm3ZhQa9TqpyI0VoLtTocQlUmTBpaeBYbDeraEm4_v41YsFbjpqTKumkA
qjCFa6j0UjW83RoeQLvgdVPBq5rv05ocw_p4BmuiP89QLQZV4Gr8Syihhpe1AIZnqWJv0AvLXxURdSlkNFQMzg]]
✓ Batch insert completed

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === Batch update response: map[failed:[]
successful:[0eh0_jHLv9Ac59JVc-rlqQLuJIKtvhBT_Rn6OriSSljxpBzvad8EhjVV5oW22LiDVVR08N7hUO4tJPUkQK3fiw
alP_KmTCmOiq2dsnn6xHz46nzW98QXgPg8CNBSCIuNSiRyaX4NyQGeGhavery6nCyWxqZWli4JgNqRCum44Nlg
aKCwVmAWE7tIgj_pwTXSE3QhxJVtgu-5R2UScz6XJdKTBR4AgZ3CbqU6a1t-YjNXLoXo7dtQ9i4cQj_EkmGIhA]]
✓ Batch update completed

=== Batch Delete === Batch delete response: map[failed:[]
successful:[0eh0_jHLv9Ac59JVc-rlqQLuJIKtvhBT_Rn6OriSSljxpBzvad8EhjVV5oW22LiDVVR08N7hUO4tJPUkQK3fiw
alP_KmTCmOiq2dsnn6xHz46nzW98QXgPg8CNBSCIuNSiRyaX4NyQGeGhavery6nCyWxqZWli4JgNqRCum44Nlg
aKCwVmAWE7tIgj_pwTXSE3QhxJVtgu-5R2UScz6XJdKTBR4AgZ3CbqU6a1t-YjNXLoXo7dtQ9i4cQj_EkmGIhA]]
✓ Batch delete completed

✓ All batch operations completed successfully ✓ batch_operations.go completed
successfully

=== Running kv_operations.go === ✓ Authentication successful

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: map[userId:123 username:john_doe]

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: map[name:Product 1 price:29.99]
cache:product:2: map[name:Product 2 price:39.989999999999995] cache:product:3:
map[name:Product 3 price:49.989999999999995]

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ kv_operations.go completed
successfully

=== Running collection_management.go === ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
qM94DizAY4N4Spp7B0MTGbtXPqpugquaUdU9EgtKRr9gGIUEVZYaasPhCkP-3teKPxtjXvC0_FPFS_X0LKeIuQ

=== List Collections === Total collections: 12 Sample collections: [batch_users
test_collection ws_ttl_test ttl_cache demo_collection schema_products_client_py
schema_employees_client_py websocket_test chat_configurations__ek0_bench
chat_messages__ek0_bench schema_users_client_py schema_documents_client_py]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓
collection_management.go completed successfully

=== Running document_ttl.go === ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
JkZ9yvpjCb2q1LAy4abf_0cUpwmsZR0L-MFGzNMzsbXs-QADo10dr2Ixr1W4WnPLFwOeX4rgB4mHzPRx1ekjLw

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 3 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully ✓ document_ttl.go completed
successfully

=== Running websocket_ttl.go === ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
BebbZ8wIPWzzd9PmW78BLA9VYKnQIKROHwWT1TL9fW3h9KU8l3aZTKyr-saFcNY8L_eJl_sZNsNlqIOFvQIVRQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 3 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully ✓ websocket_ttl.go completed
successfully

╔════════════════════════════════════════╗ ║ Test Summary ║
╚════════════════════════════════════════╝ Total: 7 Passed: 7 Failed: 0 ✅ Go
direct examples complete! 🧪 Running Go client library examples... ✓ Client
created (token exchange happens automatically)

=== Insert Document === Inserted:
map[id:YfaB1KSV47LG9HeRhLPuh-WllGpxHp2eIzbESndiL5N0-Pf0cGZCTUJhyCKXekjKPI9Nxwsd7ombdqrXr6BN3A]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:YfaB1KSV47LG9HeRhLPuh-WllGpxHp2eIzbESndiL5N0-Pf0cGZCTUJhyCKXekjKPI9Nxwsd7ombdqrXr6BN3A
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found documents: 1

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:YfaB1KSV47LG9HeRhLPuh-WllGpxHp2eIzbESndiL5N0-Pf0cGZCTUJhyCKXekjKPI9Nxwsd7ombdqrXr6BN3A
name:Updated Record value:100]

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
Tk3aRoh3_sd9aMqrbjuBU9p5fNv2RaQOZyD97Wbiu86YvFgzATWCJg0Q6VBhrquMczLo6wYx5rWTalGMpX719g

=== List Collections === Total collections: 12 Sample collections: [batch_users
test_collection ws_ttl_test client_collection_management_go ttl_cache]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
kpb8cCeLdeXPJdqQVkQGIPFLjc6K_NQSoT9pRz8sPCyJmAKPv8t7vHLakH3n7bWjJy0c-kmeC7YoUcac3AVYMg

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
GJm0GRkoYwX_vJD61bhbFmZaVJP6-R82iWpWPKuakNrzM2A4wXv6m0aj9obj4onbsZT_6J8i6nFyVz6PHpOkFw

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
XLyAHq9YhXC8maK8x28KRgrTQ4EKM8Kt8YhTFZrU2kcNiO9HED5QFN6VXDojS1t37HR64dHcPGD0yx3mr21nTg

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
AO4oxYrn2I5Vfzbic_4x4bmFeWxGih2Z8O0k1C2iApxpws9_bOJD94kIBzHrHjAm3zs1iYlX82JP4UVGZibnvg

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
TYpHvinc9xK3hjYSvriJhJn68EP9vZEn7AE0KjOj5eF_8JYy5SveK1nJpZaLQhU2IMipYdWJ5XdMke24w3-D9g

=== Sending Chat Message === Message ID:
zL3mt0ld2RSTbkiCeHRJpZHsV2v-Xe_A6UvywjEbUhiciIx0HYPRjVFdpdiD7vpIumCO1yWJo0a2Vyv4LruDpg

=== AI Response === Three products are available:

1. ekoDB: This is a high-performance database product with AI capabilities. It
   is priced at $99.
2. ekoDB Cloud: This is a fully managed cloud database service product. It costs
   $499.
3. ekoDB Pro: This is an enterprise edition product with advanced features. It
   is available for $299.

=== Context Used (3 snippets) === Snippet 1: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:A
high-performance database product with AI capabilities]
id:lRLfac47fgnm2UERMyHDQjm5VqQDBp-0zexV_2OqiKcEuPgfK7ZmXbES562LpOgKzcwfFrqgUbaj55kkElhsmQ
name:map[type:String value:ekoDB] price:map[type:Integer value:99]]
score:0.1111111111111111] Snippet 2: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:Fully
managed cloud database service product]
id:-zBlU0cqQOiFtZcXJFyPIa4Xmc82zHX3HC1r3SgtidlGmy7EQqC1Ena9Yvxyu9870Hcg-fzOtGtvP1mz3NC_AA
name:map[type:String value:ekoDB Cloud] price:map[type:Integer value:499]]
score:0.1111111111111111] Snippet 3: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String
value:Enterprise edition product with advanced features]
id:boOLx4FxhpFuXb31aorBSBLVHlovAoAWsuvQy53WNTouyTBHlMDtNmUpDtz8xeMv0LWgLPwDNB5-i9ZLtigcag
name:map[type:String value:ekoDB Pro] price:map[type:Integer value:299]]
score:0.1111111111111111]

Execution Time: 3366ms

=== Token Usage === Prompt tokens: 372 Completion tokens: 75 Total tokens: 447

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
0P3dWB9CGf1pm6szSFSi3vAHdKEKtPPZzAxvmmdFsVzbDWBO6CAnvI-DKu57u3dLQ1deUbO5hw5Y1jEipepELA

=== Sending Messages === ✓ Message 1 sent Response: Based on the context
provided, the available product is a high-performance database product named
'ekoDB'. It is priced at $99.

✓ Message 2 sent Response: As an assistant, I'm sorry but I can't provide the
information you're looking for because the context doesn't mention any specific
product. Could you please provide more details?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
0eBMjp-wQMtZWLTb6IhSEeF4SiHY1NTfvJJkrRscBVvMDsPhSbqTqw6g3KecZ-9NkzMec5fSEbnnLALZlvyI-w
Parent:
0P3dWB9CGf1pm6szSFSi3vAHdKEKtPPZzAxvmmdFsVzbDWBO6CAnvI-DKu57u3dLQ1deUbO5hw5Y1jEipepELA

=== Listing Sessions === ✓ Found 9 sessions Session 1:
0eBMjp-wQMtZWLTb6IhSEeF4SiHY1NTfvJJkrRscBVvMDsPhSbqTqw6g3KecZ-9NkzMec5fSEbnnLALZlvyI-w
(Untitled) Session 2:
0P3dWB9CGf1pm6szSFSi3vAHdKEKtPPZzAxvmmdFsVzbDWBO6CAnvI-DKu57u3dLQ1deUbO5hw5Y1jEipepELA
(Untitled) Session 3:
TYpHvinc9xK3hjYSvriJhJn68EP9vZEn7AE0KjOj5eF_8JYy5SveK1nJpZaLQhU2IMipYdWJ5XdMke24w3-D9g
(Untitled) Session 4:
1ttWfumAX4sysCJO_c_HkD1p5NgOpYCon-2-Hj6mJWd5X7ALW2F59iTaGFokRTi_GJmHG3PBbDgZdXhqN1oFXA
(Untitled) Session 5:
iQoohxHDhoCePst9Ntir0pRa5_Wkn8ll8z5WjMZF_bJ-V93TcOhO6xDkK0hZpHPQ5BQPcqibZlVEgd-R9bh23g
(Untitled) Session 6:
GpJGyY4TwEnpt94EiY1-eMbr4fU7hhB3XOzUvfWBO8XEEDS_zisfZKCzKou-zE_b4zHYl5Vwh3mZVckj1JUgvw
(Untitled) Session 7:
DXKs6VxkJxJmJTavQYM3q_3MsGmZ4bb9tAl_ILih_VZ5d2qcQHGYuWnmWJuMtNLroXVVeZOwT4H8eKwSkowJ-A
(Untitled) Session 8:
HBxCxS4SggFkCb0KDnhHmx0K5E-UyoAonpKvBBkCSrsriWxIyyjwMs6rlkbUA\_\_N965YGH-8IGT-XGUoAgpnrA
(Untitled) Session 9:
ZURF2ib3SHaypBxnBrqL2naVxuCnl23gEz-QQRuu-1IuGSqjsj96IXO-7eXCKjQB0YqRQKufcOMqmuf5Kz2ZTw
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
0eBMjp-wQMtZWLTb6IhSEeF4SiHY1NTfvJJkrRscBVvMDsPhSbqTqw6g3KecZ-9NkzMec5fSEbnnLALZlvyI-w

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
yrJD57Y0R1cnTynEBWjXh_UZ3abgQ3VirDOQX3-U7rJsgbeXIWXfMaqjKHAu_xj-l2b5jCwOeFeE_RM8b4QHOw

=== Sending Initial Message === ✓ Message sent Response: Based on the
information provided, the available product is a high-performance database
product named ekoDB. It is priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
I'm sorry, but the context provided doesn't include information about the price
of ekoDB. I would recommend checking the official website or contacting the
sales department for the most accurate and up-to-date price.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
y4sBrm2usa1Ym6Z-DgK-RqBCJbLj4yykRZB5-unT6p0s-Qi5srry9G3BLck-gntosIj8WnT8PB5kL6EgM9pUiw
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
1. Score: 0.737
1. Score: 0.732
1. Score: 0.724

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.693
1. Score: 1.495
1. Score: 0.290

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

- Alice Johnson
- Bob Smith

2. Join with filtering: Found 1 users in Engineering

- Alice Johnson

3. Multi-collection join (users with departments and profiles): Found 2 users
   with multiple joins

- Alice Johnson
- Bob Smith

4. Join orders with user data: Found 2 completed orders

- Laptop ($1200)
- Mouse ($25)

5. Complex join with multiple conditions: Found 2 users with example.com emails

- Alice Johnson (alice@example.com)
- Bob Smith (bob@example.com)

=== Cleanup === ✅ Deleted test collections

✅ Join operations examples completed! ✅ Go client examples complete! ✅ All Go
integration tests complete! 📦 Building TypeScript client library...

> @ekodb/ekodb-client@0.2.1 prepare npm run build

> @ekodb/ekodb-client@0.2.1 build tsc

up to date, audited 6 packages in 1s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.2.1 build tsc

✅ TypeScript client built! 🧪 Running TypeScript client library examples...

added 1 package, removed 1 package, and audited 13 packages in 720ms

3 packages are looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'k2Mp_XiM60Ua-UqsVvlp0Iwz8BKB17oXESUUuzrypsx6PQCiQEubWgY3yuwI1Wip0EXTvXTR3MovWlUjzoDU8w'
}

=== Find by ID === Found: { value: { value: 42, type: 'Integer' }, active: {
value: true, type: 'Boolean' }, id:
'k2Mp_XiM60Ua-UqsVvlp0Iwz8BKB17oXESUUuzrypsx6PQCiQEubWgY3yuwI1Wip0EXTvXTR3MovWlUjzoDU8w',
name: { value: 'Test Record', type: 'String' } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { id:
'k2Mp_XiM60Ua-UqsVvlp0Iwz8BKB17oXESUUuzrypsx6PQCiQEubWgY3yuwI1Wip0EXTvXTR3MovWlUjzoDU8w',
active: { value: true, type: 'Boolean' }, value: 100, name: 'Updated Record' }

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
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
jj5iZwsfS_lUf_VHU-GDdIdThEhJPIBJA-D_0GaX1rUmYMPzA7Cs4Vy5tGU4b4OYYV6QeMLBPkRLnHY2nphhPg

=== List Collections === Total collections: 16 Sample collections:
batch_users,test_collection,schema_employees_client_go,ws_ttl_test,ttl_cache

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
5V1EI0eDOPiT-owhZeD4vwlHezd2BK9QKVeUNnmdilzxG0X37Txz8AhHp-YcBMi5GjawMxKS2LZGB6xQ7k3Q2Q

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
HTjmBy8GK5vvZELPF5t0ic2cJv0kqZA-824deYiyvC5JwJFwa13MUgHdqEPci9CgSQfLo1iCVKfXfXVf8hcimA

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
S8QTQhKvEYyMQbnOo0B1Lrjxa5HYKHHp8n0C-5ctejeHSCTTXGOFC6UskBux-z1qOrEyh-LepQzC0BLeEtjcNQ

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
iSde0iYSFynB6NsDSnUtAvhirGV3gfHPZ9jbI-6ZLuoz_NG7KuJqIKpSDCMSc1lPGRwzr473lQ4w1uINWHIb7A

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
uTrnJ39c17ZF3Uf8mJag0QTGBIiTwugsDhz-RZo7Qu9y_9fUaofc95RG6V2f7o76KcynNuGS3ZVTdhunVy5YmA

=== Sending Chat Message === Message ID:
qmx4nkffATRP_UdOwrJzrFHzYqAGy3syBEfCssJ1Y7qytIHlOBgGpSncdmFtxMEgQumoyoFsN2xh3bE-Tl8K9Q

=== AI Response === There are three products available:

1. ekoDB Cloud, a fully managed cloud database service product, priced at $499.
2. ekoDB Pro, an enterprise edition product with advanced features, priced at
   $299.
3. ekoDB, a high-performance database product with AI capabilities, priced at
   $99.

=== Context Used (3 snippets) === Snippet 1: { collection:
'client_chat_basic_ts', record: { description: { type: 'String', value: 'Fully
managed cloud database service product' }, price: { type: 'Integer', value: 499
}, name: { type: 'String', value: 'ekoDB Cloud' }, id:
'mX9x5rj1F35zBM2k9VfC4pvLC0ZHPl7KEeEawvP-0V0rUnRY0jcgCs9FDYmoaLJVW929F4fXymnYdh42LIuYmw'
}, score: 0.1111111111111111, matched_fields: [ 'description' ] } Snippet 2: {
collection: 'client_chat_basic_ts', record: { name: { type: 'String', value:
'ekoDB Pro' }, price: { value: 299, type: 'Integer' }, description: { type:
'String', value: 'Enterprise edition product with advanced features' }, id:
'RrkEnbVvQxUjh6Sik5evi5niGcikww9Ytr-I2LWEGLE8eSc475F4YEt6u0zn3nHRnx-WpGAWRWiLvKSJ_QVPuw'
}, score: 0.1111111111111111, matched_fields: [ 'description' ] } Snippet 3: {
collection: 'client_chat_basic_ts', record: { id:
'hg-K5hWPh87NYIueXk8hXiy1pAMT8koNLRtsufWgWDUM9bmwl0zukASXTdwV-F2NqcAp5C5NukSSdSSl96LFRQ',
description: { type: 'String', value: 'A high-performance database product with
AI capabilities' }, name: { value: 'ekoDB', type: 'String' }, price: { value:
99, type: 'Integer' } }, score: 0.1111111111111111, matched_fields: [
'description' ] }

Execution Time: 3044ms

=== Token Usage === Prompt tokens: 375 Completion tokens: 66 Total tokens: 441

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
oWSqv_t1UwvUDXjvQwOWKfC2p_PuWLGVt3L2J7VchPKsLQ1FDykQXlXqNFZlZar62q3dbUxZtFnZSnLSeDjIUA

=== Sending Messages === ✓ Message 1 sent Response: Based on the information
provided, the available product is ekoDB, which is a high-performance database
product. It is priced at $99.

✓ Message 2 sent Response: As an assistant, I'm sorry but I can't provide the
price because there's no specific product mentioned in the context. Could you
please specify the product you're interested in?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
0XVW5AnHQeNV0MwQHe5ijXR0UBs089IPeexCPl1OY3WaABJV3OqL1Xse3FBnInGMZZhaT5Nt0ytYnpSNPOtUog
Parent:
oWSqv_t1UwvUDXjvQwOWKfC2p_PuWLGVt3L2J7VchPKsLQ1FDykQXlXqNFZlZar62q3dbUxZtFnZSnLSeDjIUA

=== Listing Sessions === ✓ Found 10 sessions Session 1:
0XVW5AnHQeNV0MwQHe5ijXR0UBs089IPeexCPl1OY3WaABJV3OqL1Xse3FBnInGMZZhaT5Nt0ytYnpSNPOtUog
(Untitled) Session 2:
oWSqv_t1UwvUDXjvQwOWKfC2p_PuWLGVt3L2J7VchPKsLQ1FDykQXlXqNFZlZar62q3dbUxZtFnZSnLSeDjIUA
(Untitled) Session 3:
uTrnJ39c17ZF3Uf8mJag0QTGBIiTwugsDhz-RZo7Qu9y_9fUaofc95RG6V2f7o76KcynNuGS3ZVTdhunVy5YmA
(Untitled) Session 4:
y4sBrm2usa1Ym6Z-DgK-RqBCJbLj4yykRZB5-unT6p0s-Qi5srry9G3BLck-gntosIj8WnT8PB5kL6EgM9pUiw
(Untitled) Session 5:
0P3dWB9CGf1pm6szSFSi3vAHdKEKtPPZzAxvmmdFsVzbDWBO6CAnvI-DKu57u3dLQ1deUbO5hw5Y1jEipepELA
(Untitled) Session 6:
TYpHvinc9xK3hjYSvriJhJn68EP9vZEn7AE0KjOj5eF_8JYy5SveK1nJpZaLQhU2IMipYdWJ5XdMke24w3-D9g
(Untitled) Session 7:
1ttWfumAX4sysCJO_c_HkD1p5NgOpYCon-2-Hj6mJWd5X7ALW2F59iTaGFokRTi_GJmHG3PBbDgZdXhqN1oFXA
(Untitled) Session 8:
iQoohxHDhoCePst9Ntir0pRa5_Wkn8ll8z5WjMZF_bJ-V93TcOhO6xDkK0hZpHPQ5BQPcqibZlVEgd-R9bh23g
(Untitled) Session 9:
GpJGyY4TwEnpt94EiY1-eMbr4fU7hhB3XOzUvfWBO8XEEDS_zisfZKCzKou-zE_b4zHYl5Vwh3mZVckj1JUgvw
(Untitled) Session 10:
DXKs6VxkJxJmJTavQYM3q_3MsGmZ4bb9tAl_ILih_VZ5d2qcQHGYuWnmWJuMtNLroXVVeZOwT4H8eKwSkowJ-A
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
0XVW5AnHQeNV0MwQHe5ijXR0UBs089IPeexCPl1OY3WaABJV3OqL1Xse3FBnInGMZZhaT5Nt0ytYnpSNPOtUog

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
qPbkysbkG8svfXtHgWSptsVKLx4VBEnh0thLEdTdLGgRK4HG2Ydd8KCOFFGM9rXvWXwkSi315VfozuY-cQnumQ

=== Sending Initial Message === ✓ Message sent Response: Based on the provided
context, the available product is ekoDB, a high-performance database product. It
is priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
I'm sorry, but the context provided does not include information on the price of
ekoDB. I recommend checking the product's official website or contacting their
sales department for the most accurate and up-to-date pricing information.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
jPwNOojN5scvSm6YQaBk2uGxyXEehdPpuzjyeq17gNXXNZFrjxA8KsW5UW1kxsLV3XpokXZBU6IfkAwNSWe15A
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
1. Score: 25.740, Matched: name.value, email, name, email.value
1. Score: 12.540, Matched: name, name.value

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: title.value, bio.value, title, bio
1. Score: 26.400, Matched: title, bio.value, bio, title.value
1. Score: 26.400, Matched: title.value, bio, title, bio.value
1. Score: 26.400, Matched: bio, title.value, title, bio.value

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: title.value, bio, title, bio.value
1. Score: 39.600, Matched: bio.value, title, title.value, bio
1. Score: 39.600, Matched: bio.value, title, bio, title.value
1. Score: 39.600, Matched: bio, bio.value, title, title.value

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio, bio.value

1. Search with stemming and exact match boosting: Found 1 results (matches:
   work, working, worked)
1. Score: 13.200, Matched: bio, bio.value

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.767, Matched:
1. Score: 0.760, Matched:
1. Score: 0.746, Matched:

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.698, Matched: title.value, title, content.value, content
1. Score: 1.507, Matched: content, title, title.value, content.value
1. Score: 0.304, Matched:

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

- Alice Johnson: No department
- Bob Smith: No department

2. Join with filtering: Found 1 users in Engineering:

- Alice Johnson: Unknown

3. Multi-collection join (users with departments and profiles): Found 2 users
   with multiple joins:

- Alice Johnson: Department: N/A Bio: N/A
- Bob Smith: Department: N/A Bio: N/A

4. Join orders with user data: Found 2 completed orders:

- Laptop ($1200) by Unknown
- Mouse ($25) by Unknown

5. Complex join with multiple conditions: Found 2 users with example.com emails:

- Alice Johnson (alice@example.com): N/A
- Bob Smith (bob@example.com): N/A

=== Cleanup === ✅ Deleted test collections

✅ Join operations examples completed! ✅ TypeScript client examples complete!
✅ All TypeScript integration tests complete! 🧪 Running JavaScript examples
(direct HTTP/WebSocket)...

added 1 package, removed 1 package, and audited 9 packages in 599ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities

╔════════════════════════════════════════╗ ║ ekoDB JavaScript Examples Test
Suite ║ ╚════════════════════════════════════════╝

=== Checking Server Connection === (node:50698) [DEP0040] DeprecationWarning:
The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created) ✓
Server is ready

=== Getting Authentication Token === ✓ Authentication successful

=== Running 7 Examples ===

=== Running /ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js
=== ✓ Authentication successful

=== Insert Document === (node:50701) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) Inserted:
{ id:
'ygptf3EuHBBZAPLY0KPEtQ76ZvZGmS44njmxuvDVxx2I42CcD6xdhKHX1Wh5y9IXQpS09zWG0oc0aDRJd9qaEA'
}

=== Find by ID === Found: { name: { type: 'String', value: 'Test Record' },
active: { type: 'Boolean', value: true }, value: { value: 42, type: 'Integer' },
id:
'ygptf3EuHBBZAPLY0KPEtQ76ZvZGmS44njmxuvDVxx2I42CcD6xdhKHX1Wh5y9IXQpS09zWG0oc0aDRJd9qaEA'
}

=== Find with Query === Found documents: 1

=== Update Document === Updated: { active: { type: 'Boolean', value: true },
name: 'Updated Record', value: 100, id:
'ygptf3EuHBBZAPLY0KPEtQ76ZvZGmS44njmxuvDVxx2I42CcD6xdhKHX1Wh5y9IXQpS09zWG0oc0aDRJd9qaEA'
}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully ✓ simple_crud.js completed
successfully

=== Running
/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js === ✓
Authentication successful

=== Inserting Test Data === (node:50722) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
test record:
gcSW3HFs2OM8aobFDZMZ7i5UiPeSzVoGU8j5PO3i4-MXH1EG6vsio6vuUszD3NOxxfqFvSwCKfj5VnIzSRB9GA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"2bEt2dz6vweiMDpuewUo_4SvAC0luxh7rUxc0l2bMF3J9ZHmw7Q7OynpyxnVUf-0oXkEH3sQYnkWcobtrWN6ig",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"WEGxpeB4YnE2t1l48ZiBo0r0i-0OapZon3QrGoRFCcA703ICWHmvRMcARry3o1kBEGbIlovbbn-bIJ-oLLmGbg",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"8U3djf9soDBOt9Ukki_aeb5abDxCt-CCiku0KZpBkWaBhFqQCRmBBv6v2-_lm61KF2WgBxA1C5bSyjyoAb-UJQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"gcSW3HFs2OM8aobFDZMZ7i5UiPeSzVoGU8j5PO3i4-MXH1EG6vsio6vuUszD3NOxxfqFvSwCKfj5VnIzSRB9GA",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 4
record(s) via WebSocket

✓ WebSocket example completed successfully ✓ simple_websocket.js completed
successfully

=== Running
/ekoDB/ekodb-client/examples/javascript/ttl-caching/batch_operations.js === ✓
Authentication successful

=== Batch Insert === (node:50725) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Batch
inserted 5 records ✓ Verified: Found 20 total records in collection

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === ✓ Batch updated 3 records ✓ Verified: Record updated with
status="active"

=== Batch Delete === ✓ Batch deleted 3 records ✓ Verified: Records successfully
deleted (not found)

✓ All batch operations completed successfully ✓ batch_operations.js completed
successfully

=== Running /ekoDB/ekodb-client/examples/javascript/ttl-caching/kv_operations.js
=== ✓ Authentication successful

=== KV Set === (node:50734) [DEP0040] DeprecationWarning: The `punycode` module
is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Set key:
session:user123

=== KV Get === Retrieved value: { loginTime: '2025-10-15T07:47:31.615Z', userId:
123, username: 'john_doe' }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ kv_operations.js completed
successfully

=== Running
/ekoDB/ekodb-client/examples/javascript/ttl-caching/collection_management.js ===
✓ Authentication successful

=== Create Collection (via insert) === (node:50750) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) Collection created with first record:
jfdLMYXUyOuX6zrVb-GhoaIYWlDa3IgGAQL1AZgXw4t98HafqBnfI8RCeYQQ-BGpsfSVMESIfGTx6WxtmVkz1A

=== List Collections === Total collections: 20 Sample collections: [
'schema_products_client_ts', 'batch_users', 'test_collection',
'schema_documents_client_ts', 'schema_employees_client_go' ]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓
collection_management.js completed successfully

=== Running /ekoDB/ekodb-client/examples/javascript/ttl-caching/document_ttl.js
=== ✓ Authentication successful

=== Insert Document with TTL (1 hour) === (node:50753) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) ✓ Inserted document:
4UodkzuHBCkB2vXNRcxyhJC7u60dXuX5F0pj3bXq5YkucI8Wx5MYUurpMBMogVmzc0JPxOvnpWuuob3oRTGVPQ

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 4 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully ✓ document_ttl.js completed
successfully

=== Running /ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js
=== ✓ Authentication successful

=== Insert Test Data with TTL === (node:50770) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
document with TTL:
loO_oghFYhfz4nRp6HIxqUvY5mOjgq6Eng6OKiLh3v4K8T1cL9b7rbMN7HZEQ-MG84lwPeNcgWU3TdO08Xm_yg

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 4 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully ✓ websocket_ttl.js completed
successfully

╔════════════════════════════════════════╗ ║ Test Summary ║
╚════════════════════════════════════════╝ Total: 7 Passed: 7 Failed: 0 ✅
JavaScript direct examples complete! 🧪 Running JavaScript client library
examples...

added 1 package, removed 1 package, and audited 9 packages in 647ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'xZYHrYI7ImLniPsXFBqa8KODA6deZ-os_nXyO1RH_XuZmvh0EIIhVKoK406ktGVq1BVzmlZDZRpvlMUtNQSwZA'
}

=== Find by ID === Found: { name: { type: 'String', value: 'Test Record' },
value: { type: 'Integer', value: 42 }, active: { value: true, type: 'Boolean' },
id:
'xZYHrYI7ImLniPsXFBqa8KODA6deZ-os_nXyO1RH_XuZmvh0EIIhVKoK406ktGVq1BVzmlZDZRpvlMUtNQSwZA'
}

=== Find with Query === Found documents: 1

=== Update Document === Updated: { id:
'xZYHrYI7ImLniPsXFBqa8KODA6deZ-os_nXyO1RH_XuZmvh0EIIhVKoK406ktGVq1BVzmlZDZRpvlMUtNQSwZA',
name: 'Updated Record', value: 100, active: { type: 'Boolean', value: true } }

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

=== Get Multiple Keys === cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
3NKI8dzl40UPoZJ5B_ka-6BH_juqTg3oVCkvX0T4CeTrMEpoe6oJP17TkF3KMVNtYn80ROkFbe37HYc2oUY44g

=== List Collections === Total collections: 20 Sample collections:
schema_products_client_ts,batch_users,test_collection,schema_documents_client_ts,schema_employees_client_go

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
aXgkZogG7Viwr3izS3D5q7pSKLcp5K7wcRNlFNxZhg8UuqnMdYIB-JfBpDtN72XsBEPZWycSYYv8RfZ-3ytYCg

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
ycTNY9IkxCQ5mq_dB57u4J-01hFa8Lk6pfU_lPvY2zRkcgc6LC5neNxlLTrHEWvJrrWsUQMnzCQFfuvewZ-Ukg

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
x0pV0hRtL6FjaIOIIB8fcry-BhhMdatjwb_iIGFAFWUQ8wNgkEGj7ZJnMPELC6BdUMDbPDANeLcrqSGD4bui2A

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
\_g2vx74d8j17q63gsKKbMvraMYILB-PSST0Axn8ZqjWUdl2IcJr_HUIQhmrDZBqvfKgssVht9TemypXgwiYwPg

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✅ JavaScript client examples complete! ✅ All JavaScript integration
tests complete! 🟣 Building Kotlin client library... To honour the JVM settings
for this build a single-use Daemon process will be forked. For more on this,
please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

BUILD SUCCESSFUL in 3s 3 actionable tasks: 1 executed, 2 up-to-date ✅ Kotlin
client built! 🧪 Running Kotlin client library examples... To honour the JVM
settings for this build a single-use Daemon process will be forked. For more on
this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - Simple CRUD Example ===

=== Create === ✓ Inserted user:
Record(fields={id=StringValue(value=bdPavszvsfOtEb8x2zFMWrju75xcBAGeeE-SSORjBwcH6kr4sp-vNO6p6wLWjb3IshF89hrPDMTsl6k_YpynJg)})
User ID:
bdPavszvsfOtEb8x2zFMWrju75xcBAGeeE-SSORjBwcH6kr4sp-vNO6p6wLWjb3IshF89hrPDMTsl6k_YpynJg

=== Read === ✓ Found user by ID:
Record(fields={id=StringValue(value=bdPavszvsfOtEb8x2zFMWrju75xcBAGeeE-SSORjBwcH6kr4sp-vNO6p6wLWjb3IshF89hrPDMTsl6k_YpynJg),
name=ObjectValue(value={value=StringValue(value=Alice Johnson),
type=StringValue(value=String)}),
email=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=alice@example.com)}),
age=ObjectValue(value={value=IntegerValue(value=28),
type=StringValue(value=Integer)}),
active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)})})

=== Update === ✓ Updated user: Record(fields={city=StringValue(value=San
Francisco), age=IntegerValue(value=29),
email=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=alice@example.com)}),
id=StringValue(value=bdPavszvsfOtEb8x2zFMWrju75xcBAGeeE-SSORjBwcH6kr4sp-vNO6p6wLWjb3IshF89hrPDMTsl6k_YpynJg),
active=ObjectValue(value={value=BooleanValue(value=true),
type=StringValue(value=Boolean)}),
name=ObjectValue(value={value=StringValue(value=Alice Johnson),
type=StringValue(value=String)})})

=== Query === ✓ Found 1 users matching query

- Record(fields={age=ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=29)}),
  active=ObjectValue(value={value=BooleanValue(value=true),
  type=StringValue(value=Boolean)}),
  id=StringValue(value=bdPavszvsfOtEb8x2zFMWrju75xcBAGeeE-SSORjBwcH6kr4sp-vNO6p6wLWjb3IshF89hrPDMTsl6k_YpynJg),
  name=ObjectValue(value={value=StringValue(value=Alice Johnson),
  type=StringValue(value=String)}),
  city=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=San Francisco)}),
  email=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=alice@example.com)})})

=== Delete === ✓ Deleted user with ID:
bdPavszvsfOtEb8x2zFMWrju75xcBAGeeE-SSORjBwcH6kr4sp-vNO6p6wLWjb3IshF89hrPDMTsl6k_YpynJg

✓ Confirmed user was deleted

=== Cleanup === ✓ Deleted collection: kotlin_users_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 3 actionable tasks: 2 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - Batch Operations Example ===

=== Batch Insert === ✓ Inserted 5 records IDs:
u82q4F5RkmPZVJIUP852e8Sp_oqR6eNWdXYA2_cnvRorS15qG2HgZR5azFn3y7f_zSXOAykWWGdjdrkuS9412w,
OsX60KSP9x-hu7H2D2YOZJIXFev35Nm1YqVLN1yFMz7Xw6TemyDAuNuK6KYyon1TpVQvzygd3pFvpO0KLOcDFQ,
V34hTamgd8D9AK8ui1r1AzWwziJ666SU7Ro9ctWMdgWnmaArlVszT0w3Eff3m87k73aZVGNJKFo1hFr-nv53Gg...

=== Batch Update === ✓ Updated 3 records

=== Batch Delete === ✓ Deleted 2 records

=== Cleanup === ✓ Deleted collection: kotlin_batch_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 3 actionable tasks: 2 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - Query Builder Example ===

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

BUILD SUCCESSFUL in 3s 3 actionable tasks: 2 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - KV Operations Example ===

=== KV Set === ✓ Set key: user:123

=== KV Get === ✓ Retrieved value:
{"name":"Alice","email":"alice@example.com","role":"admin"}

=== KV Set with TTL === ✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key === ✓ Session value:
{"user_id":"123","created_at":1760514480558} (Will expire in 10 seconds)

=== Set Multiple Keys === ✓ Set 3 configuration keys

=== KV Delete === ✓ Deleted key: user:123

=== Verify Deletion === ✓ Confirmed key was deleted

=== Cleanup === ✓ Cleaned up test keys

=== Example Complete ===

BUILD SUCCESSFUL in 3s 3 actionable tasks: 2 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - Collection Management Example ===

=== List Collections === ✓ Found 19 collections

- schema_products_client_ts
- batch_users
- test_collection
- schema_documents_client_ts
- schema_employees_client_go

=== Check Collection Existence === Collection 'kotlin_collection_example'
exists: false

=== Create Collection with Schema === ✓ Created collection with schema:
kotlin_collection_example

=== Get Collection Schema === ✓ Schema:
{"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-10-15T07:48:04.387770Z","last_modified":"2025-10-15T07:48:04.387770Z","bypass_ripple":false}

=== Cleanup === ✓ Deleted collection: kotlin_collection_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 3 actionable tasks: 2 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - Document TTL Example ===

=== Insert with TTL === ✓ Inserted document with 10s TTL Document ID:
X_AZNedTiJveitF72_a3xjE9sD24GXO1GCaswrNTz3jwH9K8Chlv5xtqu-dW7pwncPsBNvZoU3z4Sk5Tlo6cEA

=== Verify Document Exists === ✓ Document found: [session_id, created_at,
ttl_duration, user_id, id]

=== Insert with Longer TTL === ✓ Inserted document with 1h TTL Document ID:
mHnFdq0qSP2FXxnPTJpwqF6bGO8fLWhRkFoeLe9Z0akjicAsVJHj5WPpUqg09iXEURAjmWCO*JnulE7*-rEPxg

=== TTL Expiration === ✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document === ✓ Long TTL document still exists:
[ttl_duration, id, value, cache_key]

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection: kotlin_ttl_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 3 actionable tasks: 2 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client === ✓ WebSocket client created

=== Connect to WebSocket === ✓ Connected to WebSocket

=== Insert Test Record === ✓ Inserted test record

=== Find All via WebSocket === ✓ WebSocket findAll result:
{"payload":{"data":[{"id":"gQec42xBgZS-ySG6ucsb9FN9cjbp_popWnIGK9zljvO0Drt9QRDXFLO4ZKH7uNFm-3yfKeO0bmjDvdM8LZ3aPQ","name":{"type":"String","value":"Test
User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

=== Close WebSocket === ✓ WebSocket closed

=== Cleanup === ✓ Deleted collection: kotlin_websocket_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 3 actionable tasks: 2 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - Schema Management Example ===

=== Creating Collection with Schema === ✓ Created collection
'kotlin_schema_example' with schema

=== Inserting Valid Documents === ✓ Inserted user 1:
StringValue(value=QU7Vxx2a17hxdgW1_xtGLIKIWKHu5BcEa-vTqxBaY1U6zBqyxjnoCwyL-IEySUD4pGlTwvq3rtbufWL8st05qA)
✓ Inserted user 2:
StringValue(value=A99xDihZHNaYQpmuZQB1SNvRqmAqJmhdmnqaAfkda5cSKtVWq5vULwDwWUtRIZUqRJz9Hz6ssGpmyI4fE3VrrA)

=== Getting Schema === ✓ Schema for kotlin_schema_example: Fields:
{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections === ✓ Total collections: 20 Sample:
[schema_products_client_ts, batch_users, test_collection,
schema_documents_client_ts, kotlin_schema_example]

=== Cleanup === ✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

BUILD SUCCESSFUL in 3s 3 actionable tasks: 2 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - Search Example ===

=== Inserting Sample Documents === ✓ Inserted 5 sample documents

=== Basic Text Search === ✓ Search results for 'programming':
{"results":[{"record":{"views":558,"description":"Learn Rust programming
language with hands-on examples and best practices.","title":"Rust
Programming","tags":"programming,rust,tutorial","id":"sE52la6QSou_xxBz5qoHjyXDexUesFSYNVe3u7M06RcCZYEh8-pEhVZNrr8mKryYprL6Fhd8xrpYOXFtYiCftw"},"score":19.8,"matched_fields":["description","title","tags"]},{"record":{"views":54,"title":"Python
for Data
Science","id":"Xc-MBiwjp1wsrchH3sTBQrKQZtb3Sj4_UZRg5kWHH8zOah9fnpc6kfwIQYUCG-OpAhFy_o5p3Hk_ffuHge6A5g","tags":"programming,python,data-science","description":"Master
Python for data analysis, machine learning, and
visualization."},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"views":736,"title":"JavaScript
Web
Development","id":"aJ1OT3Mo3VQ-d-72wE8_4ZGISkA61kXUBiuU9_vfIL5fIHCih4pYzj076\_\_6444v7pzuAh_5kBvZ6nQmkI7kMw","description":"Build
modern web applications using JavaScript, React, and
Node.js.","tags":"programming,javascript,web"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' === ✓ Found results for 'machine learning'
{"results":[{"record":{"title":"Machine Learning
Basics","description":"Introduction to machine learning algorithms and neural
networks.","id":"s5fmaxkt7UahSFYdFK-QuRL2Ic0Mvi9XC2rVAqjJYAgIYI8jDqkQLkxxMeG_ACH4ru-ymvCxndAYFMgXNjlslA","views":764,"tags":"ai,machine-learning,python"},"score":2.7,"matched_fields":["description","tags","title"]},{"record":{"title":"Python
for Data
Science","id":"Xc-MBiwjp1wsrchH3sTBQrKQZtb3Sj4_UZRg5kWHH8zOah9fnpc6kfwIQYUCG-OpAhFy_o5p3Hk_ffuHge6A5g","views":54,"tags":"programming,python,data-science","description":"Master
Python for data analysis, machine learning, and
visualization."},"score":1.0,"matched_fields":["description"]},{"record":{"title":"Rust
Programming","views":558,"description":"Learn Rust programming language with
hands-on examples and best
practices.","tags":"programming,rust,tutorial","id":"sE52la6QSou_xxBz5qoHjyXDexUesFSYNVe3u7M06RcCZYEh8-pEhVZNrr8mKryYprL6Fhd8xrpYOXFtYiCftw"},"score":0.5,"matched_fields":["description"]},{"record":{"tags":"database,design,sql","description":"Learn
database design principles, normalization, and query
optimization.","title":"Database
Design","id":"-Bxz9b9OcAdpk0zvO2fU-3_5q3ZY1hkY37BQtDHcxbSvMoUIbKBjR4QBWx92OR-nsmw3qp7E_Tk911GBtMKEng","views":660},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

=== Cleanup === ✓ Deleted collection: kotlin_search_example

✓ All search operations completed successfully

BUILD SUCCESSFUL in 3s 3 actionable tasks: 2 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - WebSocket TTL Example ===

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
StringValue(value=BVGwnTXyyVR7KHJMTEtBh7eZfzzAE3X8ywlEfJEi2qX1YZQH5OtyFAEZua59RJP7if79oOXAQCxFCgcxPZ5Neg)

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved data via
WebSocket:
{"payload":{"data":[{"created_at":{"type":"Integer","value":1760514503457},"id":"BVGwnTXyyVR7KHJMTEtBh7eZfzzAE3X8ywlEfJEi2qX1YZQH5OtyFAEZua59RJP7if79oOXAQCxFCgcxPZ5Neg","name":{"type":"String","value":"WebSocket
TTL
Test"},"ttl_duration":{"type":"String","value":"1h"},"value":{"type":"Integer","value":42}}]},"type":"Success"}

✓ WebSocket closed

=== Cleanup === ✓ Deleted collection: kotlin_websocket_ttl_example

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration

BUILD SUCCESSFUL in 3s 3 actionable tasks: 2 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - Join Operations Example ===

=== Setting up sample data === ✓ Sample data created

=== Example 1: Single collection join (users with departments) === ✓ Found 2
users with department data

- Alice Johnson
- Bob Smith

=== Example 2: Join with filtering === ✓ Found 2 users in Engineering

- Alice Johnson
- Bob Smith

=== Example 3: Multi-collection join (users with departments and profiles) === ✓
Found 2 users with department join

- Alice Johnson
- Bob Smith

=== Example 4: Join orders with user data === ✓ Found 3 completed orders

- Keyboard ($75)
- Laptop ($1200)
- Mouse ($25)

=== Example 5: Complex join with multiple conditions === ✓ Found 2 users with
example.com emails

- Alice Johnson (alice@example.com)
- Bob Smith (bob@example.com)

=== Cleanup === ✓ Deleted test collections

✓ Join operations example completed successfully

BUILD SUCCESSFUL in 3s 3 actionable tasks: 2 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
-VD0BQIJ7Rbzf6LHZgzZdMA-k9W45shvFrpDYU9nyovDKmqV7LzMIReZLgcjw_7-jkYkQ62mKyH1cln2L8LlBg

=== Sending Chat Message === ✓ Chat response: Message ID:
"vOO2Xm2_iEtZ9muGcCw6H7v2EgsLIlHiF0QIIfKCaORJIbPxjMuw50moVN-YcR52uoLWakp2MnuFtTL_mT7gCA"
Responses: ["Answer: ekoDB is a high-performance database that offers
intelligent caching, real-time capabilities, and AI integration. It comes with
several notable features including a chat feature that allows you to query your
database using natural language and receive AI-powered responses with relevant
context. Moreover, ekoDB supports different types of search including full-text
search, vector search, and hybrid search with automatic context retrieval."]

=== Cleanup === ✓ Deleted chat session ✓ Deleted collection:
kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 5s 3 actionable tasks: 2 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
yRDF-4LViH56CG4PX5Q7vdhyPVFeV6zdT80kgW26CZC1I7w7Z6nC_xIo_VXdz0-pZsSFMrINUMOWZoRM53Se4A

=== Sending Messages === ✓ Message 1 sent Responses: ["Based on the information
provided in the context, the available product is ekoDB, which is a
high-performance database product with AI capabilities. It is priced at $99."]

✓ Message 2 sent Responses: ["I'm sorry, but there seems to be no information
provided about the product or its price in the context. Could you please specify
which product you're interested in?"]

=== Getting Message History === ✓ Retrieved message history Total messages:
[{"chat_id":{"type":"String","value":"yRDF-4LViH56CG4PX5Q7vdhyPVFeV6zdT80kgW26CZC1I7w7Z6nC_xIo_VXdz0-pZsSFMrINUMOWZoRM53Se4A"},"content":{"type":"String","value":"What
products are
available?"},"context_snippets":{"type":"Array","value":[{"type":"Object","value":{"collection":{"type":"String","value":"kotlin_chat_sessions_example"},"matched_fields":{"type":"Array","value":[{"type":"String","value":"description"}]},"record":{"type":"Object","value":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":{"type":"String","value":"VXKM7lNpKew1TbKi2uzpVXhwgZujFqfdXzcIF3Gostu9_oT0g8m2j2ZiPvEKDxfrjDvILQISCZiSyMw7WT7fjg"},"price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}}},"score":{"type":"Float","value":0.25}}}]},"created_at":{"type":"DateTime","value":"2025-10-15T07:48:39.824957Z"},"id":"WKNSqgj9Ejbww9FXR8UvoFGfQ9dmmGg5GtntLhtQcPfWPYX-57-78SFyEFvnMRhsi9l25_eG1md0NRH7xkChtA","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":35},"prompt_tokens":{"type":"Integer","value":170},"total_tokens":{"type":"Integer","value":205}}},"updated_at":{"type":"DateTime","value":"2025-10-15T07:48:39.824957Z"}},{"chat_id":{"type":"String","value":"yRDF-4LViH56CG4PX5Q7vdhyPVFeV6zdT80kgW26CZC1I7w7Z6nC_xIo_VXdz0-pZsSFMrINUMOWZoRM53Se4A"},"content":{"type":"String","value":"Based
on the information provided in the context, the available product is ekoDB,
which is a high-performance database product with AI capabilities. It is priced
at
$99."},"context_snippets":{"type":"Array","value":[{"type":"Object","value":{"collection":{"type":"String","value":"kotlin_chat_sessions_example"},"matched_fields":{"type":"Array","value":[{"type":"String","value":"description"}]},"record":{"type":"Object","value":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":{"type":"String","value":"VXKM7lNpKew1TbKi2uzpVXhwgZujFqfdXzcIF3Gostu9_oT0g8m2j2ZiPvEKDxfrjDvILQISCZiSyMw7WT7fjg"},"price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}}},"score":{"type":"Float","value":0.25}}}]},"created_at":{"type":"DateTime","value":"2025-10-15T07:48:39.828307Z"},"id":"gqmYwL-JEV7NzdnWSIhG0f224iZ67U4YzKbXayBNQR_aIojRibPRzcmld-LXjMNz_m8ZvX9qxz6oyZoEKyS-pQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":35},"prompt_tokens":{"type":"Integer","value":170},"total_tokens":{"type":"Integer","value":205}}},"updated_at":{"type":"DateTime","value":"2025-10-15T07:48:39.828307Z"}},{"chat_id":{"type":"String","value":"yRDF-4LViH56CG4PX5Q7vdhyPVFeV6zdT80kgW26CZC1I7w7Z6nC_xIo_VXdz0-pZsSFMrINUMOWZoRM53Se4A"},"content":{"type":"String","value":"What
is the
price?"},"created_at":{"type":"DateTime","value":"2025-10-15T07:48:41.560339Z"},"id":"aKj8WgBu9FBTmsVzhk73fyUXhHbmMJS5-Eli6KzEQBlt9c0kGBP0lvAJvm0dU1qm1-JVxQBkS4KSJ8_Dar6qxw","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":33},"prompt_tokens":{"type":"Integer","value":63},"total_tokens":{"type":"Integer","value":96}}},"updated_at":{"type":"DateTime","value":"2025-10-15T07:48:41.560339Z"}},{"chat_id":{"type":"String","value":"yRDF-4LViH56CG4PX5Q7vdhyPVFeV6zdT80kgW26CZC1I7w7Z6nC_xIo_VXdz0-pZsSFMrINUMOWZoRM53Se4A"},"content":{"type":"String","value":"I'm
sorry, but there seems to be no information provided about the product or its
price in the context. Could you please specify which product you're interested
in?"},"created_at":{"type":"DateTime","value":"2025-10-15T07:48:41.563381Z"},"id":"a1q3hFEKDeIjyim1aVt3y6vYs7wbiw2zCUyE_1qhHOa1L4TrGsPNDxmteIk7QapwXw71NrV2cVT-8ZDbWLTDUQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":33},"prompt_tokens":{"type":"Integer","value":63},"total_tokens":{"type":"Integer","value":96}}},"updated_at":{"type":"DateTime","value":"2025-10-15T07:48:41.563381Z"}}]

=== Updating Session === ✓ Updated session system prompt

=== Listing Sessions === ✓ Total sessions: 13

=== Branching Session === ✓ Created branched session:
Qrjypue1TP2HBpPcRMzNh4USx6X9HM3Y3KV3UsRcOeuuUHPvHJ_yTLG8QA26lgR0cevgwWaWUjCcrS5CL5abAg

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 7s 3 actionable tasks: 2 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
jXxfVecUmdw2e0rRQrG3RQdoJNtJxf7M8eCNwYd0sZZ4t4eztFhoLRF0UVibMI1J45K4abQP8GXuq_APGnjw0A

=== Sending Initial Message === ✓ Message sent Responses: ["Based on the
information provided, the available product is ekoDB. It is a high-performance
database product priced at $99."]

✓ Second message sent

=== Regenerating AI Response === ✓ AI response regenerated New responses:
["Based on the information provided, the available product is a high-performance
database product named ekoDB. It is priced at 99."]

=== Updating Message === ✓ Updated message content

=== Toggling Forgotten Status === ✓ Marked message as forgotten (excluded from
context)

=== Creating Second Session for Merge === ✓ Created second session:
4b6A846f2kCRB-L35hnELKKNURbWpqdJnuIJ3E8JSBTVtmTT0ZKi2eftS8IfN9XWEVHemAaZZQyrSPaZTJI05g

=== Merging Sessions === ✓ Merged sessions Merged session ID: null

=== Deleting Message === ✓ Deleted message

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 9s 3 actionable tasks: 2 executed, 1 up-to-date ✅ Kotlin
client examples complete! ✅ All Kotlin integration tests complete! ✅ All
integration tests complete!
