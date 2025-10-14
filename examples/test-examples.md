tek@Seans-MacBook-Pro ~/Development/teknology/ekodb
[feat/ekodb-client-package+*]$ make test-examples 🧪 Running Rust examples
(direct HTTP/WebSocket)... Finished `dev` profile [unoptimized + debuginfo]
target(s) in 0.13s Running `target/debug/examples/simple_crud` ✓ Authentication
successful

=== Insert Document === Inserted: Object {"id":
String("gw26DJPLRgWThHjNYjgLAc725xLKbSwFSPB1SuDvkFwXgOmJyT4-EWp0QSWgqga6N1UVhWD7Ovj4m7W7FANDyg")}

=== Find by ID === Found: Object {"active": Object {"type": String("Boolean"),
"value": Bool(true)}, "id":
String("gw26DJPLRgWThHjNYjgLAc725xLKbSwFSPB1SuDvkFwXgOmJyT4-EWp0QSWgqga6N1UVhWD7Ovj4m7W7FANDyg"),
"name": Object {"type": String("String"), "value": String("Test Record")},
"value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query === Found documents: Array [Object {"active": Object
{"type": String("Boolean"), "value": Bool(true)}, "id":
String("gw26DJPLRgWThHjNYjgLAc725xLKbSwFSPB1SuDvkFwXgOmJyT4-EWp0QSWgqga6N1UVhWD7Ovj4m7W7FANDyg"),
"name": Object {"type": String("String"), "value": String("Test Record")},
"value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document === Updated: Object {"active": Object {"type":
String("Boolean"), "value": Bool(true)}, "id":
String("gw26DJPLRgWThHjNYjgLAc725xLKbSwFSPB1SuDvkFwXgOmJyT4-EWp0QSWgqga6N1UVhWD7Ovj4m7W7FANDyg"),
"name": String("Updated Record"), "value": Number(100)}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/teknology/ekodb/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.66s Running
`target/debug/examples/simple_websocket` ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
"Ud9HWP3Q5qdR_BH_mcPhrPS0ThE9Esc6Pf2Egw7uZzJFsyHyEijHPBaXdseil8tIctclNV6nR6sDcEgosoUNVQ"

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"Ud9HWP3Q5qdR_BH_mcPhrPS0ThE9Esc6Pf2Egw7uZzJFsyHyEijHPBaXdseil8tIctclNV6nR6sDcEgosoUNVQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 1
record(s) via WebSocket

✓ WebSocket example completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/teknology/ekodb/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.50s Running
`target/debug/examples/batch_operations` ✓ Authentication successful

=== Batch Insert === ✓ Batch inserted 5 records ✓ Verified: Found 5 total
records in collection

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === ✓ Batch updated 3 records ✓ Verified: Record updated with
status="active"

=== Batch Delete === ✓ Batch deleted 3 records ✓ Verified: Records successfully
deleted (not found)

✓ All batch operations completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/teknology/ekodb/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.46s Running
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
(/Users/tek/Development/teknology/ekodb/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.47s Running
`target/debug/examples/collection_management` ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
"LuIm6RhbS3kN-ew3LKLPNMjoGwZu1AJD2TxuwXBSljkceE-xE09FcUAlbeQWBeT-j_VtXbl4ROBauK9MFMfyDQ"

=== List Collections === Total collections: 21 Sample collections:
["non_indexed_vector_perf", "indexed_query_scale_10k", "indexed_search_perf",
"text_search_articles", "indexed_vector_perf", "benchmark_test_collection",
"demo_collection", "non_indexed_query_scale_10k", "non_indexed_hybrid_scale",
"indexed_hybrid_scale", "usertable", "non_indexed_scale_1k",
"indexed_query_perf", "non_indexed_search_perf", "indexed_scale_1k",
"vector_search_articles", "batch_users", "simple_crud_client_py",
"websocket_test", "test_collection", "non_indexed_query_perf"]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/teknology/ekodb/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.46s Running
`target/debug/examples/document_ttl` ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
"EWp_yKaailceMWTevZ9OUbgFdcaOWlQH8aV2aAPvQ78CMFkrCEZind6DExSm-2fSVoLBFsyAVI6473JAr634WQ"

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: null

=== Query Documents === ✓ Found 1 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

✓ All document TTL operations completed successfully Compiling ekodb-examples
v0.1.0 (/Users/tek/Development/teknology/ekodb/examples/rust) Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.66s Running
`target/debug/examples/websocket_ttl` ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
String("uuzgeH4TYJPlCoIwb_hep6W3gSUzmbfmebhlkdvbLw24XjFz_5D5Io3dlt3gE3gfVSxy-7u8fGcjBqb8U1wGSg")

=== Query via WebSocket === ✓ WebSocket connected

✓ WebSocket TTL example completed successfully ✅ Rust direct examples complete!
🛠️ Building client library... cargo build -p ekodb_client Compiling ekodb_client
v0.1.0 (/Users/tek/Development/teknology/ekodb/ekodb_client) Finished `dev`
profile [unoptimized + debuginfo] target(s) in 9.14s ✅ Client build complete!
🧪 Running Rust client library examples... Compiling ekodb-examples v0.1.0
(/Users/tek/Development/teknology/ekodb/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.72s Running
`target/debug/examples/client_simple_crud` ✓ Client created (token exchange
happens automatically)

=== Insert Document === Inserted: Record { fields: {"id":
String("48e_5yks-eQ5ms10X_VYUxbVSc4qffbqlyUuhBgzz31FoP1wfasaP4kar4XFFskaD3E9AlhpVjyFrYX3ENQsfg")}
}

=== Find by ID === Found: Record { fields: {"name": Object({"type":
String("String"), "value": String("Test Record")}), "active": Object({"type":
String("Boolean"), "value": Boolean(true)}), "value": Object({"type":
String("Integer"), "value": Integer(42)}), "id":
String("48e_5yks-eQ5ms10X_VYUxbVSc4qffbqlyUuhBgzz31FoP1wfasaP4kar4XFFskaD3E9AlhpVjyFrYX3ENQsfg")}
}

=== Find with Query === Found documents: [Record { fields: {"active":
Object({"value": Boolean(true), "type": String("Boolean")}), "value":
Object({"type": String("Integer"), "value": Integer(42)}), "name":
Object({"type": String("String"), "value": String("Test Record")}), "id":
String("48e_5yks-eQ5ms10X_VYUxbVSc4qffbqlyUuhBgzz31FoP1wfasaP4kar4XFFskaD3E9AlhpVjyFrYX3ENQsfg")}
}]

=== Update Document === Updated: Record { fields: {"value": Integer(100), "id":
String("48e_5yks-eQ5ms10X_VYUxbVSc4qffbqlyUuhBgzz31FoP1wfasaP4kar4XFFskaD3E9AlhpVjyFrYX3ENQsfg"),
"name": String("Updated Record"), "active": Object({"value": Boolean(true),
"type": String("Boolean")})} }

=== Delete Document === Deleted document

=== Cleanup === ✓ Deleted collection

✓ All CRUD operations completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/teknology/ekodb/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.78s Running
`target/debug/examples/client_batch_operations` ✓ Client created

=== Batch Insert (via multiple inserts) === ✓ Inserted 5 records ✓ Verified:
Found 5 total records in collection

=== Update Records === ✓ Updated 3 records

=== Delete Records === ✓ Deleted 3 records

=== Cleanup === ✓ Deleted collection

✓ All batch operations completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/teknology/ekodb/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.64s Running
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
(/Users/tek/Development/teknology/ekodb/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.74s Running
`target/debug/examples/client_collection_management` ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
"mKgpkIHKKQMss_Vik1YIgOvlVF2zALipqN5Qj5sDg8LTvg6TuwH2bPU1qUKxrgqhS5U5dXSsXvL7vunmOIhtYw"

=== List Collections === Total collections: 23 Sample collections:
["non_indexed_vector_perf", "indexed_query_scale_10k",
"client_collection_management_rust", "ttl_cache", "indexed_search_perf"]

=== Count Documents === Document count: 1

=== Check Collection Exists === Collection exists: true

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/teknology/ekodb/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.69s Running
`target/debug/examples/client_document_ttl` ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
"lIXsb7bVsw2EoCO9sGEC8pZ1SW9cjTaFcUTWDrzkkepA6RTJQKl_sb7_d22N2ogBFfkXWrZZbt-8n3_M9zE4hA"

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
Some(String("R9yvX1yjLynuLVOlK1QkfQgbaU27KmVec52Piyx5RURMTUIl34-0UYnkGPN632tlFhoImZPtxf7Zc-KYmQbRiw"))

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration Compiling ekodb-examples v0.1.0
(/Users/tek/Development/teknology/ekodb/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.95s Running
`target/debug/examples/client_simple_websocket` ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
TwhfsyPmfJPCt_7Mom7fZDNWMMJintiZJm5KkvBsu7U0M9IrAOmX4I0X3rJy2f7--hg_ZDjNPoO5rnf_HT9rWg

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/teknology/ekodb/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.98s Running
`target/debug/examples/client_websocket_ttl` ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
Some(String("8DDSrcKpy9lBXt0bGpyTHduzW5Jz65YNsYxQxBsY_LAtLQP1pAMTKYB5ul9lOmv97N1tm3YqPMiFyVQXGm_Jvg"))

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration Compiling ekodb-examples v0.1.0
(/Users/tek/Development/teknology/ekodb/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.74s Running
`target/debug/examples/client_query_builder` === ekoDB Query Builder Example ===

=== Inserting Sample Data === ✓ Inserted 5 users

=== Simple Equality Query === ✓ Found 3 active users

- Some(Object({"type": String("String"), "value": String("David")}))
- Some(Object({"value": String("Bob"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("Alice")}))

=== Range Query (age >= 28 AND age < 35) === ✓ Found 3 users in age range

- Some(Object({"type": String("String"), "value": String("Eve")}))
- Some(Object({"type": String("String"), "value": String("David")}))
- Some(Object({"type": String("String"), "value": String("Bob")}))

=== IN Operator === ✓ Found 4 users with status active or pending

- Some(Object({"type": String("String"), "value": String("Bob")}))
- Some(Object({"type": String("String"), "value": String("Eve")}))
- Some(Object({"value": String("Alice"), "type": String("String")}))
- Some(Object({"value": String("David"), "type": String("String")}))

=== NOT IN Operator === ✓ Found 4 users not inactive

=== String Pattern Matching === ✓ Found 5 users with @example.com email

=== Regex Query === ✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) === ✓ Found 2 users
matching all conditions

- Some(Object({"type": String("String"), "value": String("David")}))
- Some(Object({"type": String("String"), "value": String("Bob")}))

=== OR Query === ✓ Found 2 users with age < 28 OR age > 32

- Some(Object({"type": String("String"), "value": String("Charlie")}))
- Some(Object({"type": String("String"), "value": String("Alice")}))

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
v0.1.0 (/Users/tek/Development/teknology/ekodb/examples/rust) Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.73s Running
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
   ["description.value", "title", "title.value", "description"]
2. Score: 2.0000 Title: Some(String("Python for Data Science")) Matched:
   ["description.value", "description"]
3. Score: 1.0000 Title: Some(String("Database Design")) Matched: ["description",
   "description.value"]
4. Score: 1.0000 Title: Some(String("Rust Programming")) Matched:
   ["description", "description.value"]

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
ekodb-examples v0.1.0 (/Users/tek/Development/teknology/ekodb/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.75s Running
`target/debug/examples/client_schema_management` === ekoDB Schema Management
Example ===

=== Creating Collection with Schema === ✓ Created collection
'schema_client_rust' with schema

=== Inserting Valid Documents === ✓ Inserted user 1:
Some(String("-48eFkmer6WIuudYemyXNcN1b0YldPLWlQohUItCHu1zQtiNnYzSG3Hr1UkzcQcN8pE8jM8VvNcUmYnvvecggQ"))
✓ Inserted user 2:
Some(String("eSZe3r8_D71o6Azp96BSsjzXipIXkdDXejffpqXHiW6tTwOD0LXWHFr1V5ybcFBXhaO1GRzqIIiHx42-NiiG2A"))

=== Getting Schema === ✓ Schema for schema_client_rust:

- age: Integer
- email: String (required)
- title: String (required)
- status: String

=== Listing Collections === ✓ Total collections: 23 Sample:
["schema_client_rust", "non_indexed_vector_perf", "indexed_query_scale_10k",
"ttl_cache", "indexed_search_perf"]

=== Cleanup === ✓ Deleted collection

✓ All schema management operations completed successfully Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/teknology/ekodb/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.71s Running
`target/debug/examples/client_chat_basic` === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
a32aml0lCgvARh0XI0CLgW-ZUZ6ggtQ7sXLIVxT7uvsPgJ-YhYDnHPvJcA5CUYOYSEqmXZO4OBmKpp_HUm--aQ

=== Sending Chat Message === Message ID:
OSkNd7zl63TG8-mxfiDA1HJGlE2youD9RN9sdFihYiMqvdeH9klkyYkvbfuZK9SOfnHHSFql0XG2ZTFnpjxM-Q

=== AI Response === Response 1: ekoDB is a high-performance database known for
its intelligent caching and real-time capabilities. It is designed with AI
integration to provide more efficient and intelligent database handling. Some of
its significant features include an AI Chat Integration, which lets you query
your database using natural language and get AI-powered responses with relevant
context. Additionally, ekoDB also supports different types of search features,
such as full-text search, vector search, and hybrid search with automatic
context retrieval. These features make ekoDB a robust and smart tool for
managing and querying your database.

=== Context Used (3 snippets) ===

Snippet 1: Collection: client_chat_basic_rust Score: 0.6333 Matched Fields:
["content", "title", "category"] Record: Object {"category": Object {"type":
String("String"), "value": String("features")}, "content": Object {"type":
String("String"), "value": String("The chat feature allows you to query your
database using natural language and get AI-powered responses with relevant
context.")}, "id":
String("RcLT3Ls6KaFpHC2q_vMY0Lr2pwfoxH_Y_iugfixl3yQxyCaGh1K7lLkPp-cJfMOXKg_Xca3JkxSY4NbNo_R01Q"),
"title": Object {"type": String("String"), "value": String("AI Chat
Integration")}}

Snippet 2: Collection: client_chat_basic_rust Score: 0.5222 Matched Fields:
["content", "title"] Record: Object {"category": Object {"type":
String("String"), "value": String("documentation")}, "content": Object {"type":
String("String"), "value": String("ekoDB is a high-performance database with
intelligent caching, real-time capabilities, and AI integration.")}, "id":
String("20ixhBaH3Cijk1A44SXG2FyyTPVCEs8Y1LQU2vAkcg3XAvGGXbBoo1uJ8kr3tFW6ocyaNTCCzww6T_AivBLPqw"),
"title": Object {"type": String("String"), "value": String("Introduction to
ekoDB")}}

Snippet 3: Collection: client_chat_basic_rust Score: 0.5222 Matched Fields:
["category", "title", "content"] Record: Object {"category": Object {"type":
String("String"), "value": String("features")}, "content": Object {"type":
String("String"), "value": String("ekoDB supports full-text search, vector
search, and hybrid search with automatic context retrieval.")}, "id":
String("yx-MOMBBhSh8LuqTH5Q3EkfgO1knJPbsGT-pZwd6MK1hRw245OolU3T11OR-iPjBsT7gOnyImgbinfMbmf_xWA"),
"title": Object {"type": String("String"), "value": String("Search Features")}}

Execution Time: 4761ms

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/teknology/ekodb/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.80s Running
`target/debug/examples/client_chat_sessions` === ekoDB Chat Session Management
Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
6DIRowivSBaPHC1eY4V5_FZpVKu1Z9560LGJEZrdVZD_TehSreJ4lzqW6CrpEOpCWTfXt_4aWS9IYnXfdI8MKQ
=== Sending Messages === ✓ Message 1 sent Response: Based on the information
provided, the available product is ekoDB, a high-performance database product
with AI capabilities. The price of this product is $99.

✓ Message 2 sent Response: As an assistant, I'm sorry but I can't provide the
information you're looking for because the context doesn't specify which
product's price you're asking about. Could you please provide more details?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
zD8QK-\_zbcPyNND0Fd0Y91HVJ3hwgOec_dG5KeWi_oVcXZN9noSyWL7SiussdSAKLRhrCrpp-wBrfOGhRVGyvQ
Parent:
6DIRowivSBaPHC1eY4V5_FZpVKu1Z9560LGJEZrdVZD_TehSreJ4lzqW6CrpEOpCWTfXt_4aWS9IYnXfdI8MKQ

=== Listing Sessions === ✓ Found 3 sessions Session 1:
zD8QK-\_zbcPyNND0Fd0Y91HVJ3hwgOec_dG5KeWi_oVcXZN9noSyWL7SiussdSAKLRhrCrpp-wBrfOGhRVGyvQ
(Untitled) Session 2:
6DIRowivSBaPHC1eY4V5_FZpVKu1Z9560LGJEZrdVZD_TehSreJ4lzqW6CrpEOpCWTfXt_4aWS9IYnXfdI8MKQ
(Untitled) Session 3:
a32aml0lCgvARh0XI0CLgW-ZUZ6ggtQ7sXLIVxT7uvsPgJ-YhYDnHPvJcA5CUYOYSEqmXZO4OBmKpp_HUm--aQ
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
zD8QK-\_zbcPyNND0Fd0Y91HVJ3hwgOec_dG5KeWi_oVcXZN9noSyWL7SiussdSAKLRhrCrpp-wBrfOGhRVGyvQ

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/teknology/ekodb/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.84s Running
`target/debug/examples/client_chat_advanced` === ekoDB Advanced Chat Features
Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
43igFLahuEry-3nG8kd9ayEfm7wL39AdDxKLdshd9oBZkHuUWF6E1CpNbfYc0S3k8E076NARv1KEUhj7KJtXXg

=== Sending Initial Message === ✓ Message sent Response: Based on the given
context, the available product is ekoDB. It is a high-performance database
product priced at $99.

✓ Second message sent === Feature 1: Regenerate AI Response === ✓ Message
regenerated New response: I'm sorry, but the context provided doesn't include
information on the price of ekoDB. I would recommend checking the official
website or contacting the sellers for the most accurate and up-to-date price.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
F7uUMqqxQ7XuajY-SPdEbMM05Tm5BzTgRRhae3dNYMbQQIwC4IqV9G2UMBKg3ner4HljFhlmF9FznoNk1dCQoA
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
'FKtXZ6a0ik3dOG1HaS4gCa6F9OE9nGb7xvYi-XX1eSj4S9s9bNqDfJrO8TqAq2A40UbuoLlkShmFOaM383UJ2w'}

=== Find by ID === Found: {'value': {'type': 'Integer', 'value': 42}, 'id':
'FKtXZ6a0ik3dOG1HaS4gCa6F9OE9nGb7xvYi-XX1eSj4S9s9bNqDfJrO8TqAq2A40UbuoLlkShmFOaM383UJ2w',
'name': {'type': 'String', 'value': 'Test Record'}, 'active': {'value': True,
'type': 'Boolean'}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'name': 'Updated Record', 'id':
'FKtXZ6a0ik3dOG1HaS4gCa6F9OE9nGb7xvYi-XX1eSj4S9s9bNqDfJrO8TqAq2A40UbuoLlkShmFOaM383UJ2w',
'active': {'value': True, 'type': 'Boolean'}, 'value': 100}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully ✓ simple_crud.py completed
successfully

=== Running simple_websocket.py === ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
JF7LOsatLKIGAi-huzbhWK46yMKOEsxkCp6xNfJBR0-hioVIZtx7tBeywjzRbu-yKqBKxo6p8yXJMDWHMtj1lg

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"JF7LOsatLKIGAi-huzbhWK46yMKOEsxkCp6xNfJBR0-hioVIZtx7tBeywjzRbu-yKqBKxo6p8yXJMDWHMtj1lg",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"Ud9HWP3Q5qdR_BH_mcPhrPS0ThE9Esc6Pf2Egw7uZzJFsyHyEijHPBaXdseil8tIctclNV6nR6sDcEgosoUNVQ",
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
cache:product:3: {'price': 49.989999999999995, 'name': 'Product 3'}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ kv_operations.py completed
successfully

=== Running collection_management.py === ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
VScWrqCneIRLgmjsb52lLMJbcPuu7TetYsRWl22EYsXkQtOgje5gJHYe4q2xbLZAGisgDogzUsDwYbMwrbbhaA

=== List Collections === Total collections: 25 Sample collections:
['non_indexed_vector_perf', 'indexed_query_scale_10k',
'chat_messages__ek0_bench', 'ttl_cache', 'indexed_search_perf']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓
collection_management.py completed successfully

=== Running document_ttl.py === ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
\_LOlmQb7UZy5sRLC0-OM31-Rpao5fOe3ZDLhb3OL3YDnhQilf2qp7N57wqvqSlxDqSHq76OCsi-C5DkaUt0Q8A

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully ✓ document_ttl.py completed
successfully

=== Running websocket_ttl.py === ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
YEdTQiJUyF0gfmcCpn7hRxNFoNrzP-okWeo6JwLAYBOvmHD-coiDsWx1qW_oY7_wllIbEhboBC5WkUPXUTxItQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 2 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully ✓ websocket_ttl.py completed
successfully

╔════════════════════════════════════════╗ ║ Test Summary ║
╚════════════════════════════════════════╝ Total: 7 Passed: 7 Failed: 0 ✅
Python direct examples complete! 🐍 Building and installing Python client
package... 🍹 Building a mixed python/rust project 🔗 Found pyo3 bindings with
abi3 support 🐍 Not using a specific python interpreter 💻 Using
`MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default Compiling
ekodb_client v0.1.0 (/Users/tek/Development/teknology/ekodb/ekodb_client)
Compiling ekodb-client-py v0.1.0
(/Users/tek/Development/teknology/ekodb/ekodb-client-py) Finished `release`
profile [optimized] target(s) in 8.60s 📦 Built wheel for abi3 Python ≥ 3.8 to
/Users/tek/Development/teknology/ekodb/ekodb-client-py/target/wheels/ekodb_client-0.1.0-cp38-abi3-macosx_11_0_arm64.whl
Processing
./ekodb-client-py/target/wheels/ekodb_client-0.1.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client Attempting uninstall: ekodb-client
Found existing installation: ekodb_client 0.1.0 Uninstalling ekodb_client-0.1.0:
Successfully uninstalled ekodb_client-0.1.0 Successfully installed
ekodb-client-0.1.0 ✅ Python client package built and installed! 🧪 Running
Python client library examples... ✓ Client created (token exchange happens
automatically)

=== Insert Document === Inserted: {'id':
'HuosszaIUl1RoxPqLj2pFlPl6QdG24htxNyP5mmvB4x6-q7QivGUfp4pfh6o7xDLyAoOTUAxtwTasuU4fCslfQ'}

=== Find by ID === Found: {'name': {'value': 'Test Record', 'type': 'String'},
'id':
'HuosszaIUl1RoxPqLj2pFlPl6QdG24htxNyP5mmvB4x6-q7QivGUfp4pfh6o7xDLyAoOTUAxtwTasuU4fCslfQ',
'active': {'value': True, 'type': 'Boolean'}, 'value': {'value': 42, 'type':
'Integer'}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'active': {'type': 'Boolean', 'value': True},
'name': 'Updated Record', 'value': 100, 'id':
'HuosszaIUl1RoxPqLj2pFlPl6QdG24htxNyP5mmvB4x6-q7QivGUfp4pfh6o7xDLyAoOTUAxtwTasuU4fCslfQ'}

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
"lrnKLDJMT_ji0PBA9Wdzw6OyqralzsAnBHgi0AN7v2fy65ZxsPk-Ls2cgrvL15QDFcu2VNYwSjY4hIed46JM7w"

=== List Collections === Total collections: 24 Sample collections:
['non_indexed_vector_perf', 'indexed_query_scale_10k',
'chat_messages__ek0_bench', 'ttl_cache', 'indexed_search_perf']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
At7huyBrgPPeo4uPnggQNjHeoV8G_5GCCf0jSZrOTwWmQuonJqzmJGqEb7TpVyRV_pZ-dVZUUKVXq7kc2aku-A

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
qh6tMHpUXWgLNcc7W18g683y3aQQQz2uPdD_vVKKtyrOmX86_vXfZKj8BzlrWAK3AfkNifTlGtdTlBaCJhRqaw

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
7lXlzBUJazy5An8OLNRv4XRlvoKqXLku6j5DVmRLcWoLCcJirUXsLyeD92NwGe_RBC5A-sxIuq8GTNuI4pXX6A

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
0GaHpJJsKttqgiIyTzpB3dxpyzVmAOtDSh9XajvVFOdAT6GLeCnia0S92qMQigxfbu0TYC0BSgb9ya4-Q73Txg

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === Query Builder Examples ===

Setting up test data... ✅ Test data created

1. Simple equality query: Found 3 active users

2. Range query with sorting: Found 3 users aged 18-65

3. String operations: Found 3 users with @example.com emails

4. IN operator: Found 3 privileged users

5. Complex query with multiple conditions: Found 3 active US users over 21

6. Pagination: Page 1: 3 users

7. NOT IN operator: Found 3 valid users

8. Using bypass flags: Found 3 users (bypassed cache)

=== Cleanup === ✅ Deleted test collection

✅ Query Builder examples completed! === Search Examples ===

Setting up test data... ✅ Test data created

1. Basic full-text search: Found 2 results
1. Score: 25.740, Matched: name, email, name.value, email.value
1. Score: 12.540, Matched: name, name.value

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: title, bio.value, title.value, bio
1. Score: 26.400, Matched: title.value, bio.value, bio, title
1. Score: 26.400, Matched: bio, title, bio.value, title.value
1. Score: 26.400, Matched: title, bio, bio.value, title.value

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: bio, bio.value, title.value, title
1. Score: 39.600, Matched: bio.value, title, bio, title.value
1. Score: 39.600, Matched: title, bio, bio.value, title.value
1. Score: 39.600, Matched: title.value, bio, bio.value, title

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio, bio.value
1. Score: 13.200, Matched: bio.value, bio

1. Search with stemming and exact match boosting: Found 2 results (matches: run,
   running, runs)
1. Score: 13.200, Matched: bio, bio.value
1. Score: 13.200, Matched: bio, bio.value

1. Vector search (semantic search): Found 0 semantically similar documents

1. Hybrid search (text + vector): Found 2 results using hybrid search (text +
   vector)
1. Score: 1.200, Matched: title.value, title, content, content.value
1. Score: 0.600, Matched: title, content.value, content, title.value

1. Case-sensitive search: Found 1 results (case-sensitive)
1. Score: 13.200, Matched: title.value, title

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

- Laptop ($1200) by Unknown
- Mouse ($25) by Unknown
- Keyboard ($75) by Unknown

5. Complex join with multiple conditions: Found 2 users with example.com emails:

- Alice Johnson (alice@example.com): N/A
- Bob Smith (bob@example.com): N/A

=== Cleanup === ✅ Deleted test collections

✅ Join operations examples completed! === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
HWiXSRaxFxP3-U1fL-oXnKbly_MYe-bmXrz3xeJI3TUyO_pWGfgNUAfgPw3YiwV-y306B37GQ3EkqNZCQVEV4g

=== Sending Chat Message === Message ID:
RkfTvAaouTlicRcWbGGLAo5l5Zc9u2Rx0YvzE56aI9k06nGK9KsxIgd7oGcL7NSGh6UtWwiNeK8WSDbdrzS2ew

=== AI Response === The available products and their prices are as follows:

1. ekoDB Pro: It is an Enterprise edition product with advanced features. The
   price is $299.

2. ekoDB Cloud: This is a fully managed cloud database service product. The
   price is $499.

3. ekoDB: This is a high-performance database product with AI capabilities. The
   price is $99.

Execution Time: 3683ms

=== Token Usage === Prompt tokens: 371 Completion tokens: 79 Total tokens: 450

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
kzxqWv6UxXJVmepsrBHNYB_Xo_EAgY3tojfh-nQbMziX1NfVHAKSw5yeSRq2R2T_u_ee4FRm-AqjI8rp-vVNuQ

=== Sending Messages === ✓ Message 1 sent Response: Based on the provided
context, the available product is ekoDB. It is a high-performance database
product priced at $99.

✓ Message 2 sent Response: As an assistant, I'm sorry but I can't provide the
price you're asking for because the context doesn't mention any specific
product. Could you please provide more details or specify the product?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
JqXd_c4mB6aYl1PQO1Z_opRe6kCkiCca1iulesbnVNm0Vvev1d7wwA69YNhms2WGAHTHU9VXe0B8XbGJKmYjyQ
Parent:
kzxqWv6UxXJVmepsrBHNYB_Xo_EAgY3tojfh-nQbMziX1NfVHAKSw5yeSRq2R2T_u_ee4FRm-AqjI8rp-vVNuQ

=== Listing Sessions === ✓ Found 6 sessions Session 1:
JqXd_c4mB6aYl1PQO1Z_opRe6kCkiCca1iulesbnVNm0Vvev1d7wwA69YNhms2WGAHTHU9VXe0B8XbGJKmYjyQ
(Untitled) Session 2:
kzxqWv6UxXJVmepsrBHNYB_Xo_EAgY3tojfh-nQbMziX1NfVHAKSw5yeSRq2R2T_u_ee4FRm-AqjI8rp-vVNuQ
(Untitled) Session 3:
HWiXSRaxFxP3-U1fL-oXnKbly_MYe-bmXrz3xeJI3TUyO_pWGfgNUAfgPw3YiwV-y306B37GQ3EkqNZCQVEV4g
(Untitled) Session 4:
F7uUMqqxQ7XuajY-SPdEbMM05Tm5BzTgRRhae3dNYMbQQIwC4IqV9G2UMBKg3ner4HljFhlmF9FznoNk1dCQoA
(Untitled) Session 5:
6DIRowivSBaPHC1eY4V5_FZpVKu1Z9560LGJEZrdVZD_TehSreJ4lzqW6CrpEOpCWTfXt_4aWS9IYnXfdI8MKQ
(Untitled) Session 6:
a32aml0lCgvARh0XI0CLgW-ZUZ6ggtQ7sXLIVxT7uvsPgJ-YhYDnHPvJcA5CUYOYSEqmXZO4OBmKpp_HUm--aQ
(Untitled)

=== Deleting Branch Session === ✓ Deleted branch session:
JqXd_c4mB6aYl1PQO1Z_opRe6kCkiCca1iulesbnVNm0Vvev1d7wwA69YNhms2WGAHTHU9VXe0B8XbGJKmYjyQ

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
8f2v9KdtTeTXj3yBWSKTcBRJbQBgkV97Crx2FTLrOYfG2J5fIg_MNwws1Iov_YFeNo_HkV2N8FBtiAo1HBoZ1w

=== Sending Initial Message === ✓ Message sent Response: Based on the provided
context, the available product is ekoDB. It is a high-performance database
product and is priced at $99.

✓ Second message sent

Debug: Found 4 messages Debug: First message keys: dict_keys(['updated_at',
'token_usage', 'chat_id', 'created_at', 'content', 'id', 'role']) Debug: First
message role: {'type': 'String', 'value': 'assistant'} === Feature 1: Regenerate
AI Response === ✓ Message regenerated New response: I'm sorry, but the context
provided does not include information on the price of ekoDB. I recommend
checking the official website or contacting the sales department of ekoDB for
the most accurate and up-to-date pricing information.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
wU8go7PH1c807KAV8RgT27mlDpS3sg0MeBNPCSiqo8X-wMf7KARghD83v5EjT1Wb3yuvRmGqnf8chxiY3JjbcA
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
map[id:Y129ca5qEJx_8XRfqM_nUopTH6JPxHROWDva8yFawZEP3cPRgsLBg3KplEQZayVa8CYyRM1g6U-ofSQJdD9s0A]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:Y129ca5qEJx_8XRfqM_nUopTH6JPxHROWDva8yFawZEP3cPRgsLBg3KplEQZayVa8CYyRM1g6U-ofSQJdD9s0A
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found 1 documents

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:Y129ca5qEJx_8XRfqM_nUopTH6JPxHROWDva8yFawZEP3cPRgsLBg3KplEQZayVa8CYyRM1g6U-ofSQJdD9s0A
name:Updated Record value:100]

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully ✓ simple_crud.go completed
successfully

=== Running simple_websocket.go === ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
WdkxVkmEg6eaKu5MrjsYteG3GbKtZ2i4U8nHb6HSE31YLVFCBwgkuySXCtK9u-p-SSlsLUbHxpmzDx_iJVr4Cg

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"WdkxVkmEg6eaKu5MrjsYteG3GbKtZ2i4U8nHb6HSE31YLVFCBwgkuySXCtK9u-p-SSlsLUbHxpmzDx_iJVr4Cg",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"JF7LOsatLKIGAi-huzbhWK46yMKOEsxkCp6xNfJBR0-hioVIZtx7tBeywjzRbu-yKqBKxo6p8yXJMDWHMtj1lg",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"Ud9HWP3Q5qdR_BH_mcPhrPS0ThE9Esc6Pf2Egw7uZzJFsyHyEijHPBaXdseil8tIctclNV6nR6sDcEgosoUNVQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 3
record(s) via WebSocket

✓ WebSocket example completed successfully ✓ simple_websocket.go completed
successfully

=== Running batch_operations.go === ✓ Authentication successful

=== Batch Insert === Batch insert response: map[failed:[]
successful:[GhQ-zgN2F3z4QqFCV1RqibyYC8_3AaMDM3YoKcm35z6VQ0MscdgIn0qpuFAbV6zQ-Ry4sxJEpzGhuPAn1TedxQ
xZf_6YplHulPeIsCQcI0DDoLD239xKprE-UN7UiKVGrbQ-1JyD5tWRZAgBBY1vkP74iHn0-_AVs3kygmH-HpOw
qW1qdJYZssdYrgt33W_DU8bp6tuOw1IJEm8QYZ1LujYzpcl5dh2280NjK78arakAsQvpv_DL2SKf2Hnb6CQ2AQ
fGKz0NKuZxUIWABGcUA-3lCLossrlBOsLmvOAYic8djt1hbPiMyQx-houGwwE59zsw-McnPUdy3zw7nzvf8fKw
Yk5yBLzaRid9I8tDWs7fhq9bafiwfE4bG43RAizTCWO5pbLLwSrYAOebcSv3zcnfydNjnXYPctPjDEsgoU0ptw]]
✓ Batch insert completed

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === Batch update response: map[failed:[]
successful:[6tDCBx0ookCzpOLpZY0HtDz6Hy1U5TQ4xqoI8Qg8RqzkXDCwLoVZbuyIaqBjcTV11dhqGG_uaN4nxHBzoeaCVw
mkI2ksDWFFyGiTIRx4-HjhZ9OD2eebiU8YpphKHcq9H6J21C4l2nyMxEJcxasbvxporUPCteahr6YuT4bk29bw
7ku-bwmxaVmJMcs3bELzX0vuQyS3ZqqGRkYEIOCUsXjN_pHv5K9NmYiNgsvV2Iz3kQmTZ_-88nsn5JgYNpP4Yg]]
✓ Batch update completed

=== Batch Delete === Batch delete response: map[failed:[]
successful:[6tDCBx0ookCzpOLpZY0HtDz6Hy1U5TQ4xqoI8Qg8RqzkXDCwLoVZbuyIaqBjcTV11dhqGG_uaN4nxHBzoeaCVw
mkI2ksDWFFyGiTIRx4-HjhZ9OD2eebiU8YpphKHcq9H6J21C4l2nyMxEJcxasbvxporUPCteahr6YuT4bk29bw
7ku-bwmxaVmJMcs3bELzX0vuQyS3ZqqGRkYEIOCUsXjN_pHv5K9NmYiNgsvV2Iz3kQmTZ_-88nsn5JgYNpP4Yg]]
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
RMJmLOt8gLpDzcBBV4JPP2_0mj9HHjpvLZZBswdBCQY6z9qc_9sCCs-K55Yd9b0ZK-DEgxjKIaWAvXrCSHVD3A

=== List Collections === Total collections: 28 Sample collections:
[non_indexed_vector_perf indexed_query_scale_10k schema_products_client_py
chat_messages__ek0_bench ttl_cache indexed_search_perf ws_ttl_test
text_search_articles indexed_vector_perf benchmark_test_collection
demo_collection non_indexed_query_scale_10k non_indexed_hybrid_scale
indexed_hybrid_scale schema_employees_client_py schema_documents_client_py
usertable non_indexed_scale_1k schema_users_client_py indexed_query_perf
non_indexed_search_perf indexed_scale_1k vector_search_articles batch_users
chat_configurations__ek0_bench websocket_test test_collection
non_indexed_query_perf]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓
collection_management.go completed successfully

=== Running document_ttl.go === ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
6MN3rMD7yob_i0eJICQkt-2w8VSblj83C48Mn7ADWX7BBnUJHXcXWhFmFv-bQM4IAC0MCaNJIUFvzi2Uj_nwOg

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 3 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully ✓ document_ttl.go completed
successfully

=== Running websocket_ttl.go === ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
Boy7uvMRO0HQaHOkZ_wTC3pOtOeLe8wMKPqeZoQY_2XIThGYNyOll20tROD5lGAbsnfq95oCukV3dW7l1_V--w

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 3 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully ✓ websocket_ttl.go completed
successfully

╔════════════════════════════════════════╗ ║ Test Summary ║
╚════════════════════════════════════════╝ Total: 7 Passed: 7 Failed: 0 ✅ Go
direct examples complete! 🧪 Running Go client library examples... ✓ Client
created (token exchange happens automatically)

=== Insert Document === Inserted:
map[id:F7JoGfe5Ezx6nmN_Dx9PjD8qbo8b9mthHSfBbTFt_PTh1h686TK0f_hQErnxtztFtvKPyHHWznJ12s6PayEpqw]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:F7JoGfe5Ezx6nmN_Dx9PjD8qbo8b9mthHSfBbTFt_PTh1h686TK0f_hQErnxtztFtvKPyHHWznJ12s6PayEpqw
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found documents: 1

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:F7JoGfe5Ezx6nmN_Dx9PjD8qbo8b9mthHSfBbTFt_PTh1h686TK0f_hQErnxtztFtvKPyHHWznJ12s6PayEpqw
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
wwatDnKsJLUtyoaBtVRuc8hNLQHrD6LyAPMPF1NKhjIDzleMu0TVbYfSqiAx5MLFh7a0sKfSB5ZROdA-4SLBBQ

=== List Collections === Total collections: 28 Sample collections:
[non_indexed_vector_perf indexed_query_scale_10k schema_products_client_py
chat_messages__ek0_bench ttl_cache]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
D-7f8tOucG_tnCDjnhR8XQKbZtvjx4OIV-E39c7BQGSd3vCmbyOtoUe2LvarLz4KHrlDLzG5jdrFEMw3sbrLhg

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
zw-o14WbH_PWK1MpQ9GLfn_b8iCUbnCyu7HPRnul1cjsIhtGCLwyhAd8NccbYiJTNNnKtgRSbP5nBv9uFwVVCA

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
YfyYx5ON78CHEAjr0tDrlqASQySZ1cfWB_jZgraUJw656SAyA0bjbxPsDr16FaMiKY7AX9djePEnKushSA3kRw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
H3XTh5T4L9er6dcpZVJnnUAlTCHV1vNj4c0M3_6Jq8RYENQ2qcoXWhYbZZW6HVu0iEowYoFuWWwH9LI127z2QQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
0KEJp7uqQ7uGrfFtahSjEoYzqC9OnuGGMxaNxKkKM8T00cRCtAK-jzlMQa3QNN1MUXWZBlANCcmcCdm4Se8mcw

=== Sending Chat Message === Message ID:
exaXUqC_j-hHn3vgWa-zK2st3C8JkOc3eHx2VzjBB7jdG0B7tg31qEOf2wx-3S0FI96WgrhGeAfsDypeseRbjA

=== AI Response === The available products are:

1. ekoDB Cloud: This is a fully managed cloud database service product. It is
   priced at $499.

2. ekoDB: This is a high-performance database product with AI capabilities. It
   is available at a price of $99.

3. ekoDB Pro: This is an enterprise edition product with advanced features. The
   cost of this product is $299.

=== Context Used (3 snippets) === Snippet 1: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:Fully
managed cloud database service product]
id:zKDbtvnH2KQvWMjQRhgnPsECJfWIzSwxNY4zEEEKASnaZmiOy2OJJyPFZ7KtperHIkRFukLZ0UB_1Tm3C2LTUw
name:map[type:String value:ekoDB Cloud] price:map[type:Integer value:499]]
score:0.1111111111111111] Snippet 2: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:A
high-performance database product with AI capabilities]
id:2yonUJwxwjSEXUQGD44pQslHPDLJ3ka3C8L66iv7gEqnBs_Y_tys_mVomjcQs8X1qd92gh0lKicA61N2iOYNrA
name:map[type:String value:ekoDB] price:map[type:Integer value:99]]
score:0.1111111111111111] Snippet 3: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String
value:Enterprise edition product with advanced features]
id:RxNoynpDnBOqeRnKZL0YiIt9ar_e1JyAMlPLB_VCoSL5N5zvvM_uDTxY84TdJYRCKZ40jMyMwSye08s4ZV_A8g
name:map[type:String value:ekoDB Pro] price:map[type:Integer value:299]]
score:0.1111111111111111]

Execution Time: 3156ms

=== Token Usage === Prompt tokens: 377 Completion tokens: 82 Total tokens: 459

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
gxEBI3CNNntf2xaehuZyTplT-Fzkvi8gRM_KHiyf4E5vBdbPCXVeKoJCd5I9JZ0GPO2Fe-4uPbeTO1Ojuh39tw

=== Sending Messages === ✓ Message 1 sent Response: Based on the context
provided, the available product is ekoDB. It is a high-performance database
product, priced at $99.

✓ Message 2 sent Response: Apologies, but there's no context provided about the
product you're inquiring about. Could you please specify the product whose price
you'd like to know?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
6GG0ovCEdepxLNfF4Fw1VVhsoO-oxMwQCLPKHfDKosGdHCqayiZPinlR_I9fumTGpy5aFZDq7XCsa5kSvXKUyA
Parent:
gxEBI3CNNntf2xaehuZyTplT-Fzkvi8gRM_KHiyf4E5vBdbPCXVeKoJCd5I9JZ0GPO2Fe-4uPbeTO1Ojuh39tw

=== Listing Sessions === ✓ Found 9 sessions Session 1:
6GG0ovCEdepxLNfF4Fw1VVhsoO-oxMwQCLPKHfDKosGdHCqayiZPinlR_I9fumTGpy5aFZDq7XCsa5kSvXKUyA
(Untitled) Session 2:
gxEBI3CNNntf2xaehuZyTplT-Fzkvi8gRM_KHiyf4E5vBdbPCXVeKoJCd5I9JZ0GPO2Fe-4uPbeTO1Ojuh39tw
(Untitled) Session 3:
0KEJp7uqQ7uGrfFtahSjEoYzqC9OnuGGMxaNxKkKM8T00cRCtAK-jzlMQa3QNN1MUXWZBlANCcmcCdm4Se8mcw
(Untitled) Session 4:
wU8go7PH1c807KAV8RgT27mlDpS3sg0MeBNPCSiqo8X-wMf7KARghD83v5EjT1Wb3yuvRmGqnf8chxiY3JjbcA
(Untitled) Session 5:
kzxqWv6UxXJVmepsrBHNYB_Xo_EAgY3tojfh-nQbMziX1NfVHAKSw5yeSRq2R2T_u_ee4FRm-AqjI8rp-vVNuQ
(Untitled) Session 6:
HWiXSRaxFxP3-U1fL-oXnKbly_MYe-bmXrz3xeJI3TUyO_pWGfgNUAfgPw3YiwV-y306B37GQ3EkqNZCQVEV4g
(Untitled) Session 7:
F7uUMqqxQ7XuajY-SPdEbMM05Tm5BzTgRRhae3dNYMbQQIwC4IqV9G2UMBKg3ner4HljFhlmF9FznoNk1dCQoA
(Untitled) Session 8:
6DIRowivSBaPHC1eY4V5_FZpVKu1Z9560LGJEZrdVZD_TehSreJ4lzqW6CrpEOpCWTfXt_4aWS9IYnXfdI8MKQ
(Untitled) Session 9:
a32aml0lCgvARh0XI0CLgW-ZUZ6ggtQ7sXLIVxT7uvsPgJ-YhYDnHPvJcA5CUYOYSEqmXZO4OBmKpp_HUm--aQ
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
6GG0ovCEdepxLNfF4Fw1VVhsoO-oxMwQCLPKHfDKosGdHCqayiZPinlR_I9fumTGpy5aFZDq7XCsa5kSvXKUyA

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
l5cvyy0YTiOfAHsaHDsTq0jHOfhz8WsEJleAzJN5nWSsoAg9xUPFDFkn9Wloh68eqwGW1eBDC0Rqa70rv5QByQ

=== Sending Initial Message === ✓ Message sent Response: Based on the context
provided, the available product is ekoDB. It is a high-performance database
product priced at $99.

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
J3iLk9IsINUsd3TG56wPK-n-4EuN38nLGoXXX6VyY_Jok11CDapZ7Q3vvqzn7UCqrVV8LBGfKxOLhrA3zYemtg
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
1. Score: 0.766
1. Score: 0.764
1. Score: 0.761

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.706
1. Score: 1.504
1. Score: 0.306

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

> @ekodb/ekodb-client@0.1.0 prepare npm run build

> @ekodb/ekodb-client@0.1.0 build tsc

up to date, audited 6 packages in 1s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.1.0 build tsc

✅ TypeScript client built! 🧪 Running TypeScript client library examples...

added 1 package, removed 1 package, and audited 13 packages in 675ms

3 packages are looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'usyMUZGASSCK4B2MzYH1y5uBow0ojtIybRwBcxWBf_ftQxcwQD0t_I_RDTBIJVmaZUO2HBfe4jEbkQ2OFONHvA'
}

=== Find by ID === Found: { id:
'usyMUZGASSCK4B2MzYH1y5uBow0ojtIybRwBcxWBf_ftQxcwQD0t_I_RDTBIJVmaZUO2HBfe4jEbkQ2OFONHvA',
active: { type: 'Boolean', value: true }, name: { value: 'Test Record', type:
'String' }, value: { value: 42, type: 'Integer' } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { active: { type: 'Boolean', value: true },
value: 100, id:
'usyMUZGASSCK4B2MzYH1y5uBow0ojtIybRwBcxWBf_ftQxcwQD0t_I_RDTBIJVmaZUO2HBfe4jEbkQ2OFONHvA',
name: 'Updated Record' }

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
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
Q8j0seYe3xzniYDLarwxywdiEcAbiies4Y4kN4RLfMIGS45w5cRev9c56dy_j6SygglWo624BOQXEfc8tXF5IA

=== List Collections === Total collections: 32 Sample collections:
non_indexed_vector_perf,indexed_query_scale_10k,client_collection_management_ts,schema_products_client_py,chat_messages\_\_ek0_bench

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
OyvAmQYoyin4uhodwlwQrF71UGhPwi0Bjroahnjsl6w8IFPJm0-v8V2UfVU8xy82Mge_MZFESPLLZ9scQBxOlA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
5otgC6epj3zGWNu_Z6hZNI0eOMf55itivnXHi3yeqkkl6uy8H608PNeavkQyDRwLH9IJ1J1ii_EbLWWxX7u40A

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
amCFDb17VXoTB8v6LXwb3r7EhTp4VZbOuwEisFre1JhrYhsMLt5Ws1Hw1HURcstXj760NDXRUAugzF27RGAwkg

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
YQpvBH9rgMPFiQoDV86iNxc7wr03Inb_9KlEvPNde9CWyWC4Q0FYHSqy8Dg5S5Ee0-Ii0dPdiIGOKX1hzzKD3w

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
5tVuxVVkjAsMXx5psq9seI33edXx5q3bz1hSVJq8kmaJAV9O9HN0UAlWq4T-X1htwdcNDXvr2X1lzHwPBEemMw

=== Sending Chat Message === Message ID:
tLIonkKvY2A-2KA-IPJRZBdURhca9ivv7D69XDZPDRqrJN8BE7ex6mt0KD3AkGyedAbbBKiricagSisolhxBEA

=== AI Response === There are three products available:

1. ekoDB Cloud: This is a fully managed cloud database service product. The
   price for this product is $499.

2. ekoDB: This is a high-performance database product with AI capabilities. The
   price for this product is $99.

3. ekoDB Pro: This is an enterprise edition product with advanced features. The
   price for this product is $299.

=== Context Used (3 snippets) === Snippet 1: { collection:
'client_chat_basic_ts', record: { name: { type: 'String', value: 'ekoDB Cloud'
}, description: { type: 'String', value: 'Fully managed cloud database service
product' }, price: { type: 'Integer', value: 499 }, id:
'V61Xgun38HyDdnN4Ag54nSI1X8eb2WS7BODcAKSt9H_XHrbAKfrjn0XtOLufQvvTpt2eexh88oMg3BxTHeh7Kw'
}, score: 0.1111111111111111, matched_fields: [ 'description' ] } Snippet 2: {
collection: 'client_chat_basic_ts', record: { description: { type: 'String',
value: 'A high-performance database product with AI capabilities' }, price: {
type: 'Integer', value: 99 }, id:
'HLA4ifLQ2y-3nPLgN2ahe8Bjay9TB53w-qRU9Wl5AjwtYNf5bHM-9K90Vt8dT4F3dU2U9pakTBYWfkPLqm152w',
name: { value: 'ekoDB', type: 'String' } }, score: 0.1111111111111111,
matched_fields: [ 'description' ] } Snippet 3: { collection:
'client_chat_basic_ts', record: { description: { type: 'String', value:
'Enterprise edition product with advanced features' }, name: { value: 'ekoDB
Pro', type: 'String' }, id:
'9IE-Eul0IqX7kfcUnYbz5-0BNwLKp44hLEtIe8iK8rgV5jFWDzLgGMvTF_SCl00KLHvS6ZAuSs3yFucbbjWK8Q',
price: { value: 299, type: 'Integer' } }, score: 0.1111111111111111,
matched_fields: [ 'description' ] }

Execution Time: 2912ms

=== Token Usage === Prompt tokens: 381 Completion tokens: 84 Total tokens: 465

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
lRUavD2flOZJbddJSLctK5nPRRZBlQ6sCJygd2tnQmtDfjdWD_rgysmTwU7xSoV5EMwUCJ1dtbjUQBRZE5xe9w

=== Sending Messages === ✓ Message 1 sent Response: Based on the information
provided, the available product is ekoDB, a high-performance database product.
It is priced at $99.

✓ Message 2 sent Response: Apologies, but the context provided does not specify
any product to provide a price for. Could you please provide more details about
the product you are interested in?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
ZV6f5sSqvWwSNB-iIxJYX2DNnT_TGsJAWnlXOwGqlAuINo0m24lH9Ixm-f1GV1GLOFUrHJOdtF1jtcb1g8MLWQ
Parent:
lRUavD2flOZJbddJSLctK5nPRRZBlQ6sCJygd2tnQmtDfjdWD_rgysmTwU7xSoV5EMwUCJ1dtbjUQBRZE5xe9w

=== Listing Sessions === ✓ Found 10 sessions Session 1:
ZV6f5sSqvWwSNB-iIxJYX2DNnT_TGsJAWnlXOwGqlAuINo0m24lH9Ixm-f1GV1GLOFUrHJOdtF1jtcb1g8MLWQ
(Untitled) Session 2:
lRUavD2flOZJbddJSLctK5nPRRZBlQ6sCJygd2tnQmtDfjdWD_rgysmTwU7xSoV5EMwUCJ1dtbjUQBRZE5xe9w
(Untitled) Session 3:
5tVuxVVkjAsMXx5psq9seI33edXx5q3bz1hSVJq8kmaJAV9O9HN0UAlWq4T-X1htwdcNDXvr2X1lzHwPBEemMw
(Untitled) Session 4:
J3iLk9IsINUsd3TG56wPK-n-4EuN38nLGoXXX6VyY_Jok11CDapZ7Q3vvqzn7UCqrVV8LBGfKxOLhrA3zYemtg
(Untitled) Session 5:
gxEBI3CNNntf2xaehuZyTplT-Fzkvi8gRM_KHiyf4E5vBdbPCXVeKoJCd5I9JZ0GPO2Fe-4uPbeTO1Ojuh39tw
(Untitled) Session 6:
0KEJp7uqQ7uGrfFtahSjEoYzqC9OnuGGMxaNxKkKM8T00cRCtAK-jzlMQa3QNN1MUXWZBlANCcmcCdm4Se8mcw
(Untitled) Session 7:
wU8go7PH1c807KAV8RgT27mlDpS3sg0MeBNPCSiqo8X-wMf7KARghD83v5EjT1Wb3yuvRmGqnf8chxiY3JjbcA
(Untitled) Session 8:
kzxqWv6UxXJVmepsrBHNYB_Xo_EAgY3tojfh-nQbMziX1NfVHAKSw5yeSRq2R2T_u_ee4FRm-AqjI8rp-vVNuQ
(Untitled) Session 9:
HWiXSRaxFxP3-U1fL-oXnKbly_MYe-bmXrz3xeJI3TUyO_pWGfgNUAfgPw3YiwV-y306B37GQ3EkqNZCQVEV4g
(Untitled) Session 10:
F7uUMqqxQ7XuajY-SPdEbMM05Tm5BzTgRRhae3dNYMbQQIwC4IqV9G2UMBKg3ner4HljFhlmF9FznoNk1dCQoA
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
ZV6f5sSqvWwSNB-iIxJYX2DNnT_TGsJAWnlXOwGqlAuINo0m24lH9Ixm-f1GV1GLOFUrHJOdtF1jtcb1g8MLWQ

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
D9yL29sqv6KRuXqP*tsLeKkRxfAiPU5ZtTrHA3pGQWxhtgxWJyI3AwvHBrBjl9n2NLaI_dQl92ous5iS*-9yQw

=== Sending Initial Message === ✓ Message sent Response: Based on the
information provided, the product available is ekoDB, which is a
high-performance database product. It is priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
I'm sorry, but there is no context provided regarding the price of ekoDB. You
may need to check the official website or contact the sales department for
accurate pricing information.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
GDxX-qUcjfyQO_F17JV-achovQEzTHHvfSaKDssAF1tHHV6kJxgkBm6AknH4I-miTyOwQns2xdAWSejgtYl7tg
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
1. Score: 12.540, Matched: name.value, name

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: bio.value, title.value, title, bio
1. Score: 26.400, Matched: title.value, bio.value, title, bio
1. Score: 26.400, Matched: title, bio.value, bio, title.value
1. Score: 26.400, Matched: title, bio, title.value, bio.value

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: bio.value, title, bio, title.value
1. Score: 39.600, Matched: bio.value, title, title.value, bio
1. Score: 39.600, Matched: title, bio.value, bio, title.value
1. Score: 39.600, Matched: title.value, bio, title, bio.value

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio, bio.value
1. Score: 13.200, Matched: bio, bio.value

1. Search with stemming and exact match boosting: Found 1 results (matches:
   work, working, worked)
1. Score: 13.200, Matched: bio, bio.value

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.750, Matched:
1. Score: 0.746, Matched:
1. Score: 0.728, Matched:

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.698, Matched: title, title.value, content.value, content
1. Score: 1.500, Matched: content.value, title, content, title.value
1. Score: 0.291, Matched:

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

added 1 package, removed 1 package, and audited 9 packages in 616ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities

╔════════════════════════════════════════╗ ║ ekoDB JavaScript Examples Test
Suite ║ ╚════════════════════════════════════════╝

=== Checking Server Connection === (node:11851) [DEP0040] DeprecationWarning:
The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created) ✓
Server is ready

=== Getting Authentication Token === ✓ Authentication successful

=== Running 7 Examples ===

=== Running
/Users/tek/Development/teknology/ekodb/examples/javascript/ttl-caching/simple_crud.js
=== ✓ Authentication successful

=== Insert Document === (node:11853) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) Inserted:
{ id:
'rNhbHPSIArzFnQKQMR25x1Sfl70Iz-U1bLR8Dn_87l7k4hXOtbHUy14mtNlftt-uppG-U0gusBjc1tOYYMadQg'
}

=== Find by ID === Found: { value: { type: 'Integer', value: 42 }, active: {
type: 'Boolean', value: true }, name: { type: 'String', value: 'Test Record' },
id:
'rNhbHPSIArzFnQKQMR25x1Sfl70Iz-U1bLR8Dn_87l7k4hXOtbHUy14mtNlftt-uppG-U0gusBjc1tOYYMadQg'
}

=== Find with Query === Found documents: 1

=== Update Document === Updated: { id:
'rNhbHPSIArzFnQKQMR25x1Sfl70Iz-U1bLR8Dn_87l7k4hXOtbHUy14mtNlftt-uppG-U0gusBjc1tOYYMadQg',
active: { type: 'Boolean', value: true }, name: 'Updated Record', value: 100 }

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully ✓ simple_crud.js completed
successfully

=== Running
/Users/tek/Development/teknology/ekodb/examples/javascript/ttl-caching/simple_websocket.js
=== ✓ Authentication successful

=== Inserting Test Data === (node:11856) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
test record:
eaL1CZPaNh7GaFHw3fhf78aNUisk3G3fUjGpKKEjzoK2_NNanznnngayneqm5aJN4r2-i_ObHyPSTRwvqZJ7jg

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"WdkxVkmEg6eaKu5MrjsYteG3GbKtZ2i4U8nHb6HSE31YLVFCBwgkuySXCtK9u-p-SSlsLUbHxpmzDx_iJVr4Cg",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"Ud9HWP3Q5qdR_BH_mcPhrPS0ThE9Esc6Pf2Egw7uZzJFsyHyEijHPBaXdseil8tIctclNV6nR6sDcEgosoUNVQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"JF7LOsatLKIGAi-huzbhWK46yMKOEsxkCp6xNfJBR0-hioVIZtx7tBeywjzRbu-yKqBKxo6p8yXJMDWHMtj1lg",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"eaL1CZPaNh7GaFHw3fhf78aNUisk3G3fUjGpKKEjzoK2_NNanznnngayneqm5aJN4r2-i_ObHyPSTRwvqZJ7jg",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 4
record(s) via WebSocket

✓ WebSocket example completed successfully ✓ simple_websocket.js completed
successfully

=== Running
/Users/tek/Development/teknology/ekodb/examples/javascript/ttl-caching/batch_operations.js
=== ✓ Authentication successful

=== Batch Insert === (node:11868) [DEP0040] DeprecationWarning: The `punycode`
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

=== Running
/Users/tek/Development/teknology/ekodb/examples/javascript/ttl-caching/kv_operations.js
=== ✓ Authentication successful

=== KV Set === (node:11871) [DEP0040] DeprecationWarning: The `punycode` module
is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Set key:
session:user123

=== KV Get === Retrieved value: { loginTime: '2025-10-14T06:46:24.766Z',
username: 'john_doe', userId: 123 }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ kv_operations.js completed
successfully

=== Running
/Users/tek/Development/teknology/ekodb/examples/javascript/ttl-caching/collection_management.js
=== ✓ Authentication successful

=== Create Collection (via insert) === (node:11873) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) Collection created with first record:
ucAwCsVz0s2DxnwWWXn_EmsF92ysEXOZQWpmN1tmt_o9MT4tfs89QUrHlwP6X3S0MKtp0kLtBHdKaNL0LmFyPg

=== List Collections === Total collections: 36 Sample collections: [
'schema_documents_client_ts', 'non_indexed_vector_perf',
'indexed_query_scale_10k', 'schema_products_client_ts',
'schema_products_client_py' ]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓
collection_management.js completed successfully

=== Running
/Users/tek/Development/teknology/ekodb/examples/javascript/ttl-caching/document_ttl.js
=== ✓ Authentication successful

=== Insert Document with TTL (1 hour) === (node:11885) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) ✓ Inserted document:
ge9NRYHlyJQPxLGh3jQGV1tzQvrB0xMQ4LOoxL6PREvD2HXNtAcaxEhOJ39nmPaCk_WdGvbMd1zpW7dcKViFbA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 4 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully ✓ document_ttl.js completed
successfully

=== Running
/Users/tek/Development/teknology/ekodb/examples/javascript/ttl-caching/websocket_ttl.js
=== ✓ Authentication successful

=== Insert Test Data with TTL === (node:11887) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
document with TTL:
s5pSBOomBx1BhTHN8X-inGV6HZ7JKnGQA3cZ98FFwPmuzcZOS11sJ07dYhHCQn-Qf1ZXx13OWP9qOQjug1FdBg

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 4 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully ✓ websocket_ttl.js completed
successfully

╔════════════════════════════════════════╗ ║ Test Summary ║
╚════════════════════════════════════════╝ Total: 7 Passed: 7 Failed: 0 ✅
JavaScript direct examples complete! 🧪 Running JavaScript client library
examples...

added 1 package, removed 1 package, and audited 9 packages in 678ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'M-9CfoZeFbM-MwyRdVXI7CYhu8eLa_YHBZkFVk9VL4snU8uCdr37odJBC5a_zSKQSUEUzVa6auRdHQXomQICpg'
}

=== Find by ID === Found: { id:
'M-9CfoZeFbM-MwyRdVXI7CYhu8eLa_YHBZkFVk9VL4snU8uCdr37odJBC5a_zSKQSUEUzVa6auRdHQXomQICpg',
name: { value: 'Test Record', type: 'String' }, active: { type: 'Boolean',
value: true }, value: { type: 'Integer', value: 42 } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { id:
'M-9CfoZeFbM-MwyRdVXI7CYhu8eLa_YHBZkFVk9VL4snU8uCdr37odJBC5a_zSKQSUEUzVa6auRdHQXomQICpg',
active: { type: 'Boolean', value: true }, value: 100, name: 'Updated Record' }

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

=== Get Multiple Keys === cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
VrPP4iOmVxwh0SiV3t0Hu0cUZM87j750jwjVQI_N8zMNzdDzixMYCl2zuidBwl_GVnUDZz6EHXoZ92RJh7PWzg

=== List Collections === Total collections: 36 Sample collections:
schema_documents_client_ts,non_indexed_vector_perf,indexed_query_scale_10k,schema_products_client_ts,schema_products_client_py

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
KFugQe76BogcwH51dFaFrRr5THPX_we8WM3SoGQRcV2Btlf96R3XJ7Di2HXHNZk-yghlD13dPcpX3iJYjfe5Dw

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
lHbS-SLdZBeZBq3NfO50RzSBd8oZgHuvu0DEC0Fh5XmZAHAKuSK_2i1U_BhoEoYdBhdHVRcqfNZ8uFblWBagjQ

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
pOGur8D3VWvega_h72BWSiRoEhuUds40XsyYsf3RAvBqEsPQ52kwDLyYEck3kcgyxkwCy8UvxNBnahHZl-N-ZA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
z112OwQTrFARl8ylG9Xwi2Z5f-syN21oH2Lh4fgqZ6jAVOWTeJCc0WjilApFkUZzE8fLduKlTNrxg00s4zfwrw

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✅ JavaScript client examples complete! ✅ All JavaScript integration
tests complete! ✅ All integration tests complete!
