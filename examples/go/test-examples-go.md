make test-examples-go 🧪 [36mRunning Go examples (direct HTTP/WebSocket)...[0m
[34m ╔════════════════════════════════════════╗[0m [34m║ ekoDB Go Examples Test
Suite ║[0m [34m╚════════════════════════════════════════╝[0m [34m === Checking
Server Connection ===[0m [32m✓ Server is ready[0m [33m === Running 7 Examples
===[0m [34m === Running simple_crud.go ===[0m ✓ Authentication successful

=== Insert Document === Inserted:
map[id:DajANNNCnjO8I4l-vHEreQ2aiRLo0jY3CbTbIZmJlBd8XWJ-BBw4_9-m5ADWvqce8klESpN56oqUaEjVJflIig]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:DajANNNCnjO8I4l-vHEreQ2aiRLo0jY3CbTbIZmJlBd8XWJ-BBw4_9-m5ADWvqce8klESpN56oqUaEjVJflIig
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found 1 documents

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:DajANNNCnjO8I4l-vHEreQ2aiRLo0jY3CbTbIZmJlBd8XWJ-BBw4_9-m5ADWvqce8klESpN56oqUaEjVJflIig
name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.go completed
successfully[0m [34m === Running simple_websocket.go ===[0m ✓ Authentication
successful

=== Inserting Test Data === ✓ Inserted test record:
FcYv2T8043aRaJq2URI56f8dcaxvd3uHgbcAoZo0VouAL3Da5L5D5MCKSAf29hThOk0F5BSTyk10f8VG0EvsDA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"bk4bCkONct5iiO7cGmRUEYih--5u3TSYBQBvnBf7oLbPFW8O9KekSQ_V6NNbM5JjduSNOA-TQs5W87l0V4qeMg",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"ygX4zwfZD59Pc7Wlu1yNBjgmQZtkOpiXEpwp9VZMKq1H-3SZoGHnVaSVZv5jcMnvUabr6bpopfbc-Um7Ps9GQg",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"FcYv2T8043aRaJq2URI56f8dcaxvd3uHgbcAoZo0VouAL3Da5L5D5MCKSAf29hThOk0F5BSTyk10f8VG0EvsDA",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 3
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.go completed
successfully[0m [34m === Running batch_operations.go ===[0m ✓ Authentication
successful

=== Batch Insert === Batch insert response: map[failed:[]
successful:[OXl4cyZBM-OWvORJO7pJ0Fq0I7e5iAUdzUAMj910m1kMDYSbVkjIRIkfrWa3yfDKm3wdULfF03hozCVstt9xRA
dGXqKsn0hXFMHdlklGf_bp0_9zZpvZ4KEjJbKoHrn0se6RsbCP7UuCll5SENpPptVK-YOFAfoAVwEGsGLkMPhQ
j2q7BZ6i9egQ79OWzuAP56iNNb7o1HX_4xZFSo0sabgtuM7frLtNuwor4iWk-z237lDaUnoGXXUcVKXXARpCcg
WfnSftDsNlWBryZm_eDa_2roV6Z_msG6G8JsZlhw_AKYjNr4C4CgiRH_SvzQsun04JWx3lcaNKSt493u3_dQrg
bD1x9NTiqdr7fsX-DJqudy30h2LENaGFIbwSWYsj80mH2qCQPZ_4Vyok3WAB5h1IInmli0uHr_1_CFrMfB2Hzg]]
✓ Batch insert completed

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === Batch update response: map[failed:[]
successful:[1XSvz3_HMzqJno9BgTsT5_5W5U817wazBsM2jEBuXQwUGZevuZYBI7dFEt00KfHrsymkTblRhRlwkV3LWf-9Sg
eKh4x6Gcy4i2nYk4weMB2wPFXCT_FhLU6LRI_d4B0KkgHjkOtmN5t4jLveIKLN3buD5f7AipuqmkWw7fkTmvhw
zKt5g_5SpZBCTDaqNa1kiD30_Wr6_L3Jk9dvfqyM9neLkOr9eH3gvjQeq-dvJBfEad1tNlxri57Yuc5h7Y7awQ]]
✓ Batch update completed

=== Batch Delete === Batch delete response: map[failed:[]
successful:[1XSvz3_HMzqJno9BgTsT5_5W5U817wazBsM2jEBuXQwUGZevuZYBI7dFEt00KfHrsymkTblRhRlwkV3LWf-9Sg
eKh4x6Gcy4i2nYk4weMB2wPFXCT_FhLU6LRI_d4B0KkgHjkOtmN5t4jLveIKLN3buD5f7AipuqmkWw7fkTmvhw
zKt5g_5SpZBCTDaqNa1kiD30_Wr6_L3Jk9dvfqyM9neLkOr9eH3gvjQeq-dvJBfEad1tNlxri57Yuc5h7Y7awQ]]
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
P4I3wwWuwvAPPEx-928ZvZclX1NdbOGap9uWTW0zBPUMWNBUM-FPyOP-VyRX-zzZ_6PXyNl3zVXUr6PzdCbuMA

=== List Collections === Total collections: 14 Sample collections:
[websocket_test chat_configurations__ek0_bench schema_users_client_py ttl_cache
batch_users schema_employees_client_py schema_documents_client_py
chat_messages__ek0_bench demo_collection schema_products_client_py
scripts__ek0_bench ws_ttl_test users test_collection]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.go completed successfully[0m [34m === Running
document_ttl.go ===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
KobX3rdvW1L2ttWnENC4VHZmYtXJ_uaa97RuzxLt-EG5W6AvJ-6eUAyL1I6O6VDDJ3faVoHo616x912lGCh6YA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 3 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.go
completed successfully[0m [34m === Running websocket_ttl.go ===[0m ✓
Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
7LGjfBIsokCWhXPjQskSv3AaNBYf7FjDhqdAcBNyNN0IWhMpYASiXzVBPlpBJ9Avgr-FgHE8uf1r1s-nESvBIA

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 3 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.go completed
successfully[0m [34m ╔════════════════════════════════════════╗[0m [34m║ Test
Summary ║[0m [34m╚════════════════════════════════════════╝[0m [34mTotal: 7[0m
[32mPassed: 7[0m [32mFailed: 0[0m ✅ [32mGo direct examples complete![0m 🧪
[36mRunning Go client library examples...[0m ✓ Client created (token exchange
happens automatically)

=== Insert Document === Inserted:
map[id:s5wrNZrE0NuevWjmjuicqEJ2Ulweli5K2abm9aXaEkL8-7fOeWdD8zYugHz3SYuB9-Jewaj_zuAY2RQrPRMklA]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:s5wrNZrE0NuevWjmjuicqEJ2Ulweli5K2abm9aXaEkL8-7fOeWdD8zYugHz3SYuB9-Jewaj_zuAY2RQrPRMklA
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found documents: 1

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:s5wrNZrE0NuevWjmjuicqEJ2Ulweli5K2abm9aXaEkL8-7fOeWdD8zYugHz3SYuB9-Jewaj_zuAY2RQrPRMklA
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
opU53uDBvQri5mNY6E6WYC2stN8x9ezdOtOy4V35DAn6BSQWrQsUV_lQZe_8CVeb5WZ78VfmuOFH2-I9_KBMyw

=== List Collections === Total collections: 14 Sample collections:
[websocket_test client_collection_management_go chat_configurations__ek0_bench
schema_users_client_py ttl_cache]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
9T6TaBdeLGLshiiR2mxQ9ZjTqGkSDW9J3pZnNmazyy6MoP61uGJcqe8c53CaJb5tzQWZePPR-aDEd_X-YJ2HwA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
JbnNZYyQMIVnsr2nKR-UOe6FZruvJ7bG3VAxnbCs8FGeQR5j3myzdTmVEmf--yDmxqryRlo4lTd4HugVkWD8zQ

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
6XSFGU2_mPi7VEF3fYLNYPYwt1ouiMa4FiBOyuq2tStVF_meeiU8mQGORx6D8_Qa2cXG9Lh6Gt5PBhddIozV8w

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
rLWOWDo8Md0D2wqwwQBgr6osaYKq9-iiDXKqKa5KxNGHpYiM0LxQwOjLKpi1tqXkbFdq0o3Cb-L0OEFTc3Q8kQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
XTIulD8-zJOCZDUG_73NrBoP2NbE8rQOTNMH9MRYPjnsmlZvR_AgohumDZpQ2-8RTGemZn1rJ_tV3eSyaHfzhA

=== Sending Chat Message === Message ID:
BvljaZn2l2_rWhC1sVkqp8pjfdaoks9_a1hO6itXUtxHCuKDbBzYDCYR8nVbQQjivHHOlj3VmE8aNFNhUMIsVg

=== AI Response === There are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. It
   is priced at $99.

2. ekoDB Cloud: This is a fully managed cloud database service product. It is
   priced at $499.

3. ekoDB Pro: This is an Enterprise edition product with advanced features. It
   is priced at $299.

=== Context Used (3 snippets) === Snippet 1: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:A
high-performance database product with AI capabilities]
id:Bh_LTw6q2VxsT89PZYbD9kMW0gfRjSH97rslsyiHbganDsVOLwqblOgm6pGs_ljniJyKpjfD49k5CBjWi-3FQw
name:map[type:String value:ekoDB] price:map[type:Integer value:99]]
score:0.1111111111111111] Snippet 2: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:Fully
managed cloud database service product]
id:sllbhFoaHzA1IQ6kqhq4gc6LapJ1wgqCqT84RB8CxflXS81DjxUAn50HTCsxjglxO4ATLRo--XQCw88ZaldCSg
name:map[type:String value:ekoDB Cloud] price:map[type:Integer value:499]]
score:0.1111111111111111] Snippet 3: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String
value:Enterprise edition product with advanced features]
id:uHDRQ4pZV0mc_0sIuTtAPJZ9tx5K1eONUaEu-0xxoPUURMQ-WFk-RZSnkhN3lVs_S_XF6j8u4xqN18ZX7qzu3g
name:map[type:String value:ekoDB Pro] price:map[type:Integer value:299]]
score:0.1111111111111111]

Execution Time: 3684ms

=== Token Usage === Prompt tokens: 430 Completion tokens: 78 Total tokens: 508

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
mi7hFPVOAsbqyVH3r6ItfGqqUuRJpQmk9TTtMq28cgDg1WOPfNt7xDphiYZ9nYl--xfnYvgKgExzfowr8g8WBw

=== Sending Messages === ✓ Message 1 sent Response: Based on the context
provided, the product available is "ekoDB", which is a high-performance database
product. It is priced at 99.

✓ Message 2 sent Response: Apologies, but the context provided doesn't include
information about a specific product or its price. Could you please provide more
details?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
kL3hYwS_H9WDoMohVlOUfgc8SiqeyjTeX0wc-7KNA30QC8PA7nnUi05VpnpMtrIfK0JWHomXyHwSFAMAv0W7LQ
Parent:
mi7hFPVOAsbqyVH3r6ItfGqqUuRJpQmk9TTtMq28cgDg1WOPfNt7xDphiYZ9nYl--xfnYvgKgExzfowr8g8WBw

=== Listing Sessions === ✓ Found 9 sessions Session 1:
kL3hYwS_H9WDoMohVlOUfgc8SiqeyjTeX0wc-7KNA30QC8PA7nnUi05VpnpMtrIfK0JWHomXyHwSFAMAv0W7LQ
(Untitled) Session 2:
mi7hFPVOAsbqyVH3r6ItfGqqUuRJpQmk9TTtMq28cgDg1WOPfNt7xDphiYZ9nYl--xfnYvgKgExzfowr8g8WBw
(Untitled) Session 3:
XTIulD8-zJOCZDUG_73NrBoP2NbE8rQOTNMH9MRYPjnsmlZvR_AgohumDZpQ2-8RTGemZn1rJ_tV3eSyaHfzhA
(Untitled) Session 4:
xk1gRyFojsYWBB7kDMWNkw5y4h5JIcVoLN-zoBLR-Ws_UEc2ZU_DROpqO2lL92f4140PY5eFcPvVj8cpfOeyxg
(Untitled) Session 5:
nhwhhgOvM3Cs7T7pPSwLlWkqiCW86qVdGG2kHPlhvy_kUxnHKlipsiOXJ_S4NiIuccj1fcrME6jXHzyY1I9Aew
(Untitled) Session 6:
bEGYtiLgEbT1EbPG8_0FOTynTXtkYIvcZGBaPcKgHVDQ8pSe4ATrA0QDir6A-85pvJpCR4dZe5GYNJPqqZY12g
(Untitled) Session 7:
gCQ0KOMfyF62QuoyABvyWlQg0eTLqa-0V8wAplTvIRrRNnlIywMtHlRzS4i-GM4rSk61UZXenP7tTndJnc_Znw
(Untitled) Session 8:
qG-MclQB4tcPo_IDrpnwKXSccRvWgHAUzM6rY63jpPZp2LRHxq5iaq4Kgn-gNcd_sQydUflDkM8gGmnoenCzhw
(Untitled) Session 9:
6vuPox7U6AzsVb7dniAaHDwJXJG7M4Vq9wwTyOecbl5pUvuf6mZ-Rk5P4p-2VHYZLB9ynnGPi8Cl0-bU_vq3NQ
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
kL3hYwS_H9WDoMohVlOUfgc8SiqeyjTeX0wc-7KNA30QC8PA7nnUi05VpnpMtrIfK0JWHomXyHwSFAMAv0W7LQ

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
943tbxFY6hHKBG3HXk19X-nHPEi6BGSLoNlPmAgBExCX-eDisj1KHNh4l72p9kB*-PO7gseJRiBqRv4vx*-GYw

=== Sending Initial Message === ✓ Message sent Response: Answer: The available
product is ekoDB, a high-performance database product. It is priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
Answer: I'm sorry, but the provided context does not include information about
the price of ekoDB.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
jb5a8ZjKnd5P0u0yjq3ZJZ1JPu9YrAcfqXKC7OQW1B7bDSq69mH1k7NvTSkXRCiqbKBDw8D7CA3n02f_4sShJA
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
1. Score: 0.777
1. Score: 0.762
1. Score: 0.740

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.711
1. Score: 1.496
1. Score: 0.305

1. Case-sensitive search: Found 1 results (case-sensitive)
1. Score: 26.400

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

- Mouse ($0) by Alice Johnson
- Laptop ($0) by Alice Johnson

5. Complex join with multiple conditions: Found 2 users with example.com emails

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✅ Deleted test collections

✅ Join operations examples completed! 🚀 ekoDB Scripts Example (Go Client)

✅ Client initialized

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
OC8-pHM3kINrzorQPj7fOOyS7kC1iRDLtcTObzkJatT5XdfWOvPfQjw4JwwaMNX7zxdWZQlNtFnyCzchI8-MTw
📊 Found 30 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 30 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups map[avg_score:60 count:15 status:active]
map[avg_score:50 count:15 status:inactive] ⏱️ Execution time: 0ms

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
