make test-examples-typescript 📦 Building TypeScript client library...

> @ekodb/ekodb-client@0.1.0 prepare npm run build

> @ekodb/ekodb-client@0.1.0 build tsc

up to date, audited 6 packages in 838ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.1.0 build tsc

✅ TypeScript client built! 🧪 Running TypeScript client library examples...

up to date, audited 13 packages in 607ms

3 packages are looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'V7S9hn08t2XojJ4Zp3OPUnLI97QlM79aI298XWipoXt2nNWmOduEKjioO8uJQwlbTsd6DHWCWIpntLs_vv6fDA'
}

=== Find by ID === Found: { id:
'V7S9hn08t2XojJ4Zp3OPUnLI97QlM79aI298XWipoXt2nNWmOduEKjioO8uJQwlbTsd6DHWCWIpntLs_vv6fDA',
active: { type: 'Boolean', value: true }, value: { value: 42, type: 'Integer' },
name: { type: 'String', value: 'Test Record' } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { name: 'Updated Record', value: 100, active: {
type: 'Boolean', value: true }, id:
'V7S9hn08t2XojJ4Zp3OPUnLI97QlM79aI298XWipoXt2nNWmOduEKjioO8uJQwlbTsd6DHWCWIpntLs_vv6fDA'
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

=== KV Get === Retrieved value: { username: 'john_doe', userId: 123 }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
6qRUjkCfXTZMPjB7KENSgKw1eLLr7pk7dAnYagzreCbs_sx-jSldnlntCjYv3800OMfcfGJFyELKsfLfmdvOIg

=== List Collections === Total collections: 1 Sample collections:
client_collection_management_ts

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
qfZPvzORTEO9b9plwvWOSo-UVgdsbMdR7WUNNnzQrLTuai7TdUdXkWoicEdXnC6nGty3R7PGmECmDse34iP5WA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
7NI_FkB5D4cP8ZdTrILYXMnpUzgrZwVIg6qmLGcFe7mr1g9UAs-l-Afbjc641f1K7QVLzPPVhBAWUTkun5gkKw

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
bjTmwHWE9PR0FkVRS9vEKYLepd_LJ3ZYn4nHvpHpm153W9zRK3yNI1quIciuHDeaWVTZL_ovOxaTK52x2H815A

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
IkmL3-bgTRypi6LhCAB_AYWh7vCiyy1E3w33K_4IH_IVRaCBH_wcaRj0eVC4lWNgAQtDj4eqqKdx97HS6mY8HA

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
hIdSf67EYHKiyswyiyAjusdt_oGeMaoD_wf9VMDlDKbODCtrWl_X8CcZHpTCG16vmsbWaN-iF8vjQvniwUTEiA

=== Sending Chat Message === Message ID:
TD29I8xcaAydjIzk0A_xZnRgYmf7ve3UejfTU5q9oGtOg9WliBRXcyVpiYNf4Xxzq6teICKzsrVT8344zBHyRA

=== AI Response === Based on the provided context, there are three products
available:

1. ekoDB, which is a high-performance database product with AI capabilities. The
   price for this product is $99.
2. ekoDB Pro, this is an enterprise edition product with advanced features. The
   price for this product is $299.
3. ekoDB Cloud, a fully managed cloud database service product. The price for
   this product is $499.

=== Context Used (3 snippets) === Snippet 1: { collection:
'client_chat_basic_ts', record: { price: { type: 'Integer', value: 99 },
description: { type: 'String', value: 'A high-performance database product with
AI capabilities' }, name: { value: 'ekoDB', type: 'String' }, id:
'9h0VEcKXt8Y5N5hfSBdgh90QGAw19BRutumgQq40wpsSjl8wzd_PJcHU3nv06P-iNBdPQAsoENml4qKlteI0Dw'
}, score: 0.1111111111111111, matched_fields: [ 'description' ] } Snippet 2: {
collection: 'client_chat_basic_ts', record: { id:
'WGrX_0FtBBVKqyTMHAeqaSBGUMVhgmbf4uETygcS3CZHTaeelEeJNPZdxUX6XcaPBY4ZgWiq1LucuUISYdMYGg',
price: { value: 299, type: 'Integer' }, description: { type: 'String', value:
'Enterprise edition product with advanced features' }, name: { type: 'String',
value: 'ekoDB Pro' } }, score: 0.1111111111111111, matched_fields: [
'description' ] } Snippet 3: { collection: 'client_chat_basic_ts', record: {
price: { type: 'Integer', value: 499 }, name: { value: 'ekoDB Cloud', type:
'String' }, description: { value: 'Fully managed cloud database service
product', type: 'String' }, id:
'LeA776kdV-Dj9as831QjDRFNLBjBdIu0nYQTAKyUsTx6VgDunYlh5W1rlVlXEOuSh0nwhrgJwsrF_ZPMtGfPRw'
}, score: 0.1111111111111111, matched_fields: [ 'description' ] }

Execution Time: 3186ms

=== Token Usage === Prompt tokens: 376 Completion tokens: 88 Total tokens: 464

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
lwVkcqGLpj-IbL5XCPwmFLvrbpCN4OcjHtuxmMJDPHJNY5M6hqul0grh-KAmvmIQcbZd5cyyxxtuCps_nY81JQ

=== Sending Messages === ✓ Message 1 sent Response: Based on the provided
context, the available product is ekoDB, which is a high-performance database
product. It is priced at 99.

✓ Message 2 sent Response: As an assistant, I'm sorry but I can't provide the
price you're asking for because the context doesn't specify any product. Could
you please give more details?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
A9hGKmu8B1eWf3FykbI6XER2vjOYAi_fgSBWNQp1f9zAc82q43HqTF2Njgw7Fj9RV2oF9wUvXl5yA-HC-64wdQ
Parent:
lwVkcqGLpj-IbL5XCPwmFLvrbpCN4OcjHtuxmMJDPHJNY5M6hqul0grh-KAmvmIQcbZd5cyyxxtuCps_nY81JQ

=== Listing Sessions === ✓ Found 3 sessions Session 1:
A9hGKmu8B1eWf3FykbI6XER2vjOYAi_fgSBWNQp1f9zAc82q43HqTF2Njgw7Fj9RV2oF9wUvXl5yA-HC-64wdQ
(Untitled) Session 2:
lwVkcqGLpj-IbL5XCPwmFLvrbpCN4OcjHtuxmMJDPHJNY5M6hqul0grh-KAmvmIQcbZd5cyyxxtuCps_nY81JQ
(Untitled) Session 3:
hIdSf67EYHKiyswyiyAjusdt_oGeMaoD_wf9VMDlDKbODCtrWl_X8CcZHpTCG16vmsbWaN-iF8vjQvniwUTEiA
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
A9hGKmu8B1eWf3FykbI6XER2vjOYAi_fgSBWNQp1f9zAc82q43HqTF2Njgw7Fj9RV2oF9wUvXl5yA-HC-64wdQ

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
-d3A_vqx1ZK2gY0SV13es4_9FTsYLjxmtHo01mqX1FGoQDFbgW51esPohnfIiYCJol6jZMfQaeGJLg6657b8TQ

=== Sending Initial Message === ✓ Message sent Response: Based on the
information available, the product that is available is a high-performance
database product named ekoDB, priced at 99.

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
I'm sorry, but the context provided does not include information on the price of
ekoDB. You may need to check the official website or contact the sales
department of ekoDB for accurate pricing information.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
xJsF01_LCPcHuzhzM9_r7smJjuisBMYCJtrbbPTVuVkFTHLuau9xjmwKFSpBV7S1A3wGKuJ7f1m9lmlETZR_Kg
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
1. Score: 25.740, Matched: name.value, email, name, email.value
1. Score: 12.540, Matched: name.value, name

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: bio, title.value, title, bio.value
1. Score: 26.400, Matched: title.value, title, bio, bio.value
1. Score: 26.400, Matched: title, bio, title.value, bio.value
1. Score: 26.400, Matched: title, title.value, bio.value, bio

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: title, title.value, bio.value, bio
1. Score: 39.600, Matched: bio.value, bio, title, title.value
1. Score: 39.600, Matched: title, title.value, bio, bio.value
1. Score: 39.600, Matched: bio.value, title, title.value, bio

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio, bio.value

1. Search with stemming and exact match boosting: Found 1 results (matches:
   work, working, worked)
1. Score: 13.200, Matched: bio, bio.value

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.746, Matched:
1. Score: 0.742, Matched:
1. Score: 0.738, Matched:

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.697, Matched: title, content.value, content, title.value
1. Score: 1.499, Matched: content.value, title, content, title.value
1. Score: 0.295, Matched:

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

- Bob Smith: No department
- Alice Johnson: No department

2. Join with filtering: Found 1 users in Engineering:

- Alice Johnson: Unknown

3. Multi-collection join (users with departments and profiles): Found 2 users
   with multiple joins:

- Bob Smith: Department: N/A Bio: N/A
- Alice Johnson: Department: N/A Bio: N/A

4. Join orders with user data: Found 2 completed orders:

- Laptop ($1200) by Unknown
- Mouse ($25) by Unknown

5. Complex join with multiple conditions: Found 2 users with example.com emails:

- Alice Johnson (alice@example.com): N/A
- Bob Smith (bob@example.com): N/A

=== Cleanup === ✅ Deleted test collections

✅ Join operations examples completed! ✅ TypeScript client examples complete!
✅ All TypeScript integration tests complete!
