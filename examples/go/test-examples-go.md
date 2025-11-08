make test-examples-go 🧪 [36mRunning Go examples (direct HTTP/WebSocket)...[0m
[34m ╔════════════════════════════════════════╗[0m [34m║ ekoDB Go Examples Test
Suite ║[0m [34m╚════════════════════════════════════════╝[0m [34m === Checking
Server Connection ===[0m [32m✓ Server is ready[0m [33m === Running 7 Examples
===[0m [34m === Running simple_crud.go ===[0m ✓ Authentication successful

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

✅ All examples completed! ✅ [32mGo client examples complete![0m
