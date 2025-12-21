make test-examples-typescript 📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.4.0 prepare npm run build

> @ekodb/ekodb-client@0.4.0 build tsc

up to date, audited 7 packages in 1s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.4.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning TypeScript client library
examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 812ms

3 packages are looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'ry4yL6LgBfo1LDEodz_QvouL6M14P3aBOfs2glCt34rWznw4j0feq9xRZvu60sa02uNq8p861S9tRzn-HL10zA'
}

=== Find by ID === Found: { id:
'ry4yL6LgBfo1LDEodz_QvouL6M14P3aBOfs2glCt34rWznw4j0feq9xRZvu60sa02uNq8p861S9tRzn-HL10zA',
value: { type: 'Integer', value: 42 }, active: { type: 'Boolean', value: true },
name: { type: 'String', value: 'Test Record' } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { id:
'ry4yL6LgBfo1LDEodz_QvouL6M14P3aBOfs2glCt34rWznw4j0feq9xRZvu60sa02uNq8p861S9tRzn-HL10zA',
active: { type: 'Boolean', value: true }, name: { type: 'String', value:
'Updated Record' }, value: { type: 'Integer', value: 100 } }

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
bEzryU5Dj9rXuSINkMWdA4N5AKW0cP9KDNKgqDtxGO1Jql4W903w5vcc8r\_-c0MuTOJETZ7fcvOADYoxO8U2xw

=== List Collections === Total collections: 18 Sample collections:
schema_documents_client_py,websocket_test,schema_employees_client_py,client_collection_management_ts,schema_documents_client_go

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
oTp5Xnnz4VmDMheWCzStQYIj_9hHuIqIIFrDIxmbX7OA-YW3duqO9e-SmeMvEOtx1ew9xzl57i5fTUFiyGWAVQ

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
cG3FFbvAAxYVTGHyr15Fh31Pn07n-vHSkacgfeMV8uEkeolNiZ0QJ00HRE_kOYxX5IpxT4lMXwvoXAm-U5Bhdw

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
\_BQr0wipSXpCriBsRESwcnvZ1tuLErhD0h6yq6kkgabXTSjJXJyWrvA12-CnNghdbwUgGwswt7O20BFeTN_uhA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
XSLhSkPG02NAkKTR1vp8zfQTl0ysxNwTpJM8V67NAAZhQoTJw34A9JLSWsT5iC8QnkQneNjS1XAk8nUImjhOlw

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
UsEFZtdDz-pNk9IjXHv5abWj5i_NokGKR6LGHOkqavIaMjdYm_9NkF9jarqHmI4ZF_0Hd01q7vwo1yhCVG71rA

=== Sending Chat Message === Message ID:
WnUOVkbAJfwStxpBCZsLrE_U5AKXSr_51pxrfOg0XKhf_Gk5YgbHB96CUdn7lZFt64bxNz1gq1uSyXMpYGgUTw

=== AI Response === I'm sorry, but the context provided does not include
specific product names or their prices. Could you please provide more
information?

=== Context Used (3 snippets) === Snippet 1: { collection:
'client*chat_basic_ts', record: { price: { value: 299, type: 'Integer' }, id:
'SjlFTw199zPrxlEHLH2obrQJIINQM1rlVqsaGWRejTrEH*-EmKPviMtyXRDdZHMBlVP1G3tarv4DYf-sVpYjEA',
description: { value: 'Enterprise edition product with advanced features', type:
'String' }, name: { type: 'String', value: 'ekoDB Pro' } }, score:
0.1111111111111111, matched_fields: [ 'description' ] } Snippet 2: { collection:
'client_chat_basic_ts', record: { id:
'6ioduVPc1ShCrLjoEiJwUeqV-NZPJDuy7xbFyaLi3XH8nDuE4VG0oIHolXSILICcD8JzxF7iDdSkVUfaOBiXqw',
price: { type: 'Integer', value: 499 }, description: { value: 'Fully managed
cloud database service product', type: 'String' }, name: { value: 'ekoDB Cloud',
type: 'String' } }, score: 0.1111111111111111, matched_fields: [ 'description' ]
} Snippet 3: { collection: 'client_chat_basic_ts', record: { name: { value:
'ekoDB', type: 'String' }, price: { type: 'Integer', value: 99 }, id:
'CEGpniyINJEpHc2aFeeRU8UaxO9YvhPB464O8WbRPckDoqgd8auwMQEYNnOc1Qk-jekTZBm9Qo6lIcpUkdeu4w',
description: { type: 'String', value: 'A high-performance database product with
AI capabilities' } }, score: 0.1111111111111111, matched_fields: [ 'description'
] }

Execution Time: 1465ms

=== Token Usage === Prompt tokens: 313 Completion tokens: 25 Total tokens: 338

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
qdxCGX6CQqVL0KOp5IvBZ1DOfwFllzXUE3ruqjh3qa76afit-K-znu7nje5LoYiBvqmoBFShqNrRWegsuKiDWQ

=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the context
provided does not contain any specific product information. Could you please
provide more details or specify the type of products you are interested in?

✓ Message 2 sent Response: Apologies, but the context provided does not include
information about a specific product or its price. Could you please provide more
details or clarify which product you're interested in?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
OMq9gymjNDHDtGiBjvQ8Wzp-WM9_CujYJ0nG1iEQtB7h8raxRtZOIF8BhxnosocbsTohPgkAV4qSxVyJcbyQbQ
Parent:
qdxCGX6CQqVL0KOp5IvBZ1DOfwFllzXUE3ruqjh3qa76afit-K-znu7nje5LoYiBvqmoBFShqNrRWegsuKiDWQ

=== Listing Sessions === ✓ Found 10 sessions Session 1:
OMq9gymjNDHDtGiBjvQ8Wzp-WM9_CujYJ0nG1iEQtB7h8raxRtZOIF8BhxnosocbsTohPgkAV4qSxVyJcbyQbQ
(Untitled) Session 2:
qdxCGX6CQqVL0KOp5IvBZ1DOfwFllzXUE3ruqjh3qa76afit-K-znu7nje5LoYiBvqmoBFShqNrRWegsuKiDWQ
(Untitled) Session 3:
UsEFZtdDz-pNk9IjXHv5abWj5i_NokGKR6LGHOkqavIaMjdYm_9NkF9jarqHmI4ZF_0Hd01q7vwo1yhCVG71rA
(Untitled) Session 4:
zkkyfdmFnYypkAqEgfCEOohsXEn_9nMMpSt8BSnC6ndXSyLLB5M6XsYAkvgkWHqVWOs9he7TmMsMqBQAyNFq_w
(Untitled) Session 5:
l2TKmXvg8rdbBtJammgCyq4Jy3yVgirnt4-pk6LvMOAAockMGXCmZRC6WNzREj_pUUnlVPXFu7miAzmx039jNw
(Untitled) Session 6:
U6ghpVmbIClTPbe95ajQzICrBvWDayyiU9jU-ksm_xwEYCjPl5G4va407cIuGPo_Qgko-e_P09vaBE52QSd46Q
(Untitled) Session 7:
j0ZNc8BrbxhRUDgb2Fwn4wHDCMGkwyehQjCVIE_5utey3QYCVumADwRrDHTrdEJPdutXXUHL_TIEvjy9tas1ig
(Untitled) Session 8:
Kw60gQSthrKhDTfMuRtxqAQIyQTjGlsO4MUe8y65_FJHpsNu1vRm7nOJP10OKVfro3MzZkmxpCCQUhBStMMfhg
(Untitled) Session 9:
LCHZxuXi5of0QvhmSm0wm2f0WqfUiFUL6umnxwqTLFma71kgbt2sXS9UnwTggcBS-gSdQZqtf466qGxqUAT_hg
(Untitled) Session 10:
pC_Hvla-W6X4dKvY4WkJu4fHKzUoGMR1u2f-5FM1qJvi3iON0PEl6jCJjAEIWgpAThKQIjskKUCCOt0Ad51qMQ
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
OMq9gymjNDHDtGiBjvQ8Wzp-WM9_CujYJ0nG1iEQtB7h8raxRtZOIF8BhxnosocbsTohPgkAV4qSxVyJcbyQbQ

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
GtwYLWVvRmo3_PULzuGdjXxy1YBB9JqJIP8P9HML35Ir-x1UC1UAFiLQ_u0541_Ijm8QY9PCqVER33Of77NBKQ

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
context provided doesn't give information about any specific product. Could you
please provide more details or specify the type of product you are interested
in?

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
Answer: I'm sorry, but the context provided does not include information on the
price of ekoDB.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
rS0mxn8V7loz_E6U8ApkdhM_fvNusgU3fGTn0ndjtJ9loLKlpvV6fYhgoVTYkizoh7P3eX-NyWIDxDu0q5YyTg
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
1. Score: 25.740, Matched: name, email, email.value, name.value
1. Score: 12.540, Matched: name, name.value

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: title, bio, bio.value, title.value
1. Score: 26.400, Matched: title.value, title, bio, bio.value
1. Score: 26.400, Matched: title.value, bio.value, bio, title
1. Score: 26.400, Matched: title.value, bio, bio.value, title

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: bio.value, title.value, title, bio
1. Score: 39.600, Matched: title.value, bio, title, bio.value
1. Score: 39.600, Matched: bio.value, title, title.value, bio
1. Score: 39.600, Matched: bio, title.value, bio.value, title

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio, bio.value
1. Score: 13.200, Matched: bio, bio.value

1. Search with stemming and exact match boosting: Found 1 results (matches:
   work, working, worked)
1. Score: 13.200, Matched: bio, bio.value

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.748, Matched:
1. Score: 0.742, Matched:
1. Score: 0.740, Matched:

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.697, Matched: content.value, title, content, title.value
1. Score: 1.496, Matched: content, title, content.value, title.value
1. Score: 0.299, Matched:

1. Case-sensitive search: Found 1 results (case-sensitive)
1. Score: 26.400, Matched: bio, skills, skills.value, bio.value

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

- Alice Johnson: Engineering
- Bob Smith: Sales

2. Join with filtering: Found 1 users in Engineering:

- Alice Johnson: Building A

3. Join with user profiles: Found 2 users with profile data:

- Alice Johnson: Senior Software Engineer
- Bob Smith: Sales Manager

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
atDYknsuaAZaWyOq0KhJL0r-jn2IORLVG6txGBdCVR0j9bQc0DqXQGy6ou2VbOLOtUqnr4Oxsj3_f7yyaClknQ
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved:
t9H06CcU0DxlpQ7rdeszfz9C8HXrZ5wHRwQxtWH9b6SOb6tDhX2pQeMSqsEVkxWqkgruX3FgFuPdJRlFgvlhHw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
L0zBh-no8aViN1lsYAg_t7iMoZbx8q1HoOMrven6XZ-aTOPEQ6kyj6WPs_ENi0nnewWftpKL1lZCpyqeivPjyw
📊 Statistics: 2 groups {"count":5,"status":"active","avg_score":60}
{"status":"inactive","count":5,"avg_score":50}

📝 Example 4: Script Management

📋 Total scripts: 18 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! 🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data... ✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved:
4tf-2IQ9eLvHj7GxOO8gSPW9oep8YBh0pn1na9wnjvOJCf_OqwI05taJ5MtgPeq5eYbd27R_pGJ-j_VJoKy76A
📊 Found 2 product groups
{"count":3,"category":"Electronics","avg_price":575.6666666666666}
{"avg_price":474,"category":"Furniture","count":2} ⏱️ Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved 📊 Found 5 products ⏱️ Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved 📊 Found 2 categories {"category":"Electronics","count":3}
{"count":2,"category":"Furniture"} ⏱️ Execution time: 0ms

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
