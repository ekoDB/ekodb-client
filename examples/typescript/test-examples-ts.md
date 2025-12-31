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

added 1 package, removed 1 package, and audited 13 packages in 655ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'GUV4y-v5eoRI8MVwNFS5UPuB_VRgzhAn8MVPFgjlPY8WS_zHSJXi1lB02ObklhTEOryY_Xl_gPM93bg3OuXIxg'
}

=== Find by ID ===
Found: {
  active: { type: 'Boolean', value: true },
  value: { value: 42, type: 'Integer' },
  name: { value: 'Test Record', type: 'String' },
  id: 'GUV4y-v5eoRI8MVwNFS5UPuB_VRgzhAn8MVPFgjlPY8WS_zHSJXi1lB02ObklhTEOryY_Xl_gPM93bg3OuXIxg'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'GUV4y-v5eoRI8MVwNFS5UPuB_VRgzhAn8MVPFgjlPY8WS_zHSJXi1lB02ObklhTEOryY_Xl_gPM93bg3OuXIxg',
  value: { type: 'Integer', value: 100 },
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Updated Record' }
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
Collection created with first record: pwJhrT1ED7jqVg7Gc3XO7JkQMsns8iPmXV1rWVRg0W8bbWbULVGRis9F_-E7zjoLRYosUxmxekoC7MDG4AEWyA

=== List Collections ===
Total collections: 18
Sample collections: schema_documents_client_py,test_accounts,ttl_cache,schema_employees_client_py,chat_messages__ek0_testing

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 6OfCKLkFTwCPbr0BaTCnB4DFpY_kkKx4wGLO4dM5tfve8S23mSt-tVKAmNhJe8GGBeDGDRDj-WRGzg0pjHmVIQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: TNB0giVI6Sq6TfEwdo4HhKg7xDeET4lAC58xBKXuI95MgPXZQEdERX-lM7sD9SgMNQgX-6E38C2sQ2jq_tvvhg

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
✓ Inserted test record: No4djAZouzGMIFBqJqgLhr-KC3CSYhbPZDDfo7I-9ku5GXxIbSrDXa6Ngn5rdMXBDfgM_dh3LabGP8S9iV0KDQ

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
✓ Inserted document with TTL: bQrXO3qIwZmaFyAnHDr9VP1GlDQz0fVYYGgXp5xkijx1Au-F7XqG5j064ye6Xo51qHtKAmApdrggTrZL_B8PqA

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
✓ Created session: PbB0LievzU6_hVtp-lUWfT4NojuhwL6hAU38LEKQHxVl0KCI3po3DM5VyzX1szgqiaKtaDXZw27Af0uCVz-26g

=== Sending Chat Message ===
Message ID: uUPib8ApRkmIZvcV60yabsnHbgnBvYKXZ-oAMWLfTLRh0t8eHfTDQLNTjVCJuH3yxcsQJWwMF90AkAgPaiddBQ

=== AI Response ===
The available products and their prices are as follows:

1. ekoDB: This is a high-performance database product with AI capabilities. The price is $99.

2. ekoDB Pro: This is an Enterprise edition product with advanced features. The price is $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price is $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    id: 'Vaqma3DylPIjJPYYHCDZDF-sUQ8Wa1aEeUHNBUPrCVIyAaX1xNv7RDrHcJJ6j6JSbDRmdJTeoD1sGlT5br0gmw',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 299,
    id: '8-oeQHsiajJkqEwmDUPhiDLvKwPxSxrnxTa22PKJgEtXAZus7cUd3BJo3elz2lv1UVcNIOaU-8broc7itr1c9w',
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 499,
    name: 'ekoDB Cloud',
    id: 'jFHbB2LLmu5rJQ-Ql8yUlKiJB5ifFBRTBoRpbl_gmxByT0drPJs6NbQqZ-gXV88-RYEfm0GqmF96EAvFcSVasQ',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2759ms

=== Token Usage ===
Prompt tokens: 605
Completion tokens: 79
Total tokens: 684

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 2ZJ0xtJW6LWyv7QdqgWFao55EsfKQtk5Ufe0AHX76gxDQMf1nUvHgbNmVQ2JEv4UnWoWjHZo1b5P6I2qAnvsqQ

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the product available is ekoDB. It is a high-performance database product and it costs $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: jvgyIyJ1UtK1PsPsix1E_c3H-29WgdtHmH9O0GQ0cck-S6zg8J_OCF-fXTAIBxNYgkzKbs4K05LkzIKI53-6LQ
  Parent: 2ZJ0xtJW6LWyv7QdqgWFao55EsfKQtk5Ufe0AHX76gxDQMf1nUvHgbNmVQ2JEv4UnWoWjHZo1b5P6I2qAnvsqQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: jvgyIyJ1UtK1PsPsix1E_c3H-29WgdtHmH9O0GQ0cck-S6zg8J_OCF-fXTAIBxNYgkzKbs4K05LkzIKI53-6LQ (Untitled)
  Session 2: 2ZJ0xtJW6LWyv7QdqgWFao55EsfKQtk5Ufe0AHX76gxDQMf1nUvHgbNmVQ2JEv4UnWoWjHZo1b5P6I2qAnvsqQ (Untitled)
  Session 3: PbB0LievzU6_hVtp-lUWfT4NojuhwL6hAU38LEKQHxVl0KCI3po3DM5VyzX1szgqiaKtaDXZw27Af0uCVz-26g (Untitled)
  Session 4: VA7G5nZiKrl6H8Ktj8nH23Bj4oSh0YEpFgFWaZxXLiRKU0n1n5G0bE1a4-s4iz-JVycQt-QexpJHbpdRQfue-A (Untitled)
  Session 5: ktjiFm4CMvu9friJvcO-UNMXZW-N5pvOqhSV5J0RQaumxSwqCx_9DOTqv_D4m8xDuwia2e8IlDKi6GMyvMAJ1A (Untitled)
  Session 6: pQWCgFFgYFbFf0-jWzKOK9RLY4tsqicjbx8LxOuYK0LBIYghOzqeIR9HfgO3zC-uTqYpJXGDlQCeUP-hr4dx-Q (Untitled)
  Session 7: MeoMxNtgzw5KEIQyZX_n66uvkZPV8Fq5Dj2SVWOUflZxg7wgntyjQc2cHezCDrKXgAXeh3-ZzT4XJZ1weC8g0g (Untitled)
  Session 8: cCXRhshO2K1u4oEgzhUn5CeenXcQlZ06uX7cjsO_4W1GoVAHNY_mU7IYMPGxynYUv3_wNHDa1sL0B9uJ7os78A (Untitled)
  Session 9: escAHAP-FcRf2PLZuTgFmENQqrE6LEvMLOpY5U5tFaxBmw5IVW1Sydk9FL3pfilwUfCqQCyiCuC8Wv5vC3eE0Q (Untitled)
  Session 10: hyGmrj0Z3bTf09rh4aobr5bED81Zs2p4vAC3-HmLu1uJjUvxMG6N94KSZw2qnG7vBqdW60KHJcX7K5l_eD_Yag (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: jvgyIyJ1UtK1PsPsix1E_c3H-29WgdtHmH9O0GQ0cck-S6zg8J_OCF-fXTAIBxNYgkzKbs4K05LkzIKI53-6LQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: dYHXb3t_WTVi7rFpvpXsRD4DEvsZ6miO4ipFobWcMLqTPG4W-9rF6_LzkE5mVkFtafISOd5hsU5wcELF-Os86w

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, one product in our database is the "ekoDB". It is a high-performance database product that is priced at $99.

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
✓ Created second session: IncVaukhknco3gW5iuJYB_BDSe90f0CVBLIL9PiM0AJRcteop3fxQKgZzziFI7QFlc1cYrYGft3N7SPYZ9SI9Q
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
  1. Score: 25.740, Matched: email.value, name.value, name, email
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, title.value, bio.value, title
  2. Score: 26.400, Matched: title, bio.value, bio, title.value
  3. Score: 26.400, Matched: bio.value, title.value, bio, title
  4. Score: 26.400, Matched: title, title.value, bio.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, title.value, bio.value, bio
  2. Score: 39.600, Matched: bio.value, title.value, title, bio
  3. Score: 39.600, Matched: title, title.value, bio, bio.value
  4. Score: 39.600, Matched: title.value, title, bio.value, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.742, Matched: 
  2. Score: 0.741, Matched: 
  3. Score: 0.739, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.696, Matched: content.value, title.value, title, content
  2. Score: 1.496, Matched: title, title.value, content, content.value
  3. Score: 0.297, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio.value, skills, skills.value, bio

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

✅ Script saved: rlOU-sR5GV73bFI-D82QrWGwJtGpu-PKjBtbghzI_xsynmtb-O77BbBrn5VIXysdVmwwdYIMgedh20YCvpKFvw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: ynew87vbJTC53i-PU1a65zdK8-RlyIRAbv5m75_Zu_FdQHJnriGFbLGkkRhuykDdbGkSHhzO_m5W-AWu-PGcsw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: Pv0YxTf4ugBfXTxs1OxWhzF70MALvenq3oH28k3T5UySYXc5_HAr__NJYbMHPaj1a33sfTE21zb4wdtXHqdUug
📊 Statistics: 2 groups
   {"status":"inactive","count":5,"avg_score":50}
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

✅ Script saved: DccKIJry8qogwS98ul3mf4EWvuFEJm2bZNvYPquCw3h6llGpUluq8rIvO6NP-YmdF76CTEGp5YVnvEAc6q-3RQ
📊 Found 2 product groups
   {"count":2,"category":"Furniture","avg_price":474}
   {"count":3,"avg_price":575.6666666666666,"category":"Electronics"}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"category":"Furniture","count":2}
   {"category":"Electronics","count":3}
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
