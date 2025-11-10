make test-examples-typescript 📦 [36mBuilding TypeScript client library...[0m

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
[32mTypeScript client examples complete![0m
