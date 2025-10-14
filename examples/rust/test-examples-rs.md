tek@Seans-MacBook-Pro ~/Development/teknology/ekodb
[feat/ekodb-client-package+*]$ make test-examples-rs 🧪 Running Rust examples
(direct HTTP/WebSocket)... Finished `dev` profile [unoptimized + debuginfo]
target(s) in 0.08s Running `target/debug/examples/simple_crud` ✓ Authentication
successful

=== Insert Document === Inserted: Object {"id":
String("WokDR6K4gIOAUOUQEg1HALDp12v7fxiBwY_6F6ymP62BO5iv5RqEjdvMHqsKVu8bSxc8e1bL4m8XLlUcukyUng")}

=== Find by ID === Found: Object {"active": Object {"type": String("Boolean"),
"value": Bool(true)}, "id":
String("WokDR6K4gIOAUOUQEg1HALDp12v7fxiBwY_6F6ymP62BO5iv5RqEjdvMHqsKVu8bSxc8e1bL4m8XLlUcukyUng"),
"name": Object {"type": String("String"), "value": String("Test Record")},
"value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query === Found documents: Array [Object {"active": Object
{"type": String("Boolean"), "value": Bool(true)}, "id":
String("WokDR6K4gIOAUOUQEg1HALDp12v7fxiBwY_6F6ymP62BO5iv5RqEjdvMHqsKVu8bSxc8e1bL4m8XLlUcukyUng"),
"name": Object {"type": String("String"), "value": String("Test Record")},
"value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document === Updated: Object {"active": Object {"type":
String("Boolean"), "value": Bool(true)}, "id":
String("WokDR6K4gIOAUOUQEg1HALDp12v7fxiBwY_6F6ymP62BO5iv5RqEjdvMHqsKVu8bSxc8e1bL4m8XLlUcukyUng"),
"name": String("Updated Record"), "value": Number(100)}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully Finished `dev` profile [unoptimized

- debuginfo] target(s) in 0.08s Running `target/debug/examples/simple_websocket`
  ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
"ixYgFHKfzof8nX2u5hfv7WDJpeb7FcFxFZqmHoPFe9JNUJTNODu356kcURQt_lIoVCWJc-\_GGRsjOk8Q0eE58g"

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"ixYgFHKfzof8nX2u5hfv7WDJpeb7FcFxFZqmHoPFe9JNUJTNODu356kcURQt_lIoVCWJc-_GGRsjOk8Q0eE58g",
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

=== Set Multiple Keys === ✓ Set 3 keys === Get Multiple Keys ===
cache:product:1: Object {"name": String("Product 1"), "price": Number(29.99)}
cache:product:2: Object {"name": String("Product 2"), "price":
Number(39.989999999999995)} cache:product:3: Object {"name": String("Product
3"), "price": Number(49.989999999999995)}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully Finished `dev` profile [unoptimized +
debuginfo] target(s) in 0.08s Running
`target/debug/examples/collection_management` ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
"1LvhgMocdm15dhq39Ol38FqJtMXOIjBgQf06JR5mQlpcVV3dWV_pzHU8_jOncQzsImCYBn0fkciUkm44i1lgHA"
=== List Collections === Total collections: 4 Sample collections:
["websocket_test", "demo_collection", "test_collection", "batch_users"] ===
Count Documents === Document count: 1 === Delete Collection === Collection
deleted successfully === Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/document_ttl` ✓ Authentication successful === Insert
Document with TTL (1 hour) === ✓ Inserted document:
"ndA9DFgd9kVkjPl8LdWPEuoOu-WAw6oMi5KNsnTpver-xyjAdUcLAey_JafXy6-9JwVw7d5apYX6MNm-5P2HbQ"
=== Insert Document with TTL (5 minutes) === ✓ Inserted document: null

=== Query Documents === ✓ Found 1 documents with TTL === Update Document === ✓
Updated document === Delete Document === ✓ Deleted document

✓ All document TTL operations completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.08s Running
`target/debug/examples/websocket_ttl` ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
String("3UVGprZ8djqr6tk244m0qDtuwhMa0HeJABlfw_OuG4JI_mbh6KgYAwYngMNFiexIYmsl9aUlHUmdJDZbb01jFA")

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully ✅ Rust direct examples complete!
🛠️ Building client library... cargo build -p ekodb_client Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.08s ✅ Client build complete! 🧪
Running Rust client library examples... Finished `dev` profile [unoptimized +
debuginfo] target(s) in 0.07s Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: Record { fields: {"id":
String("nf6Jj7Acpxy6R4B4ns8onGyDBHUED9A8Yj0BdY-z5xho3Stfiu9CBbFHdW1ArcfvoKCw29DOmo6uXUKNeXQcow")}
}

=== Find by ID === Found: Record { fields: {"value": Object({"type":
String("Integer"), "value": Integer(42)}), "id":
String("nf6Jj7Acpxy6R4B4ns8onGyDBHUED9A8Yj0BdY-z5xho3Stfiu9CBbFHdW1ArcfvoKCw29DOmo6uXUKNeXQcow"),
"active": Object({"value": Boolean(true), "type": String("Boolean")}), "name":
Object({"value": String("Test Record"), "type": String("String")})} }

=== Find with Query === Found documents: [Record { fields: {"active":
Object({"value": Boolean(true), "type": String("Boolean")}), "value":
Object({"type": String("Integer"), "value": Integer(42)}), "name":
Object({"type": String("String"), "value": String("Test Record")}), "id":
String("nf6Jj7Acpxy6R4B4ns8onGyDBHUED9A8Yj0BdY-z5xho3Stfiu9CBbFHdW1ArcfvoKCw29DOmo6uXUKNeXQcow")}
}]

=== Update Document === Updated: Record { fields: {"active": Object({"type":
String("Boolean"), "value": Boolean(true)}), "name": String("Updated Record"),
"id":
String("nf6Jj7Acpxy6R4B4ns8onGyDBHUED9A8Yj0BdY-z5xho3Stfiu9CBbFHdW1ArcfvoKCw29DOmo6uXUKNeXQcow"),
"value": Integer(100)} }

=== Delete Document === Deleted document

=== Cleanup === ✓ Deleted collection

✓ All CRUD operations completed successfully Finished `dev` profile [unoptimized

- debuginfo] target(s) in 0.08s Running
  `target/debug/examples/client_batch_operations` ✓ Client created

=== Batch Insert (via multiple inserts) === ✓ Inserted 5 records ✓ Verified:
Found 5 total records in collection

=== Update Records === ✓ Updated 3 records

=== Delete Records === ✓ Deleted 3 records

=== Cleanup === ✓ Deleted collection

✓ All batch operations completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.08s Running
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
"aoReUTJG0NbLlA_2M53GrhmDuB1c-0RF-hzDiB2B2MXtK1gdnqjzFsCcocbNqwDAjtY8z7i93qH6ZketXv6bDw"

=== List Collections === Total collections: 6 Sample collections:
["websocket_test", "ws_ttl_test", "client_collection_management_rust",
"test_collection", "ttl_cache"]

=== Count Documents === Document count: 1

=== Check Collection Exists === Collection exists: true

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.08s Running
`target/debug/examples/client_document_ttl` ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
"DC7ee5PKgXK5xbiM82QJboaRJP5jyjXXXLzh59Fxb4qCK_oSmiYJFmjd1AXBbqaFVXl7sXEzx_x8iqeTtkX3GQ"

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
Some(String("1X0jlfdwO2ojPNAM0FTkUnU3B9W49AKn-SRMyXBZ81q_0WP0uLsMlVTf-HLtmhy1KtGQHZk79QEQOITte2dhWQ"))

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
Running `target/debug/examples/client_simple_websocket` ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
AKFXPFhFnlOE-bj8uuffFvO9zPmghoVfo3VXIaEuJo4eHtRMgm0o40LHzw6jMtX3oh0jU0t5eqBu7rp6RPqkYg

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully Finished `dev` profile [unoptimized +
debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_websocket_ttl` ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
Some(String("xU5fPXdkcjf-L-NcA8XqvPWFgMsBZdtevItWdo00XY3fgPwTDBmFHCCzIZ7p1Jc17xepeZPjd7rbdBokbnrp0g"))

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
Running `target/debug/examples/client_query_builder` === ekoDB Query Builder
Example ===

=== Inserting Sample Data === ✓ Inserted 5 users

=== Simple Equality Query === ✓ Found 3 active users

- Some(Object({"value": String("David"), "type": String("String")}))
- Some(Object({"value": String("Alice"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("Bob")}))

=== Range Query (age >= 28 AND age < 35) === ✓ Found 3 users in age range

- Some(Object({"type": String("String"), "value": String("David")}))
- Some(Object({"type": String("String"), "value": String("Bob")}))
- Some(Object({"value": String("Eve"), "type": String("String")}))

=== IN Operator === ✓ Found 4 users with status active or pending

- Some(Object({"type": String("String"), "value": String("David")}))
- Some(Object({"type": String("String"), "value": String("Alice")}))
- Some(Object({"type": String("String"), "value": String("Bob")}))
- Some(Object({"type": String("String"), "value": String("Eve")}))

=== NOT IN Operator === ✓ Found 4 users not inactive

=== String Pattern Matching === ✓ Found 5 users with @example.com email

=== Regex Query === ✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) === ✓ Found 2 users
matching all conditions

- Some(Object({"type": String("String"), "value": String("David")}))
- Some(Object({"type": String("String"), "value": String("Bob")}))

=== OR Query === ✓ Found 2 users with age < 28 OR age > 32

- Some(Object({"type": String("String"), "value": String("Alice")}))
- Some(Object({"value": String("Charlie"), "type": String("String")}))

=== Sorted Query (by score descending) === ✓ Top 3 users by score:

1. Some(Object({"value": String("Bob"), "type": String("String")}))
2. Some(Object({"type": String("String"), "value": String("David")}))
3. Some(Object({"type": String("String"), "value": String("Alice")}))

=== Pagination (page 2, size 2) === ✓ Page 2 results:

- Some(Object({"value": String("Charlie"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("David")}))

=== NOT Operator === ✓ Found 4 users NOT inactive

=== Complex Chained Query === ✓ Found 3 users with all conditions

=== Cleanup === ✓ Deleted collection

✓ All query builder operations completed successfully Compiling ekodb-examples
v0.1.0 (/Users/tek/Development/teknology/ekodb/examples/rust) Finished `dev`
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
   ["description", "title.value", "title", "description.value"]
2. Score: 2.0000 Title: Some(String("Python for Data Science")) Matched:
   ["description", "description.value"]
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

✓ All search operations completed successfully Execution time: 3ms Finished
`dev` profile [unoptimized + debuginfo] target(s) in 0.08s Running
`target/debug/examples/client_schema_management` === ekoDB Schema Management
Example ===

=== Creating Collection with Schema === ✓ Created collection
'client_schema_management_rust' with schema

=== Inserting Valid Documents === ✓ Inserted user 1:
Some(String("tXmWUOMHPWQduiRxIZ_5OB9HTmSVwx3h3jAc_Ef5Ct6bXdjK3nLkXryfsA5ndtv7XTW62YWRvtXAk5r93m_0vQ"))
✓ Inserted user 2:
Some(String("GK0alD58dxrIcE6egYFbzVQQLgjWLtQXpKohE5TKe7jNhqsCWJXQBjKpVK2taBh1SlfTHDV_en9UhswKTN2HPg"))

=== Getting Schema === ✓ Schema for client_schema_management_rust:

- age: Integer
- title: String (required)
- status: String
- email: String (required)

=== Listing Collections === ✓ Total collections: 6 Sample: ["websocket_test",
"ws_ttl_test", "test_collection", "ttl_cache", "client_schema_management_rust"]

=== Cleanup === ✓ Deleted collection

✓ All schema management operations completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.08s Running
`target/debug/examples/client_chat_basic` === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
-DnokI3yMEARmlQ6L8Z4989XyMyBoYgcGVkK7xLnImR5kTe8lfEbm6ZJ15Ef5u4UWzP4yenUZcK5pDWSTw3k0A

=== Sending Chat Message === Message ID:
5vq5penvnmqYKFzMRXFQDZpfpw_Pvw3lXq6NOMd2D7CDyWcVRy2P3vQ4KlQfAza0NTRX-SuMQ2ClqckNd_V9cw

=== AI Response === Response 1: ekoDB is a high-performance database that comes
with intelligent caching, real-time capabilities, and AI integration. Some of
its key features include a chat feature that allows you to query your database
using natural language and get AI-powered responses with relevant context. It
also supports various types of search including full-text search, vector search,
and hybrid search with automatic context retrieval.

=== Context Used (3 snippets) ===

Snippet 1: Collection: client_chat_basic_rust Score: 0.6333 Matched Fields:
["content", "title", "category"] Record: Object {"category": Object {"type":
String("String"), "value": String("features")}, "content": Object {"type":
String("String"), "value": String("The chat feature allows you to query your
database using natural language and get AI-powered responses with relevant
context.")}, "id":
String("WeQBCofqFgJrswlb2mH7XwqpZAa7Mpt0xk57cPwNPPYPMJn1R17opelDBzwNxWWao7NCogBavBbK3hdkKm7r5g"),
"title": Object {"type": String("String"), "value": String("AI Chat
Integration")}}

Snippet 2: Collection: client_chat_basic_rust Score: 0.5222 Matched Fields:
["content", "title"] Record: Object {"category": Object {"type":
String("String"), "value": String("documentation")}, "content": Object {"type":
String("String"), "value": String("ekoDB is a high-performance database with
intelligent caching, real-time capabilities, and AI integration.")}, "id":
String("kgveBgr9WN_m2Ng7oSCMD178YhuOMlcf7xp2dqYOcURZ5uyuuRftO4jOU1v01kHIUzs_s0PP0RwLkBhpVwOvzw"),
"title": Object {"type": String("String"), "value": String("Introduction to
ekoDB")}}

Snippet 3: Collection: client_chat_basic_rust Score: 0.5222 Matched Fields:
["title", "category", "content"] Record: Object {"category": Object {"type":
String("String"), "value": String("features")}, "content": Object {"type":
String("String"), "value": String("ekoDB supports full-text search, vector
search, and hybrid search with automatic context retrieval.")}, "id":
String("CpjcH_8zCOV7jlPl3GyBUqexrgQGPfbpVny8bhguALSPMr5KM05CBpqTdHC8M7jnYLND1wH2GqBZfaSwyEEsrQ"),
"title": Object {"type": String("String"), "value": String("Search Features")}}

Execution Time: 2574ms

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/teknology/ekodb/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.52s Running
`target/debug/examples/client_chat_sessions` === ekoDB Chat Session Management
Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
nRvcAF-lyz-NMNNt33KSD9n8g6nfrKOoVfQ5AJFAfW-kYbGNtY_aICzv-1F1Qh_kvaVyMCtmeCshijvo2IWf9Q
=== Sending Messages === ✓ Message 1 sent Response: Based on the information
provided, the product available is ekoDB, which is a high-performance database
product with AI capabilities. The price of this product is $99.

✓ Message 2 sent Response: Apologies, but there isn't any specific product
mentioned in the context that I could provide a price for. Could you please
specify the product you're interested in?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
odWsGw-8ykGcR3HgdPNin42MJZBs_FwCaGvQUUizR_NKeKvR9vNa3YHJWPzXb-s9fkz5ZRWn_BHEnSlCswpJng
Parent:
nRvcAF-lyz-NMNNt33KSD9n8g6nfrKOoVfQ5AJFAfW-kYbGNtY_aICzv-1F1Qh_kvaVyMCtmeCshijvo2IWf9Q

=== Listing Sessions === ✓ Found 3 sessions Session 1:
odWsGw-8ykGcR3HgdPNin42MJZBs_FwCaGvQUUizR_NKeKvR9vNa3YHJWPzXb-s9fkz5ZRWn_BHEnSlCswpJng
(Untitled) Session 2:
nRvcAF-lyz-NMNNt33KSD9n8g6nfrKOoVfQ5AJFAfW-kYbGNtY_aICzv-1F1Qh_kvaVyMCtmeCshijvo2IWf9Q
(Untitled) Session 3:
-DnokI3yMEARmlQ6L8Z4989XyMyBoYgcGVkK7xLnImR5kTe8lfEbm6ZJ15Ef5u4UWzP4yenUZcK5pDWSTw3k0A
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
odWsGw-8ykGcR3HgdPNin42MJZBs_FwCaGvQUUizR_NKeKvR9vNa3YHJWPzXb-s9fkz5ZRWn_BHEnSlCswpJng

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/teknology/ekodb/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.51s Running
`target/debug/examples/client_chat_advanced` === ekoDB Advanced Chat Features
Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
OkwFsj-9EbwaVq7OGZ5Shns67AnlCvQkpinbINVCoIGrXLKZPXvhYdiFNwUHeqNh7LEeW2XJ2k7k54cOmRqyrQ

=== Sending Initial Message === ✓ Message sent Response: Based on the provided
context, the available product is a high-performance database product named
ekoDB, which is priced at $99.

✓ Second message sent === Feature 1: Regenerate AI Response === ✓ Message
regenerated New response: I'm sorry, but there's no information provided in the
context about the price of ekoDB. I recommend checking the official website or
contacting the provider for the most accurate and up-to-date pricing
information.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
rwQGu8Ssk01n0l2gGaDqQVTsQINeTCzIButoVvhZX3qhgIEAFX0Rg7L6zXVNL0q2q8RZoXPsonwX1S5Ym-F8Gg
✓ Sent message in second session ✓ Sessions merged successfully Total messages
in merged session: 4

=== Feature 5: Delete Message === ✓ Message deleted

✓ Messages remaining: 3

=== Cleanup === ✓ Deleted session ✓ Deleted collection

✓ All advanced chat features demonstrated successfully! ✅ Rust client examples
complete! ✅ All Rust integration tests complete! tek@Seans-MacBook-Pro
~/Development/teknology/ekodb [feat/ekodb-client-package+*]$
