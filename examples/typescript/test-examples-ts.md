make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.7.0 prepare
> npm run build


> @ekodb/ekodb-client@0.7.0 build
> tsc


up to date, audited 7 packages in 845ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 661ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'hZ2ERKSh2N_9d85MXl37TWx2L6221p4kenhShpKqYvrslKXpGdRF92ZH4_YX8CRDwV6pPHfNC-xOXgqbk3J28g'
}

=== Find by ID ===
Found: {
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  price: { type: 'Float', value: 99.99 },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  active: { type: 'Boolean', value: true },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  id: 'hZ2ERKSh2N_9d85MXl37TWx2L6221p4kenhShpKqYvrslKXpGdRF92ZH4_YX8CRDwV6pPHfNC-xOXgqbk3J28g',
  name: { type: 'String', value: 'Test Record' },
  created_at: { type: 'DateTime', value: '2026-01-03T09:51:28.852Z' },
  value: { type: 'Integer', value: 42 }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-03T09:51:28.852Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  metadata: { nested: { deep: true }, key: 'value' },
  price: 99.99,
  categories: [ 'electronics', 'computers' ],
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  active: true,
  data: 'aGVsbG8gd29ybGQ=',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  tags: [ 'tag1', 'tag2', 'tag3' ],
  id: 'hZ2ERKSh2N_9d85MXl37TWx2L6221p4kenhShpKqYvrslKXpGdRF92ZH4_YX8CRDwV6pPHfNC-xOXgqbk3J28g',
  name: 'Test Record',
  created_at: '2026-01-03T09:51:28.852Z',
  value: 42
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  value: { value: 100, type: 'Integer' },
  id: 'hZ2ERKSh2N_9d85MXl37TWx2L6221p4kenhShpKqYvrslKXpGdRF92ZH4_YX8CRDwV6pPHfNC-xOXgqbk3J28g',
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  created_at: { type: 'DateTime', value: '2026-01-03T09:51:28.852Z' },
  active: { type: 'Boolean', value: true },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  name: { type: 'String', value: 'Updated Record' },
  price: { type: 'Float', value: 99.99 }
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
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
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
Collection created with first record: r06iZtsqwfiY3HvxHFY02yENMaESJznJl8eTS0U8CR1VQIYmj6SbSPqcx4W8CTFbLn7Xx9b6VdcXeGkYwmmBjA

=== List Collections ===
Total collections: 18
Sample collections: test_accounts,schema_users_client_go,client_collection_management_ts,schema_employees_client_go,ttl_cache

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: v1fOVp5jhBN8dYwrmlhYEibG7kC3TYT8YXKvVdRlsALI3twTdFxtJpS0W-pdxVf2DJnlDsm5k-1dfUqw1xrrfQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: LvkVQwMievbOovAjq-aYB2JBGvkAdVltKeLSsf_vHxPlcExtJ0nj_EPfn5L-LfWoHezdthLfxwzBLS2YPErMbg

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
✓ Inserted test record: ws367Y0WVPj7aaVdBR1bLPXesRqKbLYesaipwLVTZzb3CKg6TYX0XEE-Wu3PrC7w_KsX21Worc06sROsw2Q7hQ

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
✓ Inserted document with TTL: J9-vEWJGZMeAhn5ui9uMWr7szDCutLHPHrTkFVKlcVTJwOv16bpfn_mFbKD5GA2ebO3XZ6RFMHnKKbnpx9klbA

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
✓ Created session: THUFBEFfdkmCMHn2hdYXm4n0jJJ9YnsomL5MpahhsHwun4UkrANjuobwskFCQzkTmcJPnE9KBefdHotFCGlrBQ

=== Sending Chat Message ===
Message ID: 4-9lnihgjt8USsPnGXsFcUm8VC7BXNwKQUuFHKZjJRdOx0wapWahWEBFctthZBRsWWM8VWXaI9biFJCP7-1Yvw

=== AI Response ===
There are three products available:

1. ekoDB - A high-performance database product with AI capabilities, priced at $99.
2. ekoDB Pro - An enterprise edition product with advanced features, priced at $299.
3. ekoDB Cloud - A fully managed cloud database service product, priced at $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 299,
    name: 'ekoDB Pro',
    id: 'cRxzon1HZjl6InRD0VHP4nMSvDRzDm5mwM80jmqn-ViMURHk0PRqWjI9YF-nuBfKJwDpVNoHo8Rsg88IuausTw',
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    id: 'uiCcYZq4KFdMJ7XNqVXqK8EFFUEP8SGc93XZEVkpqBIvMkYIeVUvNnPB-EsAY23_YYWthigDA0ZtBqryPMkhfQ',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Cloud',
    id: 'noHSSG6sSdfnkzEHWylRLG_rNmUQPfkapFbDWqnHP7GQRXBi2gb41I8sdOUVhtPFTdKO2W8NDORBrWn7MhIZVA',
    price: 499,
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 1809ms

=== Token Usage ===
Prompt tokens: 603
Completion tokens: 66
Total tokens: 669

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: g-_bT_eNpoXLhbWtsQaFoZBaZ1XThT42geudrg3gWt0bMgB0VETxtXs4hKsIQmF4MmT_GAvKMB-QqQC3FIzFhA

=== Sending Messages ===
✓ Message 1 sent
  Response: Answer: The product available according to the provided context is "ekoDB". It is a high-performance database product.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: fo0Jvez4Z0G95m88_BG8MxDasRd8Dk-jR2kbMF-lzgCLAiHbCvuCYkNCWOBwK392uYLyozoatkkyGdGF5wHuEQ
  Parent: g-_bT_eNpoXLhbWtsQaFoZBaZ1XThT42geudrg3gWt0bMgB0VETxtXs4hKsIQmF4MmT_GAvKMB-QqQC3FIzFhA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: fo0Jvez4Z0G95m88_BG8MxDasRd8Dk-jR2kbMF-lzgCLAiHbCvuCYkNCWOBwK392uYLyozoatkkyGdGF5wHuEQ (Untitled)
  Session 2: g-_bT_eNpoXLhbWtsQaFoZBaZ1XThT42geudrg3gWt0bMgB0VETxtXs4hKsIQmF4MmT_GAvKMB-QqQC3FIzFhA (Untitled)
  Session 3: THUFBEFfdkmCMHn2hdYXm4n0jJJ9YnsomL5MpahhsHwun4UkrANjuobwskFCQzkTmcJPnE9KBefdHotFCGlrBQ (Untitled)
  Session 4: aRl2LkGcRd7T25p5PRat3KIdAs16ofTpaDxFNR29iOaKKEM6fI71deiXeF0clC7DGdm3GL9KoHsZbKVfRlwKSg (Untitled)
  Session 5: cW1cPq-jltIWG_-9ktoHU7rmTVOUdCHtyHfIf3qKNv2xf70zgTMgqEs5j74CEdrtwEPO8J8FdZPBC7ZzrPe3EA (Untitled)
  Session 6: IA5-rWYt2lejYRO9aNTZg4W-5TeMAvxB42a9TSCPeQw1CwR_yBAuwn2TANGMOssgAlVBKIqvHCr9wPW1BHcn5g (Untitled)
  Session 7: kjK1pDTNLOKHPH9OvZuvvQLFzTMCSQt-byLD2bA78E2U9VUsJHSz_I5zfuQo1BXHoSW53CpQnsFBhK4Q530HVw (Untitled)
  Session 8: r8-q1NPgr8SOnlJxjWfntm6zuChhix2h6xtYe5ervcSBCOE-h8CFI_i7m9pJxLVzK_gop-dpdEHNtpNLVEH2Zg (Untitled)
  Session 9: YQzzCfZjPdw5kP1aBN6THyY1bRvLMtCvolkzyPoRuQRuLMHMWeb6xYWJbJXPi-iZTzqBLMxeJJFJzPXJlfwLbQ (Untitled)
  Session 10: kjC5gFDXqUMkJtTG9oQxoJ4GZsqn4-JnYCiIcGGKMR0243BGdCUoT_VC4DQP5SHwUY2nER_UmhkyajKM6mJdfQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: fo0Jvez4Z0G95m88_BG8MxDasRd8Dk-jR2kbMF-lzgCLAiHbCvuCYkNCWOBwK392uYLyozoatkkyGdGF5wHuEQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: RqHn5Rv7510ycwxyLP4Dj_qeNVOsdk6NIIWCuMIXSOkgEChTPudSesTWL_EKha3vq9T88BeWLkWYravcpNhsag

=== Sending Initial Message ===
✓ Message sent
  Response: Currently, the available product is ekoDB, which is a high-performance database product. It is priced at $99.

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
✓ Created second session: JGeXXCkOoElfPsWHUpLTVXdLCBr0umT43ZDLGkWabp_fOmMK_meJTdznkscieQlsmFufkGttBgS4SYVAO_pNqw
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
  1. Score: 25.740, Matched: email, email.value, name.value, name
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio, bio.value, title
  2. Score: 26.400, Matched: bio.value, title.value, bio, title
  3. Score: 26.400, Matched: title.value, title, bio.value, bio
  4. Score: 26.400, Matched: bio.value, bio, title, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, bio.value, title, title.value
  2. Score: 39.600, Matched: title, bio, title.value, bio.value
  3. Score: 39.600, Matched: bio, title, bio.value, title.value
  4. Score: 39.600, Matched: bio, title.value, title, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.756, Matched: 
  2. Score: 0.741, Matched: 
  3. Score: 0.736, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.694, Matched: title.value, content.value, title, content
  2. Score: 1.502, Matched: content, title, title.value, content.value
  3. Score: 0.296, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio.value, skills, bio, skills.value

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

✅ Script saved: DXu3oI_nCag5eyD79RDmV4yZf-YjfTVxOpitZclxZBMXNZgu3nX-I_B6v-5_VqSul7LRTqEQzj8I92QtbDGI3Q
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: 1q2EpYU64cVU02PEtiR9HSLtU2HbchcaEPGXmGaxLDtIZkkQFW5j0jfCAIIN1etUlf-dth8CVTU_s3X3p73_Hg
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: ry8oKS4zZwAKjURsZBYGAuQvsIXBgzEmMvAUxEJs2wOsdbLZjoI5sYEY6M6Uu23g4eTVK0TyILH9VUOzr6wJyQ
📊 Statistics: 2 groups
   {"status":"active","count":5,"avg_score":60}
   {"count":5,"avg_score":50,"status":"inactive"}

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

✅ Script saved: 6U7oKn5ZfV7NpXgLVyScd5J3UhHXgFxwZSri502EW6cX1TyD6DO-pgbesQl4CFhzExUSb40W8NicwmxiqX_4Gw
📊 Found 2 product groups
   {"category":"Furniture","avg_price":474,"count":2}
   {"category":"Electronics","count":3,"avg_price":575.6666666666666}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"category":"Furniture","count":2}
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
Created Alice: $1000 - ID: C9ePxBiKlVMTH9eg13HpgPXBzXBOVnu91LH8naA9zrZCkB6jlRTWnZPB2TIUN5bB9an9zwqUCOtp6gGtlC3_Yw
Created Bob: $500 - ID: Bu4z6PxyhdOHzMhURYsn-W5NiC6sjCMzbPbtfxfyrzp890kNTACRHdm8Emy3O5AfDEQXYLhPOMrMZD8Zh9p3dQ

=== Example 1: Begin Transaction ===
Transaction ID: 4c5a68b6-45c6-4bc9-a9b0-dc7a20a53db9

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: a94aedca-0a70-4791-8a6f-8c47c9de2518
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mTypeScript client examples complete![0m
