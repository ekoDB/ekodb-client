make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.5.0 prepare
> npm run build


> @ekodb/ekodb-client@0.5.0 build
> tsc


up to date, audited 7 packages in 1s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.5.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 636ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'RwtfcKoNUr_fWiEJwI1KkdtHtPKGcVIee5nqphru3gkIGT4isOhpW8T7tkPS3vMMK8w0nxFa4jhqJDb1jN8naA'
}

=== Find by ID ===
Found: {
  id: 'RwtfcKoNUr_fWiEJwI1KkdtHtPKGcVIee5nqphru3gkIGT4isOhpW8T7tkPS3vMMK8w0nxFa4jhqJDb1jN8naA',
  value: { type: 'Integer', value: 42 },
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Test Record' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  id: 'RwtfcKoNUr_fWiEJwI1KkdtHtPKGcVIee5nqphru3gkIGT4isOhpW8T7tkPS3vMMK8w0nxFa4jhqJDb1jN8naA',
  active: { type: 'Boolean', value: true },
  value: { value: 100, type: 'Integer' }
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
Retrieved value: { userId: 123, username: 'john_doe' }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: VqcRojpDRxFQKPEwEHPniRelbuV_VLgMU9gmA4-E8Y-2qC57oLqvfsMLbRStvvXRoCb_B8HZ_RjAYLBQmUb_BQ

=== List Collections ===
Total collections: 18
Sample collections: schema_employees_client_py,batch_users,schema_products_client_py,test_collection,schema_users_client_py

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: fzI1WdkCgYnxfA7XE38XgIi_us9JSKk-yVJSS9iykg0iSbIvVikUr0PxvgsSBqEZKIbJWepdIbtOjhiaraaROA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: UARZYU1Mp-RHRiDRZRDVxZndlcfqkjBbMvnuEHmk3qtr_ruY2v-z_-Kk-h4RuVNch-uoGL5R-gXNMT8yjdVniA

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
✓ Inserted test record: RvrdMyjxRJMOPLJ8R-MxcpzP4p5WgRm6AD_NQISarNL52I1IsYiPKCEjY58pbsUn4VXM5j3QvYDXaYaTLuh-mA

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
✓ Inserted document with TTL: 5XGiAhp8FLl7M2MhevFYTCNzyDgRcSNyrFHjZGbLDqHIRH-w316t9lqAF-f3nbIiKt7zJvfyVLMX9KxEsLdE5g

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
✓ Created session: IzHf7BsJ4jQQZYkHnASDuH6Q-GE-MrC6PNjI4rRQqbXLf4vAglyIV2uiNJpPHY_tBwbJPi64ny1Gg_fZpcJMbg

=== Sending Chat Message ===
Message ID: vB44l134IKgUcKoY2AGx4hMhHpQpUJCeo8wYTKygpksPhH9UCX__jCRqemC0w_gYTJjGYon8GMsGXYHcDAxyWA

=== AI Response ===
There are three products available:

1. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

2. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

3. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 499,
    name: 'ekoDB Cloud',
    id: '8e9Lj1d4zgc1U9nahSV6Fo9D15oOZjNDqIK7c3DSmjEezDHrfHIfmx3KW8fkf4iTQSLJ-GaCI3bOgIeR4JXA0g',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    id: 'uKEPWu9zDSE7S4ZuBWB8mRJ9_stN9Z44_Dgi1kTYBJAyZf20CEF-gj7IzoD5sVa82_NB5PeoT-I9bZKrwA1o_w'
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
    id: 'KhnBgp13MWwbxUBYhbysrm9BBS4QfFXbLUChUrjGWC1TMTyEmP8AOv0gV7JC1FUPt9A7XEDMytidvzFJcQwq1Q'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2860ms

=== Token Usage ===
Prompt tokens: 609
Completion tokens: 84
Total tokens: 693

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Ac3k0yzskAnHczgWYQ4h5sEGvcZAe2ABZHX2z1V5Lxw7ftnRDcNl5CDJM3fIm7Lhi5HoOIOk0NCFC40S6wvPmg

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, one product that is available is "ekoDB". It is described as a high-performance database product. The price for this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: wItvB5T0wCB0DDfREuxr_JBLkqb3Ixu6UhXub0iL3kSNTnuoAJ4inZeCZrXsddoferEzCPYMcErnH4nusH9_lw
  Parent: Ac3k0yzskAnHczgWYQ4h5sEGvcZAe2ABZHX2z1V5Lxw7ftnRDcNl5CDJM3fIm7Lhi5HoOIOk0NCFC40S6wvPmg

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: wItvB5T0wCB0DDfREuxr_JBLkqb3Ixu6UhXub0iL3kSNTnuoAJ4inZeCZrXsddoferEzCPYMcErnH4nusH9_lw (Untitled)
  Session 2: Ac3k0yzskAnHczgWYQ4h5sEGvcZAe2ABZHX2z1V5Lxw7ftnRDcNl5CDJM3fIm7Lhi5HoOIOk0NCFC40S6wvPmg (Untitled)
  Session 3: IzHf7BsJ4jQQZYkHnASDuH6Q-GE-MrC6PNjI4rRQqbXLf4vAglyIV2uiNJpPHY_tBwbJPi64ny1Gg_fZpcJMbg (Untitled)
  Session 4: EN1u8WYJ-XYaHi9TyUf54f-TTSu0EMrkz__RCp9y29vsugdqMwNrgyEQPBPnox61oymGm6v6jMBP1LKSqwQpoA (Untitled)
  Session 5: Nl1MXe_ym3CqGP0-zV4n4927SZunPuOrsjYNzqtKLYRSh1xVU96TXy7VOfn9nmXIpAN3UJcY80zW23qzl_fKhg (Untitled)
  Session 6: v2sHPzLi65qdQ6hf-Js9BfbjCdw237_ldwjv4O9gOAJak9wodMbphJdBNk3fkb5WCgpDQ8I3EbhOABOA8t5SbA (Untitled)
  Session 7: KVh-vuJkxdu82QEem9mEJqlsehZ-MC_e1SnOJ7Xgd_lrHTjQZhpjlMgkO11L2jZt3heiLfxZmTU0K21jAkDofA (Untitled)
  Session 8: wm2yusSL9We4iCWtVQ8OgiRhxOTYG68-HqstgTHWeEx7_p2tmw4Wp59ymCN4bMg5hHgv9tlwAeuZZYkvGMQJiQ (Untitled)
  Session 9: _1CPODs-8nj298P5KnJz-t7BEQVsQOo6EyCog69m8gTNz_pANx0bruuIZ7qnAoLYTGBamNY1ZO3vaCNcVPD0Wg (Untitled)
  Session 10: hy9LUO2nl-RZUbTZUgrqYDj1mubOHylpZk3swm78FojI3ne4KfoLyDAjFLBhNK3WVW37L9ChwSPfRWPCGo3Ezg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: wItvB5T0wCB0DDfREuxr_JBLkqb3Ixu6UhXub0iL3kSNTnuoAJ4inZeCZrXsddoferEzCPYMcErnH4nusH9_lw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: rbT6xWmWKC2HItnoYS6kigTi5-vXWEtSo0--BG9dkPxgFAfAR-28PXgMV8WXQCi4temkscrMLutdtrBGWFvIiw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, one of the available products is ekoDB. It is a high-performance database product. The price for this product is $99.

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
✓ Created second session: 5gbl1VA8jEF5e8MibX2ZTwQwzLS9ELlWLLDovjgR9ZdY8veJbNXe8C4tBix4KaN1ZUc_1sMkPHgPJip88luaEA
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
  1. Score: 25.740, Matched: name, email, email.value, name.value
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title.value, bio, title
  2. Score: 26.400, Matched: bio.value, title.value, title, bio
  3. Score: 26.400, Matched: title.value, bio.value, title, bio
  4. Score: 26.400, Matched: bio, title, title.value, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, bio.value, title, title.value
  2. Score: 39.600, Matched: title.value, bio, bio.value, title
  3. Score: 39.600, Matched: title.value, title, bio, bio.value
  4. Score: 39.600, Matched: bio, title, bio.value, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.753, Matched: 
  2. Score: 0.747, Matched: 
  3. Score: 0.717, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.687, Matched: title, title.value, content.value, content
  2. Score: 1.499, Matched: title, title.value, content, content.value
  3. Score: 0.301, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, bio, skills.value, bio.value

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

✅ Script saved: u4dH2OjN4u7UcTsjlHazzvVdAhoNofiVMZn7G8iNAybf0siIMyth6zl-K6FeQM4wpl7wzkv99zR8eI4vzGGUaA
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: lnGE4IV9ViLzIEGcYeI11x1XNTsVipTEaTBFln1vNno3AJyTqEZX0fS6L95-uc9R_uY3ybEbTyEwJvvwgAJzlw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: p5DMInLi8KLBI9sz5Z4qrgM4mef0M8nKyVIfMkVb3LznA0r3UoAtGDEzZvihi8wkDRoEGKkcnHk8CYlLOxxcqQ
📊 Statistics: 2 groups
   {"count":5,"status":"inactive","avg_score":50}
   {"status":"active","avg_score":60,"count":5}

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

✅ Script saved: 2mBau_CKOBJ9O5y2PcJ7CgcdU6B1X6tpML9cSj0iIjCRYK9_ZUG0nIwEtU-1Y1zj1GQYOyYiRcnNEgVXWP8cnw
📊 Found 2 product groups
   {"category":"Electronics","avg_price":575.6666666666666,"count":3}
   {"avg_price":474,"category":"Furniture","count":2}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"category":"Electronics","count":3}
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
✅ [32mTypeScript client examples complete![0m
