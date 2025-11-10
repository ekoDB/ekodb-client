make test-examples-go 🧪 [36mRunning Go examples (direct HTTP/WebSocket)...[0m
[34m ╔════════════════════════════════════════╗[0m [34m║ ekoDB Go Examples Test
Suite ║[0m [34m╚════════════════════════════════════════╝[0m [34m === Checking
Server Connection ===[0m [32m✓ Server is ready[0m [33m === Running 7 Examples
===[0m [34m === Running simple_crud.go ===[0m ✓ Authentication successful

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
