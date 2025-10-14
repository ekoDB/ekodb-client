tek@Seans-MacBook-Pro ~/Development/teknology/ekodb
[feat/ekodb-client-package+]$ make test-examples-go 🧪 Running Go examples
(direct HTTP/WebSocket)...

╔════════════════════════════════════════╗ ║ ekoDB Go Examples Test Suite ║
╚════════════════════════════════════════╝

=== Checking Server Connection === ✓ Server is ready

=== Running 7 Examples ===

=== Running simple_crud.go === ✓ Authentication successful

=== Insert Document === Inserted:
map[id:QhUkOpjdp_tsxzXnKBZDOZwJAZJw6NTTrBDYKJfCsJGiP5aC8cgd2hJzKYKA5pFPe2ZAg5pyqyl2bnmZUIxbDg]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:QhUkOpjdp_tsxzXnKBZDOZwJAZJw6NTTrBDYKJfCsJGiP5aC8cgd2hJzKYKA5pFPe2ZAg5pyqyl2bnmZUIxbDg
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found 1 documents

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:QhUkOpjdp_tsxzXnKBZDOZwJAZJw6NTTrBDYKJfCsJGiP5aC8cgd2hJzKYKA5pFPe2ZAg5pyqyl2bnmZUIxbDg
name:Updated Record value:100]

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully ✓ simple_crud.go completed
successfully

=== Running simple_websocket.go === ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
1hNsqfczsMEFT22MTxeZBOPhOc9wYn04Alb0XIiWaVNkAQoZH935BIEDAfVOWyNyJKLBg80eGevLtK3XWEsVxw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"1hNsqfczsMEFT22MTxeZBOPhOc9wYn04Alb0XIiWaVNkAQoZH935BIEDAfVOWyNyJKLBg80eGevLtK3XWEsVxw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 1
record(s) via WebSocket

✓ WebSocket example completed successfully ✓ simple_websocket.go completed
successfully

=== Running batch_operations.go === ✓ Authentication successful

=== Batch Insert === Batch insert response: map[failed:[]
successful:[SBxGZ3kTS-lSzct3JWuLtSZp84eaEduPmzdMtSsFsRiHZ87RH5vfLwys3PV0LMtB9wmhKcE2XcJB1dD9JwFprA
xUH25Zj2R_hsmJ8eTnLNHDFgbV5hwuChKAwZUdpZWyfV9GAOXmYRQEgYdiNF-bkeYGuCxPo_5AmRv4PDDFbxdw
UdlEEglES1rI_Tk617IJhURjHfiEM5tyGfob1LDi8HtPy9Cs4nTSKo-f6j5kYI9duBptJ5J8OYbxnLMG2c4e0Q
LH8u5UHWNcCGVQ-qAKuWGFbrFYc37xT3RPaLydovN3HGKe4sX--lP01X5TJ0RqogozSFJc1E4Z9HhYxx6PZ3qg
UOgyhKPxb3VuoPPZSD--zyMcdczkeXnz1saFloGlJv6n6t1M1Zi9sf7BDwJiPXleEXfBNsC7TBI8mlzR7Hge2w]]
✓ Batch insert completed

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === Batch update response: map[failed:[]
successful:[3IJSc-HrW7YlcNKT3YMN09VWwUup0oKN_ki8T5VU8lyQZY_wPYMPZ0nxgI3VmXXivhXSCCoyQwOt8EMsiky00Q
TqD1SaD2304dKqbAWS7xObViWQPmprVn7nwV8G_SL9EFOgin2yjOHLSTK6-g079FUqMjbbXayh2gLv8aG6ZH2g
XQ8ihN2-dUzLvH0x4BChFYMKZzDQYmJqoYlhiuq8VDfBv0R32j0caqu2IsFNuV0QfOIKvvv_rro-ch5ZiWF2pQ]]
✓ Batch update completed

=== Batch Delete === Batch delete response: map[failed:[]
successful:[3IJSc-HrW7YlcNKT3YMN09VWwUup0oKN_ki8T5VU8lyQZY_wPYMPZ0nxgI3VmXXivhXSCCoyQwOt8EMsiky00Q
TqD1SaD2304dKqbAWS7xObViWQPmprVn7nwV8G_SL9EFOgin2yjOHLSTK6-g079FUqMjbbXayh2gLv8aG6ZH2g
XQ8ihN2-dUzLvH0x4BChFYMKZzDQYmJqoYlhiuq8VDfBv0R32j0caqu2IsFNuV0QfOIKvvv_rro-ch5ZiWF2pQ]]
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
WvevO0Z3XME_ldoK6Vulp9kGkHWnp_X-NXs5eogcC7v9v4nGQvgNmgE-mUedKdCEr_igB0PVicXNkw6C_twOrw

=== List Collections === Total collections: 10 Sample collections:
[employees_with_schema_ts chat_configurations__ek0_bench websocket_test
products_with_schema_ts users_with_schema_ts test_collection demo_collection
documents_with_schema_ts chat_messages__ek0_bench batch_users]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓
collection_management.go completed successfully

=== Running document_ttl.go === ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
sYNLdHnRas-AUjyvgUfdy_J2pSzmIzitcG0F5-YnJGV67b05FhlqJB8VqQgmu2a-lmRQsQvmm9hYjUyv9SFEiw

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 1 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully ✓ document_ttl.go completed
successfully

=== Running websocket_ttl.go === ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
cPfWvql5CMXEwfiE-GtLcqnXP3UCIEiMJBA9R_T1Hrtl3vt6odXurgRNArW7UiL6ElfJOWNH_dgwMbe3MVSCpA

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully ✓ websocket_ttl.go completed
successfully

╔════════════════════════════════════════╗ ║ Test Summary ║
╚════════════════════════════════════════╝ Total: 7 Passed: 7 Failed: 0 ✅ Go
direct examples complete! 🧪 Running Go client library examples... ✓ Client
created (token exchange happens automatically)

=== Insert Document === Inserted:
map[id:qM3byJmyyHuJ1C3QNxqwfvclBKBdSfLA4kXre5JgUE6iCWHn16Up_XipMDF2hnMTRYm7m_AbRQa-1XHUx81KXA]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:qM3byJmyyHuJ1C3QNxqwfvclBKBdSfLA4kXre5JgUE6iCWHn16Up_XipMDF2hnMTRYm7m_AbRQa-1XHUx81KXA
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found documents: 1

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:qM3byJmyyHuJ1C3QNxqwfvclBKBdSfLA4kXre5JgUE6iCWHn16Up_XipMDF2hnMTRYm7m_AbRQa-1XHUx81KXA
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
BBtnIsZYFhe1ppxYQUNPlSnO2a8OKRvcZphf1XsFODW9YQKSRPckpe6pxmY_drFMdbZs9ngHSleXD6dAk-CQfA

=== List Collections === Total collections: 12 Sample collections:
[employees_with_schema_ts chat_configurations__ek0_bench websocket_test
products_with_schema_ts ws_ttl_test]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
bOXwtbPcSHEd4fpCR58sOeA8cI6YaEW8_vGXvUk3voFr2l33a5bZ6speV3-BIXlpEWXJLWwBTPVsGy6NgT3Q2A

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
7ghlyr0Wz2HQeb0YeBLOijpv9AZDY7Ttmf5ia5-AuSmloKY1qo27w463smc0T3Ni_FxuXXt95NQ8eAh4RzA3sg

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
QkhB5V0zEnfFNbVk_UyRmiiCdXMQU9LXcqfLWg8WAJ63vnzu9MjFoKCtW1fhdRu1F9Ys72swHjAqIp2wRX5-2A

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
RsA2v9_EBzektfjM6yMItC1NdLKafQedfWQZDR07Ig7NrGEr-TOG15Fzs9xERb9DbUSd1ENgZhKBA4GjuFUPQQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
XhrUAKwYD3gjl9s2R16-2NFLNEM70coQkrS3szBD7BjTivxkbdv5g0UcKB8C3Orr3cPwa-xvzNvwrEToCTgypA

=== Sending Chat Message === Message ID:
t2KlxHqZ6VACypUF_TOuUcToUUWbAoFKd5MKWUZLf8bWLV-n-Vc8E01PvdQdKdw0yvKZoGiMyrpIsSlxUwxqLA

=== AI Response === Based on the provided information, three products are
available:

1. ekoDB: This is a high-performance database product with AI capabilities. It
   is priced at $99.

2. ekoDB Pro: This is an Enterprise edition product with advanced features. It
   is available for $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. You can
   avail this for $499.

=== Context Used (3 snippets) === Snippet 1: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:A
high-performance database product with AI capabilities]
id:9zREccfVyqmc02XR_CjAxw2TBbVZryqPqVOaTZNA7t2aWnD_Ue1pwcnTKwgsSYY1u5I53b03_FqfKTgtfNqj6A
name:map[type:String value:ekoDB] price:map[type:Integer value:99]]
score:0.1111111111111111] Snippet 2: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String
value:Enterprise edition product with advanced features]
id:3YPesQJtE799dd7SrlyFrHJV7oRm9j-f0wV8LAi8-LgdJ5SldtROLACIbmB0l8YQZkh294_T7bFpgMgIUsI5bQ
name:map[type:String value:ekoDB Pro] price:map[type:Integer value:299]]
score:0.1111111111111111] Snippet 3: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:Fully
managed cloud database service product]
id:VwsIkCcIvx4WJZeryYziWMo6m3rFyhJP4JBx_0df5_pebr3bNfuE928NBuHMMMNhL9C59nwt8y2laYwRvOQuhg
name:map[type:String value:ekoDB Cloud] price:map[type:Integer value:499]]
score:0.1111111111111111]

Execution Time: 3814ms

=== Token Usage === Prompt tokens: 380 Completion tokens: 84 Total tokens: 464

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
FYGuvo9YAgxmR3aS_86MOOKPWuZXCVUgfioU1r10-IgKYYAQzljX4oPEnHgLEIMJYsNkj8oXmIcsyyux9Pvo6w

=== Sending Messages === ✓ Message 1 sent Response: Based on the context
provided, the available product is ekoDB. It is a high-performance database
product and is priced at $99.

✓ Message 2 sent Response: Apologies, but there doesn't seem to be any context
provided about a specific product. Could you please provide more details or
clarify which product's price you're inquiring about?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
ml3LczHGBa5hkhYsQa0adfxxdl4XkOtlVdmaDfpkGv8RzSKDyWxbo-I59HyYkjmy4SPYUKsvHcL0zdr_pl5Jdg
Parent:
FYGuvo9YAgxmR3aS_86MOOKPWuZXCVUgfioU1r10-IgKYYAQzljX4oPEnHgLEIMJYsNkj8oXmIcsyyux9Pvo6w

=== Listing Sessions === ✓ Found 6 sessions Session 1:
ml3LczHGBa5hkhYsQa0adfxxdl4XkOtlVdmaDfpkGv8RzSKDyWxbo-I59HyYkjmy4SPYUKsvHcL0zdr_pl5Jdg
(Untitled) Session 2:
FYGuvo9YAgxmR3aS_86MOOKPWuZXCVUgfioU1r10-IgKYYAQzljX4oPEnHgLEIMJYsNkj8oXmIcsyyux9Pvo6w
(Untitled) Session 3:
XhrUAKwYD3gjl9s2R16-2NFLNEM70coQkrS3szBD7BjTivxkbdv5g0UcKB8C3Orr3cPwa-xvzNvwrEToCTgypA
(Untitled) Session 4:
xJsF01_LCPcHuzhzM9_r7smJjuisBMYCJtrbbPTVuVkFTHLuau9xjmwKFSpBV7S1A3wGKuJ7f1m9lmlETZR_Kg
(Untitled) Session 5:
lwVkcqGLpj-IbL5XCPwmFLvrbpCN4OcjHtuxmMJDPHJNY5M6hqul0grh-KAmvmIQcbZd5cyyxxtuCps_nY81JQ
(Untitled) Session 6:
hIdSf67EYHKiyswyiyAjusdt_oGeMaoD_wf9VMDlDKbODCtrWl_X8CcZHpTCG16vmsbWaN-iF8vjQvniwUTEiA
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
ml3LczHGBa5hkhYsQa0adfxxdl4XkOtlVdmaDfpkGv8RzSKDyWxbo-I59HyYkjmy4SPYUKsvHcL0zdr_pl5Jdg

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
8QgOrpbIMsAsfdRP_Su5e-0c9MIA4Wyu1X3fZLUhTzEa7e66CWncVHYIIyarEfupAhDSu5pRMZNg_LIjD0tQYw

=== Sending Initial Message === ✓ Message sent Response: Based on the
information provided, the available product is ekoDB, a high-performance
database product priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
I'm sorry, but the context provided does not include information on the price of
ekoDB. I would recommend checking the official website or contacting their sales
department for the most accurate and up-to-date pricing.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
AsFQoGEWbkLSHEeWSQtDeTCRhmghMjLbyGVL7THyVCQXXJNJRa-9Ba9hIa5bCgnG7Y4fT14gDtyGfhEGUXdgHg
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
1. Score: 0.759
1. Score: 0.739
1. Score: 0.736

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.695
1. Score: 1.504
1. Score: 0.295

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
integration tests complete! tek@Seans-MacBook-Pro ~/Development/teknology/ekodb
[feat/ekodb-client-package+]$
