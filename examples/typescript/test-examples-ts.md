make test-examples-typescript 📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.3.0 prepare npm run build

> @ekodb/ekodb-client@0.3.0 build tsc

up to date, audited 7 packages in 1s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.3.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning TypeScript client library
examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 701ms

3 packages are looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'RXdtfsZ94Dxb9-z12wCLbFsuBBJkx-wPpgz7zuLDrVvanK2jiQifcQg4GOLoALtutB5ghkGj4rksin5J8A76bw'
}

=== Find by ID === Found: { id:
'RXdtfsZ94Dxb9-z12wCLbFsuBBJkx-wPpgz7zuLDrVvanK2jiQifcQg4GOLoALtutB5ghkGj4rksin5J8A76bw',
name: { type: 'String', value: 'Test Record' }, active: { type: 'Boolean',
value: true }, value: { value: 42, type: 'Integer' } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { id:
'RXdtfsZ94Dxb9-z12wCLbFsuBBJkx-wPpgz7zuLDrVvanK2jiQifcQg4GOLoALtutB5ghkGj4rksin5J8A76bw',
value: { type: 'Integer', value: 100 }, active: { type: 'Boolean', value: true
}, name: { type: 'String', value: 'Updated Record' } }

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
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
FuqRhbbQCFoaIwFldBup6B2qN8sAFuZSj9D5OXHK10UrJG8DAN8EW2ObXOjEPUAyUuUN0EBm6aY1fYy17L1Hnw

=== List Collections === Total collections: 18 Sample collections:
schema_documents_client_py,schema_documents_client_go,client_collection_management_ts,test_collection,ws_ttl_test

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
LybzDjEbkKWFENXuVTH84Hgg9as5_jlsAb_1ZFgrv5GV7KAcKUW27ExQkd7yEAKpvOlqyfUp4zQQmt3QdUj90g

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
neQ0oEpWFvxRBxvvj9R6d8Mb3CPhn1NM45S8IFcTxXgy6FUjH7qK1BlSg7HjLQsIgizdQIqzjJS0Zq0g-xSDmA

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
sUVbX6c7Kx6IkAWXwaULIGFS6RG2_T7V22l1jMbI_rwfpM687ZOFcAkdfSe7SuP_5BvbuVjaaTGQlitAmja3bg

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
-jkFUCiwT8ALcjxKZauWCPEsmLhk3e7JZ4ZoWAUs_2HI4QTW_PzlZMJdLhFDTe3ksd8MsLOXf5Nmq2rbKMJmVA

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
qkbLEE_1fFuGTX3Ok9Vxhk4tDPDOldzO7UZubV_3jacMhA-MkdDcfhMpn4IrPvkhZ2ZOEPMhDTevhlaRwv3hKw

=== Sending Chat Message === Message ID:
j-TVIaT0zlou4J-me60aIH2gn2lI_PncBpWkx00FoVXxWKc_wUQatXHcaATtzChTBeypetmoxtFZxyX7mW0J7w

=== AI Response === I'm sorry, but the provided context does not contain
specific information about the product names and their corresponding prices.
Could you please provide more details or context?

=== Context Used (3 snippets) === Snippet 1: { collection:
'client_chat_basic_ts', record: { name: { type: 'String', value: 'ekoDB Cloud'
}, id:
'oeSO7bII1BemmnXQnEspwai-i5yHv4UTf0CYoXa70fYkXAUdV_N_UR4eizsDYhg1vYOd4Ke02958U018CTWk1Q',
description: { value: 'Fully managed cloud database service product', type:
'String' }, price: { type: 'Integer', value: 499 } }, score: 0.1111111111111111,
matched_fields: [ 'description' ] } Snippet 2: { collection:
'client_chat_basic_ts', record: { name: { value: 'ekoDB', type: 'String' },
description: { type: 'String', value: 'A high-performance database product with
AI capabilities' }, price: { value: 99, type: 'Integer' }, id:
'lowNpdwCcR-HASV-KyaDZA1G4PqecajFtpg4SPimxvGoomG9P0d3LFvPIz-IlC-L0NjkLZYJ7NtoUP1yNjzVSw'
}, score: 0.1111111111111111, matched_fields: [ 'description' ] } Snippet 3: {
collection: 'client_chat_basic_ts', record: { price: { type: 'Integer', value:
299 }, name: { type: 'String', value: 'ekoDB Pro' }, id:
'kkHX14KdXjOK37-QlZm5sPLlBTHAemdeSfz3lq5cZlqAgeEGVeKiHCcpcWq268xB3VYnUbfqFmlOBBBXfSRTpg',
description: { value: 'Enterprise edition product with advanced features', type:
'String' } }, score: 0.1111111111111111, matched_fields: [ 'description' ] }

Execution Time: 1236ms

=== Token Usage === Prompt tokens: 316 Completion tokens: 31 Total tokens: 347

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
PW3qaGhiyvmk\_-RTKqBXfQCiFa_D30yeGTDITy1ACfJPvWTYTrchMhoucgCYaJpThCHu2WZWyAxX8pilfX3CSw

=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the context
provided doesn't contain specific information about any products. Could you
please provide more details or clarify your request?

✓ Message 2 sent Response: Apologies, but the context provided doesn't include
information about a specific product or its price. Could you please provide more
details or specify the product you're asking about?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
E6kRbN*F9ci7Cw1TfEwJu1FwytFxrqqgPm-4LCqSmFTQmn0o-mxwsbjfIjW6ULaXFM5Jrze-aLrLHbyUJSUiUA
Parent:
PW3qaGhiyvmk*-RTKqBXfQCiFa_D30yeGTDITy1ACfJPvWTYTrchMhoucgCYaJpThCHu2WZWyAxX8pilfX3CSw

=== Listing Sessions === ✓ Found 10 sessions Session 1:
E6kRbN*F9ci7Cw1TfEwJu1FwytFxrqqgPm-4LCqSmFTQmn0o-mxwsbjfIjW6ULaXFM5Jrze-aLrLHbyUJSUiUA
(Untitled) Session 2:
PW3qaGhiyvmk*-RTKqBXfQCiFa_D30yeGTDITy1ACfJPvWTYTrchMhoucgCYaJpThCHu2WZWyAxX8pilfX3CSw
(Untitled) Session 3:
qkbLEE_1fFuGTX3Ok9Vxhk4tDPDOldzO7UZubV_3jacMhA-MkdDcfhMpn4IrPvkhZ2ZOEPMhDTevhlaRwv3hKw
(Untitled) Session 4:
p_OflSDoLgj1I23BWhnbC3uURTcqR7G2QxFlCkVn03E1yvKx4CnEyAErMUHT8A0_3vjzc5lc2OkcgBJPSgd6XQ
(Untitled) Session 5:
mGV91EHKMf7NwRPDOedF5F6YMIqJ9Ud_AnWeX7HXYmkc_K-KO_y59NL7bIxkhwpaHLRWeYyQSHYXSndXJAj1Sw
(Untitled) Session 6:
XqylP4HFwIW9Ahwo41V5fM1SLJvXFqcbPoqoN5ayJ0TxY8OwX3yRTzKyRL45_3UXctd83YgI8BvXUCJYdIWkmg
(Untitled) Session 7:
48IeSr-b5OvgoPOo5UeudSGqoPBWM9t1iupVu29o83S7xz239syKzCaUsRhQY4gO-CBRPK3vUG5Fu8m694PnmA
(Untitled) Session 8:
17Zok24kRKL0JCbQzJ2miG8bWcWIQ3fqG7mD3aIpuLjAMu0V_Qv5aoQvN1lmqhrl44_BhaVGQWOZHxhvfOpnRA
(Untitled) Session 9:
\_ZfZ-i61nrfaPM1Ql5QjKywbDAUwP0ApPNSm8aSneEByuytGCny8Dec49cx3-gRpg03AvoP_26-ID7_KtQfCLw
(Untitled) Session 10:
B0IRmVduPyhJcej9WW3Kz3ykpu3orY3VUi_OYpThxcXTAAnvs_tOFH5m72d4njdGdiWzc0kZ-wkLVPly6-AHOg
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
E6kRbN_F9ci7Cw1TfEwJu1FwytFxrqqgPm-4LCqSmFTQmn0o-mxwsbjfIjW6ULaXFM5Jrze-aLrLHbyUJSUiUA

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
MWRilXZZgRSI1BREH-M41tJsZNVi8NNOBtwrb2K45pil2rUVFUNxV2O6ogc9fS1XWeVgzEdMTDw9_Fwti16y6Q

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
context provided doesn't contain specific information about any available
products. Could you please provide more detailed information?

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
Answer: I'm sorry, but the context provided does not include information on the
price of ekoDB.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
U875fq5jYR34Tng6NKbvpO2-bkyWIcONOOJYo27ABn_EyWs41NJKHZ50DczKNCcUU3Rftl8geJM4hvXHdToV6g
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
1. Score: 25.740, Matched: name.value, email.value, email, name
1. Score: 12.540, Matched: name, name.value

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: bio.value, bio, title.value, title
1. Score: 26.400, Matched: title, title.value, bio.value, bio
1. Score: 26.400, Matched: title, bio, bio.value, title.value
1. Score: 26.400, Matched: title, bio.value, bio, title.value

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: title, bio.value, bio, title.value
1. Score: 39.600, Matched: title, bio, bio.value, title.value
1. Score: 39.600, Matched: title, bio, bio.value, title.value
1. Score: 39.600, Matched: bio.value, title.value, title, bio

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio, bio.value
1. Score: 13.200, Matched: bio, bio.value

1. Search with stemming and exact match boosting: Found 1 results (matches:
   work, working, worked)
1. Score: 13.200, Matched: bio, bio.value

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.755, Matched:
1. Score: 0.740, Matched:
1. Score: 0.734, Matched:

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.694, Matched: title.value, content.value, title, content
1. Score: 1.502, Matched: title, content, content.value, title.value
1. Score: 0.296, Matched:

1. Case-sensitive search: Found 1 results (case-sensitive)
1. Score: 26.400, Matched: bio.value, bio, skills, skills.value

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
QEb-TbhM6G-SzdYfVI4CDOq4Cck3sRJNSFaHljU5st1yVZZNLNWykjRfA3Z8I5uwAKKl_fC_dmJU4ld2NnDNFA
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved:
aPfDt5pP8jimkqtX3p0HrvY9XSiMcFlN4lQx_8SKP88b0sh1e3bKyYBNpg0Tr8ryuMD0A_nwJo0rcFeM5Zxt5w
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
RaelLf-KnbL0aRK86_iy1scz_7CoYp-Z85PG71R6Xh-OpeKbH6V46czX0RXpJdopLzedJwhxrVHb95wkp4h6iQ
📊 Statistics: 2 groups {"status":"active","count":5,"avg_score":60}
{"count":5,"avg_score":50,"status":"inactive"}

📝 Example 4: Script Management

📋 Total scripts: 11 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! 🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data... ✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved:
uv4vxAOpHjRFVn6vCZtIUVO7LQVmk6r4mK61p2taXLHfFg0SrZWHagpL5lpsLa3k38VlmeGuX24vz4rfdTrQcg
📊 Found 2 product groups
{"avg_price":575.6666666666666,"category":"Electronics","count":3}
{"category":"Furniture","count":2,"avg_price":474} ⏱️ Execution time: 0ms

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
