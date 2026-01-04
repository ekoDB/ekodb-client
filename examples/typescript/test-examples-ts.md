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

added 1 package, removed 1 package, and audited 13 packages in 831ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 's5gozbcrRYw0OKLDLni8yoZJlBVgA_sySoTeQs7Kbt1uD6kPzuZzSndiikmQ6z2jzAbijQ4rv5WGJ2QWalnR6g'
}

=== Find by ID ===
Found: {
  value: { type: 'Integer', value: 42 },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  name: { value: 'Test Record', type: 'String' },
  id: 's5gozbcrRYw0OKLDLni8yoZJlBVgA_sySoTeQs7Kbt1uD6kPzuZzSndiikmQ6z2jzAbijQ4rv5WGJ2QWalnR6g',
  price: { value: 99.99, type: 'Float' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  active: { type: 'Boolean', value: true },
  created_at: { type: 'DateTime', value: '2026-01-04T19:38:07.497Z' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-04T19:38:07.497Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  value: 42,
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  categories: [ 'electronics', 'computers' ],
  name: 'Test Record',
  id: 's5gozbcrRYw0OKLDLni8yoZJlBVgA_sySoTeQs7Kbt1uD6kPzuZzSndiikmQ6z2jzAbijQ4rv5WGJ2QWalnR6g',
  price: 99.99,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  active: true,
  created_at: '2026-01-04T19:38:07.497Z',
  metadata: { key: 'value', nested: { deep: true } },
  data: 'aGVsbG8gd29ybGQ=',
  tags: [ 'tag1', 'tag2', 'tag3' ]
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  id: 's5gozbcrRYw0OKLDLni8yoZJlBVgA_sySoTeQs7Kbt1uD6kPzuZzSndiikmQ6z2jzAbijQ4rv5WGJ2QWalnR6g',
  created_at: { type: 'DateTime', value: '2026-01-04T19:38:07.497Z' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 100 },
  price: { type: 'Float', value: 99.99 },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } }
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
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

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
Collection created with first record: yQ-WmpQPON4JkTZGH0lenjIiZcKVesjQan-G1QPyAWU8iDW7sxNdEmTLxgvSdKm-RzjEuDF9VAB8NzP2mAtUUQ

=== List Collections ===
Total collections: 18
Sample collections: ttl_cache,client_collection_management_ts,schema_products_client_py,test_accounts,schema_users_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: Kn3Avmw4m1Jh8os38iNPm5uJ_eAFAGw8Ft2DIIeLgCWk3i7ekjKJnyCA2F9w04qnNFoKwiYi3-2BdQVFKabsFw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: ahFWEXg2GX9H9PhwsAF2OKJtAIkq3ssUnk9bzTZNlbOkbxHe7ThbJ5hxS1qHnwNyHR13zO5MbswDeUIpgcftSw

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
✓ Inserted test record: hXhTWMwO1ty8AboUtt4sO13vD5gZhnxfg3_0iyP1IJqa11kiRIGRoyXrJG85opdFjR0QBp08ghCVkL9gITr4LQ

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
✓ Inserted document with TTL: p6QHKA8uzgD8Tjj6k_eBOzxFV3lxeNWWLAjRPQkNsDeEAXAwDvTgFo7avBXBpTU37kDzCTRqUoIhTwOm34IdEg

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
✓ Created session: hSk2g2QahlOh5JK78hvpCH0Ibw4bL1vCzP4Xmzfooqre7n4bb5OBIsJoS2T-c9clWa-X5x3r3c6M2JG_6Skt_g

=== Sending Chat Message ===
Message ID: 78RzaPWRS6Wdgs_FpIqvbAvagNax0bbdgnIh8_MjX6xCdnOCFsqqUMIZe0bMPgFLMPRNsDKnJUmsXbZOAgpKFg

=== AI Response ===
Based on the information available, there are three products:

1. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

2. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

3. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Enterprise edition product with advanced features',
    price: 299,
    id: 'x9LUVmKBTptoYHerHD6WQTVcfsbde2ymqD07KiyyTk11YdLw5AUIS4K10sh4vpro5Qz_jgiXqdRfZ_3jMeMN9w',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    id: 'fa5WhcLB0TIZTZYTlJu06wXfJjpR1G8Al24DPpNhu86F5msX0tyDIvbwoRPZopESYndysyAtAc9Lpkkk5aMTMw',
    name: 'ekoDB Cloud',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    id: 'hc_3oHelXpYVuy6ZNMpadMuPxpkmDZoRUrFaLWkizGqEOgWZrlfCeevbiWG4YVE1gW7miB2yFrd3IjzwO3h4Ng',
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3139ms

=== Token Usage ===
Prompt tokens: 603
Completion tokens: 89
Total tokens: 692

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 8R8msFD8d66a8y7SauAyRzK9AoLBPfris-0M9ycc_ZNKBSZ1C24L2D-g7IX0BDi9Ra7_i2RtubB_-a1aO7Uz4Q

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the context provided, the product available is "ekoDB", which is a high-performance database product. It is priced at 99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: fEbUrzD93oaFZSCHpXH9BSCQGd5STK1xkmmiAXpK3B4Zm7xZMwnrpwgxIXgphDoz0uBmjdZVUMaKss58TbWA6A
  Parent: 8R8msFD8d66a8y7SauAyRzK9AoLBPfris-0M9ycc_ZNKBSZ1C24L2D-g7IX0BDi9Ra7_i2RtubB_-a1aO7Uz4Q

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: fEbUrzD93oaFZSCHpXH9BSCQGd5STK1xkmmiAXpK3B4Zm7xZMwnrpwgxIXgphDoz0uBmjdZVUMaKss58TbWA6A (Untitled)
  Session 2: 8R8msFD8d66a8y7SauAyRzK9AoLBPfris-0M9ycc_ZNKBSZ1C24L2D-g7IX0BDi9Ra7_i2RtubB_-a1aO7Uz4Q (Untitled)
  Session 3: hSk2g2QahlOh5JK78hvpCH0Ibw4bL1vCzP4Xmzfooqre7n4bb5OBIsJoS2T-c9clWa-X5x3r3c6M2JG_6Skt_g (Untitled)
  Session 4: _NL6OYBqQLoEupcizbL9l5texJ70hYuEDsy6eTjOgiLN5oGcWPOY8SXyDAtM8qpA8loeptso8XXiGosWevuMEQ (Untitled)
  Session 5: Us_Q6dEoGIzoWbZSxPo30f5vvCQWxznglrknYiDn6iY2PXbYFDs2NCf4h1Mm5xImmYJBDFwmEMjlqVTkY0-A_A (Untitled)
  Session 6: 6c6I4tbKn4s5HXGqxA8u3X9GUS68Q-_arUUTv8flyLDZAgHVoQH7Il1ZjmGO5l1Wcc0-GL1_hEctqKojwhRU-A (Untitled)
  Session 7: jmpddhHFZ8sTVNF70g7Qmqj7SExneQLGhcthObJr-pu8acdXHAXCRIuxUGP_rkBZ9_M_cZt0hyqCSWJLO55HBw (Untitled)
  Session 8: rViBhRDXhRbwLJ7q7HgIWAR-WzJpWkLXUnD6trKYXIrKNfa6L9gDBL9gBxUojcl3yMP0zwN4IH5XVgtxHLKaXw (Untitled)
  Session 9: 75BwxYE7jLzRVr74SLD1UtuvPgDxlgpFIhcnXtSRADrscb60rK4_7_n7mx6Ip0Q5uiS2ZECQcBR6Z8uuZumVcQ (Untitled)
  Session 10: QRhmpERBtntusi1QZ2IH-CYZz8YqvGsUxP5rSLCZFUJV6k9Fh9IzpoKcyUkay5qYqXNuTe4_QsJe4LM3bnFRfg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: fEbUrzD93oaFZSCHpXH9BSCQGd5STK1xkmmiAXpK3B4Zm7xZMwnrpwgxIXgphDoz0uBmjdZVUMaKss58TbWA6A

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: DEwJpm7_VLd-G5G8GionxiGwXzGCjNRsrn4xLV4j8mt4BGRpu17rTpLTSGqpWWnxG135FDFD7r8yMNJ-B1vaow

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the product available is ekoDB, a high-performance database product. It is priced at $99.

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
✓ Created second session: TXfM7lb2Ru0Uui3UoPSjYMPWbezndbeekYaKgHumU34_OQN-eEHe7s0HjV1ST93szUaIXFNTF9ls9VKDNS80ug
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
  1. Score: 25.740, Matched: email.value, name.value, email, name
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio.value, bio, title
  2. Score: 26.400, Matched: title, title.value, bio.value, bio
  3. Score: 26.400, Matched: title.value, title, bio.value, bio
  4. Score: 26.400, Matched: bio.value, title, bio, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title, title.value, bio.value
  2. Score: 39.600, Matched: bio, title.value, bio.value, title
  3. Score: 39.600, Matched: bio, bio.value, title.value, title
  4. Score: 39.600, Matched: title, bio.value, title.value, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.742, Matched: 
  2. Score: 0.724, Matched: 
  3. Score: 0.723, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.690, Matched: title.value, content.value, title, content
  2. Score: 1.489, Matched: content, content.value, title.value, title
  3. Score: 0.297, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio, skills, skills.value, bio.value

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

✅ Script saved: kD0yrik_G3T_YjLMp3CC1j4naRIKBpYMTsN9_ldenQ6Q0SkzDKet-xbPSGItQbtwsk8knE4wCgQKDUj3Rxekyg
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: dPJlZTzxJxfSQ0l1c-ito20QAgwhrI2erJpL0mqD60hlY5qIKoVB0zGPcs8KbIcABaTJsVWN6UJID5Dm6UuLFA
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: Yz6nEzuKy4NumCLlU_qalBrLi40FRQv3h0qynelRvFF6HIuhqLa7F8ouO-BGcozm5Pv25p5-uVOdWundR4x31w
📊 Statistics: 2 groups
   {"count":5,"avg_score":50,"status":"inactive"}
   {"status":"active","count":5,"avg_score":60}

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

✅ Script saved: gJToRnWRE1pKO0bnfu4-QrvgZR8Q_FCDZOwVw4qaIj3VLWlePvKpgGS8J9moI82WV4O2XxdEUt7WKynFeq7tlg
📊 Found 2 product groups
   {"avg_price":575.6666666666666,"count":3,"category":"Electronics"}
   {"category":"Furniture","count":2,"avg_price":474}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"count":3,"category":"Electronics"}
   {"count":2,"category":"Furniture"}
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
Created Alice: $1000 - ID: voTZlnu9o_WKTVRYqi7JrLgMdDoQwZZzUzELRX1_ITAasIZVbHWONxfI9W43-m7zOeoCesvUUQCrawhN-yWDhw
Created Bob: $500 - ID: 4LHS4F79-584nfaqqV3unkxWJf3Ho7eVxZzzQIcaIvTMSAUx8NMFENH11M0lxM8FbBgLdQz03MWhPvTThSjx0w

=== Example 1: Begin Transaction ===
Transaction ID: 7c377337-cda0-4be7-a82d-ac6b7fdd24be

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 0778703b-547f-45e7-b505-9cc1932ad0a0
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mTypeScript client examples complete![0m
