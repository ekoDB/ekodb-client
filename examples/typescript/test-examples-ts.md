make test-examples-typescript 📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.3.0 prepare npm run build

> @ekodb/ekodb-client@0.3.0 build tsc

up to date, audited 7 packages in 1s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.3.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning TypeScript client library
examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 664ms

3 packages are looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'FN-yR2DyqXfQx9THq4nqq2EyeHHnQ4Q9F2PQgbg-h6YVBaymjnSDb0G4bp61RdTTDHjUt9go8GtIQ9SByvB8MA'
}

=== Find by ID === Found: { name: { type: 'String', value: 'Test Record' },
active: { type: 'Boolean', value: true }, value: { value: 42, type: 'Integer' },
id:
'FN-yR2DyqXfQx9THq4nqq2EyeHHnQ4Q9F2PQgbg-h6YVBaymjnSDb0G4bp61RdTTDHjUt9go8GtIQ9SByvB8MA'
}

=== Find with Query === Found documents: 1

=== Update Document === Updated: { name: { type: 'String', value: 'Updated
Record' }, id:
'FN-yR2DyqXfQx9THq4nqq2EyeHHnQ4Q9F2PQgbg-h6YVBaymjnSDb0G4bp61RdTTDHjUt9go8GtIQ9SByvB8MA',
value: { type: 'Integer', value: 100 }, active: { type: 'Boolean', value: true }
}

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

=== KV Get === Retrieved value: { userId: 123, username: 'john_doe' }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
ijS3h5EUQcfvr-rGbA7IhpEDBWdPUdTYQLW0bBFAxXngBiZUGvdcDy07iV1nRSkN4fqv3bGobrjYHHrr97erjw

=== List Collections === Total collections: 18 Sample collections:
functions**ek0_bench,websocket_test,schema_documents_client_go,chat_configurations**ek0_bench,users

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
eFjRSNYZWn_MCTAgAQXI6hJ5pGwH6V34j2riv40EY6RyS_iK2Dc-aTnD_yTU0-WagCi_kY_3rtuAKYjl53TTyA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
535RSWKAflr7UKXjVrNgOAPnsIrOWcnzFCJSHrVRCNpVgCsX8u0W5YITAaGpb_P7nmwMqfzx3lWyp9wVuNqxbA

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
3SVD8501Zy7kyPA5lMkM_Anbfc871V3QlcSOW1wWftJtYvOR6LmJ4_WGbQqnHljbUT5-b5_LyVHj49O30Bzqjg

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
c0Zf3yavoi9bGt7mWG2-DZppiJwTKJ5i_eft7-ETe3R-AqqpHMPPkzlie6CP9h5DNgdnui1LYDmIjoKZ-dZQhg

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
4QZi*e93dhsh0eI70pnl6p1uM3XXy6gaVIoASTVXKOYxNDUrRZt6PT0N8OgJ03RZg6_X8IgUZy4RPSh4*-2ymg

=== Sending Chat Message === Message ID:
H2lWqr54E8NSfWKiRneDgBRZbJ19HasSd8w_Go_yCv60IK6OPriWH7khIPrUfijUHNZBws9IGGgwhMkkLfVxog

=== AI Response === There are three products available:

1. ekoDB Cloud: This is a fully managed cloud database service product. It's
   priced at $499.

2. ekoDB: This is a high-performance database product with AI capabilities. It's
   priced at $99.

3. ekoDB Pro: This is an enterprise edition product with advanced features. It's
   priced at $299.

=== Context Used (3 snippets) === Snippet 1: { collection:
'client_chat_basic_ts', record: { name: { type: 'String', value: 'ekoDB Cloud'
}, description: { value: 'Fully managed cloud database service product', type:
'String' }, price: { type: 'Integer', value: 499 }, id:
'F-jUz3rA53afXOJkN5C2ugpLP-0y9A9LIeJlkMfYNMZ1wzdA0JhX4Cm1FjdB-oleAQOIrIGPnNwjLaBEo30XhQ'
}, score: 0.1111111111111111, matched_fields: [ 'description' ] } Snippet 2: {
collection: 'client_chat_basic_ts', record: { description: { value: 'A
high-performance database product with AI capabilities', type: 'String' }, id:
'e53hdPphp5Y_bd0yFBNNEQPFdijsrMjCTvbLrXUHavgpdGPHS1vpuUbbLi1nchzqvJMLHv1C2Q8AqMCOg2Fcfg',
name: { type: 'String', value: 'ekoDB' }, price: { type: 'Integer', value: 99 }
}, score: 0.1111111111111111, matched_fields: [ 'description' ] } Snippet 3: {
collection: 'client_chat_basic_ts', record: { price: { type: 'Integer', value:
299 }, name: { type: 'String', value: 'ekoDB Pro' }, id:
'PHZLHbvg3WfYK5xjG8KJ5aSl7r-WYcyqf4QQQO-QorVhX06WQWYnVUUvLYdZRdnX-JG6keI5tVY-htSBhqR_fw',
description: { type: 'String', value: 'Enterprise edition product with advanced
features' } }, score: 0.1111111111111111, matched_fields: [ 'description' ] }

Execution Time: 2252ms

=== Token Usage === Prompt tokens: 386 Completion tokens: 78 Total tokens: 464

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
58N69ci6R0DjA4d3YVjcvHSpZhx65SmyadGVnWzfynELu3R1VVgd01F-cekrhY4m6V0RqYFq9fIDHnSVrVk9Qw

=== Sending Messages === ✓ Message 1 sent Response: Based on the provided
context, the available product is a high-performance database product named
ekoDB. It is priced at $99.

✓ Message 2 sent Response: Apologies, but there's no context provided about a
specific product to determine its price. Could you please specify which product
you're referring to?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
to4YhbLL1ypv9Wwl3GpGvTAwsFLTW0gU-zuf_CYjzCXadIm6s9_38-M3u7hXRXyG9tU-SGMXwgJEVxLknyYVew
Parent:
58N69ci6R0DjA4d3YVjcvHSpZhx65SmyadGVnWzfynELu3R1VVgd01F-cekrhY4m6V0RqYFq9fIDHnSVrVk9Qw

=== Listing Sessions === ✓ Found 10 sessions Session 1:
to4YhbLL1ypv9Wwl3GpGvTAwsFLTW0gU-zuf*CYjzCXadIm6s9_38-M3u7hXRXyG9tU-SGMXwgJEVxLknyYVew
(Untitled) Session 2:
58N69ci6R0DjA4d3YVjcvHSpZhx65SmyadGVnWzfynELu3R1VVgd01F-cekrhY4m6V0RqYFq9fIDHnSVrVk9Qw
(Untitled) Session 3:
4QZi_e93dhsh0eI70pnl6p1uM3XXy6gaVIoASTVXKOYxNDUrRZt6PT0N8OgJ03RZg6_X8IgUZy4RPSh4*-2ymg
(Untitled) Session 4:
64PqDH2-9s1ejYU4yAFAXB_BEdFHsAVMRbqhnMnUC9foiNvfmoJkQqgfj56sJy2lfEL4SB5P7oH3LqrbCKHSGA
(Untitled) Session 5:
eITxpP2JZEN8PKiXQdkKnmf6hvhhJGG4kb_gPI0fi4-bk2dFMoB0MMGldYqWbFSohyoEqD83yBsv1dCH9WoWEA
(Untitled) Session 6:
oPrnM6kivNtaLzP2WoGhJJ_SLoj3tHUyy2jjubt0TOY3M_7sfOSazEKacZYTNtFAvWaXtCZfyhz53pBMMU5ADA
(Untitled) Session 7:
lAHOXEH0Vn_UaU7jgvyRgTYJAwZEfq-86R88IureusiszLg6fj3OIgdsy6hU1Z29liPpbFoRXma4WbfCp7IkDg
(Untitled) Session 8:
21OnPz3iUFCAewjLBksLm7apObROKYm2hbandIHd4apRXOgAqWHWU4LltkLp14mGsGQ6y_YKWiy9CVPC6LCNew
(Untitled) Session 9:
f-eT1zQ7ClDPwYDBZ992i0813Uo2z174-H8iquiQ8whjYhSChArtxUjmeRrWF8OvsZ378-nfgD74zHcId6rURA
(Untitled) Session 10:
6YIRrX0Nw2WBJcMCmpa--QKGb8NaTSFlXRB270pQqNBrlt02ojr7tXiJzdLdk7ZEnlBZo8f2JPHTeDJF5GkUnw
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
to4YhbLL1ypv9Wwl3GpGvTAwsFLTW0gU-zuf_CYjzCXadIm6s9_38-M3u7hXRXyG9tU-SGMXwgJEVxLknyYVew

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
5ZWDXaqks_WRjzgSC_NShlE94p43Wj17yM0rcmhDBVU32ZDqzNk6qt6VZyybqVIpGAZRfELSdCg3ZVSYIeJMTw

=== Sending Initial Message === ✓ Message sent Response: Based on the
information provided, the available product is a high-performance database
product named ekoDB, priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
I'm sorry, but the context provided doesn't include information about the price
of ekoDB. I'd recommend checking the official website or contacting the sales
department of ekoDB for the most accurate and up-to-date pricing information.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
lIP5KOzTPzvMR1uaQ4cbkgoc18Vy2CqKeO9F0gAYVN0ZXDQ-yyNZdcOCjNCHLUJ0-3GfLFo0wIYD_oB537TgqQ
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
1. Score: 25.740, Matched: email, email.value, name, name.value
1. Score: 12.540, Matched: name, name.value

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: bio.value, title, title.value, bio
1. Score: 26.400, Matched: bio, bio.value, title, title.value
1. Score: 26.400, Matched: bio.value, title, title.value, bio
1. Score: 26.400, Matched: title, bio.value, bio, title.value

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: bio.value, title, title.value, bio
1. Score: 39.600, Matched: title.value, bio, title, bio.value
1. Score: 39.600, Matched: bio, bio.value, title.value, title
1. Score: 39.600, Matched: bio.value, bio, title, title.value

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio.value, bio

1. Search with stemming and exact match boosting: Found 1 results (matches:
   work, working, worked)
1. Score: 13.200, Matched: bio.value, bio

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.744, Matched:
1. Score: 0.735, Matched:
1. Score: 0.727, Matched:

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.691, Matched: content.value, title, title.value, content
1. Score: 1.494, Matched: title.value, content, content.value, title
1. Score: 0.298, Matched:

1. Case-sensitive search: Found 1 results (case-sensitive)
1. Score: 13.200, Matched: bio, bio.value

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

✅ Join operations examples completed! 🚀 ekoDB Saved Functions Example
(TypeScript)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved:
nwaUpSdPgajavxCaXeCJ82MvooKYl8jJqBrH3rttCh6Yt8F5cs_MNsTyCJD9CRmrWe7H-xsd7EybeKShFXWHLw
📊 Found 40 active users

📝 Example 2: Parameterized Function

✅ Function saved:
K1LauOyTGQitW-plvawVxOJNaC-KSy6BIGfF2E5xC52IvvrZBG6uFi9gqWMvMa7Q05oMoHcbU7gJMySzCzvKxA
📊 Found 40 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved:
myvR1eFs1Ooq40dnOmH6PEtqbRXD8cvL2fsbB6RO3egnPFULpteJA30t4826QkTYdXiEqkA2eKLR5s5T5YHYug
📊 Statistics: 2 groups {"avg_score":50,"count":20,"status":"inactive"}
{"status":"active","count":20,"avg_score":60}

📝 Example 4: Function Management

📋 Total functions: 9 🔍 Retrieved function: Get Active Users ✏️ Function
updated 🗑️ Function deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! ✅ [32mTypeScript client examples complete![0m
