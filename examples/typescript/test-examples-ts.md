make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.7.1 prepare
> npm run build


> @ekodb/ekodb-client@0.7.1 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.1 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 836ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'YLk29H1juS2IPSVqUjJczCJp7gZ3B53G8c0ZhREJJJscZWZSzf2HDbPkfAUj-SXu1Nesxfi6nwVtZApLhw0IUQ'
}

=== Find by ID ===
Found: {
  active: { value: true, type: 'Boolean' },
  price: { value: 99.99, type: 'Float' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  created_at: { value: '2026-01-04T06:32:59.457Z', type: 'DateTime' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  id: 'YLk29H1juS2IPSVqUjJczCJp7gZ3B53G8c0ZhREJJJscZWZSzf2HDbPkfAUj-SXu1Nesxfi6nwVtZApLhw0IUQ',
  value: { value: 42, type: 'Integer' },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  name: { type: 'String', value: 'Test Record' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-04T06:32:59.457Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  active: true,
  price: 99.99,
  categories: [ 'electronics', 'computers' ],
  created_at: '2026-01-04T06:32:59.457Z',
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  id: 'YLk29H1juS2IPSVqUjJczCJp7gZ3B53G8c0ZhREJJJscZWZSzf2HDbPkfAUj-SXu1Nesxfi6nwVtZApLhw0IUQ',
  value: 42,
  metadata: { nested: { deep: true }, key: 'value' },
  name: 'Test Record',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  tags: [ 'tag1', 'tag2', 'tag3' ],
  data: 'aGVsbG8gd29ybGQ='
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  price: { value: 99.99, type: 'Float' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  id: 'YLk29H1juS2IPSVqUjJczCJp7gZ3B53G8c0ZhREJJJscZWZSzf2HDbPkfAUj-SXu1Nesxfi6nwVtZApLhw0IUQ',
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  value: { type: 'Integer', value: 100 },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  created_at: { value: '2026-01-04T06:32:59.457Z', type: 'DateTime' },
  name: { type: 'String', value: 'Updated Record' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  active: { value: true, type: 'Boolean' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 5 total records in collection

=== Batch Update ===
✓ Batch updated 3 records

=== Batch Delete ===
✓ Batch deleted 3 records

=== Cleanup ===
✓ Deleted collection

✓ All batch operations completed successfully
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { username: 'john_doe', userId: 123 }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: MTGeaoKRAeftQBRFtozWNpuRyCG9lpLtdQgCN3utnR4Z6zHrota-Ftcgfcv7-6lSSJUEQlENgUoQ12D_6dMjNg

=== List Collections ===
Total collections: 18
Sample collections: client_collection_management_ts,schema_documents_client_py,schema_products_client_go,batch_users,chat_messages__ek0_testing

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: wmMyuQqlVCqrpu9Py2oYSSBtHO980Da4pgvpxxUXMRoV9RVvGAYUtfiJBDHB5UIS_GqI1LSbXti2m2kO1IIrHw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 6atOzEAUo_gFgRWf9JV0mFymWjRFyxTHaAgngralDuD2tzZdWRBvQ5bF1rFdDRlug0u_1vKEclWAu6Cd458rgQ

=== Query Documents ===
✓ Found 2 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: yat_5MBPwjXVfKjnjnPXu4gdDhg1KcWoflMwvgGbWJyd4b8Hk8VjdIEhOeqn_bj4Nlbbbq1Syn9DK5F_WXHtMA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: 0RhoQzRE4gX6aoLUFzQGGraOWvicZ3Wd5IgZJhSDU3QAzvis_h0Kw1XN92jdesuaWrSgQca9ipzhrwdGNFbDnA

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: _Q3z9V4ral49KsLaZCPRhQLTypHlHhLrSiavxOY5Ft_uNeE_kCw0H3ReiELoGkEQ4exN3_SdDk00VedvAMNXdg

=== Sending Chat Message ===
Message ID: 9a7jKYyX8JJvvA3SUB1W3piIfLe0f6jY-Ow-ThSsHjwSM8sZqFt-vbksrhFYQZj9oxUKZn_2neMZMQD8imHXNg

=== AI Response ===
We have three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. It costs $99.

2. ekoDB Cloud: This is a fully managed cloud database service product. It costs $499.

3. ekoDB Pro: This is an enterprise edition product with advanced features. It costs $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    id: 'Eb7DvvgzW7OQ18vX7pckD8kGB6LWncMr-THb50Qwx0ObYexezM1yQnYdP9EshCwFOdJGiE9GDf_zoGgYEFZCiA'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'UkkK2L6tvg7FgqorKVEsAXUjrW0P0-S0l4fZIpGi9MODjvQbmtsy_GjO36KLuBIA-hMmlGRbieBJHHP71CYs3g',
    name: 'ekoDB Cloud',
    price: 499,
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Enterprise edition product with advanced features',
    price: 299,
    name: 'ekoDB Pro',
    id: 'KvFMvM81Jo_hv9A65DNfVQPGgj6n7eRWNB9YnFMXBHGHzImLDtovhe0wHFSfj-EHpm1Hjq_yFrv3RR8pIFhV5Q'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2493ms

=== Token Usage ===
Prompt tokens: 610
Completion tokens: 72
Total tokens: 682

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: T8IFMGiKq7J04dr7uPnsXGcrdC4u0PqP8QH0M0J7Yqvf2zLSLGX3vDPzM5hCYkUqpjCSmpt1LtnB1YQlwhNxQA

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, one available product is "ekoDB," which is a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: tl-UXSRgx8q-H-PAZLxAPKwww-Iyp9gPeLdoHrczSWljeqMZrkpp7PGWkTLxVqc_AXzz6mbIwbrpP2HgodBCcg
  Parent: T8IFMGiKq7J04dr7uPnsXGcrdC4u0PqP8QH0M0J7Yqvf2zLSLGX3vDPzM5hCYkUqpjCSmpt1LtnB1YQlwhNxQA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: tl-UXSRgx8q-H-PAZLxAPKwww-Iyp9gPeLdoHrczSWljeqMZrkpp7PGWkTLxVqc_AXzz6mbIwbrpP2HgodBCcg (Untitled)
  Session 2: T8IFMGiKq7J04dr7uPnsXGcrdC4u0PqP8QH0M0J7Yqvf2zLSLGX3vDPzM5hCYkUqpjCSmpt1LtnB1YQlwhNxQA (Untitled)
  Session 3: _Q3z9V4ral49KsLaZCPRhQLTypHlHhLrSiavxOY5Ft_uNeE_kCw0H3ReiELoGkEQ4exN3_SdDk00VedvAMNXdg (Untitled)
  Session 4: VsNDCHqfYmt3Nf653dwidqprl7O2ITkbnM1QbK71O_rGwgwCv8SHLolCOC7Z0uRjJIeYv3eqkk8KCgzjJ_9_Iw (Untitled)
  Session 5: BAfXzFTQQ1fyRUxlY_HZ0T5wLMlnNo2y5ZrgIRBDG8qIaDl9HssTKUdCrinchUSF9kJ9ZLJIY4Vhom3zZVrw_A (Untitled)
  Session 6: iFUlZ1cqvlI-L5mwayMbK6rRYZHtvP0C1ZkjPMznuqaHhq4fUa3SisMopOjLCPYjrpobzAeyVmHK1WkSnbFhvQ (Untitled)
  Session 7: hIhtRI8S4wACiDzJQuUxJAcNLdKhXI3vCzy7Zx2uvCBYmr-lHtb8oxO_AQ4pU8uXdwKlEqOb0Gt7PX7MWaCmww (Untitled)
  Session 8: BGziR8-8gkMr75-xFjh9ze6J3YD333YrHqF2Rj1S2EGv9Nmw4wg9zAVCZqH3ftp7Snp1WZ2xdzgRPwCwswUE_g (Untitled)
  Session 9: rWDdGI5jTi6Tm3bG4At0VPgu8m0mVWOpMz-31zOK6dkYgequ9swcbxoeAfq68y86Op-7mF-ee8-n4vDcw5fGCA (Untitled)
  Session 10: ckT-vsSlrRFV67BvKFS30PxVBECc1qDvSkEeAZIHWGmSDdCBnk96BxjiLq-EJhaQMWUZQ-4ePmY9X4BckHHBMw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: tl-UXSRgx8q-H-PAZLxAPKwww-Iyp9gPeLdoHrczSWljeqMZrkpp7PGWkTLxVqc_AXzz6mbIwbrpP2HgodBCcg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Qpgf19xdg3pHQPz4KpFRgpRMkO-R0zPG7yIklfd6EmT5MZe3T6_vrKVzu7_FCmEDlDfVJr42eK5ZA3v_Fs9y6Q

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, we have the ekoDB, which is a high-performance database product. It is priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: kwBXnbkiLJGSxp87uoiu6Kt7FaM6KTRboe_ci49g690Wy9iswDroZRN9UzErRYVwFb7T3ldAzdK73WPxF3cCrw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 4

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 3

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== Query Builder Examples ===

Setting up test data...
✅ Test data created

1. Simple equality query:
Found 2 active users

2. Range query with sorting:
Found 3 users aged 18-65

3. String operations:
Found 2 users with @example.com emails

4. IN operator:
Found 2 privileged users

5. Complex query with multiple conditions:
Found 1 active US users over 21

6. Pagination:
Page 1: 2 users

7. NOT IN operator:
Found 3 valid users

8. Using bypass flags:
Found 2 users (bypassed cache)

=== Cleanup ===
✅ Deleted test collection

✅ Query Builder examples completed!
=== Search Examples ===

Setting up test data...
✅ Test data created

1. Basic full-text search:
Found 2 results
  1. Score: 25.740, Matched: email, name.value, email.value, name
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio, bio.value, title
  2. Score: 26.400, Matched: title.value, title, bio, bio.value
  3. Score: 26.400, Matched: bio.value, bio, title.value, title
  4. Score: 26.400, Matched: title, title.value, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio, title.value, bio.value
  2. Score: 39.600, Matched: title, bio, title.value, bio.value
  3. Score: 39.600, Matched: title.value, bio, bio.value, title
  4. Score: 39.600, Matched: title.value, bio.value, bio, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.783, Matched: 
  2. Score: 0.761, Matched: 
  3. Score: 0.760, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.713, Matched: content, title.value, content.value, title
  2. Score: 1.504, Matched: title, title.value, content.value, content
  3. Score: 0.304, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, skills.value, bio, bio.value

=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
=== Schema Management Examples ===

1. Creating user schema with basic fields:
✅ User schema created

2. Creating product schema with text index:
✅ Product schema with indexes created

3. Creating document schema with vector index:
✅ Document schema with vector index created

4. Retrieving collection schema:
Schema fields: [ 'age', 'email', 'name', 'status' ]
Schema version: 1

5. Retrieving collection metadata:
Collection has 4 fields

6. Creating employee schema with all constraint types:
✅ Employee schema with all constraints created

✅ Schema management examples completed!
=== Join Operations Examples ===

Setting up sample data...
✅ Sample data created

1. Single collection join (users with departments):
Found 2 users with department data:
  - Bob Smith: Sales
  - Alice Johnson: Engineering

2. Join with filtering:
Found 1 users in Engineering:
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data:
  - Bob Smith: Sales Manager
  - Alice Johnson: Senior Software Engineer

4. Join orders with user data:
Found 2 completed orders:
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails:
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
🚀 ekoDB Scripts Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: M3eaDszLi_h_WVzd57NfSuQk5fRHEPAAvAgdf7YGg47Z579SjeEBfkfSJ_XaW_UPN5SasroPIe6BYkEDksZKRw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: DULeK047uGNjIVCBCxK8fJGwEdKW_qRQXSxoeg76xg-XOohcyQxUNCrId1jQ_5BcqdF3vI0HLcfBN4u3uOC33w
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: pyu6_c0SogozlSR0ZrODHz8XwR3D7c8NLwVLjUnw4w9olA9PXpLdk6gNJD8Hx5q4j4cOc0x0Ost_hx0IEmH9dQ
📊 Statistics: 2 groups
   {"status":"inactive","count":5,"avg_score":50}
   {"count":5,"avg_score":60,"status":"active"}

📝 Example 4: Script Management

📋 Total scripts: 18
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: HoUFwxbkjCK-gTA8lKt5b1qL1wD6Z6CbCxYt-ePBUGnowPkv2HqV9RZ3aIRq4cGcT2FdnLKkLWg_6oFmFTXIUg
📊 Found 2 product groups
   {"category":"Furniture","count":2,"avg_price":474}
   {"category":"Electronics","avg_price":575.6666666666666,"count":3}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"count":2,"category":"Furniture"}
   {"count":3,"category":"Electronics"}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: Script with Parameter Definition

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Script saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's Script system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Script management (save, call, delete)
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: YcefyFgxatVBf1xzVKN_EbfIbhrar1BfspCR4uKFA7qXBp7_lZiGKK1AytENbTdC9QN1aArU_qcLBPTHt10OPA
Created Bob: $500 - ID: FplAbMOmvrY4oWsvk-d5FX9vQD0yYzPd22JLH8-uOygukK9UpkBpEp_YjRcdh1XsDoXuxUONIMNIWUo7VNtaqQ

=== Example 1: Begin Transaction ===
Transaction ID: 555ade7f-c448-452b-bf2a-e5d9aa3bf848

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 0af2a318-716c-4b70-99b5-9b84a2b17258
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mTypeScript client examples complete![0m
