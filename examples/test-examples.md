make test-examples
🧪 Running Rust examples (direct HTTP/WebSocket)...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("rneQ4Drot7jJ_Dwv5GicMxTRz-mlqKWRRfZuPb2NfXYmzWuJCgJ4OzFjbB_csCa2Rxi0BHW8S3MvVh1VwMFS2g")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("rneQ4Drot7jJ_Dwv5GicMxTRz-mlqKWRRfZuPb2NfXYmzWuJCgJ4OzFjbB_csCa2Rxi0BHW8S3MvVh1VwMFS2g"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("rneQ4Drot7jJ_Dwv5GicMxTRz-mlqKWRRfZuPb2NfXYmzWuJCgJ4OzFjbB_csCa2Rxi0BHW8S3MvVh1VwMFS2g"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("rneQ4Drot7jJ_Dwv5GicMxTRz-mlqKWRRfZuPb2NfXYmzWuJCgJ4OzFjbB_csCa2Rxi0BHW8S3MvVh1VwMFS2g"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: -qd47qa1iWg3p6wvntrCdM-Z0LSWjX8apAEx80lsPHr2wXOCgj1VDwzNTrb-_bKtjwcJ-XCmlwxvPsNqtwxDSA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1791434850593408000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "-qd47qa1iWg3p6wvntrCdM-Z0LSWjX8apAEx80lsPHr2wXOCgj1VDwzNTrb-_bKtjwcJ-XCmlwxvPsNqtwxDSA",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "value": {
          "type": "Integer",
          "value": 42
        }
      }
    ]
  },
  "type": "Success"
}
✓ Retrieved 1 record(s) via WebSocket
✓ Deleted test collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/batch_operations`
✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 5 total records in collection

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records
✓ Verified: Record updated with status="active"

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/kv_operations`
✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: Object {"type": String("Object"), "value": Object {"userId": Number(123), "username": String("john_doe")}}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
kv_operations:direct:rs:cache:product:1: Object {"type": String("Object"), "value": Object {"name": String("Product 1"), "price": Number(29.99)}}
kv_operations:direct:rs:cache:product:2: Object {"type": String("Object"), "value": Object {"name": String("Product 2"), "price": Number(39.989999999999995)}}
kv_operations:direct:rs:cache:product:3: Object {"type": String("Object"), "value": Object {"name": String("Product 3"), "price": Number(49.989999999999995)}}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "sZRic0CfklEAFlDm7zwvGgX12d20AJOE6tkCZZTC4cVPe3VyiVOIH7WJjt2LEEEByRZ_WdpRMjnxBjjAe3Klyg"

=== List Collections ===
Total collections: 2
Sample collections: ["demo_collection", "audit__ek0_testing"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/document_ttl`
=== ekoDB Document TTL Example (Rust) ===
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: YLNn2oIIAS35ivGhEcPanDHIcJgeekte92n_snJ0Q3PSyW8ji9q8Fu1Kt-xoqUYsogdqUHZQOw41lcT2FTCDVQ

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: XfsL-tq-BuIpi0tlr3riVCLrNbzMFrLaa_4qdSf4yzLXRM08EMGFhz3YC6wiq9W3e5OBUnyrG4FU3sjfqVcJog

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: xwmocoAO8oczPuNx8Mga1Rd_5KRmoGMNalJxnsn2loSMsEmOdIx3IfQMyAV8l8xW1qgi-v7G1LNmMCjusngQyw

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("91UyNfBN8A8dBS7UGeiuuboOBMSwkY6MyvhDAdEkWmuAsDe775Rx3p2r1f8gkwDIaXFQwJxqz6Rc9ZY07IU55A")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record via WebSocket

Record 1:
  id: "91UyNfBN8A8dBS7UGeiuuboOBMSwkY6MyvhDAdEkWmuAsDe775Rx3p2r1f8gkwDIaXFQwJxqz6Rc9ZY07IU55A"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-10-08T05:52:13.349698Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Functions Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: CqavK1b1IsbZyk3eUnB5EHwJxu5S6QWAfmCXW8CVPu6ZVW4qq6oCkaZJlNw5TSusAixGw8eZ3MR86XBrZPsN5w
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: OYpXnJqcIv43MXJ7_STKpKOcTMtZQ9RNz0eh9bjFky2NQa8X0V_zPmk3mcJnaB43o0uKJGJnaRN2nzwVf8UB3Q
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Function saved: TjLRttEkg9uVLx7YVdnQgzHCpEOj0V1sPdFLDZJWZ4bnwPHEgwDZK6FljgQNNZtnIdjM6bK419y8q6O2FkNcig
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: TX_4OukLVHdM_Lv4tcdDJBYRU7PQzxCPqlinDGWi9LwLM3iSGNBM-7GOV7tSpF6cLj1GrDrH2-IbB7iRKKyYbA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":{"type":"Float","value":50.0},"count":{"type":"Integer","value":5},"max_score":{"type":"Integer","value":90},"status":{"type":"String","value":"inactive"}}
   {"avg_score":{"type":"Float","value":60.0},"count":{"type":"Integer","value":5},"max_score":{"type":"Integer","value":100},"status":{"type":"String","value":"active"}}

📝 Example 4: Function Management

📋 Total scripts: 4
🔍 Retrieved script: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: yDUjNrBui5OrE_OMrATj5uqME1SKlGxTme30vXHE4amdL9cChi87gXqtVG6KekmOEfkkuGd3ejMF6X3ltmVPOA
Created Bob: $500 - ID: rtMaQp8n0r2KGpP677b_LknfYrKWSgOHcFShC11StejR68r39TZq22_V5eozda2sgs_qxOmFS43aFCnGmkVxkA

=== Example 1: Begin Transaction ===
Transaction ID: 771e6c5c-4fb1-49ed-9e4d-5a551605c391

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: {"type":"Integer","value":800}
Bob: {"type":"Integer","value":700}

=== Example 5: Rollback ===
New transaction: e6840fc3-44aa-4921-948a-91722823cd34
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ Rust direct examples complete!
🛠️  Building client library...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
✅ Client build complete!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_advanced_crud`
=== ekoDB Advanced CRUD Example (Rust) ===

--- Inserting base document ---
Inserted: Widget Counter (id: II-BZviqn76en4JOjPLsVZtVCt79JM1ttJ63NTLEgxIftOYHH0scWYLbuZD8eFhq65dH0uu1etsdU43YbPDydw)

--- update_with_action: increment views by 10 ---
views after increment: Some(Object({"value": Integer(110), "type": String("Integer")}))

--- update_with_action: decrement views by 3 ---
views after decrement: Some(Object({"type": String("Integer"), "value": Integer(107)}))

--- update_with_action: multiply score by 2 ---
score after multiply: Some(Object({"type": String("Float"), "value": Float(9.0)}))

--- update_with_action: push 'ekodb' to tags ---
tags after push: Some(Object({"value": Array([String("rust"), String("database"), String("ekodb")]), "type": String("Array")}))

--- update_with_action: append '-suffix' to label ---
label after append: Some(Object({"value": String("prefix-suffix"), "type": String("String")}))

--- update_with_action: pop last element from tags ---
tags after pop: Some(Object({"type": String("Array"), "value": Array([String("rust"), String("database")])}))

--- update_with_action: remove 'rust' from tags ---
tags after remove: Some(Object({"type": String("Array"), "value": Array([String("database")])}))

--- update_with_action_sequence: increment views + push tag + append label ---
views after sequence: Some(Object({"value": Integer(157), "type": String("Integer")}))
tags after sequence: Some(Object({"type": String("Array"), "value": Array([String("database"), String("batch")])}))
label after sequence: Some(Object({"value": String("prefix-suffix-v2"), "type": String("String")}))

--- Cleanup ---
Deleted collection

=== All advanced CRUD operations completed ===
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_batch_operations`
✓ Client created

=== Batch Insert (via multiple inserts) ===
✓ Inserted 5 records
✓ Verified: Found 5 total records in collection

=== Update Records ===
✓ Updated 3 records

=== Delete Records ===
✓ Deleted 3 records

=== Cleanup ===
✓ Deleted collection

✓ All batch operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: MfFJcPic3Z34GoOm2uNvFfeD2ac-FPxdq0tQAC9p1yqVnt8Gd0JsOlqZN63DdvbNhUAAi76dfI8XKwGj2dDVEw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or additional products, let me know!

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
✓ Created second session: I1xLA46JOFhdEg1K3s_5QDqNWwI0vkxq0AGrDxJFIToWSaceKC-VKjsSZl5npFyXcy2e3tLECOZqkcm4kdI69Q
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: I1xLA46JOFhdEg1K3s_5QDqNWwI0vkxq0AGrDxJFIToWSaceKC-VKjsSZl5npFyXcy2e3tLECOZqkcm4kdI69Q
✓ Deleted chat session: MfFJcPic3Z34GoOm2uNvFfeD2ac-FPxdq0tQAC9p1yqVnt8Gd0JsOlqZN63DdvbNhUAAi76dfI8XKwGj2dDVEw
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: pcYsaHXOPcjzUqOEFbPUcpAGPiwnBSz3VQKH-LrLj7PxGaF7EiQ92OO4yIZ3mgLDd5rk4Km_ZCfkwdk8urx1cA

=== Sending Chat Message ===
Message ID: DtV-b8kfPXu_gENR5jx0923K2BtdPyCovxnAeeW51rMbHvQiHo_srCVWNI555vNtGPG3SEv1YCY9yfQXW2is1Q

=== AI Response ===
Response 1: ekoDB is a high-performance database that integrates intelligent caching, real-time capabilities, and AI features, making it suitable for applications requiring efficient data handling and interaction.

### Key Features:
1. **AI Chat Integration**: ekoDB allows you to query your database using natural language, enabling AI-powered responses with relevant context.

2. **Search Capabilities**:
   - **Full-Text Search**: Supports extensive keyword matching across records.
   - **Vector Search**: Utilizes vector embeddings for meaning-based searches.
   - **Hybrid Search**: Combines both full-text and vector search for comprehensive querying with automatic context retrieval.

These features enhance the usability and functionality of ekoDB, making it a versatile choice for developers looking to incorporate intelligent data querying and management into their applications.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["category", "content", "title"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("8z6Xr2y7GKYLKiEVFdDVpC1LARFDBD54gHnKvnerg2gxLdlP3Uq5sNfxZWseCCnt4EUlrFMeYm5JDzmc0zBOJg"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content", "category"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("D3VzJjVoBn4rzelsQwYXy0v5dqiXadZb1cxKfJ0vo2g8LxJDI-iMEmbu7n2x70NwISmRbC8kzcCyHXpy23AW4A"), "title": String("Search Features")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("MF3NAlOhroDXWXrET-Y33M7hMhJE2wj50OF8Jp79kPYOUDnc8wkO0jIrLHvbnK2ePUeGE3L-31Gf0Xvf8PwNwg"), "title": String("Introduction to ekoDB")}

Execution Time: 4688ms

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_message_stream`
=== ekoDB Chat Message Stream (SSE) Example (Rust) ===

Created session: bhLaVQLlGAc7y4mr7sQ9xFR8a-bALUK9caM4ZhaTO3OujT2iZSir7uxVoE7YWuB4k1rIpW0dxVgRc8y1MTSj8A

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}**ekoDB** is a comprehensive database developed to collect, integrate, and organize biological and environmental data, particularly focusing on the interactions between environmental factors and organisms. The term "ekoDB" can refer to different specific databases depending on the field, but it is most commonly known as:

### ekoDB: A Platform for Environmental and Biological Data Integration

**ekoDB** is a curated resource that provides structured information about ecological interactions, experimental data, and functional relationships in biology. The main features of ekoDB usually include:

- **Compendium of Environmental Interactions**: ekoDB catalogues how different organisms respond to environmental stimuli, such as temperature, chemicals, stress factors, or pollutants.
- **Gene and Pathway Information**: It often integrates data collections of genes, proteins, and pathways involved in environmental responses.
- **Experimental Data Curation**: It gathers data from published literature and experimental repositories to provide a single access point for ecological genomics research.
- **Analytical Tools**: ekoDB offers tools for querying, visualizing, and analyzing the effects of environmental changes on biological systems.

### Example: ekoDB for E. coli

One notable implementation is the **ekoDB for Escherichia coli (E. coli)**, which focuses on:

- Integrating experimental expression data (transcriptomics, proteomics) under different environmental conditions.
- Linking omics data with gene regulatory networks.
- Enabling researchers to investigate how E. coli adapts to changing environments.

### Related Databases

ekoDB-type frameworks can be applied to various organisms and ecosystems. They are related to systems biology and ecological databases such as EcoCyc, KEGG, or EnvO. Sometimes, project-specific databases use similar names (such as "ekoDB"), so it’s always good to check the context or organism in reference.

---

**In summary:**
ekoDB is a specialized database (or database framework) designed to help researchers study the complex relationships between organisms and their environment, enabling systems-level analysis of ecological and biological data.

If you have a specific field or organism in mind (“ekoDB” for a particular use case), please let me know for a more targeted explanation!

--- Stream complete ---
Message ID: 3kQo6uRt4hdn4MsoC9I9Ko2CTohoKIRK_ptiHGQelJ-R4qAh44h7o_p4FvnC_IvJaAd-6FX9WtIkV1zlG3ntIw
Execution time: 4056ms
Context window: 1000000 tokens

✓ Chat message stream example completed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_models`
✓ Client created

=== Get All Chat Models ===
Available chat models by provider:

OpenAI models (135):
  - o3-2025-04-16
  - gpt-5.1-chat-latest
  - gpt-5.3-chat-latest
  - gpt-5.2-2025-12-11
  - sora-2
  - chatgpt-image-latest
  - gpt-4.1-mini
  - gpt-3.5-turbo
  - gpt-4o-mini
  - gpt-image-2.5-sunburst
  - gpt-image-2.5-flare-2026-09-08
  - gpt-6-luna
  - gpt-5.4-mini-2026-03-17
  - gpt-4-turbo
  - gpt-4.1-nano
  - gpt-image-2
  - gpt-5.4-pro
  - gpt-realtime-mini-2025-12-15
  - gpt-3.5-turbo-16k
  - babbage-002
  - text-embedding-ada-002
  - gpt-4o-mini-tts
  - gpt-4o-2024-08-06
  - o4-mini-2025-04-16
  - gpt-4o-2024-11-20
  - omni-moderation-latest
  - gpt-realtime
  - tts-1-hd
  - gpt-realtime-1.5
  - gpt-realtime-2
  - gpt-5-search-api
  - gpt-5.5-2026-04-23
  - gpt-5.1
  - gpt-5.1-codex
  - gpt-4o
  - o1-2024-12-17
  - gpt-5.2-chat-latest
  - gpt-4o-mini-tts-2025-12-15
  - gpt-image-1
  - davinci-002
  - gpt-image-2-2026-04-21
  - gpt-4o-mini-transcribe
  - o3-mini
  - gpt-audio
  - gpt-5.4-nano
  - o3
  - gpt-5.1-codex-max
  - gpt-realtime-2.1
  - gpt-4.1
  - gpt-5.5
  - gpt-4o-mini-search-preview-2025-03-11
  - text-embedding-3-large
  - gpt-4o-mini-search-preview
  - gpt-4o-2024-05-13
  - gpt-6-sol
  - gpt-5.3-codex
  - gpt-3.5-turbo-instruct
  - sora-2-pro
  - gpt-5
  - gpt-audio-mini-2025-12-15
  - gpt-transcribe
  - o4-mini-deep-research-2025-06-26
  - o1-pro-2025-03-19
  - gpt-4.1-nano-2025-04-14
  - gpt-5-search-api-2025-10-14
  - gpt-4-turbo-2024-04-09
  - gpt-realtime-2.1-mini
  - tts-1
  - gpt-5-mini
  - omni-moderation-2024-09-26
  - gpt-5-mini-2025-08-07
  - gpt-realtime-2025-08-28
  - gpt-5.4-2026-03-05
  - gpt-live-transcribe
  - gpt-3.5-turbo-0125
  - gpt-5-nano
  - gpt-5.6-luna
  - gpt-4
  - gpt-image-2.5-sunburst-2026-09-08
  - whisper-1
  - gpt-5.4-mini
  - gpt-5-pro
  - o1-pro
  - gpt-realtime-whisper
  - gpt-5.2-pro
  - gpt-5-chat-latest
  - gpt-live-1
  - gpt-4o-mini-transcribe-2025-12-15
  - gpt-5-2025-08-07
  - tts-1-1106
  - gpt-5.5-pro
  - gpt-5-pro-2025-10-06
  - chat-latest
  - gpt-5.2-pro-2025-12-11
  - gpt-4o-mini-transcribe-2025-03-20
  - gpt-audio-2025-08-28
  - gpt-5.4-pro-2026-03-05
  - gpt-realtime-translate
  - gpt-5.6-sol
  - gpt-audio-mini-2025-10-06
  - gpt-5.5-pro-2026-04-23
  - tts-1-hd-1106
  - gpt-realtime-mini
  - gpt-5.4
  - gpt-4o-transcribe-diarize
  - gpt-5-codex
  - gpt-image-1-mini
  - gpt-image-1.5
  - gpt-5.1-2025-11-13
  - o4-mini-deep-research
  - gpt-4o-search-preview-2025-03-11
  - o3-mini-2025-01-31
  - gpt-4-0613
  - gpt-4o-search-preview
  - gpt-5.6-terra
  - text-embedding-3-small
  - gpt-audio-1.5
  - gpt-3.5-turbo-instruct-0914
  - gpt-6-astra
  - gpt-5.2-codex
  - gpt-3.5-turbo-1106
  - gpt-5-nano-2025-08-07
  - gpt-audio-mini
  - gpt-4o-mini-tts-2025-03-20
  - gpt-5.2
  - gpt-4.1-mini-2025-04-14
  - gpt-4.1-2025-04-14
  - gpt-4o-mini-2024-07-18
  - o4-mini
  - o1
  - gpt-5.4-nano-2026-03-17
  - gpt-5.1-codex-mini
  - gpt-4o-transcribe
  - gpt-image-2.5-flare
  - gpt-6.1-sol

Anthropic models (14):
  - claude-haiku-5-5
  - claude-sonnet-5-5
  - claude-opus-5-5
  - claude-fable-5-1
  - claude-opus-5
  - claude-sonnet-5
  - claude-fable-5
  - claude-opus-4-8
  - claude-opus-4-7
  - claude-sonnet-4-6
  - claude-opus-4-6
  - claude-opus-4-5-20251101
  - claude-haiku-4-5-20251001
  - claude-sonnet-4-5-20250929

Perplexity models (0):

Gemini models (0):

Provider status:
  anthropic: ok 14 models
  gemini: not_configured (unverified) No Gemini API Key
  openai: ok 135 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Models for Specific Provider ===

openai models (135):
  - o3-2025-04-16
  - gpt-5.1-chat-latest
  - gpt-5.3-chat-latest
  - gpt-5.2-2025-12-11
  - sora-2
  ... and 130 more

anthropic models (14):
  - claude-haiku-5-5
  - claude-sonnet-5-5
  - claude-opus-5-5
  - claude-fable-5-1
  - claude-opus-5
  ... and 9 more
GetChatModel(perplexity) error: Record not found

✓ Chat Models API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: fbUzyFs30YCVWatZ3mIH1zuzO0k8b8_OztPh8fu-24Kt3wVmyy0sz5dTzxis5ODZKRC74GbE6z50YcRfWmawAw
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Product Name:** ekoDB
- **Description:** A high-performance database product with AI capabilities
- **Price:** $99

If you need more information or have other questions, feel free to ask!

✓ Message 2 sent
  Response: The price of the product ekoDB is **$99**. If you have any more questions or need further information, let me know!

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: w1rA1Un8gureoWeSOAOLreTDomUgBkV24ImMoMTOySPdzx850I0ZLMI3uzUl_Pc3PhZOV06PwfYXoKm4dEjrpQ
  Parent: fbUzyFs30YCVWatZ3mIH1zuzO0k8b8_OztPh8fu-24Kt3wVmyy0sz5dTzxis5ODZKRC74GbE6z50YcRfWmawAw

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: w1rA1Un8gureoWeSOAOLreTDomUgBkV24ImMoMTOySPdzx850I0ZLMI3uzUl_Pc3PhZOV06PwfYXoKm4dEjrpQ (Untitled)
  Session 2: fbUzyFs30YCVWatZ3mIH1zuzO0k8b8_OztPh8fu-24Kt3wVmyy0sz5dTzxis5ODZKRC74GbE6z50YcRfWmawAw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: w1rA1Un8gureoWeSOAOLreTDomUgBkV24ImMoMTOySPdzx850I0ZLMI3uzUl_Pc3PhZOV06PwfYXoKm4dEjrpQ

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "gyV22EGffPkvXyuXlGF8zd59czSsqs1kRYg0CWkazvasrEPZj4n2I-cU_sFO-PPsjUIxbowdDdDKl8X18tLkIg"

=== List Collections ===
Total collections: 7
Sample collections: ["audit__ek0_testing", "agent_function_versions__ek0_testing", "chat_configurations__ek0_testing", "chat_messages__ek0_testing", "chat_turns__ek0_testing"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_collection_utils`
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test_rust' exists: false

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test_rust' exists: true

=== Count Documents ===
Document count in 'collection_utils_test_rust': 5

=== List Collections ===
Total collections: 7
  - audit__ek0_testing
  - agent_function_versions__ek0_testing
  - chat_configurations__ek0_testing
  - chat_messages__ek0_testing
  - chat_turns__ek0_testing
  - functions__ek0_testing
  - collection_utils_test_rust <-- our test

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_rust'

✓ Collection Utilities example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_concurrency_stages`
✓ Client created
✓ conc_demo_pay saved
✓ conc_demo_rl_fail saved
✓ conc_demo_rl_skip saved
✓ conc_demo_lock saved

Invoke them like:
  POST /api/functions/conc_demo_pay        { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail    { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip    { "user_id": 42 }
  POST /api/functions/conc_demo_lock       { "resource": "queue:drain" }

✓ Cleaned up demo functions
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("OxkKz6aGpSfeGgLnOutipXKpSwkzlf6CMKPElDNSzU1x5isk84u96IeyKSI9HIKDmJvDUzgvolX2RcMqo0Zb3A")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("bob-user-id")})
✓ Second upsert (update): Record({"name": Object({"type": String("String"), "value": String("Bob Smith")}), "id": String("bob-user-id"), "email": Object({"type": String("String"), "value": String("bob.smith@newdomain.com")}), "age": Object({"type": String("Integer"), "value": Integer(36)})})

=== Find One Operation ===
✓ Found user by email: Record({"name": Object({"value": String("Alice Johnson"), "type": String("String")}), "id": String("OxkKz6aGpSfeGgLnOutipXKpSwkzlf6CMKPElDNSzU1x5isk84u96IeyKSI9HIKDmJvDUzgvolX2RcMqo0Zb3A"), "email": Object({"type": String("String"), "value": String("alice@example.com")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "age": Object({"type": String("Integer"), "value": Integer(28)})})
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: true
✓ Fake record exists: false (should be false)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 7 records (expected 7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_crypto_stages`
✓ Client created
✓ crypto_demo_hmac_rs saved
✓ crypto_demo_aes_rs saved
✓ crypto_demo_uuid_rs saved
✓ crypto_demo_totp_rs saved
✓ crypto_demo_encoding_rs saved

All crypto-stage demos defined. Invoke any of them with:
  POST /api/functions/crypto_demo_hmac_rs { "payload": "hi" }
  POST /api/functions/crypto_demo_aes_rs { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid_rs
  POST /api/functions/crypto_demo_totp_rs
  POST /api/functions/crypto_demo_encoding_rs { "title": "Héllo World" }

✓ Cleaned up demo functions
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_distinct_values`
=== ekoDB Distinct Values Example ===

=== Inserting Sample Products ===
Inserted 8 products

=== Distinct Categories (all products) ===
Found 3 distinct categories:
  - {"type":"String","value":"books"}
  - {"type":"String","value":"clothing"}
  - {"type":"String","value":"electronics"}

=== Distinct Statuses (all products) ===
Found 3 distinct statuses:
  - {"type":"String","value":"active"}
  - {"type":"String","value":"archived"}
  - {"type":"String","value":"discontinued"}

=== Distinct Statuses in Electronics ===
Found 2 distinct statuses for electronics:
  - {"type":"String","value":"active"}
  - {"type":"String","value":"discontinued"}

Cleanup done.
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "87gp2kFf2UIuDVUz-ClBhslgkcyvlCihgrrt8K1jfPReUmljXeJr5xUpVMJ1_54GzvjClk7SUS-VENJ7v0dMdw"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("JRqsHqkEFCoipoLYytgxdHesbBxJfAPtLyV34bXYoTeBvuanplgJNFr8Q7KSYJ-sEybMq9qV-PE3KD4JtkOghg"))

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_edge_cache`
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: BZfHHlPFgdAF3NNckgSLgltmMLULbXztm3nDo9_ENLmW_iOZQk0hLZnADIF89I7B1MhU1aEFw35OISXZJrQ8Ig

Call 1: Cache lookup
Response time: 1ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 1ms
Found 1 cached entries

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

🧹 Cleaning up...
✓ Cleanup complete

✓ Example complete!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_function_composition`
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user_rs
✅ Saved composed function: get_user_wrapper_rs (calls fetch_user_rs + projects fields)

📊 Result from composed function:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user_rs (uses KV)
✅ Saved SWR function using composition: swr_user_rs

First call (cache miss - will fetch from API):
   ⏱️  Duration: 86.836292ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "phone": "1-770-736-8031 x56442",
      "email": "Sincere@april.biz",
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net",
        "n...

Second call (cache hit - from cache):
   ⏱️  Duration: 5.432834ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "email": "Sincere@april.biz",
      "phone": "1-770-736-8031 x56442",
      "company": {
        "catchPhrase": "Multi-layered client-server n...
   🚀 Cache speedup: 16.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user_rs
✅ Level 2 function: fetch_slim_user_rs (calls validate_user_rs)
✅ Level 3 function: get_verified_user_rs (calls fetch_slim_user_rs)

📊 Result from 3-level nested composition:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_enriched_user: Used in 50 workflows
   - get_engineering_user: Specific workflow


✅ Cleanup complete
✅ All composition examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_function_contract`
client_function_contract: ok
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Functions Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: 2_csFxAhzWAwoIjZyw_brkALPffcYvHfkZta50DU4kfxTmZzPB7qywiYxKA9lt6nAZcSJsg9CprwpoYFag367g
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: YXmVp-PM-HvYuXfbR_bhPCrjwCCg3yMq1yL2RT77--0nk7BH3QDaLj999-wHTsctPG2Ek84qtqm5EDe6UXPSjA
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: FPS1aB8d4MBFPCpiIkHj8UBw8ANH-81aEQxGY-NNr0ikSNRZ2lAUR1A_fmHnD4rEku_SePaC6ins2p-Pqqt3QQ
📊 Statistics: 2 groups

📝 Example 4: Function Management

📋 Total functions: 3
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_advanced`
🚀 ekoDB Rust Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   Record({"category": Object({"value": String("Electronics"), "type": String("String")}), "avg_price": Object({"value": Float(367.0), "type": String("Float")}), "count": Object({"value": Integer(5), "type": String("Integer")})})
   Record({"count": Object({"value": Integer(3), "type": String("Integer")}), "avg_price": Object({"type": String("Float"), "value": Float(365.6666666666667)}), "category": Object({"type": String("String"), "value": String("Furniture")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits:

1. **Efficient Storage of High-Dimensional Data**: They are optimized for storing and querying high-dimensional vectors, which is essential for applications like machine learning and natural language processing.

2. **Fast Similarity Search**: Vector databases enable rapid nearest neighbor searches, allowing for quick retrieval of similar items based on their vector representations.

3. **Scalability**: They are designed to handle large datasets and can easily scale as data grows.

4. **Support for Complex Queries**: Advanced querying capabilities such as cosine similarity, Euclidean distance, and more complex similarity functions can be utilized.

5. **Enhanced Recall and Ranking**: Using embeddings in vector form improves the relevance and accuracy of search results.

6. **Integration with AI/ML Workflows**: They seamlessly integrate into AI and machine learning pipelines, making it easier to deploy AI models.

7. **Versatile Applications**: Useful in various fields such as recommendation systems, image search, natural language understanding, and more.

8. **Real-time Performance**: Many vector databases are optimized for low-latency queries, making them suitable for real-time applications.

Overall, vector databases excel at managing and retrieving complex, high-dimensional data efficiently.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
📊 Generated 2 embeddings
   Dimensions: 1536
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions_complete`
🚀 ekoDB Rust Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: L-uwhp2i-rVri2d8VAmhM4mHdIBsZqU6J8sNvaxnE5j4MxCi3yMAbPmmOKO6lsnokchJqpVAxrkfQW3V4sFflg
📊 Found 2 product groups
   Record({"count": Object({"value": Integer(2), "type": String("Integer")}), "category": Object({"value": String("Furniture"), "type": String("String")}), "avg_price": Object({"type": String("Float"), "value": Float(474.0)})})
   Record({"count": Object({"type": String("Integer"), "value": Integer(3)}), "category": Object({"type": String("String"), "value": String("Electronics")}), "avg_price": Object({"type": String("Float"), "value": Float(575.6666666666666)})})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   Record({"count": Object({"type": String("Integer"), "value": Integer(2)}), "category": Object({"type": String("String"), "value": String("Furniture")})})
   Record({"category": Object({"value": String("Electronics"), "type": String("String")}), "count": Object({"value": Integer(3), "type": String("Integer")})})
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 3 products
⏱️  Execution time: 0ms

📝 Example 5: function with Parameter Definition

✅ Function saved
📊 Found 3 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_functions_crud`
🚀 ekoDB Rust CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   Record({"status": Object({"value": String("inactive"), "type": String("String")}), "count": Object({"type": String("Integer"), "value": Integer(3)})})
   Record({"status": Object({"value": String("active"), "type": String("String")}), "count": Object({"value": Integer(7), "type": String("Integer")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("BtAmN-5vKL-47iNxetRQmpPwH7fGLbzf3C5IuHqT1q0adXrNUkphx9fBtnpyANUUjRr1NdAOyu0IV4_CUDh78w"))
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: zmvRtIYZtY_WWVd3G__k5xWA6veM7jy_RjoFUZmb9ygrSU5X3WkCysNipkSSkiAn3N_JcgA-3pv_LPoCq9MsuA
📊 function executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"type": String("Object"), "value": Object {"role": String("admin"), "userId": String("user_abc")}})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: PSbfs6q22VH4cBrIO5sbXIVf_49ztwRoTRnDrvio_v7rAeVoqqVmDJ-zH5MsyXLeV0WEAlSLaA44sHhDfgk2Dg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: atmTXoZMgfuREh8SPr_zZCcDrypdEmvd2F-CBR0VjjKfb1qojINy4FzRFCkdoXEanbQtHIcBrAmGoVxZ8Upd-g
📊 Processed order with caching
⏱️  Stages executed: 1
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use FieldType variants for type-safe wrapped values
   ✅ FieldType::Decimal preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ KV operations work within scripts
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Database Design Principles (Database)
   2. Getting Started with ekoDB (Database)
   3. Introduction to Machine Learning (AI)
   4. Vector Databases Explained (Database)
   5. Natural Language Processing (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   Record({"count": Object({"type": String("Integer"), "value": Integer(3)}), "category": Object({"type": String("String"), "value": String("Database")})})
   Record({"count": Object({"value": Integer(2), "type": String("Integer")}), "category": Object({"value": String("AI"), "type": String("String")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_goal_templates`
=== ekoDB Goal Template CRUD Example (Rust) ===

--- Creating goal template ---
Created template: Data Migration (id: sC07AHPR9Xs4yuH1vxyfpZxNFKwi3UZKmboEZ3sxLdIqVupYrKsh95ERVpqxHrXCkjgOKK-nuwMEvbksO8dkzA)

--- Listing templates ---
Templates: {"count":1,"items":[{"description":{"type":"String","value":"Template for migrating data between schemas"},"id":"sC07AHPR9Xs4yuH1vxyfpZxNFKwi3UZKmboEZ3sxLdIqVupYrKsh95ERVpqxHrXCkjgOKK-nuwMEvbksO8dkzA","steps":{"type":"Array","value":[{"description":"Analyze source schema"},{"description":"Create target schema"},{"description":"Migrate records"},{"description":"Validate results"}]},"title":{"type":"String","value":"Data Migration"}}]}

--- Getting template ---
Fetched: {"type":"String","value":"Data Migration"}

--- Updating template ---
Updated description: {"type":"String","value":"Updated: comprehensive data migration workflow"}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_goals_tasks_agents`
=== ekoDB Goals, Tasks & Agents Integration Example (Rust) ===

--- Goal: create ---
Created goal: "Deploy v2.0 to production" (id: L5O60WyBEwXluXFPgKBwYrNjbSddR9dQnfELAecRyLDIo_9i7CFSjngPcBJRamofwnhEU7lDRNbD_q0LcPwqEg)

--- Goal: list ---
Goals: {"count":1,"goals":[{"created_at":"2026-10-08T05:20:40.676111+00:00","description":"Full release cycle for version 2.0","id":"L5O60WyBEwXluXFPgKBwYrNjbSddR9dQnfELAecRyLDIo_9i7CFSjngPcBJRamofwnhEU7lDRNbD_q0LcPwqEg","status":"pending","steps":"[{\"description\":\"Run integration tests\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]","title":"Deploy v2.0 to production","updated_at":"2026-10-08T05:20:40.676111+00:00"}]}

--- Goal: get ---
Fetched goal: {"type":"String","value":"Deploy v2.0 to production"}

--- Goal: update ---
Updated description: {"type":"String","value":"Updated: full v2.0 release with rollback plan"}

--- Goal: search ---
Search results: {"count":1,"items":[{"_score":12.87,"created_at":{"type":"DateTime","value":"2026-10-08T05:20:40.676111+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"L5O60WyBEwXluXFPgKBwYrNjbSddR9dQnfELAecRyLDIo_9i7CFSjngPcBJRamofwnhEU7lDRNbD_q0LcPwqEg","status":{"type":"String","value":"pending"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-10-08T05:20:40.713651+00:00"}}]}

--- Goal step: start (step 0) ---
Step 0 started: {"created_at":{"type":"DateTime","value":"2026-10-08T05:20:40.676111+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"L5O60WyBEwXluXFPgKBwYrNjbSddR9dQnfELAecRyLDIo_9i7CFSjngPcBJRamofwnhEU7lDRNbD_q0LcPwqEg","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"status\":\"InProgress\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-10-08T05:20:40.723530+00:00"}}

--- Goal step: complete (step 0) ---
Step 0 completed: {"created_at":{"type":"DateTime","value":"2026-10-08T05:20:40.676111+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"L5O60WyBEwXluXFPgKBwYrNjbSddR9dQnfELAecRyLDIo_9i7CFSjngPcBJRamofwnhEU7lDRNbD_q0LcPwqEg","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-10-08T05:20:40.731275+00:00"}}

--- Goal step: start (step 1) ---
Step 1 started

--- Goal step: fail (step 1) ---
Step 1 failed: {"created_at":{"type":"DateTime","value":"2026-10-08T05:20:40.676111+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"L5O60WyBEwXluXFPgKBwYrNjbSddR9dQnfELAecRyLDIo_9i7CFSjngPcBJRamofwnhEU7lDRNbD_q0LcPwqEg","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-10-08T05:20:40.750912+00:00"}}

--- Goal: complete ---
Goal completed (pending review): {"completed_at":{"type":"DateTime","value":"2026-10-08T05:20:40.760945+00:00"},"created_at":{"type":"DateTime","value":"2026-10-08T05:20:40.676111+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"L5O60WyBEwXluXFPgKBwYrNjbSddR9dQnfELAecRyLDIo_9i7CFSjngPcBJRamofwnhEU7lDRNbD_q0LcPwqEg","status":{"type":"String","value":"pending_review"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"summary":{"type":"String","value":"Partial completion — step 1 failed"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-10-08T05:20:40.760945+00:00"}}

--- Goal: approve ---
Goal approved: {"completed_at":{"type":"DateTime","value":"2026-10-08T05:20:40.760945+00:00"},"created_at":{"type":"DateTime","value":"2026-10-08T05:20:40.676111+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"L5O60WyBEwXluXFPgKBwYrNjbSddR9dQnfELAecRyLDIo_9i7CFSjngPcBJRamofwnhEU7lDRNbD_q0LcPwqEg","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"summary":{"type":"String","value":"Partial completion — step 1 failed"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-10-08T05:20:40.773686+00:00"}}

--- Goal: create (for rejection) ---
Created goal2: "Refactor auth module" (id: SPZIjc4jyw32d-3nRcsrrX4sZRWc9Ji-3eYysc2f3xyeOeHKTRRtuy79fNwfSerb9Di0PH45dfjREsfKnYzSeA)

--- Goal: complete (goal2) ---

--- Goal: reject ---
Goal rejected: {"completed_at":{"type":"DateTime","value":"2026-10-08T05:20:40.799844+00:00"},"created_at":{"type":"DateTime","value":"2026-10-08T05:20:40.785881+00:00"},"description":{"type":"String","value":"Rewrite JWT handling"},"id":"SPZIjc4jyw32d-3nRcsrrX4sZRWc9Ji-3eYysc2f3xyeOeHKTRRtuy79fNwfSerb9Di0PH45dfjREsfKnYzSeA","reason":{"type":"String","value":"Breaks backward compatibility"},"status":{"type":"String","value":"failed"},"steps":{"type":"String","value":"[]"},"summary":{"type":"String","value":"Completed refactor"},"title":{"type":"String","value":"Refactor auth module"},"updated_at":{"type":"DateTime","value":"2026-10-08T05:20:40.799844+00:00"}}

--- Task: create ---
Created task: Nightly backup (id: R3Skg3BjNKqLhaXvpBe03QCOZ7-OI0wuay8PFqGVKDPPejwm6TNATAcGJjXMMfnrI7aVgZyWsIFpXhk-1kaEOA)

--- Task: list ---
Tasks: {"count":1,"items":[{"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"R3Skg3BjNKqLhaXvpBe03QCOZ7-OI0wuay8PFqGVKDPPejwm6TNATAcGJjXMMfnrI7aVgZyWsIFpXhk-1kaEOA","max_retries":{"type":"Integer","value":3},"schedule":{"type":"String","value":"0 2 * * *"},"title":{"type":"String","value":"Nightly backup"}}]}

--- Task: get ---
Fetched task: {"type":"String","value":"Nightly backup"}

--- Task: start ---
Task started: {"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"R3Skg3BjNKqLhaXvpBe03QCOZ7-OI0wuay8PFqGVKDPPejwm6TNATAcGJjXMMfnrI7aVgZyWsIFpXhk-1kaEOA","max_retries":{"type":"Integer","value":3},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"running"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-10-08T05:20:40.848744+00:00"}}

--- Task: succeed ---
Task succeeded: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"R3Skg3BjNKqLhaXvpBe03QCOZ7-OI0wuay8PFqGVKDPPejwm6TNATAcGJjXMMfnrI7aVgZyWsIFpXhk-1kaEOA","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-10-08T05:20:40.857427+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-10-08T05:20:40.857427+00:00"}}

--- Task: start (round 2) ---

--- Task: pause ---
Task paused: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"R3Skg3BjNKqLhaXvpBe03QCOZ7-OI0wuay8PFqGVKDPPejwm6TNATAcGJjXMMfnrI7aVgZyWsIFpXhk-1kaEOA","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-10-08T05:20:40.857427+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"paused"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-10-08T05:20:40.879320+00:00"}}

--- Task: resume ---
Task resumed: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"R3Skg3BjNKqLhaXvpBe03QCOZ7-OI0wuay8PFqGVKDPPejwm6TNATAcGJjXMMfnrI7aVgZyWsIFpXhk-1kaEOA","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-10-08T05:20:40.857427+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-10-08T05:20:40.888491+00:00"}}

--- Task: fail ---
Task failed: {"consecutive_failures":{"type":"Integer","value":1},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"R3Skg3BjNKqLhaXvpBe03QCOZ7-OI0wuay8PFqGVKDPPejwm6TNATAcGJjXMMfnrI7aVgZyWsIFpXhk-1kaEOA","last_error":{"type":"String","value":"S3 bucket access denied"},"last_run":{"type":"DateTime","value":"2026-10-08T05:20:40.897765+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-10-08T05:20:40.897765+00:00"}}

--- Task: due ---
Due tasks: {"count":0,"items":[]}

--- Task: delete ---
Task deleted

--- Agent: create ---
Created agent: backup-agent (id: bxACy7_MLHE28jxLk2agLJZ0wcYkXR1HIcvto3nJ_iUPp01HO9yneF6X6A4lF6cQIQM6l4tAoZVfzg-Nc4UkMg)

--- Agent: list ---
Agents: {"count":1,"items":[{"capabilities":{"type":"Array","value":["backup","restore","verify"]},"deployment_id":{"type":"String","value":"deploy-abc-123"},"description":{"type":"String","value":"Handles nightly backups and restores"},"id":"bxACy7_MLHE28jxLk2agLJZ0wcYkXR1HIcvto3nJ_iUPp01HO9yneF6X6A4lF6cQIQM6l4tAoZVfzg-Nc4UkMg","llm_model":{"type":"String","value":"gpt-4.1"},"name":{"type":"String","value":"backup-agent"}}]}

--- Agent: get ---
Fetched agent: {"type":"String","value":"backup-agent"}

--- Agent: get_by_name ---
Found by name: {"type":"String","value":"backup-agent"}

--- Agent: update ---
Updated agent description: {"type":"String","value":"Handles backups, restores, and disaster recovery"}

--- Agent: agents_by_deployment ---
Agents in deployment: {"count":0,"items":[]}
WARNING: agents_by_deployment omitted created agent bxACy7_MLHE28jxLk2agLJZ0wcYkXR1HIcvto3nJ_iUPp01HO9yneF6X6A4lF6cQIQM6l4tAoZVfzg-Nc4UkMg; TODO: check/fix the server-side deployment lookup

--- Agent: delete ---
Agent deleted

--- Goal: delete ---
Goal 1 deleted
Goal 2 deleted

=== All goals, tasks & agents operations completed ===
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_joins`
=== ekoDB Rust Client - Join Operations Example ===

=== Setting up sample data ===
✓ Sample data created

=== Example 1: Single collection join (users with departments) ===
✓ Found 2 users with department data
  - user 1: name present=true, joined departments=1
  - user 2: name present=true, joined departments=1

=== Example 2: Join with filtering ===
✓ Found 1 users in Engineering
  - user 1: name present=true, department location present=true

=== Example 3: Join with user profiles ===
✓ Found 2 users with profile data
  - user 1: name present=true, profile bio present=true
  - user 2: name present=true, profile bio present=true

=== Example 4: Join orders with user data ===
✓ Found 2 completed orders
  - order 1: product present=true, amount present=true, joined user name present=true
  - order 2: product present=true, amount present=true, joined user name present=true

=== Example 5: Complex join with multiple conditions ===
✓ Found 2 users with example.com emails
  - user 1: name present=true, email domain matched=true, joined departments=1
  - user 2: name present=true, email domain matched=true, joined departments=1

=== Cleanup ===
✓ Deleted test collections

✓ Join operations example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_jwt_auth_flow`
✓ Client created
✓ rs_users_register saved
✓ rs_users_login saved
✓ rs_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/rs_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/rs_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/rs_users_verify_token { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking — the {{env.JWT_SECRET}} placeholder reads from that whitelist, NEVER from the function definition.

✓ Cleaned up demo functions
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_kv_links`
=== ekoDB KV Document Linking Example (Rust) ===

--- Inserting documents ---
Inserted doc1: Project Alpha (id: -VrrKn9TZSV24Y0ml0Ex-MxnCtrXzfIZmo21psVBQjqDrdJeWFhICxsfi12gBl-yBguTtttshcfV3ifc2ZCKZw)
Inserted doc2: Project Beta (id: oC8QklYSmg-BHJCHAG11lcNGH0H6_r52DoX_Kp9QO0U0wrd6RzZDt9zmHo39kjolh4uiuIZsYgoRY-bux9YYag)

--- Setting KV key ---
Set key: kv_links:rs:user:alice:projects

--- Linking documents ---
Linked doc1: null
Linked doc2: null

--- Getting links ---
Links for kv_links:rs:user:alice:projects: [{"collection":"kv_links_example_rs","created_at":"2026-10-08T05:23:52.905925Z","document_id":"oC8QklYSmg-BHJCHAG11lcNGH0H6_r52DoX_Kp9QO0U0wrd6RzZDt9zmHo39kjolh4uiuIZsYgoRY-bux9YYag","field_path":null,"last_accessed":"2026-10-08T05:23:52.906723Z","metadata":{}},{"collection":"kv_links_example_rs","created_at":"2026-10-08T05:23:52.904797Z","document_id":"-VrrKn9TZSV24Y0ml0Ex-MxnCtrXzfIZmo21psVBQjqDrdJeWFhICxsfi12gBl-yBguTtttshcfV3ifc2ZCKZw","field_path":null,"last_accessed":"2026-10-08T05:23:52.906723Z","metadata":{}}]

--- Unlinking doc1 ---
Unlinked doc1: null

--- Verifying remaining links ---
Remaining links: [{"collection":"kv_links_example_rs","created_at":"2026-10-08T05:23:52.905925Z","document_id":"oC8QklYSmg-BHJCHAG11lcNGH0H6_r52DoX_Kp9QO0U0wrd6RzZDt9zmHo39kjolh4uiuIZsYgoRY-bux9YYag","field_path":null,"last_accessed":"2026-10-08T05:23:52.908199Z","metadata":{}}]

--- Cleanup ---
Unlinked remaining document
Deleted KV key
Deleted documents
Deleted collection

=== All KV linking operations completed ===
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_kv_operations`
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: Object {"type": String("Object"), "value": Object {"userId": Number(123), "username": String("john_doe")}}

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_operations:rs:cache:product:1: success
  kv_operations:rs:cache:product:2: success
  kv_operations:rs:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_operations:rs:cache:product:1: Record({"name": String("Product 1"), "price": Float(29.99)})
  kv_operations:rs:cache:product:2: Record({"price": Float(39.989999999999995), "name": String("Product 2")})
  kv_operations:rs:cache:product:3: Record({"name": String("Product 3"), "price": Float(49.989999999999995)})

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  kv_operations:rs:cache:product:1: deleted
  kv_operations:rs:cache:product:2: deleted
  kv_operations:rs:cache:product:3: deleted

✓ All KV operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_kv_precision`
=== KV Precision: Float vs Decimal ===

=== Test 1: Using f64 Floats (MAY LOSE PRECISION) ===
Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) OK
  Widget B: $39.99 (expected $39.99) OK
  Widget C: $49.99 (expected $49.99) OK

=== Test 2: Using FieldType::decimal() (PRESERVES PRECISION) ===
Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99) OK
  Widget B: $39.99 (expected $39.99) OK
  Widget C: $49.99 (expected $49.99) OK

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.30000000000000004 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Cleanup ===
Cleaned up test keys

=== Summary ===
Use FieldType::decimal() for monetary values, percentages, and
any case where floating-point errors are unacceptable.
Decimal values are stored as exact strings internally,
preserving precision across all operations.
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_path_routed_function`
✓ Client created
✓ rs_route_admin → GET /api/route/users/admin
✓ rs_route_user_by_id → GET /api/route/users/:id
✓ rs_route_user_posts → GET /api/route/users/:id/posts/:post_id
✓ rs_route_org_create_member → POST /api/route/orgs/:org/members

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Prefix Query (StartsWith) ===
✓ Found 1 users with names starting with A
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"value": String("Charlie"), "type": String("String")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
  2. Some(Object({"type": String("String"), "value": String("David")}))
  3. Some(Object({"type": String("String"), "value": String("Alice")}))

=== Pagination (page 2, size 2) ===
✓ Page 2 results:
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== NOT Operator ===
✓ Found 4 users NOT inactive

=== Complex Chained Query ===
✓ Found 3 users with all conditions

=== Cleanup ===
✓ Deleted collection

✓ All query builder operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_raw_completion_stream`
=== ekoDB Raw Completion Stream (SSE) Example ===

--- Basic SSE Raw Completion ---
Response: The three primary colors are red, blue, and yellow.

--- Structured Output via SSE ---
JSON response: [
  {
    "name": "Mercury",
    "diameter_km": 4879
  },
  {
    "name": "Jupiter",
    "diameter_km": 139820
  },
  {
    "name": "Neptune",
    "diameter_km": 49244
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.57s
     Running `target/debug/examples/client_schedules`
=== ekoDB Schedule Management Example (Rust) ===

--- Creating schedule ---
Created schedule: "nightly-cleanup" (id: b1771f0c-2ae8-4287-b578-61db1f9b92b9)

--- Listing schedules ---
Schedules: {"count":1,"schedules":[{"created_at":"2026-10-08T05:30:52.131154Z","cron_expression":"0 0 3 * * *","description":"Remove expired sessions and temp files","enabled":true,"function_label":"schedule_noop_rust_68813","id":"b1771f0c-2ae8-4287-b578-61db1f9b92b9","last_execution":null,"name":"nightly-cleanup","next_execution":"2026-10-09T03:00:00Z","parameters":{},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-10-08T05:30:52.131154Z"}]}

--- Getting schedule ---
Fetched schedule: "nightly-cleanup"

--- Updating schedule ---
Updated cron: "0 0 4 * * *"
Updated description: "Remove expired sessions, temp files, and orphaned uploads"

--- Triggering schedule ---
Trigger response: {"schedule_id":"b1771f0c-2ae8-4287-b578-61db1f9b92b9","status":"triggered"}

--- Pausing schedule ---
Schedule paused: {"created_at":"2026-10-08T05:30:52.131154Z","cron_expression":"0 0 4 * * *","description":"Remove expired sessions, temp files, and orphaned uploads","enabled":false,"function_label":"schedule_noop_rust_68813","id":"b1771f0c-2ae8-4287-b578-61db1f9b92b9","last_execution":"2026-10-08T05:30:52.169053Z","name":"nightly-cleanup","next_execution":null,"parameters":{"type":"Object","value":{}},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-10-08T05:30:52.171863Z"}

--- Resuming schedule ---
Schedule resumed: {"created_at":"2026-10-08T05:30:52.131154Z","cron_expression":"0 0 4 * * *","description":"Remove expired sessions, temp files, and orphaned uploads","enabled":true,"function_label":"schedule_noop_rust_68813","id":"b1771f0c-2ae8-4287-b578-61db1f9b92b9","last_execution":"2026-10-08T05:30:52.169053Z","name":"nightly-cleanup","next_execution":"2026-10-09T04:00:00Z","parameters":{"type":"Object","value":{}},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-10-08T05:30:52.174636Z"}

--- Deleting schedule ---
Schedule deleted successfully

=== All schedule operations completed ===
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.17s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("q_mmrEC9-pFZXqKIWmbs_iEeCSyq8c8F_Pu2tl5wZnhwv_buqQw-927WbPVDwXMjk0FG_8XR-mut4jyYxWWO3w"))
✓ Inserted user 2: Some(String("kgPFV2-sb49qD-rXGhuX4XIjK8aryJXv5XBunVC2tKRBUWIfoVe682kJnzgGlvhckNdh-37I3dtPMVX0thfB3g"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - status: String
  - email: String
    (required)
  - title: String
    (required)
  - age: Integer

=== Listing Collections ===
✓ Total collections: 13
  Sample: ["chat_agent_configs__ek0_testing", "chat_goal_templates__ek0_testing", "audit__ek0_testing", "chat_tasks__ek0_testing", "agent_function_versions__ek0_testing"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.17s
     Running `target/debug/examples/client_search`
=== ekoDB Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Found 3 results for 'programming'
  1. Score: 26.4000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 13.2000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})
  3. Score: 13.2000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})

=== Fuzzy Search ===
✓ Found 3 results for 'progamming' (typo)
  1. Score: 2.2000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 1.1000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})
  3. Score: 1.1000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})

=== Field-Specific Search ===
✓ Found 4 results in title/description
  1. Score: 2.0000
     Title: Some(Object {"type": String("String"), "value": String("Machine Learning Basics")})
     Matched: ["title", "description"]
  2. Score: 1.0000
     Title: Some(Object {"type": String("String"), "value": String("Python for Data Science")})
     Matched: ["description"]
  3. Score: 0.5000
     Title: Some(Object {"type": String("String"), "value": String("Database Design")})
     Matched: ["description"]
  4. Score: 0.5000
     Title: Some(Object {"type": String("String"), "value": String("Rust Programming")})
     Matched: ["description"]

=== Weighted Search ===
✓ Found 2 results with field weights
  1. Score: 23.1000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})
  2. Score: 3.3000 - Some(Object {"type": String("String"), "value": String("Machine Learning Basics")})

=== Advanced Search Options ===
✓ Found 1 results with stemming
  1. Score: 26.4000 - Some(Object {"type": String("String"), "value": String("Database Design")})

=== Search with Limit ===
✓ Limited to 2 results (requested 2)
  1. Score: 26.4000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 13.2000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})

=== Search with a metadata pre-filter (category = programming) ===
✓ Found 2 results in category 'programming' (database/ai excluded)
  1. Some(Object {"type": String("String"), "value": String("Python for Data Science")}) (category: Some(Object {"type": String("String"), "value": String("programming")}))
  2. Some(Object {"type": String("String"), "value": String("Rust Programming")}) (category: Some(Object {"type": String("String"), "value": String("programming")}))

Execution time: 2ms
=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.16s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("2OcB4lpYrUq6iXzw2ZO15i-12fhyWkzLQalAxh7U7OALFh1z6QC6_KGzg3Sr0gmuubHApzs0pilNEP7vHi_KJA")})

=== Find by ID ===
Found: Record({"user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "name": Object({"type": String("String"), "value": String("Test Record")}), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "id": String("2OcB4lpYrUq6iXzw2ZO15i-12fhyWkzLQalAxh7U7OALFh1z6QC6_KGzg3Sr0gmuubHApzs0pilNEP7vHi_KJA"), "value": Object({"type": String("Integer"), "value": Integer(42)}), "created_at": Object({"type": String("DateTime"), "value": String("2026-10-08T05:33:50.382504+00:00")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "price": Object({"value": Float(99.99), "type": String("Float")})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-10-08T05:33:50.382504+00:00")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "name": Object({"value": String("Test Record"), "type": String("String")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "value": Object({"value": Integer(42), "type": String("Integer")}), "id": String("2OcB4lpYrUq6iXzw2ZO15i-12fhyWkzLQalAxh7U7OALFh1z6QC6_KGzg3Sr0gmuubHApzs0pilNEP7vHi_KJA"), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "created_at": Object({"value": String("2026-10-08T05:33:50.382504+00:00"), "type": String("DateTime")})})]

=== Update Document ===
Updated: Record({"data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "name": Object({"type": String("String"), "value": String("Updated Record")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "value": Object({"type": String("Integer"), "value": Integer(100)}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "id": String("2OcB4lpYrUq6iXzw2ZO15i-12fhyWkzLQalAxh7U7OALFh1z6QC6_KGzg3Sr0gmuubHApzs0pilNEP7vHi_KJA"), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-10-08T05:33:50.382504+00:00")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.53s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: m0Ys2143r9bSVKkXKYgf2rDhtf8cZrjFu2NGvFveSra3uWPaC0WSnjf5ax8rM3JQYkimR-gxQXNfNZTIsOpaKQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  name: "WebSocket Test Record"
  active: true
  id: "m0Ys2143r9bSVKkXKYgf2rDhtf8cZrjFu2NGvFveSra3uWPaC0WSnjf5ax8rM3JQYkimR-gxQXNfNZTIsOpaKQ"
  value: 42

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.59s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native_rs (OzgMKvzsb5pSOT9N6h0-i93UrS0heL1casA_qT5CAIzPMBrdE_yqTYS3O3Y99r33-O-5PnRZTxO-5UoP7viFjQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 134ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 5ms
  Speedup: 26.8x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit_rs (S9TEgeNTP8KBrE3WVzZJ95_StjMQXMco_uK_jECKTAK6c8HQQDPm_j7QWQ6k_mzE3F6y9ziffWNG-fMz27UqNw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_rs' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_rs (V9rbA7wOH_R30z6oXwD0t6w6INZv2FfiGkG4STbMUGSh4Q4oKzPgHcTAi0_dYu7loUrhTx2Q3wigJFKg5Sfbqw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_rs' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache_rs (4cZ5DCa2CquryTYpokzzEkE01OZBI0Q9b19ikMJO8aLJFJRmf1AwitIWbgStqC-xgChbwslQITBNCr5g15vONA)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

=== Key Benefits of Native SWR Function ===
✅ Simpler: One function instead of 4 (FindById → If → HttpRequest → Insert)
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Old pattern (manual):  ~4 function calls, complex script logic
New pattern (native):  ~1 function call, handled by server
Result: Simpler code, faster execution, easier maintenance

🧹 Cleaning up test data...
✓ Cleanup complete

=== Summary ===
The native SWR function provides a production-ready cache-aside pattern
with minimal code and maximum flexibility. Use it for:
  • API gateway caching
  • External API aggregation
  • Microservice response caching
  • Edge computing patterns
  • Real-time data enrichment pipelines

   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.54s
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_rs (P6pv9hMvPI3nbNoJYPGkCtB5s7j4FN_Wy2iOtgmdQ0l7iH6yTHxBbEfxyVyaf8n3IhX9_yly6i98ispl2AKqGA)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 5ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.19s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: jtJSpxPHQ0r41Rink5TRTLbngm2dR64gMKcFkzhpFjXn-XP2VxTwktgp6L7bzd78zd5d7uYBtpTqOEqfCxtHEg
Created Bob: $500 - ID: y4MBGz3OPo6Hb1-Tj9MXHuwhxYjn4-WGPntREneHSejS1J0_0OwRblLnfIcE2QG2vpoHGC3_QSfiYrB2QLN_4A

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 924e5201-2401-4628-942c-75481b185773

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

✓ Verified committed balances: Alice=$800, Bob=$700

=== Example 5: Rollback Demo ===
New transaction: 410248c4-6f60-4a99-97fc-32c0196f2986
Updated Bob: $700 → $600 (in transaction)
Status before rollback: "Active"
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/examples/client_user_functions`
✓ Client created

=== Create User Function ===
Created user function with ID: JpcdLKStX3nhKUIqa64He1om1Ots_eDaEpsUc1dEiOkFmRWkE1T1mlof8ixjjNSjagqUH20XIaSlGpKOQ1wN9A

=== Get User Function ===
Retrieved: get_active_users_rs - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 1 user functions:
  - get_active_users_rs: Get Active Users

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.18s
     Running `target/debug/examples/client_websocket_chat_stream`
=== WebSocket Chat Streaming Example (Rust) ===

✓ Authentication successful
✓ Created chat session: i2-Peiyx_Q-sG1hl8k4ubHbUt7bl_Pa42Q_Qm1nsl-4Ot4z17BBaBklpYHl1IWHtZ9ODwH97R_p_1B5Ph55TxA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Sending message: 'What is the capital of France?' ===
✓ Message accepted, streaming...

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: "fkU620llEkkkwnKrdFA6GAPvMvIfYiP5PIhYNdtGlKsmQrXhoejdCDjmHkg47i1Hm-UhjsfiXsuBRMXmgrBSuQ"
Execution time: 776ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}
Context window: 1000000 tokens

Full response: {"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris....

✓ WebSocket chat streaming example completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.20s
     Running `target/debug/examples/client_websocket_subscribe`
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_rs' ===
✓ Subscribed (subscription_id: sub_2a6ba17a4ad94976b8c1dfd282441c25)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: yCcX6YaWg3LSyEDnc4d2QMcaByPlsDT-3xOGtL7fJIzwgQ5gR4HiIpcnMyHHS8bo4vnxWtSknMEnb5RPeCAAWA

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_rs"
     Record IDs: ["yCcX6YaWg3LSyEDnc4d2QMcaByPlsDT-3xOGtL7fJIzwgQ5gR4HiIpcnMyHHS8bo4vnxWtSknMEnb5RPeCAAWA"]
     Timestamp:  "2026-10-08T05:41:33.367772+00:00"

Inserting record 2...
✓ Inserted: --o76HCwCgOPApuQIhtXhgJ1i9BNu9-mMS7vutlqrpf6z7ZULREJ6GKJix15cp9CRI9mNauomon0hcvmopuW2w

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["--o76HCwCgOPApuQIhtXhgJ1i9BNu9-mMS7vutlqrpf6z7ZULREJ6GKJix15cp9CRI9mNauomon0hcvmopuW2w"]

=== Unsubscribing ===
✓ Unsubscribed

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_rs'

✓ WebSocket subscription example completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.38s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("BDBquPMXMmgKa6xccdTFS6ITz4PNEEzGFEvZfSXJ_u3VWm2RuWMsy-9qLei8qPqv-gKKoInotk06k4VgR_GSCA"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.12s
     Running `target/debug/examples/bypass_ripple_example`
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: Record({"id": String("Ne5WUK-98VA4G5dV5zSI3o7oj6h8GD_aFBaDvFs4XVgu6tiF--NxWA5i2SLTnJXonyrUiYKCjFgvl7f4P4ANJg")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("K3YCVNh3FF75UozKsYFwdJ6OVBC6rEUPs5-HllMen0S4oX2pMuEmoR6YlviP6qyPlmsmjOu1pAUKSb-BlrZ6nA")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"name": Object({"type": String("String"), "value": String("Product 1")}), "id": String("Ne5WUK-98VA4G5dV5zSI3o7oj6h8GD_aFBaDvFs4XVgu6tiF--NxWA5i2SLTnJXonyrUiYKCjFgvl7f4P4ANJg"), "price": Object({"type": String("Integer"), "value": Integer(150)})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("custom-id")})

✅ All bypass_ripple operations completed successfully!
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.13s
     Running `target/debug/examples/projection_example`
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["name", "email", "id"]

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["email", "user_role", "created_at", "name", "avatar_url", "status", "id", "bio", "age"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["user_role", "age", "api_key", "password", "email", "status", "created_at", "secret_token", "bio", "id", "name", "avatar_url"]
  Projected query:
    - 3 fields per record
    - Fields: ["name", "id", "email"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ All Rust integration tests complete!
📦 Ensuring Python example dependencies in .venv...
🧪 Running Python examples (direct HTTP/WebSocket)...
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'nhp0Gkid2WiE8eUo5RBE9Id-rDfsOUtTlSGhJIeLDMeKh013lnN0myQlPNHDDDW9vX4RBHuBjOe8NEKfQ3HXrw'}

=== Find by ID ===
Found: {'name': {'type': 'String', 'value': 'Test Record'}, 'active': {'type': 'Boolean', 'value': True}, 'value': {'value': 42, 'type': 'Integer'}, 'id': 'nhp0Gkid2WiE8eUo5RBE9Id-rDfsOUtTlSGhJIeLDMeKh013lnN0myQlPNHDDDW9vX4RBHuBjOe8NEKfQ3HXrw'}

=== Find with Query ===
Found documents: [{'id': 'nhp0Gkid2WiE8eUo5RBE9Id-rDfsOUtTlSGhJIeLDMeKh013lnN0myQlPNHDDDW9vX4RBHuBjOe8NEKfQ3HXrw', 'name': {'type': 'String', 'value': 'Test Record'}, 'value': {'value': 42, 'type': 'Integer'}, 'active': {'value': True, 'type': 'Boolean'}}]

=== Update Document ===
Updated: {'value': {'type': 'Integer', 'value': 100}, 'id': 'nhp0Gkid2WiE8eUo5RBE9Id-rDfsOUtTlSGhJIeLDMeKh013lnN0myQlPNHDDDW9vX4RBHuBjOe8NEKfQ3HXrw', 'active': {'value': True, 'type': 'Boolean'}, 'name': {'value': 'Updated Record', 'type': 'String'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: lSfJX0_6nppbwdFi5dS5H8c0uBWMcsD9PiPCBFZWEUlpRsH9FnNbGZCOPvCM_ZUlGosN6klFTTQiglHNkTKJiA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "value": {
          "type": "Integer",
          "value": 42
        },
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "id": "lSfJX0_6nppbwdFi5dS5H8c0uBWMcsD9PiPCBFZWEUlpRsH9FnNbGZCOPvCM_ZUlGosN6klFTTQiglHNkTKJiA",
        "active": {
          "type": "Boolean",
          "value": true
        }
      }
    ]
  },
  "messageId": "1061086415"
}
✓ Retrieved 1 record via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Functions Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: pVnRL31QzPd7zsvJzN7erZ2fhrDFa2X3pYB6AaxvxcR1udwI9WxseJBV5bVJwMNX8B6uOMiqgucZBGR6PMT6bw
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: dvA1ubouDW8Ld2Ic3-d7tgOXUtB9wdCYVUs9ExvHwkD56dGqXPvS7rMykCUJtlyaSPfGZlADWXyejGwMIw5oJQ
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: tj9tLhi38FwTL-Z8So_RD0AfUSXeW6bYJP1h1P71jIztHYllRn1JB-2mwSOBHb_JwdqJhEp9bbx9J66OyB665w
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'max_score': {'type': 'Integer', 'value': 90}, 'count': {'type': 'Integer', 'value': 5}, 'avg_score': {'value': 50.0, 'type': 'Float'}, 'status': {'value': 'inactive', 'type': 'String'}}
   {'max_score': {'value': 100, 'type': 'Integer'}, 'avg_score': {'type': 'Float', 'value': 60.0}, 'count': {'value': 5, 'type': 'Integer'}, 'status': {'value': 'active', 'type': 'String'}}

📝 Example 4: Function Management

📋 Total scripts: 3
🔍 Retrieved script: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
=== Batch Operations (Direct HTTP) ===

✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 5 total records in collection

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records
✓ Verified: Record updated with status="active"

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
=== Key-Value Operations (Direct HTTP) ===

✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {'type': 'Object', 'value': {'username': 'john_doe', 'userId': 123}}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
kv_operations:direct:py:cache:product:1: {'value': {'price': 29.99, 'name': 'Product 1'}, 'type': 'Object'}
kv_operations:direct:py:cache:product:2: {'value': {'name': 'Product 2', 'price': 39.989999999999995}, 'type': 'Object'}
kv_operations:direct:py:cache:product:3: {'value': {'name': 'Product 3', 'price': 49.989999999999995}, 'type': 'Object'}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: 9y74vt7USrWa5ALTrOJzH2DiPEeVyTI7wbV3ELnG1FmqKR62NrKAINFtptTwxU6kRnsWBuPmh2xY4ugTOWUwfg

=== List Collections ===
Total collections: 13
Sample collections: ['demo_collection', 'chat_agent_configs__ek0_testing', 'chat_goal_templates__ek0_testing', 'audit__ek0_testing', 'chat_tasks__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

📋 Getting original configuration...
   Original durable_operations: True


============================================================
🔥 TEST 1: Original Config (durable=True)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Bgij-8gqXrm0-WoS500pxQkKYMSYU-Wpqst48srlM7R5udMgfJKbYaiBgrRqh1mA9iTu62NEInTwOwLJKCSLfQ
Created Bob: $500 - ID: -4aEiqpSYHZwS4hX0n8ySv17E15_Gu1rgy2RENcO5VVQDc-ZSOxDzCGd-u-S118mqwLkauxPL66D8OPuYbYGoQ

=== Example 1: Begin Transaction ===
Transaction ID: a8bcf720-20df-41c9-befc-b16cbd7d10e6

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'value': 800, 'type': 'Integer'}
Bob: ${'value': 700, 'type': 'Integer'}

=== Example 5: Rollback ===
New transaction: a35a8a77-ae48-4088-ae39-7642f007dbbb
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: rMuDGt9ozEnSlUDrnNTA-NgDZjZznG2hfjS4nYEtO08PeORYC2DQWE8qkaKHcbO8_R4jk7SI21Ly0WCVHc6p1Q
Created Bob: $500 - ID: 1NHE_USjiJ1HUd5U7SxxEbmOss96I_YNbHb-C1VCYPWYkI95vZZieLQ75dNOXPArh6Ifwcru6CX3KBIxhEwSHQ

=== Example 1: Begin Transaction ===
Transaction ID: 4f90c0fb-ace2-4731-a4e4-cff4333bbe7b

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to NON-DURABLE mode...
   ✓ Config updated: durable_operations=false


============================================================
🔥 TEST 2: Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: hruEE7vyJA89iYAPPV6Nl-3WSkiud9FLCLsTdCkhpPDhm4uVhKfiRvyXSyKLr-p_XMKHLGJvIDXzakl5kfHL1A
Created Bob: $500 - ID: Xuk6uEItn0YBWW9Nz4TJV5a99SdsB4m1jEnmpAxg3XDQmoDCqVy2Oy0Tna6c34u7e4M-lOS6FBEmvTn8-75NGQ

=== Example 1: Begin Transaction ===
Transaction ID: da0e223e-b408-4d63-bc30-440bb59d5be7

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'value': 700, 'type': 'Integer'}

=== Example 5: Rollback ===
New transaction: c0dfe708-7e23-418a-9d24-9c398b26a237
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: -EKWcWUIgjW-VL2r7jQEpvWCJBtXCdL8hehN4lD96Dk987Lf6AtT3rhCR8DEpP3Cb6-_kMFF079mUAfcA6nR2Q
Created Bob: $500 - ID: fhca_h537Gvmiihr7zqKXOqgymbLH7Xf0vXlC5VjINiM5yekxfk1z9QNewLxCW-REUH3QcUgcuI60H3PwZjjqg

=== Example 1: Begin Transaction ===
Transaction ID: 5a7ad27e-de07-4749-b0b2-ef3ce105e3c8

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'value': 800, 'type': 'Integer'}
Bob: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to DURABLE mode...
   ✓ Config updated: durable_operations=true


============================================================
🔥 TEST 3: Durable Mode (durable=true)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: tWfQe_KfAMIOSTdA5wetxZGheEXj1iKeu6Mse7y4OeSwupkX0eFZNfwAKcAEaHsyZ-SdUiA9x2Jpwdey8pQv5w
Created Bob: $500 - ID: F1LhkxWjdkFOjXesiIyMDpsfR8qcH4Umc0ZE-eRsT5eX2Ve2KnFQbo8BKG7gafdy0SkRH1zKs6R1ddKFss76rg

=== Example 1: Begin Transaction ===
Transaction ID: c89c99fb-340c-495c-a080-5dbf26ecca10

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'value': 700, 'type': 'Integer'}

=== Example 5: Rollback ===
New transaction: 35c7439c-7344-4e36-bb73-15f7701e34b5
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: y_ry2u5D4AUqIdsoVoQnyS3bj5y5bkdBEXz78hNndUWfR0ie3xwM8XoZ-k1meKGOax_6loOnPN8XRKmkdXbErQ
Created Bob: $500 - ID: wg5oz6eYBxxYukK0zNjPeU58lmmRkm66t1ThQcrJGAmm5q2467P9lPxJS9PvL1OT_wxyrZ5x1PtHOdxCsYLJ0A

=== Example 1: Begin Transaction ===
Transaction ID: 25b6b494-f7b3-404a-b726-292d8de7222e

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ Deleted test accounts


🔄 Restoring original configuration...
   ✓ Config restored: durable_operations=True


============================================================
✅ ALL TESTS PASSED - Transactions successful
============================================================

🚀 ekoDB Complete CRUD Functions Example
============================================================
Demonstrates:
  • Insert + Verify (using Query)
  • Query + Update Status + Verify
  • Query + Update Credits + Verify
  • Query Before Delete + Delete + Verify Gone

Each function shows Functions chaining with proper verification
============================================================

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted collection: crud_functions_users_py
============================================================
📝 function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: MmTAAfTxGZvWyDnB0yyfw-iHVtpjEoUYI9arXj4xHtvsX8pCPJE2_3_KPxk7TqYNn8MZA8P5jfnjp89g751YEg

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: QxDkLGDaXd4fSMlblHndqFIo5Q0irDMkJacCC9Kz9zjisjaM9_AO4onJnl74enHsk99fjWZz9qRGn0tzY8csPg
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: jBA0D-SSzeBUVlNtSHiwrmdHE8SQCqGo1Un3ijS1sOTSdRGUKHeOWjuqfEu46vEpkGAOmpwKpBpRvwHVbbRDYw

2️⃣ Calling function (Query + Update + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'value': 'active', 'type': 'String'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: hu1T_pcX4WD8VJgUo7B57yeRSs4khrRabRgrLMB_QS8YwaSPO4x0CkPOKpYxXsclUAaxkXmtIJOPaH5-fbDh0A

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'value': 100, 'type': 'Integer'}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: QKwGoOr4ZJdzDW-WXpjKExmSCzj2lD_4x2WO3m2OSC8HhnX9SVo-9xrgiM4OCldutqebo9QGMUJ2zXiaFmv_dw

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: MmTAAfTxGZvWyDnB0yyf...
   ✅ Deleted script: jBA0D-SSzeBUVlNtSHiw...
   ✅ Deleted script: hu1T_pcX4WD8VJgUo7B5...
   ✅ Deleted script: QKwGoOr4ZJdzDW-WXpjK...
   ✅ Deleted collection: crud_functions_users_py

============================================================
✅ Complete CRUD Functions Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Functions chain Functions together
   ✅ Each function demonstrates operation + verification
   ✅ Parameters make functions reusable
   ✅ Verification is built into the function itself
   ✅ Complete CRUD lifecycle in 4 focused functions
╔════════════════════════════════════════════════════════╗
║     TTL EXPIRATION VERIFICATION TEST                   ║
╚════════════════════════════════════════════════════════╝

This test verifies that document TTL expiration works correctly.
We will insert documents with short TTL and verify they expire.

✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = ECY3CGWexpBkXz_EVkW_Kor_nNEaZsKXwF863TxJV-pL8EZU9aftVO7ULl1aHdbxMOymcnFTnbzHOBLDxBNLQw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(ECY3CGWexpBkXz_EVkW_Kor_nNEaZsKXwF863TxJV-pL8EZU9aftVO7ULl1aHdbxMOymcnFTnbzHOBLDxBNLQw)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(ECY3CGWexpBkXz_EVkW_Kor_nNEaZsKXwF863TxJV-pL8EZU9aftVO7ULl1aHdbxMOymcnFTnbzHOBLDxBNLQw)
  Output: Error (expected) - Find failed: Record not found
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
CLEANUP
═══════════════════════════════════════════════════════════
✓ Deleted test collection

╔════════════════════════════════════════════════════════╗
║              ALL TTL TESTS PASSED ✓                    ║
╚════════════════════════════════════════════════════════╝

TTL expiration is working correctly:
  • Documents with TTL expire after the specified time
  • Documents without TTL persist indefinitely
  • Different TTL durations are handled correctly
╔════════════════════════════════════════════════════════╗
║   WEBSOCKET TTL EXPIRATION VERIFICATION TEST           ║
╚════════════════════════════════════════════════════════╝

This test verifies TTL expiration works via WebSocket connections.
We will use WebSocket to insert, query, and verify TTL expiration.

✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = mAFne1i_4KVaC4ntIfdmfO1LXzRfugfT8izkLCT8TqtjmgPvme-vZSq2eiywpuB0aY8FtEejVTxuP5ePfhU0mA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(mAFne1i_4KVaC4ntIfdmfO1LXzRfugfT8izkLCT8TqtjmgPvme-vZSq2eiywpuB0aY8FtEejVTxuP5ePfhU0mA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(mAFne1i_4KVaC4ntIfdmfO1LXzRfugfT8izkLCT8TqtjmgPvme-vZSq2eiywpuB0aY8FtEejVTxuP5ePfhU0mA)
  Output: Error (expected) - Find failed: Record not found
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
CLEANUP
═══════════════════════════════════════════════════════════
✓ Deleted test collection

╔════════════════════════════════════════════════════════╗
║          WEBSOCKET TTL TEST PASSED ✓                   ║
╚════════════════════════════════════════════════════════╝

WebSocket TTL expiration is working correctly:
  • Documents with TTL inserted via client expire correctly
  • Queries correctly return None for expired documents

╔════════════════════════════════════════╗
║   ekoDB Python Examples Test Suite    ║
╚════════════════════════════════════════╝

=== Checking Server Connection ===
✓ Server is ready

=== Getting Authentication Token ===
✓ Authentication successful

=== Running 10 Examples ===

=== Running simple_crud.py ===
✓ simple_crud.py completed successfully

=== Running simple_websocket.py ===
✓ simple_websocket.py completed successfully

=== Running http_functions.py ===
✓ http_functions.py completed successfully

=== Running batch_operations.py ===
✓ batch_operations.py completed successfully

=== Running kv_operations.py ===
✓ kv_operations.py completed successfully

=== Running collection_management.py ===
✓ collection_management.py completed successfully

=== Running transactions.py ===
✓ transactions.py completed successfully

=== Running crud_functions.py ===
✓ crud_functions.py completed successfully

=== Running document_ttl.py ===
✓ document_ttl.py completed successfully

=== Running websocket_ttl.py ===
✓ websocket_ttl.py completed successfully

╔════════════════════════════════════════╗
║           Test Summary                 ║
╚════════════════════════════════════════╝
Total: 10
Passed: 10
Failed: 0
✅ Python direct examples complete!
🐍 Building Python client package...
🔧 Ensuring maturin is available in .venv...
🔨 Building wheel...
🍹 Building a mixed python/rust project
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.17s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.27.0-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.27.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.27.0
    Uninstalling ekodb_client-0.27.0:
      Successfully uninstalled ekodb_client-0.27.0
Successfully installed ekodb-client-0.27.0
🧪 Ensuring test dependencies (pytest) in .venv...
✅ Python client package built and installed!
=== ekoDB Advanced CRUD Example (Python) ===

--- insert ---
Inserted: fMNib1n0TJVuvDbjxL81MQfdGJX-AroycDs0uhhQqTnmBfJ3jXrsQ_sgR4-SXXoKSlhf6hgBA2j730iK1nfxlw

--- update_with_action (increment) ---
After increment count by 5: {'id': 'fMNib1n0TJVuvDbjxL81MQfdGJX-AroycDs0uhhQqTnmBfJ3jXrsQ_sgR4-SXXoKSlhf6hgBA2j730iK1nfxlw', 'count': {'type': 'Integer', 'value': 15}, 'tags': {'value': ['initial'], 'type': 'Array'}, 'score': {'type': 'Float', 'value': 50.0}, 'name': {'type': 'String', 'value': 'Counter Record'}}

--- update_with_action (decrement) ---
After decrement score by 10: {'id': 'fMNib1n0TJVuvDbjxL81MQfdGJX-AroycDs0uhhQqTnmBfJ3jXrsQ_sgR4-SXXoKSlhf6hgBA2j730iK1nfxlw', 'name': {'type': 'String', 'value': 'Counter Record'}, 'score': {'type': 'Float', 'value': 40.0}, 'tags': {'type': 'Array', 'value': ['initial']}, 'count': {'type': 'Integer', 'value': 15}}

--- update_with_action_sequence ---
After action sequence: {'name': {'type': 'String', 'value': 'Counter Record'}, 'count': {'type': 'Integer', 'value': 115}, 'score': {'type': 'Float', 'value': 40.0}, 'id': 'fMNib1n0TJVuvDbjxL81MQfdGJX-AroycDs0uhhQqTnmBfJ3jXrsQ_sgR4-SXXoKSlhf6hgBA2j730iK1nfxlw', 'tags': {'type': 'Array', 'value': ['initial', 'sequenced']}}

--- cleanup ---
Cleaned up collection

=== Advanced CRUD example completed ===
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
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: sWTRtLnlMDIIeAR8ZhJP4C6nIAEFyrshZRb-AlYv8DG7ZExyh4y9yy7btb-jlFlRgUPTxoyJ17hO5t_p4qNK7g

=== Sending Initial Message ===
✓ Message sent
  Response: It seems there are currently no products available in the database, based on the query for relevant records. Would you like to add products or check something else?

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['context_snippets', 'llm_model', 'id', 'llm_provider', 'chat_id', 'content', 'token_usage', 'created_at', 'role', 'updated_at'])
Debug: First message role: {'value': 'assistant', 'type': 'String'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: kQ5BBk5EsbJjs3EBFfQXm3A0sy5kdAbvfIuYn6xJHE82ZwcuKKfdF_X5LsxU1ZqsVd29il0LNTqj1v-Q8slF3w
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: kQ5BBk5EsbJjs3EBFfQXm3A0sy5kdAbvfIuYn6xJHE82ZwcuKKfdF_X5LsxU1ZqsVd29il0LNTqj1v-Q8slF3w
✓ Deleted chat session: sWTRtLnlMDIIeAR8ZhJP4C6nIAEFyrshZRb-AlYv8DG7ZExyh4y9yy7btb-jlFlRgUPTxoyJ17hO5t_p4qNK7g
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 9j2uxJBCNy_OrB2g6d2mK8zwIWIy-_z3XA7mIF6JGTQyA8aJpeO6jLOC5qegAaoJ7FaGdkHSn85JY2jr3WddZA

=== Sending Chat Message ===
Message ID: ZlYzlm4SBZoBysox2ef2VZK-D3h14z1p_7sibo1cf2rQP6o3LqH0bINUxAzgFHTT6l6LL0q-AOX-N4_HF8BWkQ

=== AI Response ===
Here are the available products and their prices:

1. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features.

2. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product.

3. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities.

Execution Time: 3044ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 90
Total tokens: 3503

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (Python) ===

Created session: uP3Etkc42s1kGvmSHFk6GJFiP4UMqhYZX26GHpVYa0ttaaakaNdRQOQRO5nsHs5vf1PdVy0dlO3gBepS8tsm8Q

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}ekoDB is a high-performance, scalable NoSQL database designed for use in distributed systems. It is built to provide efficient storage and retrieval of large volumes of unstructured or semi-structured data, making it suitable for applications that require quick access to data across multiple nodes.

Key features of ekoDB typically include:

1. **Scalability**: It allows for easy scaling horizontally by adding more servers to accommodate growing amounts of data and higher request loads.

2. **High Availability**: ekoDB often includes mechanisms for replication and fault tolerance, ensuring data remains accessible even in the event of server failures.

3. **Schema Flexibility**: As a NoSQL database, ekoDB enables developers to store data without a fixed schema, making it easier to adapt to changing application requirements.

4. **Performance**: It is optimized for fast data access and can handle high throughput of read and write operations.

5. **Distributed Architecture**: Designed to operate in a distributed environment, ekoDB can efficiently manage data across various geographical locations.

Unlike traditional relational databases, which use tables and structured query languages, ekoDB allows for more flexible data models, catering to applications that need to work with diverse datasets.

It's worth noting that specific features and functionalities can vary depending on the implementation and the version of ekoDB, as well as the context in which it is used, such as specific industries or use cases.

--- Stream complete ---
Message ID: 3-kBZB4SexPkFsgmmNBchdPD555LY79_cEztG9ZC4zm-6EuCeBjTCCOuzWbtkzdxwuehbFYuOEWnHYK8tQujPA
Execution time: 3498ms
Context window: 128000 tokens

✓ Chat message stream example completed
✓ Client created

=== Get All Chat Models ===
OpenAI models: ['o3-2025-04-16', 'gpt-5.1-chat-latest', 'gpt-5.3-chat-latest', 'gpt-5.2-2025-12-11', 'sora-2', 'chatgpt-image-latest', 'gpt-4.1-mini', 'gpt-3.5-turbo', 'gpt-4o-mini', 'gpt-image-2.5-sunburst', 'gpt-image-2.5-flare-2026-09-08', 'gpt-6-luna', 'gpt-5.4-mini-2026-03-17', 'gpt-4-turbo', 'gpt-4.1-nano', 'gpt-image-2', 'gpt-5.4-pro', 'gpt-realtime-mini-2025-12-15', 'gpt-3.5-turbo-16k', 'babbage-002', 'text-embedding-ada-002', 'gpt-4o-mini-tts', 'gpt-4o-2024-08-06', 'o4-mini-2025-04-16', 'gpt-4o-2024-11-20', 'omni-moderation-latest', 'gpt-realtime', 'tts-1-hd', 'gpt-realtime-1.5', 'gpt-realtime-2', 'gpt-5-search-api', 'gpt-5.5-2026-04-23', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-4o', 'o1-2024-12-17', 'gpt-5.2-chat-latest', 'gpt-4o-mini-tts-2025-12-15', 'gpt-image-1', 'davinci-002', 'gpt-image-2-2026-04-21', 'gpt-4o-mini-transcribe', 'o3-mini', 'gpt-audio', 'gpt-5.4-nano', 'o3', 'gpt-5.1-codex-max', 'gpt-realtime-2.1', 'gpt-4.1', 'gpt-5.5', 'gpt-4o-mini-search-preview-2025-03-11', 'text-embedding-3-large', 'gpt-4o-mini-search-preview', 'gpt-4o-2024-05-13', 'gpt-6-sol', 'gpt-5.3-codex', 'gpt-3.5-turbo-instruct', 'sora-2-pro', 'gpt-5', 'gpt-audio-mini-2025-12-15', 'gpt-transcribe', 'o4-mini-deep-research-2025-06-26', 'o1-pro-2025-03-19', 'gpt-4.1-nano-2025-04-14', 'gpt-5-search-api-2025-10-14', 'gpt-4-turbo-2024-04-09', 'gpt-realtime-2.1-mini', 'tts-1', 'gpt-5-mini', 'omni-moderation-2024-09-26', 'gpt-5-mini-2025-08-07', 'gpt-realtime-2025-08-28', 'gpt-5.4-2026-03-05', 'gpt-live-transcribe', 'gpt-3.5-turbo-0125', 'gpt-5-nano', 'gpt-5.6-luna', 'gpt-4', 'gpt-image-2.5-sunburst-2026-09-08', 'whisper-1', 'gpt-5.4-mini', 'gpt-5-pro', 'o1-pro', 'gpt-realtime-whisper', 'gpt-5.2-pro', 'gpt-5-chat-latest', 'gpt-live-1', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-5-2025-08-07', 'tts-1-1106', 'gpt-5.5-pro', 'gpt-5-pro-2025-10-06', 'chat-latest', 'gpt-5.2-pro-2025-12-11', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-audio-2025-08-28', 'gpt-5.4-pro-2026-03-05', 'gpt-realtime-translate', 'gpt-5.6-sol', 'gpt-audio-mini-2025-10-06', 'gpt-5.5-pro-2026-04-23', 'tts-1-hd-1106', 'gpt-realtime-mini', 'gpt-5.4', 'gpt-4o-transcribe-diarize', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-image-1.5', 'gpt-5.1-2025-11-13', 'o4-mini-deep-research', 'gpt-4o-search-preview-2025-03-11', 'o3-mini-2025-01-31', 'gpt-4-0613', 'gpt-4o-search-preview', 'gpt-5.6-terra', 'text-embedding-3-small', 'gpt-audio-1.5', 'gpt-3.5-turbo-instruct-0914', 'gpt-6-astra', 'gpt-5.2-codex', 'gpt-3.5-turbo-1106', 'gpt-5-nano-2025-08-07', 'gpt-audio-mini', 'gpt-4o-mini-tts-2025-03-20', 'gpt-5.2', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-2025-04-14', 'gpt-4o-mini-2024-07-18', 'o4-mini', 'o1', 'gpt-5.4-nano-2026-03-17', 'gpt-5.1-codex-mini', 'gpt-4o-transcribe', 'gpt-image-2.5-flare', 'gpt-6.1-sol']
Anthropic models: ['claude-haiku-5-5', 'claude-sonnet-5-5', 'claude-opus-5-5', 'claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']
Perplexity models: []
Gemini models: []

=== Get OpenAI Models ===
OpenAI models: ['o3-2025-04-16', 'gpt-5.1-chat-latest', 'gpt-5.3-chat-latest', 'gpt-5.2-2025-12-11', 'sora-2', 'chatgpt-image-latest', 'gpt-4.1-mini', 'gpt-3.5-turbo', 'gpt-4o-mini', 'gpt-image-2.5-sunburst', 'gpt-image-2.5-flare-2026-09-08', 'gpt-6-luna', 'gpt-5.4-mini-2026-03-17', 'gpt-4-turbo', 'gpt-4.1-nano', 'gpt-image-2', 'gpt-5.4-pro', 'gpt-realtime-mini-2025-12-15', 'gpt-3.5-turbo-16k', 'babbage-002', 'text-embedding-ada-002', 'gpt-4o-mini-tts', 'gpt-4o-2024-08-06', 'o4-mini-2025-04-16', 'gpt-4o-2024-11-20', 'omni-moderation-latest', 'gpt-realtime', 'tts-1-hd', 'gpt-realtime-1.5', 'gpt-realtime-2', 'gpt-5-search-api', 'gpt-5.5-2026-04-23', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-4o', 'o1-2024-12-17', 'gpt-5.2-chat-latest', 'gpt-4o-mini-tts-2025-12-15', 'gpt-image-1', 'davinci-002', 'gpt-image-2-2026-04-21', 'gpt-4o-mini-transcribe', 'o3-mini', 'gpt-audio', 'gpt-5.4-nano', 'o3', 'gpt-5.1-codex-max', 'gpt-realtime-2.1', 'gpt-4.1', 'gpt-5.5', 'gpt-4o-mini-search-preview-2025-03-11', 'text-embedding-3-large', 'gpt-4o-mini-search-preview', 'gpt-4o-2024-05-13', 'gpt-6-sol', 'gpt-5.3-codex', 'gpt-3.5-turbo-instruct', 'sora-2-pro', 'gpt-5', 'gpt-audio-mini-2025-12-15', 'gpt-transcribe', 'o4-mini-deep-research-2025-06-26', 'o1-pro-2025-03-19', 'gpt-4.1-nano-2025-04-14', 'gpt-5-search-api-2025-10-14', 'gpt-4-turbo-2024-04-09', 'gpt-realtime-2.1-mini', 'tts-1', 'gpt-5-mini', 'omni-moderation-2024-09-26', 'gpt-5-mini-2025-08-07', 'gpt-realtime-2025-08-28', 'gpt-5.4-2026-03-05', 'gpt-live-transcribe', 'gpt-3.5-turbo-0125', 'gpt-5-nano', 'gpt-5.6-luna', 'gpt-4', 'gpt-image-2.5-sunburst-2026-09-08', 'whisper-1', 'gpt-5.4-mini', 'gpt-5-pro', 'o1-pro', 'gpt-realtime-whisper', 'gpt-5.2-pro', 'gpt-5-chat-latest', 'gpt-live-1', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-5-2025-08-07', 'tts-1-1106', 'gpt-5.5-pro', 'gpt-5-pro-2025-10-06', 'chat-latest', 'gpt-5.2-pro-2025-12-11', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-audio-2025-08-28', 'gpt-5.4-pro-2026-03-05', 'gpt-realtime-translate', 'gpt-5.6-sol', 'gpt-audio-mini-2025-10-06', 'gpt-5.5-pro-2026-04-23', 'tts-1-hd-1106', 'gpt-realtime-mini', 'gpt-5.4', 'gpt-4o-transcribe-diarize', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-image-1.5', 'gpt-5.1-2025-11-13', 'o4-mini-deep-research', 'gpt-4o-search-preview-2025-03-11', 'o3-mini-2025-01-31', 'gpt-4-0613', 'gpt-4o-search-preview', 'gpt-5.6-terra', 'text-embedding-3-small', 'gpt-audio-1.5', 'gpt-3.5-turbo-instruct-0914', 'gpt-6-astra', 'gpt-5.2-codex', 'gpt-3.5-turbo-1106', 'gpt-5-nano-2025-08-07', 'gpt-audio-mini', 'gpt-4o-mini-tts-2025-03-20', 'gpt-5.2', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-2025-04-14', 'gpt-4o-mini-2024-07-18', 'o4-mini', 'o1', 'gpt-5.4-nano-2026-03-17', 'gpt-5.1-codex-mini', 'gpt-4o-transcribe', 'gpt-image-2.5-flare', 'gpt-6.1-sol']

=== Get Anthropic Models ===
Anthropic models: ['claude-haiku-5-5', 'claude-sonnet-5-5', 'claude-opus-5-5', 'claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']

✓ Chat Models API example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: fJVCAPvSlSUqLopkLDrLuNhSDSg8c0-kMEn0_YN5RuTbfr8Hy7lCYlyUjfxCmlbVJ1w0k8iNvynluyac-NOCiA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
  - **Description**: A high-performance database product
  - **Price**: $99

If you need more information or assistance, feel free to ask!

✓ Message 2 sent
  Response: The price of the product **ekoDB** is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: Z5yx30TLt4ELsrQidkM4tS2c2QNyB8NAms0XAyJJsm8aJH1hDhV_H4f_tD-x1Iy3RcnOoXpzqULH3gkIzbcd4w
  Parent: fJVCAPvSlSUqLopkLDrLuNhSDSg8c0-kMEn0_YN5RuTbfr8Hy7lCYlyUjfxCmlbVJ1w0k8iNvynluyac-NOCiA

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: Z5yx30TLt4ELsrQidkM4tS2c2QNyB8NAms0XAyJJsm8aJH1hDhV_H4f_tD-x1Iy3RcnOoXpzqULH3gkIzbcd4w (Untitled)
  Session 2: fJVCAPvSlSUqLopkLDrLuNhSDSg8c0-kMEn0_YN5RuTbfr8Hy7lCYlyUjfxCmlbVJ1w0k8iNvynluyac-NOCiA (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: Z5yx30TLt4ELsrQidkM4tS2c2QNyB8NAms0XAyJJsm8aJH1hDhV_H4f_tD-x1Iy3RcnOoXpzqULH3gkIzbcd4w

=== Cleanup ===
✓ Deleted sessions and collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "Sn-2UnIpDKBXonfCDIW0A6kuY1WdCio0g1em0gBrFZ16-dTnpD3ADg52-F5BNBClSmSMjJ3vCDnDJ1_sLDRiIA"

=== List Collections ===
Total collections: 13
Sample collections: ['chat_agent_configs__ek0_testing', 'chat_goal_templates__ek0_testing', 'audit__ek0_testing', 'chat_tasks__ek0_testing', 'agent_function_versions__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test_py' exists: False

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test_py' exists: True

=== Count Documents ===
Document count in 'collection_utils_test_py': 5

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: False

=== Cleanup ===
Deleted collection 'collection_utils_test_py'

✓ Collection Utilities example complete
✓ Client created
✓ conc_demo_pay saved
✓ conc_demo_rl_fail saved
✓ conc_demo_rl_skip saved
✓ conc_demo_lock saved

Invoke them like:
  POST /api/functions/conc_demo_pay        { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail    { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip    { "user_id": 42 }
  POST /api/functions/conc_demo_lock       { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'ge8QCZbjvZA8ounlIMfITOpIT-R8SGO393aqU7oVPLMnRTRQb9Eai6rxlXo1n4vBwmiZ0m_LUc6ApeotD20gHQ'}

=== Upsert Operation ===
✓ Upsert (update existing record): ge8QCZbjvZA8ounlIMfITOpIT-R8SGO393aqU7oVPLMnRTRQb9Eai6rxlXo1n4vBwmiZ0m_LUc6ApeotD20gHQ
✓ Inserted second record: AGAuT5-tZFTcEBKgskowqCEP17EskZr5UQ-cj8TFEwQy8hmfWp5M90rYAiM2py6YY3RVJ0HoypY3ncM8uYFQBQ
✓ Upsert (update second record): AGAuT5-tZFTcEBKgskowqCEP17EskZr5UQ-cj8TFEwQy8hmfWp5M90rYAiM2py6YY3RVJ0HoypY3ncM8uYFQBQ

=== Find One Operation ===
✓ Found user by email: {'email': {'value': 'alice.j@newdomain.com', 'type': 'String'}, 'age': {'type': 'Integer', 'value': 29}, 'name': {'type': 'String', 'value': 'Alice Johnson'}, 'active': {'type': 'Boolean', 'value': True}, 'id': 'ge8QCZbjvZA8ounlIMfITOpIT-R8SGO393aqU7oVPLMnRTRQb9Eai6rxlXo1n4vBwmiZ0m_LUc6ApeotD20gHQ'}
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: True
✓ Fake record exists: False (should be False)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 7 records (expected ~7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!
✓ Client created
✓ crypto_demo_hmac_py saved
✓ crypto_demo_aes_py saved
✓ crypto_demo_uuid_py saved
✓ crypto_demo_totp_py saved
✓ crypto_demo_encoding_py saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac_py { "payload": "hi" }
  POST /api/functions/crypto_demo_aes_py { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid_py
  POST /api/functions/crypto_demo_totp_py
  POST /api/functions/crypto_demo_encoding_py { "title": "Héllo World" }

✓ Cleaned up demo functions
=== Distinct Values Example ===

Inserting sample products...
Inserted 8 products

=== Distinct Categories (all products) ===
Found 3 distinct categories:
  - {'type': 'String', 'value': 'books'}
  - {'type': 'String', 'value': 'clothing'}
  - {'type': 'String', 'value': 'electronics'}

=== Distinct Statuses (all products) ===
Found 3 distinct statuses:
  - {'type': 'String', 'value': 'active'}
  - {'type': 'String', 'value': 'archived'}
  - {'type': 'String', 'value': 'discontinued'}

=== Distinct Statuses in Electronics ===
Found 2 distinct statuses for electronics:
  - {'type': 'String', 'value': 'active'}
  - {'type': 'String', 'value': 'discontinued'}

Cleanup done.
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: TRzRH7DJTKskfBsZNkkXTCgTuwLu1n2Ce4VzWMwYmsgCXYF1WO1mpDnP4Db-M0Q7zAp8EXjgx_7ZBuv7R3F8uA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: QFi93IXgLCbGYN6IAwfgnxFy5PartSs23GF1ZzUirpkR-sTZAyH69cW_B8lQ1DpJFmtbVdnn0trb_fPTI8xiMw

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
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: nGQtFXhG1XLa_F6YZ5U9ZBsICYij3m5H3HMdiGosAshikMWQcB572jzdAmaaGUn7Lqhqm09mlSWknsC70AtU7A

Call 1: Cache miss (fetches from API)
Response time: 172ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 13.6,
            "time": "2026-10-08T05:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.045180320739746094,
          "latitude": 40.710335,
          "longitude": -73.99308,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0
        }
      }
    }
  ],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 1,
    "stage_stats": [],
    "stages_executed": 2
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 4ms (38.9x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 13.6,
            "time": "2026-10-08T05:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.045180320739746094,
          "latitude": 40.710335,
          "longitude": -73.99308,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0
        }
      }
    }
  ],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 1,
    "stage_stats": [],
    "stages_executed": 2
  }
}

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!

=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user_py
✅ Saved composed function: get_user_wrapper_py (calls fetch_user_py + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type": "String", "value": "User 1"}
   Department: {"type": "String", "value": "engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user_py (uses KV)
✅ Saved SWR function using composition: swr_user_py

First call (cache miss - will fetch from API):
   ⏱️  Duration: 54.1ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...

Second call (cache hit - from cache):
   ⏱️  Duration: 1.8ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 29.4x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user_py
✅ Level 2 function: fetch_slim_user_py (calls validate_user_py)
✅ Level 3 function: get_verified_user_py (calls fetch_slim_user_py)

📊 Result from 3-level nested composition:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ Cleanup complete
✅ All composition examples completed!
client_function_contract: ok
🚀 ekoDB Functions Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: J85xQ8UfBWqprHXFjVzopFuoEa0N5dH1_FEtGUq2wFIu4Z2RatWwyoBFSd_WO2RSok8TSnxvtHI9a8VSM1zR8Q
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: duivbzKf_h_skxfJmGMBBnFvXvRedgB0d4uy1_bbGGhKEgMYkCxsAZ36iFXyW8B-v6DnNxdojLEQhTXQbKBLYA
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: -rSR_-tGHSL_HnYaSEQvnA06HemyE1W8ICyk_CZ0rO9ARKZby0rqJmOaSCwTgndEUgORoCoV8zW4FrqI-yeFiQ
📊 Statistics: 2 groups
   {'avg_score': {'type': 'Float', 'value': 60.0}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'type': 'String', 'value': 'active'}}

   {'avg_score': {'type': 'Float', 'value': 50.0}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'type': 'String', 'value': 'inactive'}}

📝 Example 4: Function Management

📋 Total functions: 3
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
🚀 ekoDB Python Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   {'avg_price': {'type': 'Float', 'value': 367.0}, 'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 5}}
   {'avg_price': {'type': 'Float', 'value': 365.6666666666667}, 'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 3}}
⏱️  Execution time: 0ms

📝 Example 3: Count Total Products

✅ Function saved
📊 Total products: [{'total': {'type': 'Integer', 'value': 8}}]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB Python AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They excel at searching for similar items based on high-dimensional vector representations, making them ideal for applications like image and text retrieval.

2. **High Performance**: Optimized for handling large-scale data, vector databases provide fast query response times even with extensive datasets.

3. **Scalability**: Designed to scale horizontally, they can manage growing amounts of data without significant drops in performance.

4. **Flexible Data Representation**: Support for various data types (text, images, audio) can be converted into vectors, making them versatile for different applications.

5. **Natural Language Processing**: They are particularly useful in NLP tasks where word embeddings and contextual embeddings are utilized for semantic searches.

6. **Integration with Machine Learning**: Seamless integration with AI and ML pipelines for real-time inference and analytics.

7. **Support for Complex Queries**: Ability to perform complex queries beyond simple key-value lookups, incorporating relevance and ranking.

8. **Real-time Data Handling**: Capable of handling real-time data updates, making them suitable for dynamic applications.

9. **Enhanced User Experience**: By providing more accurate and context-aware search results, they improve user engagement and satisfaction.

10. **Open Source Options**: Many vector databases are open source, allowing for customization and community support.

These benefits make vector databases particularly advantageous for machine learning, AI, and data retrieval tasks.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
📊 Generated 2 embeddings
   Dimensions: 1536
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text
🚀 ekoDB Python Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: nDfhaENKPJQwx0yc0AJMY2w6V6_rgXERemQbQhD0pJMMx9STyLPyVznXZMZTpfqk6P9MEg-PsQBCNwiILoArjA
📊 Found 2 product groups
   {'avg_price': {'type': 'Float', 'value': 575.6666666666666}, 'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 3}}
   {'avg_price': {'type': 'Float', 'value': 474.0}, 'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 2}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 2}}
   {'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 3}}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 3 products
⏱️  Execution time: 0ms

📝 Example 5: function with Parameter Definition

✅ Function saved
📊 Found 3 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
🧹 Cleaning up...
✅ Cleanup complete

🚀 ekoDB Python CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average score by role:
   {'avg_score': {'type': 'Float', 'value': 70.0}, 'count': {'type': 'Integer', 'value': 7}, 'role': {'type': 'String', 'value': 'user'}}
   {'avg_score': {'type': 'Float', 'value': 20.0}, 'count': {'type': 'Integer', 'value': 3}, 'role': {'type': 'String', 'value': 'admin'}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB Python KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: _A6zA4GorLSrMzeX0_C12PJuDvz19o0QkYv9GoDSACFwdoZJDZ-pL5stYRlcqGqAvgDWtD0YAypcmiqskht1Eg
✅ Inserted 2 products with wrapped types

📝 Example 2: Querying and Extracting Wrapped Types

📊 Found 2 products
   • Wireless Mouse
   • Laptop Pro

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {'value': '{"type":"Object","value":{"role":"admin","userId":"user_abc"}}'}
🔍 Key exists: True
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 5: Combined Wrapped Types + KV Usage

✅ Inserted order: cuKrV-cb5Gdab8pNGe2ZD9z5Edqtj-g2r3xNJvehMxVAwAQj7Zl3QW2l7cHvUxamQE9qPDH5zdwfQv79BD9rwg
✅ Cached order status
📊 Quick status lookup: {'value': '{"type":"Object","value":{"status":"processing","updated_at":"2026-10-08T05:45:58.957885+00:00"}}'}

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use field_* helpers for type-safe wrapped values
   ✅ field_decimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ Combine KV caching with collection inserts for real workflows
🚀 ekoDB Python Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Introduction to Machine Learning (AI)
   2. Database Design Principles (Database)
   3. Natural Language Processing (AI)
   4. Vector Databases Explained (Database)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {'category': {'type': 'String', 'value': 'AI'}, 'count': {'type': 'Integer', 'value': 2}}
   {'category': {'type': 'String', 'value': 'Database'}, 'count': {'type': 'Integer', 'value': 3}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Goal Template CRUD Example (Python) ===

--- Creating goal template ---
Created template: Data Migration (id: 4s5kE-fk8rmYsVPOvu5a_QIPhOwqgzTM8nqIN1kY6gGzp0PfmkGCso1xUsDKTFxpeOdJNEBd82rXGlFNIR6ccw)

--- Listing templates ---
Templates: {'count': 1, 'items': [{'description': {'type': 'String', 'value': 'Template for migrating data between schemas'}, 'id': '4s5kE-fk8rmYsVPOvu5a_QIPhOwqgzTM8nqIN1kY6gGzp0PfmkGCso1xUsDKTFxpeOdJNEBd82rXGlFNIR6ccw', 'steps': {'type': 'Array', 'value': [{'description': 'Analyze source schema'}, {'description': 'Create target schema'}, {'description': 'Migrate records'}, {'description': 'Validate results'}]}, 'title': {'type': 'String', 'value': 'Data Migration'}}]}

--- Getting template ---
Fetched: {'type': 'String', 'value': 'Data Migration'}

--- Updating template ---
Updated description: {'type': 'String', 'value': 'Updated: comprehensive data migration workflow'}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
=== ekoDB Goals / Tasks / Agents Integration Example (Python) ===

--- goal_create ---
Created goal: Deploy v3 (id: HOv0x2RqRWGKMyr68wEBQZtG_zAqloHY5-eJUKjEgkelSoziSF1u-YqmlysNSUAPtlPTdqoFf6w-qYcCVEwsAg)

--- goal_list ---
Goals: {'count': 1, 'goals': [{'created_at': '2026-10-08T05:45:59.301219+00:00', 'description': 'Deploy version 3 to production', 'id': 'HOv0x2RqRWGKMyr68wEBQZtG_zAqloHY5-eJUKjEgkelSoziSF1u-YqmlysNSUAPtlPTdqoFf6w-qYcCVEwsAg', 'status': 'pending', 'steps': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]', 'title': 'Deploy v3', 'updated_at': '2026-10-08T05:45:59.301219+00:00'}]}

--- goal_get ---
Fetched: {'type': 'String', 'value': 'Deploy v3'}

--- goal_update ---
Updated description: {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}

--- goal_search ---
Search results: {'count': 1, 'items': [{'_score': 13.2, 'created_at': {'type': 'DateTime', 'value': '2026-10-08T05:45:59.301219+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'HOv0x2RqRWGKMyr68wEBQZtG_zAqloHY5-eJUKjEgkelSoziSF1u-YqmlysNSUAPtlPTdqoFf6w-qYcCVEwsAg', 'status': {'type': 'String', 'value': 'pending'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-08T05:45:59.310952+00:00'}}]}

--- goal_step_start (step 0) ---
Step 0 started: {'created_at': {'type': 'DateTime', 'value': '2026-10-08T05:45:59.301219+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'HOv0x2RqRWGKMyr68wEBQZtG_zAqloHY5-eJUKjEgkelSoziSF1u-YqmlysNSUAPtlPTdqoFf6w-qYcCVEwsAg', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"InProgress"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-08T05:45:59.317924+00:00'}}

--- goal_step_complete (step 0) ---
Step 0 completed: {'created_at': {'type': 'DateTime', 'value': '2026-10-08T05:45:59.301219+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'HOv0x2RqRWGKMyr68wEBQZtG_zAqloHY5-eJUKjEgkelSoziSF1u-YqmlysNSUAPtlPTdqoFf6w-qYcCVEwsAg', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-08T05:45:59.324254+00:00'}}

--- goal_step_fail (step 1) ---
Step 1 failed: {'created_at': {'type': 'DateTime', 'value': '2026-10-08T05:45:59.301219+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'HOv0x2RqRWGKMyr68wEBQZtG_zAqloHY5-eJUKjEgkelSoziSF1u-YqmlysNSUAPtlPTdqoFf6w-qYcCVEwsAg', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","error":"Migration failed: column already exists","status":"Failed"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-08T05:45:59.329281+00:00'}}

--- goal_complete ---
Goal completed: {'type': 'String', 'value': 'pending_review'}

--- goal_approve ---
Goal approved: {'type': 'String', 'value': 'in_progress'}

--- goal_reject ---
Goal rejected: {'type': 'String', 'value': 'failed'}

--- task_create ---
Created task: Nightly Backup (id: LOzG2_LunU6so0RZiXIFpD6E6xj0g34ApwiICBUi0ma2ZnONkDsUT7pR1ylCRC24_jJsFCjg6EZbYVZDp17ZOA)

--- task_list ---
Tasks: {'count': 1, 'items': [{'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'LOzG2_LunU6so0RZiXIFpD6E6xj0g34ApwiICBUi0ma2ZnONkDsUT7pR1ylCRC24_jJsFCjg6EZbYVZDp17ZOA', 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}}]}

--- task_get ---
Fetched: {'type': 'String', 'value': 'Nightly Backup'}

--- task_start ---
Task started: {'type': 'String', 'value': 'running'}

--- task_succeed ---
Task succeeded: {'consecutive_failures': {'type': 'Integer', 'value': 0}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'LOzG2_LunU6so0RZiXIFpD6E6xj0g34ApwiICBUi0ma2ZnONkDsUT7pR1ylCRC24_jJsFCjg6EZbYVZDp17ZOA', 'last_error': {'type': 'Null', 'value': None}, 'last_run': {'type': 'DateTime', 'value': '2026-10-08T05:45:59.386354+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-08T05:45:59.386354+00:00'}}

--- task_pause ---
Task paused: {'type': 'String', 'value': 'paused'}

--- task_resume ---
Task resumed: {'type': 'String', 'value': 'active'}

--- task_fail ---
Task failed: {'consecutive_failures': {'type': 'Integer', 'value': 1}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'LOzG2_LunU6so0RZiXIFpD6E6xj0g34ApwiICBUi0ma2ZnONkDsUT7pR1ylCRC24_jJsFCjg6EZbYVZDp17ZOA', 'last_error': {'type': 'String', 'value': 'Disk full'}, 'last_run': {'type': 'DateTime', 'value': '2026-10-08T05:45:59.412627+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'next_run': {'type': 'DateTime', 'value': '2026-03-22T02:00:00+00:00'}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-08T05:45:59.412627+00:00'}}

--- task_due ---
Due tasks: {'count': 0, 'items': []}

--- task_delete ---
Task deleted

--- agent_create ---
Created agent: CodeReviewer (id: orjAWGtByDTrFU8Q4S3F0DPb4IlLntIR2KVpG_8FnIsJ5oQcW2BY10AujhI0SXfjOSbzI7i3TQyEWOzCYKP-dA)

--- agent_list ---
Agents: {'count': 1, 'items': [{'deployment_id': {'type': 'String', 'value': 'deploy_test'}, 'id': 'orjAWGtByDTrFU8Q4S3F0DPb4IlLntIR2KVpG_8FnIsJ5oQcW2BY10AujhI0SXfjOSbzI7i3TQyEWOzCYKP-dA', 'llm_model': {'type': 'String', 'value': 'gpt-4o'}, 'name': {'type': 'String', 'value': 'CodeReviewer'}, 'system_prompt': {'type': 'String', 'value': 'You review code for correctness and style.'}, 'tools': {'type': 'Array', 'value': ['web_search', 'file_read']}}]}

--- agent_get ---
Fetched: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_get_by_name ---
By name: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_update ---
Updated agent prompt: {'type': 'String', 'value': 'You review code. Be concise.'}

--- agents_by_deployment ---
Agents in deploy_test: {'count': 0, 'items': []}
WARNING: agents_by_deployment omitted created agent orjAWGtByDTrFU8Q4S3F0DPb4IlLntIR2KVpG_8FnIsJ5oQcW2BY10AujhI0SXfjOSbzI7i3TQyEWOzCYKP-dA; TODO: check/fix the server-side deployment lookup

--- agent_delete ---
Agent deleted

--- cleanup ---
Goals cleaned up

=== All goals/tasks/agents operations completed successfully ===
=== Join Operations Examples ===

Setting up sample data...
✅ Sample data created

1. Single collection join (users with departments):
Found 2 users with department data:
  - Alice Johnson: Engineering
  - Bob Smith: Sales

2. Join with filtering:
Found 1 users in Engineering:
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data:
  - Alice Johnson: Senior Software Engineer
  - Bob Smith: Sales Manager

4. Join orders with user data:
Found 2 completed orders:
  - Mouse ($25) by Alice Johnson
  - Laptop ($1200) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails:
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
✓ Client created
✓ py_users_register saved
✓ py_users_login saved
✓ py_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/py_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/py_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/py_users_verify_token { "token": "<jwt>" }

✓ Cleaned up demo functions
=== ekoDB KV Links Example (Python) ===

--- kv_set ---
Set KV key: app:config

--- insert document ---
Inserted document: QWzuHyUdAA3IkZNliYROjfmAhnPUhRmi8dTX_MvQuPKsxtnJWod6MwqaTtBsGWY4xzblItzhWjD8wPwnBv5_5Q

--- kv_link ---
Linked: None

--- kv_get_links ---
Links for app:config: [{'collection': 'kv_links_example_py', 'created_at': '2026-10-08T05:45:59.925372Z', 'document_id': 'QWzuHyUdAA3IkZNliYROjfmAhnPUhRmi8dTX_MvQuPKsxtnJWod6MwqaTtBsGWY4xzblItzhWjD8wPwnBv5_5Q', 'field_path': None, 'last_accessed': '2026-10-08T05:45:59.925998Z', 'metadata': {}}]

--- kv_unlink ---
Unlinked: None

--- kv_get_links (verify empty) ---
Links after unlink: []

--- cleanup ---
Cleaned up KV key and collection

=== KV links example completed ===
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {'value': '{"type":"Object","value":{"userId":123,"username":"john_doe"}}'}

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_operations:py:cache:product:1: success
  kv_operations:py:cache:product:2: success
  kv_operations:py:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_operations:py:cache:product:1: {'name': 'Product 1', 'price': 29.99}
  kv_operations:py:cache:product:2: {'name': 'Product 2', 'price': 39.99}
  kv_operations:py:cache:product:3: {'name': 'Product 3', 'price': 49.99}

=== KV Exists ===
Key exists: True

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: False

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  kv_operations:py:cache:product:1: deleted
  kv_operations:py:cache:product:2: deleted
  kv_operations:py:cache:product:3: deleted

✓ All KV operations completed successfully
=== KV Precision: Float vs Decimal ===

=== Test 1: Using Python Floats (LOSES PRECISION) ===
✓ Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 2: Using field_decimal() (PRESERVES PRECISION) ===
✓ Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.30000000000000004 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Cleanup ===
✓ Cleaned up test keys

=== Summary ===
✅ Use field_decimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ field_decimal() stores values as strings internally,
   preserving exact precision across all operations.
✓ Client created
✓ py_route_admin saved
✓ py_route_user_by_id saved
✓ py_route_user_posts saved
✓ py_route_org_create_member saved

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions
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
Found 2 valid users

8. Using bypass flags:
Found 2 users (bypassed cache)

=== Cleanup ===
✅ Deleted test collection

✅ Query Builder examples completed!
=== ekoDB Raw Completion Stream (SSE) Example ===

--- Basic SSE Raw Completion ---
Response: The three primary colors are red, blue, and yellow.

--- Structured Output via SSE ---
JSON response: [
  {
    "name": "Mercury",
    "diameter_km": 4879
  },
  {
    "name": "Venus",
    "diameter_km": 12104
  },
  {
    "name": "Mars",
    "diameter_km": 6779
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===
=== ekoDB Schedule Management Example (Python) ===

--- create_schedule ---
Created schedule: Nightly Report (id: 368dff75-4ff9-4a77-9278-b329c018676d)

--- list_schedules ---
Schedules: {'count': 1, 'schedules': [{'created_at': '2026-10-08T05:46:02.453192Z', 'cron_expression': '0 0 0 * * *', 'description': 'Generate and email nightly analytics report', 'enabled': True, 'function_label': 'schedule_noop_python_73526', 'id': '368dff75-4ff9-4a77-9278-b329c018676d', 'last_execution': None, 'name': 'Nightly Report', 'next_execution': '2026-10-09T00:00:00Z', 'parameters': {}, 'stats': {'avg_execution_time_ms': 0.0, 'failed_executions': 0, 'last_error': None, 'successful_executions': 0, 'total_executions': 0}, 'timezone': 'UTC', 'updated_at': '2026-10-08T05:46:02.453192Z'}]}

--- get_schedule ---
Fetched: Nightly Report cron=0 0 0 * * *

--- update_schedule ---
Updated cron: 0 30 1 * * *

--- trigger_schedule ---
Triggered: {'schedule_id': '368dff75-4ff9-4a77-9278-b329c018676d', 'status': 'triggered'}

--- pause_schedule ---
Paused: enabled=False

--- resume_schedule ---
Resumed: enabled=True
Schedule deleted successfully

=== Schedule management example completed ===
=== Schema Management Examples ===

1. Creating user schema with basic fields:
✅ User schema created

2. Creating product schema with text index:
✅ Product schema with indexes created

3. Creating document schema with vector index:
✅ Document schema with vector index created

4. Retrieving collection schema:
Schema fields: ['age', 'email', 'name', 'status']
Schema version: 1

5. Retrieving collection metadata:
Collection has 4 fields

6. Creating employee schema with all constraint types:
✅ Employee schema with all constraints created

✅ Schema management examples completed!
=== Search Examples ===

Setting up test data...
✅ Test data created

1. Basic full-text search:
Found 2 results
  1. Score: 12.870, Matched: name, email
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: bio, title
  2. Score: 13.200, Matched: title, bio
  3. Score: 13.200, Matched: title, bio
  4. Score: 13.200, Matched: bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: title, bio
  2. Score: 26.400, Matched: bio, title
  3. Score: 26.400, Matched: title, bio
  4. Score: 26.400, Matched: title, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.741
  2. Score: 0.726
  3. Score: 0.725

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.371, Matched: content, title
  2. Score: 0.863, Matched: content, title
  3. Score: 0.363, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 6.600, Matched: title

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Introduction to Machine Learning (category: ml)
  2. Deep Learning Fundamentals (category: ml)

=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'QCAKHolDXEdl5ttxMdtgTn_9VNohbHVua1vtzs-1QOsa39dPVNu6Q4QmFLTmeJ_wfGI2Lvirtvu92edfIUB_xw'}

=== Find by ID ===
Found: {'price': {'type': 'Float', 'value': 99.99}, 'created_at': {'type': 'String', 'value': '2026-10-08T01:46:02.912572'}, 'id': 'QCAKHolDXEdl5ttxMdtgTn_9VNohbHVua1vtzs-1QOsa39dPVNu6Q4QmFLTmeJ_wfGI2Lvirtvu92edfIUB_xw', 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'name': {'value': 'Test Record', 'type': 'String'}, 'active': {'value': True, 'type': 'Boolean'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'value': {'type': 'Integer', 'value': 42}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-10-08 01:46:02.912572
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'price': 99.99, 'created_at': '2026-10-08T01:46:02.912572', 'id': 'QCAKHolDXEdl5ttxMdtgTn_9VNohbHVua1vtzs-1QOsa39dPVNu6Q4QmFLTmeJ_wfGI2Lvirtvu92edfIUB_xw', 'data': 'aGVsbG8gd29ybGQ=', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'name': 'Test Record', 'active': True, 'tags': ['tag1', 'tag2', 'tag3'], 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'value': 42, 'categories': ['electronics', 'computers'], 'user_id': '550e8400-e29b-41d4-a716-446655440000'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'id': 'QCAKHolDXEdl5ttxMdtgTn_9VNohbHVua1vtzs-1QOsa39dPVNu6Q4QmFLTmeJ_wfGI2Lvirtvu92edfIUB_xw', 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'value': {'type': 'Integer', 'value': 100}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'active': {'value': True, 'type': 'Boolean'}, 'created_at': {'type': 'String', 'value': '2026-10-08T01:46:02.912572'}, 'price': {'value': 99.99, 'type': 'Float'}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: itAuQ4PJmD3ApiSv2gwkI0Bk7wPkeL9UI8ZP0kQPBKzsDNg8ft0RswBrmMXi3fSEKAOGJ5kxkyI5fGZTUuZzcA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
🚀 ekoDB Python Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


🧹 Cleaning up...
✓ Deleted 0 test scripts and owned SWR resources

Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native_py (ekoudmiNn6HiDbOVjaOi95WVmdpJuf8LVr03pQb_tJkjwjxkbne9khZj9TCLfpThadoU43FCUTQakL-Rg3vtGQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 128ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 72.6x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_py (gmtmgHCiMIgHatUPqljuEYkBvmkDrOQJoQCkXtxHWql_vYpOxf_3Bu5REt5uoirDllg0HckykbLevFeY3522GA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_py' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_py (ORp9UryUxzIVksWta4_DGAn5QEsuXYcQzCMDd7unEwj2Oh-IyxJ54r7tjoG5XzH0hKk_T6VgIq9Q7GT4jCB6Fg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_py' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_py (K9dZXlZt2f9Y5iAyvDQSeHFehtHHKe9_-BnY9gBd-_7radjoegmr0yIvGHLWiUzCDpADGNQKvCBG_EMSSIM2LQ)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

================================================================================
✅ Key Benefits of Native SWR:
✅ Single function: Replaces 4-function cache-aside pattern
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)
Native SWR:     SWR → Insert (2 functions)
Result:         60% fewer functions, cleaner code, same behavior 🎯

🧹 Cleaning up...
✓ Deleted 4 test scripts and owned SWR resources

✅ All examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_py (fN4R3TDU_K5wQwdKQvcEhis0T4Td6-0nQfduy_ccVmpvZjiTIhm8D_kM5B_EpyGwymYbredxRsirxkha6ufkpA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "zipcode": "92998-3874"
          },
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "email": "Sincere@april.biz",
          "id": 1,
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "website": "hildegard.org"
        }
      }
    }
  ],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 1,
    "stage_stats": [],
    "stages_executed": 2
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 2ms (served from cache)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "zipcode": "92998-3874"
          },
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "email": "Sincere@april.biz",
          "id": 1,
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "website": "hildegard.org"
        }
      }
    }
  ],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 1,
    "stage_stats": [],
    "stages_executed": 2
  }
}
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: UMe2dTk3y2Rl7EBFw8jA2le0bqWkTNF2-WxhVs8ZwyeVrI50UoXnLm0D9rky8bxupl-AJFN1dKRKPl4CR1hiSA
Created Bob: $500 - ID: sreO7wYjYljKmBTOysoT1HHY4snO3k6nMFNPdbtfTxug-Md6QVMwcfhrmlDhyWWM_yNpkOLGbPMH3g17ztu5AA

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 43ee772a-e5c1-44f6-8ace-8e046dd07c2c

=== Example 2: Operations within Transaction ===
Alice in transaction: {'balance': {'type': 'Integer', 'value': 800}, 'id': 'UMe2dTk3y2Rl7EBFw8jA2le0bqWkTNF2-WxhVs8ZwyeVrI50UoXnLm0D9rky8bxupl-AJFN1dKRKPl4CR1hiSA', 'account_id': {'type': 'String', 'value': 'ACC001'}, 'name': {'type': 'String', 'value': 'Alice'}}
Bob in transaction: {'name': {'type': 'String', 'value': 'Bob'}, 'account_id': {'value': 'ACC002', 'type': 'String'}, 'balance': {'value': 700, 'type': 'Integer'}, 'id': 'sreO7wYjYljKmBTOysoT1HHY4snO3k6nMFNPdbtfTxug-Md6QVMwcfhrmlDhyWWM_yNpkOLGbPMH3g17ztu5AA'}
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

✓ Verified committed balances: Alice=$800, Bob=$700

=== Example 5: Rollback Demo ===
New transaction: fca3200a-8f41-46a2-b713-a836579b0291
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: lfe3Nvbakg8E2l0LmDnVfnGY5CxIDqxyPAWctOE_1NM8DSxuNRG61dzwjCWtDuXPl2xF9h6fwf5esx57urmzbg

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 1 user functions:
  - get_active_users_py: Get Active Users

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (Python) ===

Created chat session: pYFzo_1eGiuE3BLAWR58Vl8ETAeqPrNX2lscWrnFJ3XHQmJOL_i_0WW3hWF0-kRUkdLrVr2PUUo7Ej-ySaT2wA

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: HWvw-1hiTKIb3fbqDMOFrYlE2qIeoxt81bueUb1i4JXRhyv7HTLm0BnL53eaOOJZF62A7qUH8f6oayqfjCpv0g
Execution time: 647ms
Token usage: {'completion_tokens': 8, 'prompt_tokens': 15, 'total_tokens': 23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example (Python) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_py' ===
✓ Subscribed (subscription_id: sub_85eb612cb8874e79b9f6e26011e3764a)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: tRgyu7VudRgeLRQ8Bg_tN7YUR3DNyzurDrREusPbdKRDfslFWTDWibpI5EfDY5md3CJZlscLOYPvDVfCgIObyg

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: tRgyu7VudRgeLRQ8Bg_tN7YUR3DNyzurDrREusPbdKRDfslFWTDWibpI5EfDY5md3CJZlscLOYPvDVfCgIObyg
     Timestamp:  2026-10-08T05:46:05.312015+00:00

Inserting record 2...
✓ Inserted: zT_DVQceB0h-MkEYIL-xkdKxaLRx09fF6TfIZfKA3HKv0D5tnXrZFa-oG-ybAeCz8EnG-t27kIoayYSd-HDfVA

  📡 Notification received:
     Event:      insert
     Record IDs: zT_DVQceB0h-MkEYIL-xkdKxaLRx09fF6TfIZfKA3HKv0D5tnXrZFa-oG-ybAeCz8EnG-t27kIoayYSd-HDfVA

=== Unsubscribing ===
✓ Unsubscribed: {'collection': 'ws_subscribe_example_py', 'found': True, 'unsubscribed': True}

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_py'

✓ WebSocket subscription example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: SK3Id7oreuZBLreU_KTis4jd18gRXjhTyyTjqs6fLc1w3VFimHBA9Kxcwg_NWaTc_hPLoXxEsoN6FgDj6qMnQA

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: {'id': 'sqzoXavluRWSCXOE2mRBh3GI4DNEZFm0TJYHsCAO5hvp0YlQEy8Ax7UGcACTe9ctTR6qc8CpkV563PJzRKXZmA'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'hmQRhGLGz0bUl30hiBxcC1BwNl4lCzLzfROQFB32blngjUXtAfhdnUqvXl4Ryg6RCZCOW8Y_SHM9LEbAV73OSQ'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'price': {'value': 150, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Product 1'}, 'id': 'sqzoXavluRWSCXOE2mRBh3GI4DNEZFm0TJYHsCAO5hvp0YlQEy8Ax7UGcACTe9ctTR6qc8CpkV563PJzRKXZmA'}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'id': 'sqzoXavluRWSCXOE2mRBh3GI4DNEZFm0TJYHsCAO5hvp0YlQEy8Ax7UGcACTe9ctTR6qc8CpkV563PJzRKXZmA', 'price': {'value': 500, 'type': 'Integer'}, 'name': {'value': 'Upsert Product', 'type': 'String'}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['id', 'name', 'email']
  First user: {'value': 'Dave Brown', 'type': 'String'} <{'type': 'String', 'value': 'dave@example.com'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['name', 'created_at', 'bio', 'age', 'status', 'avatar_url', 'user_role', 'id', 'email']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'type': 'String', 'value': 'Dave Brown'} (age {'value': 45, 'type': 'Integer'})
    - {'type': 'String', 'value': 'Alice Johnson'} (age {'type': 'Integer', 'value': 30})
    - {'type': 'String', 'value': 'Bob Smith'} (age {'value': 25, 'type': 'Integer'})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'type': 'String', 'value': 'Carol White'}: {'type': 'String', 'value': 'Manager'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['secret_token', 'email', 'id', 'api_key', 'avatar_url', 'user_role', 'status', 'name', 'created_at', 'password', 'bio', 'age']
  Projected query:
    - 3 fields per record
    - Fields: ['email', 'name', 'id']
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ All Python integration tests complete!
🧪 Running Go examples (direct HTTP/WebSocket)...

╔════════════════════════════════════════╗
║     ekoDB Go Examples Test Suite      ║
╚════════════════════════════════════════╝

=== Checking Server Connection ===
✓ Server is ready

=== Running 10 Examples ===

=== Running simple_crud.go ===
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: map[id:yFPY0vOQFkyupnZwSMBkPp-c3q-PI7Xuj-e_mevTplsU3CLH1sHpWCG0ELvPQFPla4X7IAf-jf6oiTZJv6UlBw]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:yFPY0vOQFkyupnZwSMBkPp-c3q-PI7Xuj-e_mevTplsU3CLH1sHpWCG0ELvPQFPla4X7IAf-jf6oiTZJv6UlBw name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: [map[active:map[type:Boolean value:true] id:yFPY0vOQFkyupnZwSMBkPp-c3q-PI7Xuj-e_mevTplsU3CLH1sHpWCG0ELvPQFPla4X7IAf-jf6oiTZJv6UlBw name:map[type:String value:Test Record] value:map[type:Integer value:42]]]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:yFPY0vOQFkyupnZwSMBkPp-c3q-PI7Xuj-e_mevTplsU3CLH1sHpWCG0ELvPQFPla4X7IAf-jf6oiTZJv6UlBw name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.go completed successfully

=== Running simple_websocket.go ===
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: F9sbSTwl_OlQzEcgOF30OUhCSVfi3xaFlD24EG-yE2DlYVlWoC4RTUmOJAMM4J2z3WtyNOYz5nV7Q8ZLzEuusQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1791438381815597000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "F9sbSTwl_OlQzEcgOF30OUhCSVfi3xaFlD24EG-yE2DlYVlWoC4RTUmOJAMM4J2z3WtyNOYz5nV7Q8ZLzEuusQ",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "value": {
          "type": "Integer",
          "value": 42
        }
      }
    ]
  },
  "type": "Success"
}
✓ Retrieved 1 record via WebSocket

✓ WebSocket example completed successfully
✓ simple_websocket.go completed successfully

=== Running http_functions.go ===
🚀 ekoDB Functions Example (Go/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: qAb5yGT_F44-3Ib22hiH8MXJMpbTMLAgHk58jZoFslDOXCAzTTvrUFTKEjbFYnKgF_1aEU2oklWwCrBXlHHpiw
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: ALO1qivam5l-QDxQEK7IJ4IrEwCZ5pxag5m06HD0Q4FMsGtEuQ7RuyjSXpJNAIPVdTcL1fTpb7UhNXhCxa0QwQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: aLils6lZkWCBvxXDoizQwEA9_lqiutbmB7yRQdfEHR3GsdnUDq4Y3vvhys9LpfBL-9FbOcvN9hYLB6oIyzl0Cg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":{"type":"Float","value":50},"count":{"type":"Integer","value":5},"max_score":{"type":"Integer","value":90},"status":{"type":"String","value":"inactive"}}
   {"avg_score":{"type":"Float","value":60},"count":{"type":"Integer","value":5},"max_score":{"type":"Integer","value":100},"status":{"type":"String","value":"active"}}

📝 Example 4: Function Management

📋 Total functions: 3
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
✓ http_functions.go completed successfully

=== Running batch_operations.go ===
=== Batch Operations (Direct HTTP) ===

✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
✓ batch_operations.go completed successfully

=== Running kv_operations.go ===
=== Key-Value Operations (Direct HTTP) ===

✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123:go

=== KV Get ===
Retrieved value: map[type:Object value:map[userId:123 username:john_doe]]

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1:go: map[type:Object value:map[name:Product 1 price:29.99]]
cache:product:2:go: map[type:Object value:map[name:Product 2 price:39.989999999999995]]
cache:product:3:go: map[type:Object value:map[name:Product 3 price:49.989999999999995]]

=== KV Delete ===
✓ Deleted key: session:user123:go
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ kv_operations.go completed successfully

=== Running collection_management.go ===
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: 3vqlZlGbV7Mn4CpN-gVghOktcIp64I0R_0dvaLJu8RUYFRzZmQ__FE3vwC2jApEWQ3hBLjkg-xm0Hyfqw3in0w

=== List Collections ===
Total collections: 13
Sample collections: [chat_agent_configs__ek0_testing chat_goal_templates__ek0_testing audit__ek0_testing chat_tasks__ek0_testing agent_function_versions__ek0_testing]

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ collection_management.go completed successfully

=== Running transactions.go ===
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: om_rKRJBuTlZGJT7iTIUllVYGJ11F5JBn7VHXGpxYuCIVJ67uL5DUDca6rksfrpE4xSn5lxPrh-vXK4m0WsVMg
Created Bob: $500 - ID: qYdGP3NtCaJVjtat1TbZ7FkVc4Dbc09G3W1ZofWpqfVczn-015fm1KOyJTPzngfO0dC7LcpSwhUdY7AODZjKLA

=== Example 1: Begin Transaction ===
Transaction ID: 2f0fc5c5-7e21-4971-a188-33583c504930

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: map[type:Integer value:800]
Bob: map[type:Integer value:700]

=== Example 5: Rollback ===
New transaction: eb085050-5bfe-4ca6-b93a-5f31e5e71939
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: map[type:Integer value:700]

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✓ transactions.go completed successfully

=== Running crud_functions.go ===
🚀 ekoDB Complete CRUD Functions Example
============================================================
Demonstrates:
  • Insert + Verify (using Query)
  • Query + Update Status + Verify
  • Query + Update Credits + Verify
  • Query Before Delete + Delete + Verify Gone

Each function shows Functions chaining with proper verification
============================================================

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted collection: crud_functions_users_go

============================================================
📝 function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: TxT_u06c6rGmCM55CxyhtMveIP7lh6PlnTyPMPrj5EPTn3OyIKQqpeNWN_Syfe_l5Vb930rPIPHnZp3Tu3maZw

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: fdMXVZlXyFC_BZgPMVWZnko0DUXbWOQMkPA4h_JWGOqN_4Pe36TUSHmwZW4mrv6-USouHHOsFW-vVvPzuNEEAg
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: HpWIQBjvPdGyrfCMGzcxFt4uvFtu15ckr9gI8XTf2MhJqHpWsIFLw25BfX9frdkL_oGoEBOUFF4LwAx_km8H9w

2️⃣ Calling function (Query + Update + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: map[type:String value:active]
   📋 Name: map[type:String value:Alice Smith]

============================================================
📝 function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 2R7yTuS09ARUESnMmhAlY0IzpQ970NbfAZSm_xYLVjfs7yb88LmKk36M1BCJnY8prPn33BiyjrKeNoNSlSXrqQ

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: map[type:Integer value:100]
   📋 Status: map[type:String value:active]
   📋 Name: map[type:String value:Alice Smith]

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: jyYy5Y1tE3GPLd1Lm5xJYL-7l4-Netp0FYE0D9pD928U2Du4SkCl-j-Ll28LN4UbORj35_NTX_gS4tq6qP2jTQ

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: TxT_u06c6rGmCM55Cxyh...
   ✅ Deleted script: HpWIQBjvPdGyrfCMGzcx...
   ✅ Deleted script: 2R7yTuS09ARUESnMmhAl...
   ✅ Deleted script: jyYy5Y1tE3GPLd1Lm5xJ...
   ✅ Deleted collection: crud_functions_users_go

============================================================
✅ Complete CRUD Functions Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Functions chain steps together
   ✅ Each function demonstrates operation + verification
   ✅ Parameters make functions reusable
   ✅ Verification is built into the function itself
   ✅ Complete CRUD lifecycle in 4 focused functions
✓ crud_functions.go completed successfully

=== Running document_ttl.go ===
╔════════════════════════════════════════════════════════╗
║     TTL EXPIRATION VERIFICATION TEST                   ║
╚════════════════════════════════════════════════════════╝

This test verifies that document TTL expiration works correctly.
We will insert documents with short TTL and verify they expire.

✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = sahZeaLMFExioV4GF4bB80xCH3JUK1sesEhg2eE1iRdYoMQGkrEuqtvdVDasjOfDwCQxYrZLrMxdGAb9LHYN4g
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(sahZeaLMFExioV4GF4bB80xCH3JUK1sesEhg2eE1iRdYoMQGkrEuqtvdVDasjOfDwCQxYrZLrMxdGAb9LHYN4g)
  Output: Found document with name = map[type:String value:TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(sahZeaLMFExioV4GF4bB80xCH3JUK1sesEhg2eE1iRdYoMQGkrEuqtvdVDasjOfDwCQxYrZLrMxdGAb9LHYN4g)
  Output: Error (expected) - request failed with status 404: ��error�Record not found (expired)
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
CLEANUP
═══════════════════════════════════════════════════════════

╔════════════════════════════════════════════════════════╗
║              ALL TTL TESTS PASSED ✓                    ║
╚════════════════════════════════════════════════════════╝

TTL expiration is working correctly:
  • Documents with TTL expire after the specified time
  • Documents without TTL persist indefinitely
  • Different TTL durations are handled correctly
✓ Deleted test collection
✓ document_ttl.go completed successfully

=== Running websocket_ttl.go ===
╔════════════════════════════════════════════════════════╗
║   WEBSOCKET TTL EXPIRATION VERIFICATION TEST           ║
╚════════════════════════════════════════════════════════╝

This test verifies TTL expiration works via WebSocket connections.
We will use WebSocket to insert, query, and verify TTL expiration.

✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = WJJzrpPeCtqLaNITbJBeO3FcRrqM2bt65a0JKKqGXIoXyC5Lqu65__luJfxqWWLHrFGNT5uHSA-5_P6xWXlueA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(WJJzrpPeCtqLaNITbJBeO3FcRrqM2bt65a0JKKqGXIoXyC5Lqu65__luJfxqWWLHrFGNT5uHSA-5_P6xWXlueA)
  Output: Found document with name = map[type:String value:WS TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(WJJzrpPeCtqLaNITbJBeO3FcRrqM2bt65a0JKKqGXIoXyC5Lqu65__luJfxqWWLHrFGNT5uHSA-5_P6xWXlueA)
  Output: Error (expected) - request failed with status 404: ��error�Record not found (expired)
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
CLEANUP
═══════════════════════════════════════════════════════════

╔════════════════════════════════════════════════════════╗
║          WEBSOCKET TTL TEST PASSED ✓                   ║
╚════════════════════════════════════════════════════════╝

WebSocket TTL expiration is working correctly:
  • Documents with TTL inserted via client expire correctly
  • Queries correctly return nil for expired documents
✓ Deleted test collection
✓ websocket_ttl.go completed successfully

╔════════════════════════════════════════╗
║           Test Summary                 ║
╚════════════════════════════════════════╝
Total: 10
Passed: 10
Failed: 0
✅ Go direct examples complete!
=== ekoDB Advanced CRUD Integration Example (Go) ===

--- Inserting base record ---
Inserted record: IAedDqipupmIIU8NZe9CoMi4CuTwKYFxV-HNmLMIjEUzbWoNyr5q1-uuOsFRdW-kXflvN2ckVQYCQ2l4QmS_jA

--- UpdateWithAction: increment ---
After increment by 5: count = map[type:Integer value:5]
After increment by 3: count = map[type:Integer value:8]

--- UpdateWithAction: push ---
After push 'new-tag': tags = map[type:Array value:[initial new-tag]]
After push 30.0: scores = map[type:Array value:[10 20 30]]

--- UpdateWithAction: decrement ---
After decrement by 2: count = map[type:Integer value:6]

--- UpdateWithActionSequence ---
After sequence: count=map[type:Integer value:16], tags=map[type:Array value:[initial new-tag batch-added]], scores=map[type:Array value:[10 20 30 99.9]]

--- Cleanup ---
Cleaned up all resources

=== All advanced CRUD operations completed successfully ===
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
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: lyrSuE2NYGvjR5S8luiVtbFioXC_9FG5gRotBZacTbU7VDlo1DUwceDOSPkhh5nuf2LVSki4akkLWhqiY6kVOw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you have any more questions or need further assistance, feel free to ask!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any other questions or need more information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: wKaLpXE3DB_BBRGwI2MVSJFwR5PRHc5NShcFZEYGdDdVsKVqYsLFwITSBSXNYCV6b8sjJaX5ghSZTBfqSWUYdw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: wKaLpXE3DB_BBRGwI2MVSJFwR5PRHc5NShcFZEYGdDdVsKVqYsLFwITSBSXNYCV6b8sjJaX5ghSZTBfqSWUYdw
✓ Deleted chat session: lyrSuE2NYGvjR5S8luiVtbFioXC_9FG5gRotBZacTbU7VDlo1DUwceDOSPkhh5nuf2LVSki4akkLWhqiY6kVOw
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: YbbbU20q9hKKDzH0ZKUfZ3-mpi242httbchCPS1qiGlWmW9Meo4tAskU4Wm4vvQYrA9I7O6wAmDroyMD4hlZ0Q

=== Sending Chat Message ===
Message ID: 0X3ICNvhEL5sWONh9jTHqZaWyhsBZTX-oEjMGQzSgeUUUQFAjtPnQ-OMQG89LOe_eF0-dC16JxHcuBXCsLSUqA

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB**
   - Description: A high-performance database product with AI capabilities
   - Price: $99

2. **ekoDB Pro**
   - Description: Enterprise edition product with advanced features
   - Price: $299

3. **ekoDB Cloud**
   - Description: Fully managed cloud database service product
   - Price: $499

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:L_FAgAp2JdDreLg9O7-TJaNrSlkxrg1wZ0viVRJqpyygws8PbehkRDyVA1MjEI3fiLQC95QdRAVzhUefNZC-Jg name:ekoDB price:99] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:izEFrrwpB2kChhF6gUZ9kAcegp25IWU15UD0EVFq8fFFU9q-QHn3FaTPCFI1Orr9Y56gCLfIQzUdmZDt0Tq78A name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:tBQnY6kAcS6vBX3lt4U5T-N5bKHlj-eq_Dx0Pre5Z0yn9-xMUQopW6Oz_ugMJGKRRSo8_goS241BBvBDX_sgUw name:ekoDB Cloud price:499] score:0.1111111111111111]

Execution Time: 2890ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 90
Total tokens: 3503

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (Go) ===

Created session: tHmYnbuxsoCJpzfbuSTJ22gXNu-Z2MdEWR05_Ob1kooGDC8nUWtMJwApLRxOST0tb77ZPKu-K6nTdVzkx2gjJQ

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}EkoDB is a high-performance, embedded database designed for use in various applications where efficient data storage and retrieval are crucial. It's characterized by its lightweight nature, making it suitable for use in environments with limited resources, such as mobile devices, Internet of Things (IoT) devices, and other embedded systems.

EkoDB typically features:

1. **Embedded Design**: It runs within the application rather than as a separate server, allowing for faster access to data since there's no network overhead.

2. **High Performance**: EkoDB is optimized for low-latency operations and can handle a significant amount of transactions per second, making it ideal for real-time applications.

3. **ACID Compliance**: The database likely adheres to the principles of ACID (Atomicity, Consistency, Isolation, Durability), ensuring reliable transactions even in the event of failures.

4. **Schema Flexibility**: It may support various data models, such as key-value pairs, which allows developers to choose the most suitable model for their particular use case.

5. **Concurrency Control**: The database is designed to allow multiple users or processes to read and write to the database simultaneously while maintaining data integrity.

EkoDB is utilized in applications where traditional databases might be too bulky or where resource constraints are a concern. Examples include mobile applications, automotive systems, and various embedded platforms.

For specific features, performance benchmarks, and configurations, it would be beneficial to refer to the official documentation or website of EkoDB, as implementations and capabilities can vary.

--- Stream complete ---
Message ID: MSLMr_k2IcyN-hY1AOKEBhTbjQZUTPOE8oO5pb3bPhJwvPwqOnrZnSJ8iWtScu2GnIa49y6UZS3A2S64W9LvBA
Execution time: 4511ms
Context window: 128000 tokens

✓ Chat message stream example completed
✓ Client created

=== Get All Chat Models ===
OpenAI models: [o3-2025-04-16 gpt-5.1-chat-latest gpt-5.3-chat-latest gpt-5.2-2025-12-11 sora-2 chatgpt-image-latest gpt-4.1-mini gpt-3.5-turbo gpt-4o-mini gpt-image-2.5-sunburst gpt-image-2.5-flare-2026-09-08 gpt-6-luna gpt-5.4-mini-2026-03-17 gpt-4-turbo gpt-4.1-nano gpt-image-2 gpt-5.4-pro gpt-realtime-mini-2025-12-15 gpt-3.5-turbo-16k babbage-002 text-embedding-ada-002 gpt-4o-mini-tts gpt-4o-2024-08-06 o4-mini-2025-04-16 gpt-4o-2024-11-20 omni-moderation-latest gpt-realtime tts-1-hd gpt-realtime-1.5 gpt-realtime-2 gpt-5-search-api gpt-5.5-2026-04-23 gpt-5.1 gpt-5.1-codex gpt-4o o1-2024-12-17 gpt-5.2-chat-latest gpt-4o-mini-tts-2025-12-15 gpt-image-1 davinci-002 gpt-image-2-2026-04-21 gpt-4o-mini-transcribe o3-mini gpt-audio gpt-5.4-nano o3 gpt-5.1-codex-max gpt-realtime-2.1 gpt-4.1 gpt-5.5 gpt-4o-mini-search-preview-2025-03-11 text-embedding-3-large gpt-4o-mini-search-preview gpt-4o-2024-05-13 gpt-6-sol gpt-5.3-codex gpt-3.5-turbo-instruct sora-2-pro gpt-5 gpt-audio-mini-2025-12-15 gpt-transcribe o4-mini-deep-research-2025-06-26 o1-pro-2025-03-19 gpt-4.1-nano-2025-04-14 gpt-5-search-api-2025-10-14 gpt-4-turbo-2024-04-09 gpt-realtime-2.1-mini tts-1 gpt-5-mini omni-moderation-2024-09-26 gpt-5-mini-2025-08-07 gpt-realtime-2025-08-28 gpt-5.4-2026-03-05 gpt-live-transcribe gpt-3.5-turbo-0125 gpt-5-nano gpt-5.6-luna gpt-4 gpt-image-2.5-sunburst-2026-09-08 whisper-1 gpt-5.4-mini gpt-5-pro o1-pro gpt-realtime-whisper gpt-5.2-pro gpt-5-chat-latest gpt-live-1 gpt-4o-mini-transcribe-2025-12-15 gpt-5-2025-08-07 tts-1-1106 gpt-5.5-pro gpt-5-pro-2025-10-06 chat-latest gpt-5.2-pro-2025-12-11 gpt-4o-mini-transcribe-2025-03-20 gpt-audio-2025-08-28 gpt-5.4-pro-2026-03-05 gpt-realtime-translate gpt-5.6-sol gpt-audio-mini-2025-10-06 gpt-5.5-pro-2026-04-23 tts-1-hd-1106 gpt-realtime-mini gpt-5.4 gpt-4o-transcribe-diarize gpt-5-codex gpt-image-1-mini gpt-image-1.5 gpt-5.1-2025-11-13 o4-mini-deep-research gpt-4o-search-preview-2025-03-11 o3-mini-2025-01-31 gpt-4-0613 gpt-4o-search-preview gpt-5.6-terra text-embedding-3-small gpt-audio-1.5 gpt-3.5-turbo-instruct-0914 gpt-6-astra gpt-5.2-codex gpt-3.5-turbo-1106 gpt-5-nano-2025-08-07 gpt-audio-mini gpt-4o-mini-tts-2025-03-20 gpt-5.2 gpt-4.1-mini-2025-04-14 gpt-4.1-2025-04-14 gpt-4o-mini-2024-07-18 o4-mini o1 gpt-5.4-nano-2026-03-17 gpt-5.1-codex-mini gpt-4o-transcribe gpt-image-2.5-flare gpt-6.1-sol]
Anthropic models: [claude-haiku-5-5 claude-sonnet-5-5 claude-opus-5-5 claude-fable-5-1 claude-opus-5 claude-sonnet-5 claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929]
Perplexity models: []
Gemini models: []
  anthropic: ok 14 models
  gemini: not_configured (unverified) No Gemini API Key
  openai: ok 135 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get OpenAI Models ===
OpenAI models: [o3-2025-04-16 gpt-5.1-chat-latest gpt-5.3-chat-latest gpt-5.2-2025-12-11 sora-2 chatgpt-image-latest gpt-4.1-mini gpt-3.5-turbo gpt-4o-mini gpt-image-2.5-sunburst gpt-image-2.5-flare-2026-09-08 gpt-6-luna gpt-5.4-mini-2026-03-17 gpt-4-turbo gpt-4.1-nano gpt-image-2 gpt-5.4-pro gpt-realtime-mini-2025-12-15 gpt-3.5-turbo-16k babbage-002 text-embedding-ada-002 gpt-4o-mini-tts gpt-4o-2024-08-06 o4-mini-2025-04-16 gpt-4o-2024-11-20 omni-moderation-latest gpt-realtime tts-1-hd gpt-realtime-1.5 gpt-realtime-2 gpt-5-search-api gpt-5.5-2026-04-23 gpt-5.1 gpt-5.1-codex gpt-4o o1-2024-12-17 gpt-5.2-chat-latest gpt-4o-mini-tts-2025-12-15 gpt-image-1 davinci-002 gpt-image-2-2026-04-21 gpt-4o-mini-transcribe o3-mini gpt-audio gpt-5.4-nano o3 gpt-5.1-codex-max gpt-realtime-2.1 gpt-4.1 gpt-5.5 gpt-4o-mini-search-preview-2025-03-11 text-embedding-3-large gpt-4o-mini-search-preview gpt-4o-2024-05-13 gpt-6-sol gpt-5.3-codex gpt-3.5-turbo-instruct sora-2-pro gpt-5 gpt-audio-mini-2025-12-15 gpt-transcribe o4-mini-deep-research-2025-06-26 o1-pro-2025-03-19 gpt-4.1-nano-2025-04-14 gpt-5-search-api-2025-10-14 gpt-4-turbo-2024-04-09 gpt-realtime-2.1-mini tts-1 gpt-5-mini omni-moderation-2024-09-26 gpt-5-mini-2025-08-07 gpt-realtime-2025-08-28 gpt-5.4-2026-03-05 gpt-live-transcribe gpt-3.5-turbo-0125 gpt-5-nano gpt-5.6-luna gpt-4 gpt-image-2.5-sunburst-2026-09-08 whisper-1 gpt-5.4-mini gpt-5-pro o1-pro gpt-realtime-whisper gpt-5.2-pro gpt-5-chat-latest gpt-live-1 gpt-4o-mini-transcribe-2025-12-15 gpt-5-2025-08-07 tts-1-1106 gpt-5.5-pro gpt-5-pro-2025-10-06 chat-latest gpt-5.2-pro-2025-12-11 gpt-4o-mini-transcribe-2025-03-20 gpt-audio-2025-08-28 gpt-5.4-pro-2026-03-05 gpt-realtime-translate gpt-5.6-sol gpt-audio-mini-2025-10-06 gpt-5.5-pro-2026-04-23 tts-1-hd-1106 gpt-realtime-mini gpt-5.4 gpt-4o-transcribe-diarize gpt-5-codex gpt-image-1-mini gpt-image-1.5 gpt-5.1-2025-11-13 o4-mini-deep-research gpt-4o-search-preview-2025-03-11 o3-mini-2025-01-31 gpt-4-0613 gpt-4o-search-preview gpt-5.6-terra text-embedding-3-small gpt-audio-1.5 gpt-3.5-turbo-instruct-0914 gpt-6-astra gpt-5.2-codex gpt-3.5-turbo-1106 gpt-5-nano-2025-08-07 gpt-audio-mini gpt-4o-mini-tts-2025-03-20 gpt-5.2 gpt-4.1-mini-2025-04-14 gpt-4.1-2025-04-14 gpt-4o-mini-2024-07-18 o4-mini o1 gpt-5.4-nano-2026-03-17 gpt-5.1-codex-mini gpt-4o-transcribe gpt-image-2.5-flare gpt-6.1-sol]

=== Get Anthropic Models ===
Anthropic models: [claude-haiku-5-5 claude-sonnet-5-5 claude-opus-5-5 claude-fable-5-1 claude-opus-5 claude-sonnet-5 claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929]

✓ Chat Models API example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: ARfT2xTQMOLJIeLMCqenq39dHxNg4otXFEZQKWD7pHZfxBQARiFHwFH_qkTaK7Y0cFDe4yCr6NABkPisthTQVQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need more information or have any other questions, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 6mHGmHiBbH2Z5DJpIJgc8za_swyFYMaWNxEyghUjugT1yuHu5DJmRs_5zHdGLcaBnl1HhW9OUovoJYtPMRlkAw
  Parent: ARfT2xTQMOLJIeLMCqenq39dHxNg4otXFEZQKWD7pHZfxBQARiFHwFH_qkTaK7Y0cFDe4yCr6NABkPisthTQVQ

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: 6mHGmHiBbH2Z5DJpIJgc8za_swyFYMaWNxEyghUjugT1yuHu5DJmRs_5zHdGLcaBnl1HhW9OUovoJYtPMRlkAw (Untitled)
  Session 2: ARfT2xTQMOLJIeLMCqenq39dHxNg4otXFEZQKWD7pHZfxBQARiFHwFH_qkTaK7Y0cFDe4yCr6NABkPisthTQVQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 6mHGmHiBbH2Z5DJpIJgc8za_swyFYMaWNxEyghUjugT1yuHu5DJmRs_5zHdGLcaBnl1HhW9OUovoJYtPMRlkAw

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: RrXWRk6cmqnKkBOLDirYusLCMAk30KRrzZEch7eBbHUFEUzQqv2SBonOSPV-IStnCoY90DBn6zliFdDiE_MwRQ

=== List Collections ===
Total collections: 13
Sample collections: [chat_agent_configs__ek0_testing chat_goal_templates__ek0_testing audit__ek0_testing chat_tasks__ek0_testing agent_function_versions__ek0_testing]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test_go' exists: false

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test_go' exists: true

=== Count Documents ===
Document count in 'collection_utils_test_go': 5

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_go'

✓ Collection Utilities example complete
✓ Client created
✓ conc_demo_pay_go saved
✓ conc_demo_rl_fail_go saved
✓ conc_demo_rl_skip_go saved
✓ conc_demo_lock_go saved

Invoke them like:
  POST /api/functions/conc_demo_pay_go        { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail_go    { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip_go    { "user_id": 42 }
  POST /api/functions/conc_demo_lock_go       { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:f61T59RtfVKCqtSc8wYNPgacz_sM4RHnqsbLPlmIiNDB8l0Ifphks_CbkXvVrv79ke3q6m-AdRy2zgn5B57meg]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:f61T59RtfVKCqtSc8wYNPgacz_sM4RHnqsbLPlmIiNDB8l0Ifphks_CbkXvVrv79ke3q6m-AdRy2zgn5B57meg name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:new-user-1791438447792625000]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:f61T59RtfVKCqtSc8wYNPgacz_sM4RHnqsbLPlmIiNDB8l0Ifphks_CbkXvVrv79ke3q6m-AdRy2zgn5B57meg name:map[type:String value:Alice Johnson]]
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: true
✓ Fake record exists: false (should be false)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 7 records (expected 7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!
✓ Client created
✓ crypto_demo_hmac_go saved
✓ crypto_demo_aes_go saved
✓ crypto_demo_uuid_go saved
✓ crypto_demo_totp_go saved
✓ crypto_demo_encoding_go saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac_go     { "payload": "hi" }
  POST /api/functions/crypto_demo_aes_go      { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid_go
  POST /api/functions/crypto_demo_totp_go
  POST /api/functions/crypto_demo_encoding_go { "title": "Héllo World" }

✓ Cleaned up demo functions
=== Distinct Values Example ===

Inserting sample products...
Inserted 8 products

=== Distinct Categories (all products) ===
Found 3 distinct categories:
  - map[type:String value:books]
  - map[type:String value:clothing]
  - map[type:String value:electronics]

=== Distinct Statuses (all products) ===
Found 3 distinct statuses:
  - map[type:String value:active]
  - map[type:String value:archived]
  - map[type:String value:discontinued]

=== Distinct Statuses in Electronics ===
Found 2 distinct statuses for electronics:
  - map[type:String value:active]
  - map[type:String value:discontinued]

Cleanup done.
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: PR27CW6jDGaWMvOGX791wR18XtsoLuXs8wBofNUPkfwYWOETZXoE2U-nSMuFkS0q1CtA_QoeQ9peX3Is9ren5g

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 2xr2-5wWCmig1nwihwa_cA1UrCXDg59cqb5yZNbzxUjJetzcEHR1p_dKTMD7is9V-PKuy1WnurBJCvmAumf53w

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
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: oDyQzbZeh7jpbQM7jXZnP-m4ADoTNsYPreQffbwLr67IufF0hA6z-Y_wS7YTPq66Q1DsBq71RojaB-f0A017Vg

Call 1: Cache lookup
Response time: 2ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 2ms
Found 1 cached entries

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!

🧹 Cleaning up...
✓ Cleanup complete
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 63.571292ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "type": "Object",
          "value": {
            "address": {
              "city": "Gwenborough",
              "geo": {
                "lat": "-37.3159",
          ...

Second call (cache hit - from cache):
   ⏱️  Duration: 4.047083ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "type": "Object",
          "value": {
            "address": {
              "city": "Gwenborough",
              "geo": {
                "lat": "-37.3159",
          ...
   🚀 Cache speedup: 15.8x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ All composition examples completed!
client_function_contract: ok
🚀 ekoDB Functions Example (Go Client)

✅ Client initialized

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test scripts

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: 0dNJTYcKykhGRdY-XaD5zbzhLdiM-mU8K9K_q1v6aYVUYCRl9mLApU2TDYOlByMU6P7hKHsEcXMqKUCkAoB5qw
📊 Found 5 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 3 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   map[avg_score:map[type:Float value:50] count:map[type:Integer value:5] status:map[type:String value:inactive]]
   map[avg_score:map[type:Float value:60] count:map[type:Integer value:5] status:map[type:String value:active]]
⏱️  Execution time: 0ms

📝 Example 4: Function Management

📋 Total functions: 3
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved
📊 Total user count: 10
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test scripts

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • Built-in error handling
🚀 ekoDB Go Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   map[avg_price:map[type:Float value:365.6666666666667] category:map[type:String value:Furniture] count:map[type:Integer value:3]]
   map[avg_price:map[type:Float value:367] category:map[type:String value:Electronics] count:map[type:Integer value:5]]
⏱️  Execution time: 0ms

✅ All advanced script examples finished!
🧹 Cleaning up...
✅ Cleanup complete
🚀 ekoDB Go AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
📊 AI Response generated
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
📊 Embedding generated
⏱️  Execution time: 0ms

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text
🧹 Cleaning up...
✅ Cleanup complete
🚀 ekoDB Go Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: N9tVb-e9VL46HKpcGLwbHnaxQp-d2y6Yf0VTJD6YVDCJ0ySnRbDpVx9eQL6q-UF1g2gNCPuRsj69PmVGqsMrsQ
📊 Found 2 category stats

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories

📝 Example 4: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages


✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Function management (save, call, delete)
🧹 Cleaning up...
✅ Cleanup complete
🚀 ekoDB Go CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   map[count:map[type:Integer value:7] status:map[type:String value:active]]
   map[count:map[type:Integer value:3] status:map[type:String value:inactive]]
⏱️  Execution time: 0ms

✅ All CRUD script examples finished!
🧹 Cleaning up...
✅ Cleanup complete
🚀 ekoDB Go KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: IzEK4UdSB9ZTtZu0ABgqWABNjyVSAzt3OVGudCC12FXfPpzGHTqEx1T_wC6iBdvdgfcS86YLCiKNyR7Zcq6H1w
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: vj63LT1EN6INhuLeOTeVLlv0pC_yqJs9nFeJ0gUATRroTD3zkHkAcs-UA5chHi7wScScmbPr1Y6ryaarppQ61A
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[type:Object value:map[role:admin userId:user_abc]]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Functions

✅ Function saved: f-Bf1Q8s5x_CtDN8YYjsdGwbFQc7gnhSGNPeeJxeyAOCEF7zO3dVeN7rc04HmKuS6A7n7fkN4Tqre41Xo8-58A
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: yEGOzLQIvlkGw8BO-Qd0Qs82Ob60kFHH_2yZDGTGrZrMH-2WUb7AFVen6vQLJ5wdWAiXCb-Ochihe3ZVXnF9-w
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use Field* helpers for type-safe wrapped values
   ✅ FieldDecimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ StageKv*() functions work within scripts
🧹 Cleaning up...
✅ Cleanup complete

🚀 ekoDB Go Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   2. map[type:String value:Database Design Principles] (map[type:String value:Database])
   3. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
   4. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   5. map[type:String value:Natural Language Processing] (map[type:String value:AI])
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   map[category:map[type:String value:AI] count:map[type:Integer value:2]]
   map[category:map[type:String value:Database] count:map[type:Integer value:3]]
⏱️  Execution time: 0ms

✅ All search script examples finished!
🧹 Cleaning up...
✅ Cleanup complete
=== ekoDB Goal Template CRUD Example (Go) ===

--- Creating goal template ---
Created template: Data Migration (id: O2R5wfu1cPCpj3ZGaO08vwNoBqwYzAlWnaIre0V8-eqxkoTJRDH8Q09XzxQCZJaYeH3J3XaBqD_irsLhpYYKOg)

--- Listing templates ---
Templates: map[count:1 items:[map[description:map[type:String value:Template for migrating data between schemas] id:O2R5wfu1cPCpj3ZGaO08vwNoBqwYzAlWnaIre0V8-eqxkoTJRDH8Q09XzxQCZJaYeH3J3XaBqD_irsLhpYYKOg steps:map[type:Array value:[map[description:Analyze source schema] map[description:Create target schema] map[description:Migrate records] map[description:Validate results]]] title:map[type:String value:Data Migration]]]]

--- Getting template ---
Fetched: Data Migration

--- Updating template ---
Updated description: Updated: comprehensive data migration workflow

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
=== ekoDB Goals, Tasks & Agents Integration Example (Go) ===

--- Creating goal ---
Created goal: Deploy v2.0 (id: 5d_OVx3uNrDI5onvqbCxIqoH2nEgAGhsUp44wI1nE4M1HPk2kKjI8zvBoRFjdBeMFNvtobun0-06GQ6Ysi-X_A)

--- Listing goals ---
Goals response: map[count:1 goals:[map[created_at:2026-10-08T05:47:39.951713+00:00 description:Ship the v2.0 release to production id:5d_OVx3uNrDI5onvqbCxIqoH2nEgAGhsUp44wI1nE4M1HPk2kKjI8zvBoRFjdBeMFNvtobun0-06GQ6Ysi-X_A status:pending steps:[{"description":"Run integration tests"},{"description":"Build release artifacts"},{"description":"Deploy to staging"}] title:Deploy v2.0 updated_at:2026-10-08T05:47:39.951713+00:00]]]

--- Getting goal ---
Fetched goal: Deploy v2.0

--- Updating goal ---
Updated description: Ship the v2.0 release to production (with hotfix)

--- Searching goals ---
Search result: map[count:1 items:[map[_score:12.870000000000001 created_at:map[type:DateTime value:2026-10-08T05:47:39.951713+00:00] description:map[type:String value:Ship the v2.0 release to production (with hotfix)] id:5d_OVx3uNrDI5onvqbCxIqoH2nEgAGhsUp44wI1nE4M1HPk2kKjI8zvBoRFjdBeMFNvtobun0-06GQ6Ysi-X_A status:map[type:String value:pending] steps:map[type:String value:[{"description":"Run integration tests"},{"description":"Build release artifacts"},{"description":"Deploy to staging"}]] title:map[type:String value:Deploy v2.0] updated_at:map[type:DateTime value:2026-10-08T05:47:39.964591+00:00]]]]

--- Goal step lifecycle ---
Step 0 started
Step 0 completed
Step 1 started
Step 1 failed

--- Completing and approving goal ---
Goal status after complete: pending_review
Goal status after approve: in_progress

--- Testing GoalReject ---
Goal status after reject: failed

--- Creating task ---
Created task: Run benchmarks (id: z8c9Xn9gF2JuofE8MzhGLxP1ULfawGy7wohIjIRRTLd5UjEoH2lpOsongce8aKQv3SH1ypTIwJE9ysSTQo1VCA)

--- Listing tasks ---
Tasks: map[count:1 items:[map[description:map[type:String value:Execute YCSB benchmarks against staging] due_at:map[type:DateTime value:2026-10-09T05:47:40+00:00] id:z8c9Xn9gF2JuofE8MzhGLxP1ULfawGy7wohIjIRRTLd5UjEoH2lpOsongce8aKQv3SH1ypTIwJE9ysSTQo1VCA name:map[type:String value:Run benchmarks]]]]

--- Getting task ---
Fetched task: Run benchmarks

--- Task lifecycle: start -> succeed ---
Task status: running
Task status: active

--- Task lifecycle: pause -> resume -> fail ---
Task status after pause: paused
Task status after resume: active
Task failed successfully

--- Due tasks ---
Due tasks: map[count:0 items:[]]

--- Deleting tasks ---
Tasks deleted

--- Creating agent ---
Created agent: benchmark-runner (id: cs3SN1y2XHBqw9AM8ACqr5JbAPYv0H7hoP1tTz67Y8uzTdKEUUnmvjvRfsBFJOIEYzTRSnfXLPscj5tFHFscrg)

--- Listing agents ---
Agents: map[count:1 items:[map[deployment_id:map[type:String value:deploy_prod_1] description:map[type:String value:Runs periodic benchmarks] id:cs3SN1y2XHBqw9AM8ACqr5JbAPYv0H7hoP1tTz67Y8uzTdKEUUnmvjvRfsBFJOIEYzTRSnfXLPscj5tFHFscrg llm_model:map[type:String value:gpt-4.1] name:map[type:String value:benchmark-runner]]]]

--- Getting agent by ID ---
Fetched agent: benchmark-runner

--- Getting agent by name ---
Agent by name: benchmark-runner

--- Updating agent ---
Updated agent description: Runs hourly YCSB benchmarks

--- Agents by deployment ---
Agents in deploy_prod_1: map[count:0 items:[]]

--- Deleting agent ---
Agent deleted

--- Deleting goal ---
Goal deleted

=== All goals, tasks & agents operations completed successfully ===
=== Join Operations Examples ===

Setting up sample data...
✅ Sample data created

1. Single collection join (users with departments):
Found 2 users with department data
  - Bob Smith: Sales
  - Alice Johnson: Engineering

2. Join with filtering:
Found 1 users in Engineering
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data
  - Bob Smith: Sales Manager
  - Alice Johnson: Senior Software Engineer

4. Join orders with user data:
Found 2 completed orders
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
✓ Client created
✓ go_users_register saved
✓ go_users_login saved
✓ go_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/go_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/go_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/go_users_verify_token { "token": "<jwt>" }

✓ Cleaned up demo functions
=== ekoDB KV Links Integration Example (Go) ===

--- Inserting documents ---
Inserted doc1: ZDJEafpUfXCVIJoFlIRm9hhaE5tWpjpMINF829fcvzHsYC7gMQxgXAv2RMUOAeXmOQX-fHfCFbVPQeyl5PwOqg
Inserted doc2: Aqo5GbfIbLJHGZDaqAAc3OEblXi7OZY5reaWOvd3CjkEhU9US5PK-l5y9L6YAWy2PHsSHdQNCs3uWEkcNq6ABA

--- Setting KV entry ---
KV entry set: team:engineering:go

--- Linking KV key to documents ---
Linked doc1: map[]
Linked doc2: map[]

--- Getting links ---
Links for team:engineering:go: [map[collection:kv_links_example_go created_at:2026-10-08T05:47:41.990531Z document_id:Aqo5GbfIbLJHGZDaqAAc3OEblXi7OZY5reaWOvd3CjkEhU9US5PK-l5y9L6YAWy2PHsSHdQNCs3uWEkcNq6ABA field_path:<nil> last_accessed:2026-10-08T05:47:41.991649Z metadata:map[]] map[collection:kv_links_example_go created_at:2026-10-08T05:47:41.989059Z document_id:ZDJEafpUfXCVIJoFlIRm9hhaE5tWpjpMINF829fcvzHsYC7gMQxgXAv2RMUOAeXmOQX-fHfCFbVPQeyl5PwOqg field_path:<nil> last_accessed:2026-10-08T05:47:41.991649Z metadata:map[]]]

--- Unlinking doc2 ---
Unlink result: map[]

--- Verifying links after unlink ---
Links after unlink: [map[collection:kv_links_example_go created_at:2026-10-08T05:47:41.989059Z document_id:ZDJEafpUfXCVIJoFlIRm9hhaE5tWpjpMINF829fcvzHsYC7gMQxgXAv2RMUOAeXmOQX-fHfCFbVPQeyl5PwOqg field_path:<nil> last_accessed:2026-10-08T05:47:41.993693Z metadata:map[]]]

--- Cleanup ---
Cleaned up all resources

=== All KV link operations completed successfully ===
✓ Client created

=== KV Set ===
✓ Set key: session:user123:go

=== KV Get ===
Retrieved value: map[type:Object value:map[userId:123 username:john_doe]]

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1:go: success
  cache:product:2:go: success
  cache:product:3:go: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1:go: map[name:Product 1 price:29.99]
  cache:product:2:go: map[name:Product 2 price:39.99]
  cache:product:3:go: map[name:Product 3 price:49.99]

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*:go'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123:go
✓ Verified: Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  cache:product:1:go: deleted
  cache:product:2:go: deleted
  cache:product:3:go: deleted

✓ All KV operations completed successfully
=== KV Precision: Float vs Decimal ===

=== Test 1: Using Go Floats (LOSES PRECISION) ===
✓ Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 2: Using FieldDecimal() (PRESERVES PRECISION) ===
✓ Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.29999999999999999 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Summary ===
✅ Use FieldDecimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ FieldDecimal() stores values as strings internally,
   preserving exact precision across all operations.

=== Cleanup ===
✓ Cleaned up test keys
✓ Client created
✓ go_route_admin saved
✓ go_route_user_by_id saved
✓ go_route_user_posts saved
✓ go_route_org_create_member saved

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions
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
=== ekoDB Raw Completion Stream (SSE) Example ===

--- Basic SSE Raw Completion ---
Response: The three primary colors are red, blue, and yellow.

--- Structured Output via SSE ---
JSON response: [
  {
    "name": "Earth",
    "diameter_km": 12742
  },
  {
    "name": "Jupiter",
    "diameter_km": 139820
  },
  {
    "name": "Mars",
    "diameter_km": 6779
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a great day.

=== Done ===
=== ekoDB Schedules Integration Example (Go) ===

--- Creating schedule ---
Created schedule: nightly-backup (id: 05b2ea64-e92b-404b-9d76-f801a6b68477)

--- Listing schedules ---
Schedules: map[count:1 schedules:[map[created_at:2026-10-08T05:47:47.319236Z cron_expression:0 0 0 * * * description:Runs a database backup every night at midnight enabled:true function_label:schedule_noop_go_74638 id:05b2ea64-e92b-404b-9d76-f801a6b68477 last_execution:<nil> name:nightly-backup next_execution:2026-10-09T00:00:00Z parameters:map[] stats:map[avg_execution_time_ms:0 failed_executions:0 last_error:<nil> successful_executions:0 total_executions:0] timezone:UTC updated_at:2026-10-08T05:47:47.319236Z]]]

--- Getting schedule ---
Fetched schedule: nightly-backup (cron: 0 0 0 * * *)

--- Updating schedule ---
Updated cron: 0 0 2 * * *

--- Triggering schedule ---
Trigger response: map[schedule_id:05b2ea64-e92b-404b-9d76-f801a6b68477 status:triggered]

--- Pausing schedule ---
Schedule enabled: false

--- Resuming schedule ---
Schedule enabled: true

--- Deleting schedule ---
Schedule deleted

=== All schedule operations completed successfully ===
=== Schema Management Examples ===

1. Creating user schema with basic fields:
✅ User schema created

2. Creating product schema with text index:
✅ Product schema with indexes created

3. Creating document schema with vector index:
✅ Document schema with vector index created

4. Retrieving collection schema:
Schema fields: 4 fields
Schema version: 1

5. Retrieving collection metadata:
Collection has 4 fields

6. Creating employee schema with all constraint types:
✅ Employee schema with all constraints created

✅ Schema management examples completed!
=== Search Examples ===

Setting up test data...
✅ Test data created

1. Basic full-text search:
Found 2 results
  1. Score: 12.870
  2. Score: 6.270

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200
  2. Score: 13.200
  3. Score: 13.200
  4. Score: 13.200

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400
  2. Score: 26.400
  3. Score: 26.400
  4. Score: 26.400

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600
  2. Score: 6.600

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 6.600

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.767
  2. Score: 0.764
  3. Score: 0.760

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.507
  2. Score: 0.906
  3. Score: 0.304

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Introduction to Machine Learning (category: ml)
  2. Deep Learning Fundamentals (category: ml)


✅ Search examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: map[id:tNtd0kp8OE6WvZQZ0fzL4q1-QPLSWS34gkEVjFP9hnCOP2i0Y0a8BE5W9pFKuZuxPq5mxdpkogB3v4k6ldpu4g]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-10-08T05:47:49+00:00] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:tNtd0kp8OE6WvZQZ0fzL4q1-QPLSWS34gkEVjFP9hnCOP2i0Y0a8BE5W9pFKuZuxPq5mxdpkogB3v4k6ldpu4g metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-10-08 05:47:49 +0000 +0000
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-10-08T05:47:49+00:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:tNtd0kp8OE6WvZQZ0fzL4q1-QPLSWS34gkEVjFP9hnCOP2i0Y0a8BE5W9pFKuZuxPq5mxdpkogB3v4k6ldpu4g metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-10-08T05:47:49+00:00] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:tNtd0kp8OE6WvZQZ0fzL4q1-QPLSWS34gkEVjFP9hnCOP2i0Y0a8BE5W9pFKuZuxPq5mxdpkogB3v4k6ldpu4g metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: DD1OK1MCVlp7b3-1FnX9S3teCx7LWew03DBiULHumvK6YY9nsS3lr4k1VLW2AGTHonnxGVuX8UJnKeC5zvq7OQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
🚀 ekoDB Go Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native_go (5BXfj1UsaPAMa7s0o2b-2d_EuMYBcDJ8nwHL3iuzvGK5wA2QfkPC54lgMbwk4gSZaUeBPcljXM0F4ekoAEnzfQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 138ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 11ms
  Speedup: 12.5x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_go (RBRP0IazdnAe3VJXqJKLoLacJchA0y-XmC9ZPZd4rxEWCgj9Ys7ZHaRZeUJhyPrckdOC_9-125I50T-9Ssewog)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_go' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_go (CiaVQptbmhuw2cm63pwdfr_fOrBDRVVGuyoPDmfAqQErdzjRiJBURhjzorjE1OD6VvEBaM0P5tk1DADIuJyNgQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_go' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_go (qwNfOWAgpLYRuc08-t4NrrYV7tPcwk5EEp2zK9nfuDkogrC4aFXgPiHx0q9lCL2wjyo9VUIVOFtEHjeXYlcFVg)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

================================================================================
✅ Key Benefits of Native SWR:
✅ Single function: Replaces 4-function cache-aside pattern
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)
Native SWR:     SWR → Insert (2 functions)
Result:         60% fewer functions, cleaner code, same behavior 🎯

🧹 Cleaning up...
✓ Deleted 4 test scripts

✅ All examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_go (ulx3zQKS9YzVC2lh0zElTgXLKXCsc0ExS6R4fZ6z1eaaQ8l7--Ae_zJP4yQtt4BGAC0I4JWtHkZe6BgMBrCyWA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": {
        "type": "DateTime",
        "value": "2026-10-08T05:47:51+00:00"
      },
      "data": {
        "type": "Object",
        "value": {
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "zipcode": "92998-3874"
          },
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "email": "Sincere@april.biz",
          "id": 1,
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "website": "hildegard.org"
        }
      },
      "id": "1"
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 11ms (served from cache)
✓ Lightning fast cache hit

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🧹 Cleaning up...
✓ Cleanup complete

✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 3q7aAWtSiAc4CV5qG6MOJdkmNFWJl8dj4NdgGootqvgillcxIgrxkz4_cbn-W5RnjrOR2TgM94nX-LvbRTBfSg
Created Bob: $500 - ID: w1Jxhrfogb0zS29mc0o5IIh7lp3LrbNmlxuqsKMUK9rOxWafGqApomJUfUpCrDfAoyDTHUwDEKcNNzV4ucNjQQ

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): a1f5bfc9-4b8c-4fa6-88c5-170efef534b1

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

✓ Verified committed balances: Alice=$800, Bob=$700

=== Example 5: Rollback Demo ===
New transaction: 7d17f41e-acf4-4463-8305-94f5dde90dc2
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: LkfYvkal_ZzfPppCd1nMl_U2vyk5NAD83-u9a0zAmvtUpEldjoD1WV8casayC7ff1-NgHDQgX6xEWbR5Te1tMA

=== Get User Function ===
Retrieved: get_active_users_go - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 1 user functions:
  - get_active_users_go: Get Active Users

=== List User Functions by Tag ===
Found 1 user functions with 'users' tag:
  - get_active_users_go

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (Go) ===

Created chat session: JL8QjUu3Yldzd4qorz05ZPGdHI9oujkR8d5LOp14_lROFh89uUcPBdg4oBQ3BliqYmYOuIvPU1X32k28sa8Vxw

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: HNgTVqONETkkcsVfbxCrVYX3YU-qeTB9rBjIF04GPyPseZd0ETdgV1K-eKpOzT1mvz6BzSkGTZASd-phCfppKQ
Execution time: 686ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example (Go) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_go' ===
✓ Subscribed (subscription_id: sub_bb04575f2add44cb9474f0b5675e0074)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: UHhse2YSc0B7F0dlcFUhTQPsq05K9KSXZO-GE92tqi0j6HC3YI_L4jtNumAIwh8PpfEIqTam0Pmk2-esXznxnQ

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_go
     Record IDs: [UHhse2YSc0B7F0dlcFUhTQPsq05K9KSXZO-GE92tqi0j6HC3YI_L4jtNumAIwh8PpfEIqTam0Pmk2-esXznxnQ]
     Timestamp:  2026-10-08T05:47:54.492204+00:00

Inserting record 2...
✓ Inserted: ibbrEPmoeL-stZ_pPk9I-6PqHmF5CQyGdNpuGg5dxg6QOZFx88qdjTy6AD1KzJo90ntsv190yxT0E9FVPx1kIQ

  📡 Notification received:
     Event:      insert
     Record IDs: [ibbrEPmoeL-stZ_pPk9I-6PqHmF5CQyGdNpuGg5dxg6QOZFx88qdjTy6AD1KzJo90ntsv190yxT0E9FVPx1kIQ]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: 2IHkYP3VIYeaPhazEONnS9TRv6AZ0ivMBrxxpzLoyud8bjzCnJ_C_y_rqaAySxo92ETwnR6OvA4-yC9y8z9ePg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
Inserted with ripple: map[id:SH4aCv3j4GQYGRLZzrZfKlksj8oMf_0-f3WI_xBytu7bXZlqpveGkeVLT8pAvf9STwTW0I45BYtsng47I_gZHg]
Inserted with bypass_ripple: map[id:PenfClmVMUSgJd1G2PG7nsqPySCxLv5LLmSHv_TNlo6_YRgbtHFQNpq4n3zk9ZMzCtijc2oOgS2qy5gxafKg_A]
Inserted with TTL and bypass_ripple: map[id:QFdWBwvUR9B37Nwp4ouGKCMMmnb2sSzK2hZlCvFZJW2LlwMxeEv0HhL0PmDbw8ilFUbuqQ9wnXeYzDnNitVlLQ]
Updated with bypass_ripple: map[id:SH4aCv3j4GQYGRLZzrZfKlksj8oMf_0-f3WI_xBytu7bXZlqpveGkeVLT8pAvf9STwTW0I45BYtsng47I_gZHg name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:custom-id]
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: [email id name]
  First user: Bob Smith <bob@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: [age avatar_url bio created_at email id name status user_role]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - Dave Brown (age 45)
    - Alice Johnson (age 30)
    - Bob Smith (age 25)

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - Carol White: Manager

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: [age api_key avatar_url bio created_at email id name password secret_token status user_role]
  Projected query:
    - 3 fields per record
    - Fields: [email id name]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ All Go integration tests complete!
📦 Building TypeScript client library...

> @ekodb/ekodb-client@0.27.0 prepare
> npm run build


> @ekodb/ekodb-client@0.27.0 build
> tsc


up to date, audited 44 packages in 993ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.27.0 build
> tsc

✅ TypeScript client built!

added 1 package, removed 1 package, and audited 13 packages in 389ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
=== ekoDB Advanced CRUD Example (TypeScript) ===

--- Inserting base record ---
Inserted: NFl83suLfaKGbK3RKTg5jHaNPG74hOfUwE9EpB20memb2olIqZUUQsHuduty-F-0QtdQSmiZ5ynlX9DV9jSzIw

--- updateWithAction: increment ---
Score after increment: 150

--- updateWithAction: push ---
Tags after push: ["beginner","pro"]

--- updateWithAction: clear ---
temp_data after clear: null

--- updateWithActionSequence: multiple atomic actions ---
After sequence - score: 160, lives: 2, tags: ["beginner","pro","veteran"]

--- Cleanup ---
Deleted collection

=== All advanced CRUD operations completed ===
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
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 7_gKcQ4EJMvE39N9w3E2YOlzNFE8d6DiSpMADpxzdVEkFgkMSfNtRK_IZK9udiaSl_MukhAzdoxtc_UWtZfsZg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or have other questions, feel free to ask!

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
✓ Created second session: AaVxj4u5JE1ZUObToWxSCupny9qQ9avEZzl9vnVLQJ2RzU9XnyIQ5QZTXDumaBNtOl4aKDpZ4Gn-sRXGZhkDQw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: AaVxj4u5JE1ZUObToWxSCupny9qQ9avEZzl9vnVLQJ2RzU9XnyIQ5QZTXDumaBNtOl4aKDpZ4Gn-sRXGZhkDQw
✓ Deleted chat session: 7_gKcQ4EJMvE39N9w3E2YOlzNFE8d6DiSpMADpxzdVEkFgkMSfNtRK_IZK9udiaSl_MukhAzdoxtc_UWtZfsZg
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: UzdiDwJTzFPobT6dyFoqzjk3JrglBx9MsmRMUiT0MMCCaeYBfBwLux7ecuLlobgNRq6z192_6lsC8xyj_XxHqQ

=== Sending Chat Message ===
Message ID: Xi__genEL2tIqfiA_27APLwJSJyellhm1Nor8zzx0Z9LaDg_I80XaS0Sf5jG7hYQnbA2VeNM7P6gIf39HB3lzw

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities.

2. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features.

3. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    id: '9JqNyu2ybyTcxmi2hRdrsPAeMXJTsaUti5yr2nP06QCbqrVEqrskuGbcGDrJ3x13PCqswIttrOOAo-UP4g4NaQ',
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'WEJug2LXsn-ayd35HE8mMsjPqFIVHZIMa4RWxOlSQNn_V3_E518LqwbGPoLBowEoy52poBG0H-bkpRHLWttCiQ',
    price: 299,
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud',
    price: 499,
    id: 'oxy95jXdM-QSomKza0yoxYCfL7UGjMkyfuUA_DbtjwEjrVUw-3e9MitqditC9xSZnFY4r16EwoSfQHfE_0g2kw'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 7067ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 91
Total tokens: 3504

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (TypeScript) ===

Created session: j1BBc1B-D79SHw60VcOtpHGqh-vz9ydSdgWF_vTwHHs4xxFx9eqg4548cNgW9Civ9Q9OAgEEyyJpwEgljDMCHQ

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}**ekoDB** is a curation-based database that focuses on the **ecological and evolutionary information of prokaryotic taxa** (mainly bacteria and archaea). The name "ekoDB" comes from "ecology" and "database."

### Key Features of ekoDB:
- **Curation-Based:** Information in ekoDB is hand-curated from scientific literature, ensuring high quality and reliability, rather than relying solely on automated computational predictions.
- **Taxonomy:** Organizes data according to prokaryotic taxonomy.
- **Ecological/Evolutionary Data:** Contains detailed information on lifestyles (e.g., symbiosis, pathogenesis, extremophily), habitats (e.g., marine, soil, host-associated), and evolutionary traits of prokaryotes.
- **User-Friendly Search:** Allows users to search, browse, and analyze ecological and evolutionary attributes of microbes at different taxonomic levels (species, genus, etc.).
- **Integration:** Complements genomic databases by providing context on ecological functions and evolutionary history.

### Applications:
- Understanding the adaptation and diversity of prokaryotes.
- Comparing the ecology of microbial taxa.
- Assisting in metagenomic studies and microbial ecology research.

### Reference:
For example, see the original publication:
- Yamashita, S., et al. (2023) "**ekoDB: a curation-based database of ecological and evolutionary information of prokaryotic taxa**." *Nucleic Acids Research*, 51(D1), D808–D815. [doi:10.1093/nar/gkac979](https://academic.oup.com/nar/article/51/D1/D808/6849227)

**In summary:**
ekoDB is a specialized, literature-curated database providing ecological and evolutionary information about prokaryotic organisms, designed to help researchers explore how these microbes live, adapt, and interact with their environments.

--- Stream complete ---
Message ID: iX0D_TTqMwn7hsTxdlTFBpiEXoC769kSXBVuac_fV-OtmKbbSF05151CJX_w9f8HkcJgW5QJcwyWmNqfaBWzsg
Execution time: 3383ms
Context window: 1000000 tokens

✓ Chat message stream example completed
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
    - o3-2025-04-16
    - gpt-5.1-chat-latest
    - gpt-5.3-chat-latest
    - gpt-5.2-2025-12-11
    - sora-2
    - chatgpt-image-latest
    - gpt-4.1-mini
    - gpt-3.5-turbo
    - gpt-4o-mini
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-6-luna
    - gpt-5.4-mini-2026-03-17
    - gpt-4-turbo
    - gpt-4.1-nano
    - gpt-image-2
    - gpt-5.4-pro
    - gpt-realtime-mini-2025-12-15
    - gpt-3.5-turbo-16k
    - babbage-002
    - text-embedding-ada-002
    - gpt-4o-mini-tts
    - gpt-4o-2024-08-06
    - o4-mini-2025-04-16
    - gpt-4o-2024-11-20
    - omni-moderation-latest
    - gpt-realtime
    - tts-1-hd
    - gpt-realtime-1.5
    - gpt-realtime-2
    - gpt-5-search-api
    - gpt-5.5-2026-04-23
    - gpt-5.1
    - gpt-5.1-codex
    - gpt-4o
    - o1-2024-12-17
    - gpt-5.2-chat-latest
    - gpt-4o-mini-tts-2025-12-15
    - gpt-image-1
    - davinci-002
    - gpt-image-2-2026-04-21
    - gpt-4o-mini-transcribe
    - o3-mini
    - gpt-audio
    - gpt-5.4-nano
    - o3
    - gpt-5.1-codex-max
    - gpt-realtime-2.1
    - gpt-4.1
    - gpt-5.5
    - gpt-4o-mini-search-preview-2025-03-11
    - text-embedding-3-large
    - gpt-4o-mini-search-preview
    - gpt-4o-2024-05-13
    - gpt-6-sol
    - gpt-5.3-codex
    - gpt-3.5-turbo-instruct
    - sora-2-pro
    - gpt-5
    - gpt-audio-mini-2025-12-15
    - gpt-transcribe
    - o4-mini-deep-research-2025-06-26
    - o1-pro-2025-03-19
    - gpt-4.1-nano-2025-04-14
    - gpt-5-search-api-2025-10-14
    - gpt-4-turbo-2024-04-09
    - gpt-realtime-2.1-mini
    - tts-1
    - gpt-5-mini
    - omni-moderation-2024-09-26
    - gpt-5-mini-2025-08-07
    - gpt-realtime-2025-08-28
    - gpt-5.4-2026-03-05
    - gpt-live-transcribe
    - gpt-3.5-turbo-0125
    - gpt-5-nano
    - gpt-5.6-luna
    - gpt-4
    - gpt-image-2.5-sunburst-2026-09-08
    - whisper-1
    - gpt-5.4-mini
    - gpt-5-pro
    - o1-pro
    - gpt-realtime-whisper
    - gpt-5.2-pro
    - gpt-5-chat-latest
    - gpt-live-1
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-5-2025-08-07
    - tts-1-1106
    - gpt-5.5-pro
    - gpt-5-pro-2025-10-06
    - chat-latest
    - gpt-5.2-pro-2025-12-11
    - gpt-4o-mini-transcribe-2025-03-20
    - gpt-audio-2025-08-28
    - gpt-5.4-pro-2026-03-05
    - gpt-realtime-translate
    - gpt-5.6-sol
    - gpt-audio-mini-2025-10-06
    - gpt-5.5-pro-2026-04-23
    - tts-1-hd-1106
    - gpt-realtime-mini
    - gpt-5.4
    - gpt-4o-transcribe-diarize
    - gpt-5-codex
    - gpt-image-1-mini
    - gpt-image-1.5
    - gpt-5.1-2025-11-13
    - o4-mini-deep-research
    - gpt-4o-search-preview-2025-03-11
    - o3-mini-2025-01-31
    - gpt-4-0613
    - gpt-4o-search-preview
    - gpt-5.6-terra
    - text-embedding-3-small
    - gpt-audio-1.5
    - gpt-3.5-turbo-instruct-0914
    - gpt-6-astra
    - gpt-5.2-codex
    - gpt-3.5-turbo-1106
    - gpt-5-nano-2025-08-07
    - gpt-audio-mini
    - gpt-4o-mini-tts-2025-03-20
    - gpt-5.2
    - gpt-4.1-mini-2025-04-14
    - gpt-4.1-2025-04-14
    - gpt-4o-mini-2024-07-18
    - o4-mini
    - o1
    - gpt-5.4-nano-2026-03-17
    - gpt-5.1-codex-mini
    - gpt-4o-transcribe
    - gpt-image-2.5-flare
    - gpt-6.1-sol
  anthropic:
    - claude-haiku-5-5
    - claude-sonnet-5-5
    - claude-opus-5-5
    - claude-fable-5-1
    - claude-opus-5
    - claude-sonnet-5
    - claude-fable-5
    - claude-opus-4-8
    - claude-opus-4-7
    - claude-sonnet-4-6
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
Provider status:
  anthropic: ok 14 models
  gemini: not_configured (unverified) No Gemini API Key
  openai: ok 135 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Specific Provider Models ===
OpenAI models: o3-2025-04-16, gpt-5.1-chat-latest, gpt-5.3-chat-latest, gpt-5.2-2025-12-11, sora-2, chatgpt-image-latest, gpt-4.1-mini, gpt-3.5-turbo, gpt-4o-mini, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-6-luna, gpt-5.4-mini-2026-03-17, gpt-4-turbo, gpt-4.1-nano, gpt-image-2, gpt-5.4-pro, gpt-realtime-mini-2025-12-15, gpt-3.5-turbo-16k, babbage-002, text-embedding-ada-002, gpt-4o-mini-tts, gpt-4o-2024-08-06, o4-mini-2025-04-16, gpt-4o-2024-11-20, omni-moderation-latest, gpt-realtime, tts-1-hd, gpt-realtime-1.5, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, gpt-5.1, gpt-5.1-codex, gpt-4o, o1-2024-12-17, gpt-5.2-chat-latest, gpt-4o-mini-tts-2025-12-15, gpt-image-1, davinci-002, gpt-image-2-2026-04-21, gpt-4o-mini-transcribe, o3-mini, gpt-audio, gpt-5.4-nano, o3, gpt-5.1-codex-max, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-4o-mini-search-preview-2025-03-11, text-embedding-3-large, gpt-4o-mini-search-preview, gpt-4o-2024-05-13, gpt-6-sol, gpt-5.3-codex, gpt-3.5-turbo-instruct, sora-2-pro, gpt-5, gpt-audio-mini-2025-12-15, gpt-transcribe, o4-mini-deep-research-2025-06-26, o1-pro-2025-03-19, gpt-4.1-nano-2025-04-14, gpt-5-search-api-2025-10-14, gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, tts-1, gpt-5-mini, omni-moderation-2024-09-26, gpt-5-mini-2025-08-07, gpt-realtime-2025-08-28, gpt-5.4-2026-03-05, gpt-live-transcribe, gpt-3.5-turbo-0125, gpt-5-nano, gpt-5.6-luna, gpt-4, gpt-image-2.5-sunburst-2026-09-08, whisper-1, gpt-5.4-mini, gpt-5-pro, o1-pro, gpt-realtime-whisper, gpt-5.2-pro, gpt-5-chat-latest, gpt-live-1, gpt-4o-mini-transcribe-2025-12-15, gpt-5-2025-08-07, tts-1-1106, gpt-5.5-pro, gpt-5-pro-2025-10-06, chat-latest, gpt-5.2-pro-2025-12-11, gpt-4o-mini-transcribe-2025-03-20, gpt-audio-2025-08-28, gpt-5.4-pro-2026-03-05, gpt-realtime-translate, gpt-5.6-sol, gpt-audio-mini-2025-10-06, gpt-5.5-pro-2026-04-23, tts-1-hd-1106, gpt-realtime-mini, gpt-5.4, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-image-1-mini, gpt-image-1.5, gpt-5.1-2025-11-13, o4-mini-deep-research, gpt-4o-search-preview-2025-03-11, o3-mini-2025-01-31, gpt-4-0613, gpt-4o-search-preview, gpt-5.6-terra, text-embedding-3-small, gpt-audio-1.5, gpt-3.5-turbo-instruct-0914, gpt-6-astra, gpt-5.2-codex, gpt-3.5-turbo-1106, gpt-5-nano-2025-08-07, gpt-audio-mini, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-4.1-mini-2025-04-14, gpt-4.1-2025-04-14, gpt-4o-mini-2024-07-18, o4-mini, o1, gpt-5.4-nano-2026-03-17, gpt-5.1-codex-mini, gpt-4o-transcribe, gpt-image-2.5-flare, gpt-6.1-sol

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: JAW-F2jLawSx5tSDvxVir2hfDVXx9QvELK4MbNumt8rp2O6OGDggXkCgPqzSQpozBZKm1YJkQrQXsuBEFDwd7A

=== Sending Messages ===
✓ Message 1 sent
  Response: It seems that there are no records available in the specified collection. If you're looking for products, particularly the ekoDB product, I can share the details I have:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need further information or specific queries related to available products, feel free to ask!

✓ Message 2 sent
  Response: It seems that I do not have access to the 'products' collection directly. However, based on the context you provided, the price of the product "ekoDB" is **$99**. If you need more information or details on a specific aspect, let me know!

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 3KJqn5p1rf2_gr5ttkzd3MKnJgdSNXn3RA5SUTQer-uuHpvNFJe_CIuU5Gx6lPFiSyqujJRJD7AZhsOTurvvLw
  Parent: JAW-F2jLawSx5tSDvxVir2hfDVXx9QvELK4MbNumt8rp2O6OGDggXkCgPqzSQpozBZKm1YJkQrQXsuBEFDwd7A

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: 3KJqn5p1rf2_gr5ttkzd3MKnJgdSNXn3RA5SUTQer-uuHpvNFJe_CIuU5Gx6lPFiSyqujJRJD7AZhsOTurvvLw (Untitled)
  Session 2: JAW-F2jLawSx5tSDvxVir2hfDVXx9QvELK4MbNumt8rp2O6OGDggXkCgPqzSQpozBZKm1YJkQrQXsuBEFDwd7A (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 3KJqn5p1rf2_gr5ttkzd3MKnJgdSNXn3RA5SUTQer-uuHpvNFJe_CIuU5Gx6lPFiSyqujJRJD7AZhsOTurvvLw

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: RHCX__fDZ6av-aVrKtln80ouevSq7ogm5a_jbcbuipnA3ofjzjvwd-uWZGz2-1ne2qVYjFokpB57nXmpTGkxww

=== List Collections ===
Total collections: 17
Sample collections: chat_agent_configs__ek0_testing,chat_goal_templates__ek0_testing,schema_documents_client_go,audit__ek0_testing,chat_tasks__ek0_testing

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test_ts' exists: false

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test_ts' exists: true

=== Count Documents ===
Document count in 'collection_utils_test_ts': 5

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_ts'

✓ Collection Utilities example complete
✓ Client created
✓ conc_demo_pay saved
✓ conc_demo_rl_fail saved
✓ conc_demo_rl_skip saved
✓ conc_demo_lock saved

Invoke them like:
  POST /api/functions/conc_demo_pay_ts_75441_1791438514383 { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail_ts_75441_1791438514383 { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip_ts_75441_1791438514383 { "user_id": 42 }
  POST /api/functions/conc_demo_lock_ts_75441_1791438514383 { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'h-2Gr1hZtFXlXVAKKWURSd4Bg72FLwSj5nuGE3G_NFLYB1Y_cVlElNRuz1fgsXn6w_EEgufJkPnAvOrb49D0lA'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: 'h-2Gr1hZtFXlXVAKKWURSd4Bg72FLwSj5nuGE3G_NFLYB1Y_cVlElNRuz1fgsXn6w_EEgufJkPnAvOrb49D0lA',
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  age: { type: 'Integer', value: 29 },
  active: { value: true, type: 'Boolean' },
  name: { value: 'Alice Johnson', type: 'String' }
}
✓ Second upsert (insert): { id: 'new-user-id' }

=== Find One Operation ===
✓ Found user by email: {
  age: { value: 29, type: 'Integer' },
  name: { type: 'String', value: 'Alice Johnson' },
  active: { type: 'Boolean', value: true },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  id: 'h-2Gr1hZtFXlXVAKKWURSd4Bg72FLwSj5nuGE3G_NFLYB1Y_cVlElNRuz1fgsXn6w_EEgufJkPnAvOrb49D0lA'
}
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: true
✓ Fake record exists: false (should be false)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 7 records (expected ~7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!
✓ Client created
✓ crypto_demo_hmac_ts saved
✓ crypto_demo_aes_ts saved
✓ crypto_demo_uuid_ts saved
✓ crypto_demo_totp_ts saved
✓ crypto_demo_encoding_ts saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac_ts { "payload": "hi" }
  POST /api/functions/crypto_demo_aes_ts { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid_ts
  POST /api/functions/crypto_demo_totp_ts
  POST /api/functions/crypto_demo_encoding_ts { "title": "Héllo World" }

✓ Cleaned up demo functions
=== Distinct Values Example ===

Inserting sample products...
Inserted 8 products

=== Distinct Categories (all products) ===
Found 3 distinct categories:
  - books
  - clothing
  - electronics

=== Distinct Statuses (all products) ===
Found 3 distinct statuses:
  - active
  - archived
  - discontinued

=== Distinct Statuses in Electronics ===
Found 2 distinct statuses for electronics:
  - active
  - discontinued

Cleanup done.
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: BFjgXMfP8NeAlQAxeXSMb4V7Hwx2kWzRcYaEH3f7LckoPzqaR46vjwsdCck6FFTcyJIw8_jinNqP4ecx9CZw7Q

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 4w-wiliM1NzPuTApGgrbERoyv__L1Nhevx-l1_G9njVud5ua1NkrbiYEThU2mDDYZo8Qzp2phAyUwl8onVgIng

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
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: 7l4POmPBPmoi1fq8S8bMmliIbul7uj773aTJwS7Eh6JFoJ0XRANPlsxyfb9ISkNgXTjDMx1r2ElEySQ-aRihpw

Call 1: Cache miss (fetches from API)
Response time: 62ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "email": "Sincere@april.biz",
          "website": "hildegard.org",
          "name": "Leanne Graham",
          "address": {
            "suite": "Apt. 556",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "city": "Gwenborough",
            "zipcode": "92998-3874",
            "street": "Kulas Light"
          },
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "id": 1
        }
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 3ms (20.7x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "email": "Sincere@april.biz",
          "website": "hildegard.org",
          "name": "Leanne Graham",
          "address": {
            "suite": "Apt. 556",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "city": "Gwenborough",
            "zipcode": "92998-3874",
            "street": "Kulas Light"
          },
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "id": 1
        }
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!

=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 51ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "id": 1,
      "username": "Bret",
      "phone": "1-770-736-8031 x56442",
      "email": "Sincere@april.biz",
      "name": "Leanne Graham",
...

Second call (cache hit - from cache):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "id": 1,
      "username": "Bret",
      "phone": "1-770-736-8031 x56442",
      "email": "Sincere@april.biz",
      "name": "Leanne Graham",
      "website": "hild...
   🚀 Cache speedup: 17.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ All composition examples completed!
client_function_contract: ok
🚀 ekoDB Functions Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: xf0UaBXfXf6V2mViBVRorXRoaJmsN7RvkLSbqt8sThT8VwQZt9KZFCWNuCkBocJ8sxLYzq8NMlFjdzYpcXYhTQ
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: GJ0RT5LIs6t2bPS-Da0uRa_UpeD1us3OH-RPuKzXiglgEGJC-9bRu7Zel73N0XrwFRmZdhVzc-mve_kGB_0e4A
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: 6fnvUpg-D7VATB4veNqq99IVAU4PYP6QFETPaLxou2rZNaFxtN55Zy6O2GrGUnKkL_vwEYpbA5Im44w1IqQGqA
📊 Statistics: 2 groups
   {"status":{"type":"String","value":"active"},"avg_score":{"value":60,"type":"Float"},"count":{"type":"Integer","value":5}}
   {"avg_score":{"type":"Float","value":50},"status":{"type":"String","value":"inactive"},"count":{"value":5,"type":"Integer"}}

📝 Example 4: UserFunction Management

📋 Total scripts: 5
🔍 Retrieved script: Get Active Users
✏️  function updated
🗑️  function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
🚀 ekoDB TypeScript Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   {"category":{"value":"Electronics","type":"String"},"count":{"type":"Integer","value":5},"avg_price":{"value":367,"type":"Float"}}
   {"category":{"type":"String","value":"Furniture"},"avg_price":{"type":"Float","value":365.6666666666667},"count":{"type":"Integer","value":3}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB TypeScript AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits, including:

1. **Efficient Similarity Search**: They excel at finding similar items based on vector embeddings, ideal for applications in recommendation systems and image retrieval.

2. **High Dimensionality Support**: Designed to manage high-dimensional data effectively, which is common in machine learning and AI applications.

3. **Scalability**: Many vector databases are built to scale horizontally, handling large datasets with ease.

4. **Real-time Processing**: They often support fast, real-time queries, enabling immediate responses for applications like chatbots and search engines.

5. **Integration with Machine Learning**: Seamlessly integrates with ML workflows, making it easier to store and query embeddings generated by models.

6. **Flexibility**: Supports various data types, including text, images, and videos, which can be represented as vectors.

7. **Advanced Indexing Techniques**: Uses methods like Approximate Nearest Neighbor (ANN) to speed up searches without compromising much on accuracy.

8. **Enhanced Analytics**: Facilitates powerful analytics on unstructured data, providing deeper insights from the stored vectors.

These features make vector databases increasingly popular in fields like AI, natural language processing, and computer vision.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
📊 Generated 2 embeddings
   Dimensions: 1536
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text
🚀 ekoDB TypeScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: 1akDP9MpqfGjK88EmHBCagued158TXjAvtIXJhxVoKzHT3Z0dgS-SgaCaK7L7ShR2sv5oPFLXxOnVcVxSrDCEw
📊 Found 2 product groups
   {"category":{"type":"String","value":"Electronics"},"count":{"type":"Integer","value":3},"avg_price":{"value":575.6666666666666,"type":"Float"}}
   {"avg_price":{"value":474,"type":"Float"},"count":{"value":2,"type":"Integer"},"category":{"value":"Furniture","type":"String"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Electronics"}}
   {"category":{"value":"Furniture","type":"String"},"count":{"value":2,"type":"Integer"}}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: UserFunction with Parameter Definition

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
🚀 ekoDB TypeScript CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average score by role:
   {"role":{"type":"String","value":"admin"},"count":{"type":"Integer","value":3},"avg_score":{"value":20,"type":"Float"}}
   {"role":{"value":"user","type":"String"},"count":{"value":7,"type":"Integer"},"avg_score":{"type":"Float","value":70}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB TypeScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: WPSDQoWq1gYHn0gTfBIKXGIW_-AM4ciOaXpEKapMW3MWZRELllv7oHs0UzbcrVpemQDMnTDGVUOSJcRzYEKY5A
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: Qxgq39XFnRMJ3sx_08V_A627xfOh9ZWIn4VZgBUuy7s4l8hegpGlsye1wf9k_rvuYpUTLZ-1zYN3QPwxDzpZDQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"role":"admin","userId":"user_abc"}}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: O9mz0SaXPGFhawaQKgHAipSjI7GqlEw5hgXY2C7zuKub6xtT09-nvJjC4vBaeJFIH1THjKuiKdC1jkYo6Vy4dg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: 6NAOaFrKTz205hit3_cslSIQg1VRClQETUFzfRUXgiLVMTQdYSjYbhnhQcleLTw9KrwAFCbjRWH1WYe4vNO2XQ
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use Field.* helpers for type-safe wrapped values
   ✅ Field.decimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ Stage.kv*() functions work within scripts
   ✅ Combine KV caching with collection inserts for real workflows
🚀 ekoDB TypeScript Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Database Design Principles (Database)
   2. Introduction to Machine Learning (AI)
   3. Vector Databases Explained (Database)
   4. Natural Language Processing (AI)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"type":"String","value":"Database"},"count":{"type":"Integer","value":3}}
   {"count":{"value":2,"type":"Integer"},"category":{"value":"AI","type":"String"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Goal Template CRUD Example (TypeScript) ===

--- Creating goal template ---
Created template: Data Migration (id: qVG4M9AhrqKFYRm-Lzk-sxs2NjKoGbcsHa4dB7sxmAxYZD1EUAeEl0npEIyFAqxq4v9X4DqBOMWLTT8VYGNVyg)

--- Listing templates ---
Templates: {
  count: 1,
  items: [
    {
      description: [Object],
      id: 'qVG4M9AhrqKFYRm-Lzk-sxs2NjKoGbcsHa4dB7sxmAxYZD1EUAeEl0npEIyFAqxq4v9X4DqBOMWLTT8VYGNVyg',
      steps: [Object],
      title: [Object]
    }
  ]
}

--- Getting template ---
Fetched: Data Migration

--- Updating template ---
Updated description: Updated: comprehensive data migration workflow

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
=== ekoDB Goals, Tasks & Agents Example (TypeScript) ===

--- Creating goal ---
Created goal: Deploy v2.0 (id: 6NZwEUPBVu3tOi5ndbqheBQl-aoIu63a5CXELnMscPGLtamV2mBYiOtPifIM7PGaX6jJhPXud08VoA4HXMLB6g)

--- Listing goals ---
Goals: {
  "count": 1,
  "goals": [
    {
      "created_at": "2026-10-08T05:48:47.801030+00:00",
      "description": "Ship version 2.0 to production",
      "id": "6NZwEUPBVu3tOi5ndbqheBQl-aoIu63a5CXELnMscPGLtamV2mBYiOtPifIM7PGaX6jJhPXud08VoA4HXMLB6g",
      "status": "pending",
      "steps": "[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]",
      "title": "Deploy v2.0",
      "updated_at": "2026-10-08T05:48:47.801030+00:00"
    }
  ]
}

--- Getting goal ---
Fetched: Deploy v2.0

--- Updating goal ---
Updated description: Ship version 2.0 with hot-fix patches

--- Searching goals ---
Search results: {
  "count": 1,
  "items": [
    {
      "_score": 12.870000000000001,
      "created_at": {
        "type": "DateTime",
        "value": "2026-10-08T05:48:47.801030+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 with hot-fix patches"
      },
      "id": "6NZwEUPBVu3tOi5ndbqheBQl-aoIu63a5CXELnMscPGLtamV2mBYiOtPifIM7PGaX6jJhPXud08VoA4HXMLB6g",
      "status": {
        "type": "String",
        "value": "pending"
      },
      "steps": {
        "type": "String",
        "value": "[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"
      },
      "title": {
        "type": "String",
        "value": "Deploy v2.0"
      },
      "updated_at": {
        "type": "DateTime",
        "value": "2026-10-08T05:48:47.813287+00:00"
      }
    }
  ]
}

--- Goal step: start step 0 ---
Step 0 started on goal 6NZwEUPBVu3tOi5ndbqheBQl-aoIu63a5CXELnMscPGLtamV2mBYiOtPifIM7PGaX6jJhPXud08VoA4HXMLB6g
--- Goal step: complete step 0 ---
Step 0 completed on goal 6NZwEUPBVu3tOi5ndbqheBQl-aoIu63a5CXELnMscPGLtamV2mBYiOtPifIM7PGaX6jJhPXud08VoA4HXMLB6g
--- Goal step: fail step 1 ---
Step 1 failed on goal 6NZwEUPBVu3tOi5ndbqheBQl-aoIu63a5CXELnMscPGLtamV2mBYiOtPifIM7PGaX6jJhPXud08VoA4HXMLB6g

--- Completing goal ---
Goal status: pending_review
--- Approving goal ---
Goal status after approve: in_progress

--- Creating goal to reject ---
--- Rejecting goal ---
Goal status after reject: failed


--- Creating task ---
Created task: Hourly Health Check (id: KoU9ltKZ4geVKvjwrIslaBWPz4HwInnkc2Xkx6yzbZx4a15L96OMA85Ge6Bi8QskMem9qGMvBhYCzQsziKjkBg)

--- Listing tasks ---
Tasks: {
  "count": 1,
  "items": [
    {
      "action": {
        "type": "String",
        "value": "health_check"
      },
      "config": {
        "type": "Object",
        "value": {
          "endpoint": "/health"
        }
      },
      "cron": {
        "type": "String",
        "value": "0 * * * *"
      },
      "id": "KoU9ltKZ4geVKvjwrIslaBWPz4HwInnkc2Xkx6yzbZx4a15L96OMA85Ge6Bi8QskMem9qGMvBhYCzQsziKjkBg",
      "name": {
        "type": "String",
        "value": "Hourly Health Check"
      }
    }
  ]
}

--- Getting task ---
Fetched: Hourly Health Check

--- Starting task ---
Task status: running

--- Succeeding task ---
Task status after succeed: active

--- Pausing task ---
Task status after pause: paused

--- Resuming task ---
Task status after resume: active

--- Failing task ---
Task status after fail: active

--- Getting due tasks ---
Due tasks: {
  "count": 0,
  "items": []
}

--- Deleting task ---
Task deleted successfully


--- Creating agent ---
Created agent: SupportBot (id: 3PBXiosS0p0QzHYhu-u3-RfIKsZxKJH5r2P3xqN3QOQvJ7qTYrMy0LEO9e7vcxnw5XCJmCeCjFQjCg3qCGkCzg)

--- Listing agents ---
Agents: {
  "count": 1,
  "items": [
    {
      "deployment_id": {
        "type": "String",
        "value": "deploy_prod_1"
      },
      "id": "3PBXiosS0p0QzHYhu-u3-RfIKsZxKJH5r2P3xqN3QOQvJ7qTYrMy0LEO9e7vcxnw5XCJmCeCjFQjCg3qCGkCzg",
      "llm_model": {
        "type": "String",
        "value": "gpt-4"
      },
      "name": {
        "type": "String",
        "value": "SupportBot"
      },
      "system_prompt": {
        "type": "String",
        "value": "You are a helpful customer support agent."
      }
    }
  ]
}

--- Getting agent by ID ---
Fetched: SupportBot

--- Getting agent by name ---
By name: SupportBot (id: 3PBXiosS0p0QzHYhu-u3-RfIKsZxKJH5r2P3xqN3QOQvJ7qTYrMy0LEO9e7vcxnw5XCJmCeCjFQjCg3qCGkCzg)

--- Updating agent ---
Updated agent: SupportBot

--- Getting agents by deployment ---
Agents in deployment: {
  "count": 0,
  "items": []
}
WARNING: agents-by-deployment omitted created agent 3PBXiosS0p0QzHYhu-u3-RfIKsZxKJH5r2P3xqN3QOQvJ7qTYrMy0LEO9e7vcxnw5XCJmCeCjFQjCg3qCGkCzg; TODO: check/fix the server-side deployment lookup

--- Deleting agent ---
Agent deleted successfully

--- Cleanup: deleting goals ---
Goals deleted successfully

=== All goals, tasks & agents operations completed ===
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


✅ Join operations examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created
✓ ts_users_register saved
✓ ts_users_login saved
✓ ts_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/jwt_register_ts_76029_1791438528902 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_login_ts_76029_1791438528902 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_verify_ts_76029_1791438528902 { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

✓ Cleaned up demo functions
=== ekoDB KV Document Linking Example (TypeScript) ===

--- Setup: creating KV key and documents ---
Set KV key: session:admin
Inserted document 1: MdsmsdmjJOeLx6YB74ku0jlUBr_FsKr_zH0v2St5gamiyjigMufqO0sFUqAsaEoDwMDBbjWftXWbZQnF250_FA
Inserted document 2: JqKMHbSSj_XYjmjT6ZClnYDXabGFKbUhLJNskIbpMU-WKayN7AIZOJPoeM3D2cmHchb6kbIn56RJBOGZlxqIHw

--- Linking documents to KV key ---
Linked doc MdsmsdmjJOeLx6YB74ku0jlUBr_FsKr_zH0v2St5gamiyjigMufqO0sFUqAsaEoDwMDBbjWftXWbZQnF250_FA: null
Linked doc JqKMHbSSj_XYjmjT6ZClnYDXabGFKbUhLJNskIbpMU-WKayN7AIZOJPoeM3D2cmHchb6kbIn56RJBOGZlxqIHw: null

--- Getting links for KV key ---
Links: [
  {
    "collection": "kv_links_example_ts_76060_1791438529259",
    "document_id": "MdsmsdmjJOeLx6YB74ku0jlUBr_FsKr_zH0v2St5gamiyjigMufqO0sFUqAsaEoDwMDBbjWftXWbZQnF250_FA",
    "field_path": null,
    "created_at": "2026-10-08T05:48:49.337006Z",
    "last_accessed": "2026-10-08T05:48:49.340013Z",
    "metadata": {}
  },
  {
    "collection": "kv_links_example_ts_76060_1791438529259",
    "document_id": "JqKMHbSSj_XYjmjT6ZClnYDXabGFKbUhLJNskIbpMU-WKayN7AIZOJPoeM3D2cmHchb6kbIn56RJBOGZlxqIHw",
    "field_path": null,
    "created_at": "2026-10-08T05:48:49.338656Z",
    "last_accessed": "2026-10-08T05:48:49.340013Z",
    "metadata": {}
  }
]

--- Unlinking document ---
Unlinked doc JqKMHbSSj_XYjmjT6ZClnYDXabGFKbUhLJNskIbpMU-WKayN7AIZOJPoeM3D2cmHchb6kbIn56RJBOGZlxqIHw: null

--- Verifying remaining links ---
Remaining links: [
  {
    "collection": "kv_links_example_ts_76060_1791438529259",
    "document_id": "MdsmsdmjJOeLx6YB74ku0jlUBr_FsKr_zH0v2St5gamiyjigMufqO0sFUqAsaEoDwMDBbjWftXWbZQnF250_FA",
    "field_path": null,
    "created_at": "2026-10-08T05:48:49.337006Z",
    "last_accessed": "2026-10-08T05:48:49.342408Z",
    "metadata": {}
  }
]

=== All KV linking operations completed ===

--- Cleanup ---
Cleanup complete
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { type: 'Object', value: { userId: 123, username: 'john_doe' } }

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_ops_ts_76091_1791438529710:cache:product:1: success
  kv_ops_ts_76091_1791438529710:cache:product:2: success
  kv_ops_ts_76091_1791438529710:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_ts_76091_1791438529710:cache:product:1: { price: 29.99, name: 'Product 1' }
  kv_ops_ts_76091_1791438529710:cache:product:2: { price: 39.99, name: 'Product 2' }
  kv_ops_ts_76091_1791438529710:cache:product:3: { price: 49.99, name: 'Product 3' }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  kv_ops_ts_76091_1791438529710:cache:product:1: deleted
  kv_ops_ts_76091_1791438529710:cache:product:2: deleted
  kv_ops_ts_76091_1791438529710:cache:product:3: deleted

✓ All KV operations completed successfully
=== KV Precision: Float vs Decimal ===

=== Test 1: Using JavaScript Floats (LOSES PRECISION) ===
✓ Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 2: Using Field.decimal() (PRESERVES PRECISION) ===
✓ Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.30000000000000004 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Cleanup ===

=== Summary ===
✅ Use Field.decimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ Field.decimal() stores values as strings internally,
   preserving exact precision across all operations.

=== Cleanup ===
✓ Cleaned up test keys
✓ Client created
✓ ts_route_admin → GET /api/route/users/admin
✓ ts_route_user_by_id → GET /api/route/users/:id
✓ ts_route_user_posts → GET /api/route/users/:id/posts/:post_id
✓ ts_route_org_create_member → POST /api/route/orgs/:org/members

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions
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
=== ekoDB Raw Completion Stream (SSE) Example ===

--- Basic SSE Raw Completion ---
Response: The three primary colors are red, blue, and yellow.

--- Structured Output via SSE ---
JSON response: [
  {
    "name": "Mercury",
    "diameter_km": 4879
  },
  {
    "name": "Venus",
    "diameter_km": 12104
  },
  {
    "name": "Mars",
    "diameter_km": 6779
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===
=== ekoDB Schedule Management Example (TypeScript) ===

--- Creating schedule ---
Created schedule: Nightly Database Backup (id: c90d9636-f802-4c93-98de-0564e63f620d, cron: 0 0 2 * * *)

--- Listing schedules ---
Schedules: {
  "count": 1,
  "schedules": [
    {
      "created_at": "2026-10-08T05:48:53.617089Z",
      "cron_expression": "0 0 2 * * *",
      "description": null,
      "enabled": true,
      "function_label": "schedule_noop_typescript_76251_1791438533586",
      "id": "c90d9636-f802-4c93-98de-0564e63f620d",
      "last_execution": null,
      "name": "Nightly Database Backup",
      "next_execution": "2026-10-09T02:00:00Z",
      "parameters": {},
      "stats": {
        "avg_execution_time_ms": 0,
        "failed_executions": 0,
        "last_error": null,
        "successful_executions": 0,
        "total_executions": 0
      },
      "timezone": "UTC",
      "updated_at": "2026-10-08T05:48:53.617089Z"
    }
  ]
}

--- Getting schedule ---
Fetched: Nightly Database Backup (cron: 0 0 2 * * *)

--- Updating schedule ---
Updated: Nightly Full Backup (new cron: 0 0 3 * * *)

--- Triggering schedule ---
Trigger response: {
  "schedule_id": "c90d9636-f802-4c93-98de-0564e63f620d",
  "status": "triggered"
}

--- Pausing schedule ---
Schedule enabled after pause: false

--- Resuming schedule ---
Schedule enabled after resume: true

--- Deleting schedule ---
Schedule deleted successfully

=== All schedule operations completed ===
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
=== Search Examples ===

Setting up test data...
✅ Test data created

1. Basic full-text search:
Found 2 results
  1. Score: 12.870, Matched: email, name
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: title, bio
  2. Score: 13.200, Matched: bio, title
  3. Score: 13.200, Matched: bio, title
  4. Score: 13.200, Matched: bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: title, bio
  2. Score: 26.400, Matched: title, bio
  3. Score: 26.400, Matched: title, bio
  4. Score: 26.400, Matched: bio, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 6.600, Matched: bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.774, Matched:
  2. Score: 0.766, Matched:
  3. Score: 0.753, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.501, Matched: content, title
  2. Score: 0.910, Matched: content, title
  3. Score: 0.306, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: skills, bio

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Deep Learning Fundamentals (category: ml)
  2. Introduction to Machine Learning (category: ml)


✅ Search examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'Wc8R7r5Yor_LWwKY_mDqaG-pKxGcJ3MzACoDvYcmGUt93Ca65YSOTzjXqfG0Os_fmILVQilfLyyStX13V4P-iA'
}

=== Find by ID ===
Found: {
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  name: { value: 'Test Record', type: 'String' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  value: { value: 42, type: 'Integer' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  created_at: { value: '2026-10-08T05:48:55.023+00:00', type: 'DateTime' },
  id: 'Wc8R7r5Yor_LWwKY_mDqaG-pKxGcJ3MzACoDvYcmGUt93Ca65YSOTzjXqfG0Os_fmILVQilfLyyStX13V4P-iA',
  active: { value: true, type: 'Boolean' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  price: { value: 99.99, type: 'Float' },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-10-08T05:48:55.023Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  data: 'aGVsbG8gd29ybGQ=',
  categories: [ 'electronics', 'computers' ],
  name: 'Test Record',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  value: 42,
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  created_at: '2026-10-08T05:48:55.023+00:00',
  id: 'Wc8R7r5Yor_LWwKY_mDqaG-pKxGcJ3MzACoDvYcmGUt93Ca65YSOTzjXqfG0Os_fmILVQilfLyyStX13V4P-iA',
  active: true,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  price: 99.99,
  metadata: { nested: { deep: true }, key: 'value' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  value: { type: 'Integer', value: 100 },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  active: { type: 'Boolean', value: true },
  created_at: { type: 'DateTime', value: '2026-10-08T05:48:55.023+00:00' },
  name: { value: 'Updated Record', type: 'String' },
  id: 'Wc8R7r5Yor_LWwKY_mDqaG-pKxGcJ3MzACoDvYcmGUt93Ca65YSOTzjXqfG0Os_fmILVQilfLyyStX13V4P-iA',
  price: { value: 99.99, type: 'Float' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: wsIioEfq10moQKqUEBeWx7Eck7k-D2VvwwWcZUTwaMWC5euLFM6ICEvA779LdCqiLmajUUz77O-RAp8wHhB5HA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
🚀 ekoDB TypeScript Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


🧹 Cleaning up...
✓ Deleted 0 test scripts and owned SWR resources

Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native_ts (UVpLhFfG2EyIqAE4O5bkWXgqyQHI_oLlo2waUvvXO-MyNOZSAqZAfmrEiMKOirpn-Gk5Qvw7dEcehygQvyPNTg)

First call (cache miss - will fetch from GitHub API):
  Response time: 127ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 42.3x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_ts (XIoUkrUGfGI83f1b_zmf_1YCYr9lnhZlFd9-Tvb6vgtoYaYzeUlnoeteRgRsaNCQlu5eNecFWqacIpgk2D1SpA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_ts' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_ts (43uNMAYI3V5tBFpXUMuevGj3hzijpcYTT883DKjhQ9NIQoXb48QKyxtoEh9ysUCs_dV6AaFknRDlxLgIQUmQtQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_ts' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_ts (9PxuhIbWw5-na5bvAwrUA7QUCZwKTMsU44i4NQR0r70rr8S8hTtKIut3iUzJgXP_CmFP-KZO2X6ndnAOx6isbA)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

================================================================================
✅ Key Benefits of Native SWR:
✅ Single function: Replaces 4-function cache-aside pattern
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)
Native SWR:     SWR → Insert (2 functions)
Result:         60% fewer functions, cleaner code, same behavior 🎯

🧹 Cleaning up...
✓ Deleted 4 test scripts and owned SWR resources

✅ All examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_ts_76477_1791438537065 (CxJKN8F0SB0RPl7T0nCFIUGTeZyRtltVrGv_m04NymSSEEi3YN9N3uogMmwlBZLj0c_ryynp0lXI6qdaYRhL4w)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "id": 1,
          "address": {
            "suite": "Apt. 556",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "city": "Gwenborough"
          },
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "username": "Bret",
          "website": "hildegard.org",
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "email": "Sincere@april.biz"
        },
        "type": "Object"
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 2ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "id": 1,
          "address": {
            "suite": "Apt. 556",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "city": "Gwenborough"
          },
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "username": "Bret",
          "website": "hildegard.org",
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "email": "Sincere@april.biz"
        }
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_reviews_ts_76477_1791438537065 (rk5GDFT28L2p5BE2Yos-QB9nN67DK3Ln_kb9FXJcyjCa2bxHjBLjrd57LnWMqf2ifjcMZOs6SSqzfohfpJAj6g)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "value": {
          "category": "beauty",
          "tags": [
            "beauty",
            "mascara"
          ],
          "title": "Essence Mascara Lash Princess",
          "meta": {
            "createdAt": "2025-10-09T14:47:01.588Z",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "updatedAt": "2026-05-23T11:27:41.868Z",
            "barcode": "5784719087687"
          },
          "weight": 4,
          "dimensions": {
            "depth": 22.99,
            "height": 13.08,
            "width": 15.14
          },
          "reviews": [
            {
              "reviewerName": "Eleanor Collins",
              "comment": "Would not recommend!",
              "rating": 3,
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com"
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Lucas Gordon",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "comment": "Very satisfied!",
              "rating": 4
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Highly impressed!",
              "reviewerName": "Eleanor Collins",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "rating": 5
            }
          ],
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "returnPolicy": "No return policy",
          "availabilityStatus": "In Stock",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "discountPercentage": 10.48,
          "price": 9.99,
          "stock": 99,
          "minimumOrderQuantity": 48,
          "shippingInformation": "Ships in 3-5 business days",
          "id": 1,
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "warrantyInformation": "1 week warranty",
          "sku": "BEA-ESS-ESS-001",
          "brand": "Essence",
          "rating": 2.56
        },
        "type": "Object"
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Multi-API data fetched, merged, and cached atomically

=== Why This Is Powerful ===

✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===

1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!

✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: CCMgisAy_j9j3iZedIyV109w8xRnfakny5UjHoRPisbVu_T3uHRfdFZei3ZmOldsDrWIn0H5Qv4iE7UqZh7JoA
Created Bob: $500 - ID: lXqzZgkkj7ugIU_dENFgxZU_4xJnFiWyfcq9ZC2LJMcNhQGXXRPSVNRIYmdnbyhFZGONy8vgp4l443ltQo_EdQ

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 7ea5ab15-2501-461f-aa33-582a85ab6df9

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

✓ Verified committed balances: Alice=$800, Bob=$700

=== Example 5: Rollback Demo ===
New transaction: 565ddf6d-54a8-497c-9339-b300e86e7604
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: Hqwvj0sN8FnFYbL5nUJwuMeHsgbKXaEXUwIYVZadzRzzqQK4R31dl2uAVHEFW4nAl1HgpopV3FWLbpA8EpycXg

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 5 user functions:
  - get_active_users_ts: Get Active Users
  - conc_demo_rl_skip_ts_75441_1791438514383: Rate-limit (skip mode)
  - get_active_users_client_ts_updated: Get Active Users (Updated)
  - conc_demo_rl_fail_ts_75441_1791438514383: Rate-limit (fail mode)
  - fetch_product_reviews_ts_76477_1791438537065: Fetch Product with Reviews (Multi-API)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (TypeScript) ===

Created chat session: cTvWGdHW7zTTDlgRYhtKFP2RCzeL5jDX-Px4lQkGH3gZap2i25CHXq8CJP70T_yiN3qfYCghJfLIOo6vCF1Lhw

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: NzBL1_N7X4UYLXG6pZR5zULrExy5SToleGKncYH5JuZ-oDGKFZXRx6j7igG-8k2JbnibjSYb2wX7101RHsRrrg
Execution time: 728ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_f748bc11001647679bb922131a6b138d)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: njACYSVJNMjJ1GU5n38G5pCi0h9tQ_QiI0UcbPjnVnK001OEjf1TqKSav__8BNz5Rr_yDcNny_S0_JER6OIyOw
  📡 Notification received for njACYSVJNMjJ1GU5n38G5pCi0h9tQ_QiI0UcbPjnVnK001OEjf1TqKSav__8BNz5Rr_yDcNny_S0_JER6OIyOw

Inserting another record...
✓ Inserted record: _jO_6kr_IEVsifZChPeDb0OZMPzzwmjUeoAsmTClIH6mTwxyp-9KODK1zzW1GxUpToQhXpp9N-Re4wQvpLridg
  📡 Notification received for _jO_6kr_IEVsifZChPeDb0OZMPzzwmjUeoAsmTClIH6mTwxyp-9KODK1zzW1GxUpToQhXpp9N-Re4wQvpLridg

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_ts'
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: WH-Biw-b1HegR-WCGclp6MxhhoXjhS__IMRahZXw9DzG0GJYWrMzG_YVfcR9uR9djPlv6PrT78e8bhGru4tCqA

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: {"id":"seKtKBoGv8GUebyqVrf3dtVdEyWsXpUuRNcxRXAQQKTlQsxDRPKldeXNJ6iG6d-eAmrL4WeL5YJVS95vJWy7RA"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"X0kEpALmDUiqckMaZe8eOwa_nzG32qNN23U8VA6hRzDXcHuW512FkGTfKHhruurtX2s3cvQZcarBUUvBlXVXPg"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"name":{"type":"String","value":"Product 1"},"price":{"type":"Integer","value":150},"id":"seKtKBoGv8GUebyqVrf3dtVdEyWsXpUuRNcxRXAQQKTlQsxDRPKldeXNJ6iG6d-eAmrL4WeL5YJVS95vJWy7RA"}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"custom-id"}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["id","name","email"]
  First user: Alice Johnson <alice@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["created_at","bio","name","avatar_url","status","id","user_role","age","email"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - Dave Brown (age 45)
    - Alice Johnson (age 30)
    - Bob Smith (age 25)

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - Carol White: Manager

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["created_at","status","email","api_key","name","avatar_url","secret_token","id","password","user_role","bio","age"]
  Projected query:
    - 3 fields per record
    - Fields: ["name","email","id"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ All TypeScript integration tests complete!
🧪 Running JavaScript examples (direct HTTP/WebSocket)...

added 1 package, removed 1 package, and audited 9 packages in 378ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities

╔════════════════════════════════════════╗
║  ekoDB JavaScript Examples Test Suite ║
╚════════════════════════════════════════╝

=== Checking Server Connection ===
✓ Server is ready

=== Getting Authentication Token ===
✓ Authentication successful

=== Running 10 Examples ===

=== Running ekoDB/ekodb-client/examples/javascript/simple_crud.js ===
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {
  id: 'QERp5AlG6u9CuuhIsktFgPhhhKHdoBWwPypqPqdOfLPFVndPWLNEI2cJjS-LsSv6ywAZLm6cBeycAuElQZVPsw'
}

=== Find by ID ===
Found: {
  value: { value: 42, type: 'Integer' },
  id: 'QERp5AlG6u9CuuhIsktFgPhhhKHdoBWwPypqPqdOfLPFVndPWLNEI2cJjS-LsSv6ywAZLm6cBeycAuElQZVPsw',
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Test Record' }
}

=== Find with Query ===
Found documents: [
  {
    active: { type: 'Boolean', value: true },
    name: { type: 'String', value: 'Test Record' },
    value: { value: 42, type: 'Integer' },
    id: 'QERp5AlG6u9CuuhIsktFgPhhhKHdoBWwPypqPqdOfLPFVndPWLNEI2cJjS-LsSv6ywAZLm6cBeycAuElQZVPsw'
  }
]

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  id: 'QERp5AlG6u9CuuhIsktFgPhhhKHdoBWwPypqPqdOfLPFVndPWLNEI2cJjS-LsSv6ywAZLm6cBeycAuElQZVPsw',
  value: { value: 100, type: 'Integer' },
  active: { type: 'Boolean', value: true }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/simple_websocket.js ===
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: u_c_S5-uQmyrr76QIMoc4DEI1tBV4sjXXUw_LERNJTrb6gege-7k2T_lVb0rYXZcqgi3cUeu5voECrLrwoX0qw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "value": {
          "type": "Integer",
          "value": 42
        },
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        },
        "id": "u_c_S5-uQmyrr76QIMoc4DEI1tBV4sjXXUw_LERNJTrb6gege-7k2T_lVb0rYXZcqgi3cUeu5voECrLrwoX0qw"
      }
    ]
  },
  "messageId": "1791438543805"
}
✓ Retrieved 1 record via WebSocket

✓ WebSocket example completed successfully
WebSocket closed
✓ simple_websocket.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/http_functions.js ===
🚀 ekoDB Functions Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: rj_UgGBfIiYiYdQflnyMweyvuM65qqwUsQgth9rdJk-CD3Cee9i7GnPcBE4UieVi1IYgmY5PKhcq7NoU20Nx-Q
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: enJlxb-RrP6-YzHdbC6S9rDf6Qyg-tX2kHLbtUiXptyl3o5uN8Q-d2Hwq999ETEnymYELaMefU-4iXJRq1GMqA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: Qe3JzxvKOaO15coozrxrFXONzyfJUycO9TatGkRN3KcKIRHuCjxI8xVNBOIhUU1kzQISfcz2f9Jz5F82630rVQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":{"type":"Float","value":50},"max_score":{"value":90,"type":"Integer"},"status":{"value":"inactive","type":"String"},"count":{"type":"Integer","value":5}}
   {"count":{"value":5,"type":"Integer"},"status":{"value":"active","type":"String"},"avg_score":{"value":60,"type":"Float"},"max_score":{"value":100,"type":"Integer"}}

📝 Example 4: Function Management

📋 Total functions: 7
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
✓ http_functions.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/batch_operations.js ===
=== Batch Operations (Direct HTTP) ===

✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 5 total records in collection

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records
✓ Verified: Record updated with status="active"

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
✓ batch_operations.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/kv_operations.js ===
=== Key-Value Operations (Direct HTTP) ===

✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { type: 'Object', value: { username: 'john_doe', userId: 123 } }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { value: { price: 29.99, name: 'Product 1' }, type: 'Object' }
cache:product:2: {
  value: { name: 'Product 2', price: 39.989999999999995 },
  type: 'Object'
}
cache:product:3: {
  value: { name: 'Product 3', price: 49.989999999999995 },
  type: 'Object'
}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ kv_operations.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/collection_management.js ===
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: njVQi5O4co6BaTsvRXBc-IXzqniuRd4BWSK9oqHWtQncFfoIQOqGZR4LHqkysi3mBQ-EZyWHZDCGxgqf_0OhCA

=== List Collections ===
Total collections: 22
Sample collections: [
  'demo_collection',
  'chat_agent_configs__ek0_testing',
  'chat_goal_templates__ek0_testing',
  'schema_documents_client_go',
  'audit__ek0_testing'
]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ collection_management.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/transactions.js ===
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: u6butrdzZ6q7trQ4Y6kOOYNQFfHt5VIdZCSNQ0Lm0DnrGofWJUlF6lqUe-PwECtWnoU1kNeAvkWcX_cUiTYfZQ
Created Bob: $500 - ID: gvAOKQapJut8H10olHavnzoJPgKl3H8ssMQy64RqdDsC5UWonoyjb25ZZnyGnydBPGIwdRyrG76PWi8Jr0ZvFQ

=== Example 1: Begin Transaction ===
Transaction ID: fbcd82b7-a768-4fd7-af99-3f5ddfadb4e5

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: {"value":800,"type":"Integer"}
Bob: {"type":"Integer","value":700}

=== Example 5: Rollback ===
New transaction: 4b4debf1-973a-44c4-a04d-adea5fa4aa15
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"value":700,"type":"Integer"}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✓ transactions.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/crud_functions.js ===
🚀 ekoDB Complete CRUD Functions Example
============================================================
Demonstrates:
  • Insert + Verify (using Query)
  • Query + Update Status + Verify
  • Query + Update Credits + Verify
  • Query Before Delete + Delete + Verify Gone

Each function shows Functions chaining with proper verification
============================================================

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted collection: crud_functions_users_js

============================================================
📝 Function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: jxqteduOlx_75iRXfbD03iaIQB18_A3LX0udmh_dl6mOHgYooncgxWinhJkmSLotBwlKkdRU5Qxq6GFqnu0dag

2️⃣ Calling Function (Insert + Verify)...
   ✅ Function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Name: Alice Smith
   📋 Email: alice@example.com
   📋 Status: pending
   📋 Credits: 0

============================================================
📝 Function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 8Jl8gR0U7v6suR7rOkTgma5Wp_b7DDf8SCMMhUKHcRgPXotg0Ise0wA26-2WdT0jIZNKhDzo2Em3Kt9JWuQkdg

2️⃣ Calling Function (Query + Update + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"type":"String","value":"active"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: grsJVHUlT6iKrAFpEZDFfxHwwvW6ZjqDSZSEdwMHO-hbupedNwQChSCpCPb8v_7GgkfN41PVXfQjhZOJf9nzBw

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":100}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: HErQ0NMPIgUvSwf61MAWRVYnNQQlc4pW0fMlnHCW9wpzS1jeeLLd8CEIseN00zp5fl10UTsmzanC2PnKt9KybQ

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: jxqteduOlx_75iRXfbD0...
   ✅ Deleted function: 8Jl8gR0U7v6suR7rOkTg...
   ✅ Deleted function: grsJVHUlT6iKrAFpEZDF...
   ✅ Deleted function: HErQ0NMPIgUvSwf61MAW...
   ✅ Deleted collection: crud_functions_users_js

============================================================
✅ Complete CRUD Functions Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Functions chain Functions together
   ✅ Each function demonstrates operation + verification
   ✅ Parameters make functions reusable
   ✅ Verification is built into the function itself
   ✅ Complete CRUD lifecycle in 4 focused functions
✓ crud_functions.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/document_ttl.js ===
╔════════════════════════════════════════════════════════╗
║     TTL EXPIRATION VERIFICATION TEST                   ║
╚════════════════════════════════════════════════════════╝

This test verifies that document TTL expiration works correctly.
We will insert documents with short TTL and verify they expire.

✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = nAhefpGch-k-y46NeHbiR7bpHmUftq7JbCCna0Wjv9C1lQziL3OFCGefIcVXckL45CoTKe8tsk4LltPRhAffmw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(nAhefpGch-k-y46NeHbiR7bpHmUftq7JbCCna0Wjv9C1lQziL3OFCGefIcVXckL45CoTKe8tsk4LltPRhAffmw)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(nAhefpGch-k-y46NeHbiR7bpHmUftq7JbCCna0Wjv9C1lQziL3OFCGefIcVXckL45CoTKe8tsk4LltPRhAffmw)
  Output: Error (expected) - Request failed with status 404: {"error":"Record not found (expired)"}
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
CLEANUP
═══════════════════════════════════════════════════════════
✓ Deleted test collection

╔════════════════════════════════════════════════════════╗
║              ALL TTL TESTS PASSED ✓                    ║
╚════════════════════════════════════════════════════════╝

TTL expiration is working correctly:
  • Documents with TTL expire after the specified time
  • Documents without TTL persist indefinitely
  • Different TTL durations are handled correctly
✓ document_ttl.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/websocket_ttl.js ===
╔════════════════════════════════════════════════════════╗
║   WEBSOCKET TTL EXPIRATION VERIFICATION TEST           ║
╚════════════════════════════════════════════════════════╝

This test verifies TTL expiration works via WebSocket connections.
We will use WebSocket to insert, query, and verify TTL expiration.

✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = rd9Vo3i-pIhHLFlY5m7VOSfkdwbBtJejVqAuGzQO4TMC3J4K0f8sLB9QZil09o1p9QuDObF7fAIP_Em28XPgKg
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(rd9Vo3i-pIhHLFlY5m7VOSfkdwbBtJejVqAuGzQO4TMC3J4K0f8sLB9QZil09o1p9QuDObF7fAIP_Em28XPgKg)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(rd9Vo3i-pIhHLFlY5m7VOSfkdwbBtJejVqAuGzQO4TMC3J4K0f8sLB9QZil09o1p9QuDObF7fAIP_Em28XPgKg)
  Output: Error (expected) - Request failed with status 404: {"error":"Record not found (expired)"}
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
CLEANUP
═══════════════════════════════════════════════════════════
✓ Deleted test collection

╔════════════════════════════════════════════════════════╗
║          WEBSOCKET TTL TEST PASSED ✓                   ║
╚════════════════════════════════════════════════════════╝

WebSocket TTL expiration is working correctly:
  • Documents with TTL inserted via client expire correctly
  • Queries correctly return null for expired documents
✓ websocket_ttl.js completed successfully

╔════════════════════════════════════════╗
║           Test Summary                 ║
╚════════════════════════════════════════╝
Total: 10
Passed: 10
Failed: 0
✅ JavaScript direct examples complete!
📦 Building TypeScript client library...

> @ekodb/ekodb-client@0.27.0 prepare
> npm run build


> @ekodb/ekodb-client@0.27.0 build
> tsc


up to date, audited 44 packages in 564ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.27.0 build
> tsc

✅ TypeScript client built!

added 1 package, removed 1 package, and audited 9 packages in 360ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities

added 1 package, removed 1 package, and audited 13 packages in 377ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> ekodb-typescript-examples@1.0.0 build
> tsc

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
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: vh79aGx_pEbOhp-nnIqW-5IHWep0BtY6bVtG-sNzrrXwcSQSVaErEYa-MqrK2pkLoSqp8RBj0LOmFFZDbe-pZA

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

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
✓ Created second session: sGxbCWrELqpt8SV1wmzl5ama5T_wHJs81Cf6hKpy8qRVncVNGNE5AEwmmQ4ngfX16QxG1QWzLnbIjxEFOEnX5A
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted second session
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: dfe1xl2yCaZR5-09gE5adchEofIHL6MD-loDbtOnmX7wwsyl03_hS8ZZ-mCzRFNs5IXZ3z6S8-amPeXWmuvtEQ

=== Sending Chat Message ===
Message ID: ffzcBnvkFIbPH7gCva3uLxpCgnTMrkaRJ_1OlQIHg1UNBXEZHviq1DzHUkGnZcZG14odX-tjf5knMr_8y3DSkA

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB**
   - **Price:** $99
   - **Description:** A high-performance database product with AI capabilities.

2. **ekoDB Pro**
   - **Price:** $299
   - **Description:** Enterprise edition product with advanced features.

3. **ekoDB Cloud**
   - **Price:** $499
   - **Description:** Fully managed cloud database service product.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'A high-performance database product with AI capabilities',
    id: '4b9C8axznDvbM6hMtbMG9TLtcPK2Rg1jZjSt2Ng6ub3FonpqxXdEOF53mNPcmMhQ40tyGNaUpbElaF9DShV3Zg',
    name: 'ekoDB',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'OiEwtEgQhluRfyTAtSFYVUwr4xYJRSAmm1eUl1w-eOjFtwlsjzlWH1K9Q9jM6XH6pef98VPYw4SwC7a8N0HXBA',
    price: 299,
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    price: 499,
    id: 'OlBtKm9sUU9bCNn5AGFxX0HnP1yyI_lL-feQp8wsNUJvHygFkhyjjGQkQOAIHgTYbYo0DbfDAnn3sNvLksYy4Q',
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3598ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 97
Total tokens: 3510

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
    - o3-2025-04-16
    - gpt-5.1-chat-latest
    - gpt-5.3-chat-latest
    - gpt-5.2-2025-12-11
    - sora-2
    - chatgpt-image-latest
    - gpt-4.1-mini
    - gpt-3.5-turbo
    - gpt-4o-mini
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-6-luna
    - gpt-5.4-mini-2026-03-17
    - gpt-4-turbo
    - gpt-4.1-nano
    - gpt-image-2
    - gpt-5.4-pro
    - gpt-realtime-mini-2025-12-15
    - gpt-3.5-turbo-16k
    - babbage-002
    - text-embedding-ada-002
    - gpt-4o-mini-tts
    - gpt-4o-2024-08-06
    - o4-mini-2025-04-16
    - gpt-4o-2024-11-20
    - omni-moderation-latest
    - gpt-realtime
    - tts-1-hd
    - gpt-realtime-1.5
    - gpt-realtime-2
    - gpt-5-search-api
    - gpt-5.5-2026-04-23
    - gpt-5.1
    - gpt-5.1-codex
    - gpt-4o
    - o1-2024-12-17
    - gpt-5.2-chat-latest
    - gpt-4o-mini-tts-2025-12-15
    - gpt-image-1
    - davinci-002
    - gpt-image-2-2026-04-21
    - gpt-4o-mini-transcribe
    - o3-mini
    - gpt-audio
    - gpt-5.4-nano
    - o3
    - gpt-5.1-codex-max
    - gpt-realtime-2.1
    - gpt-4.1
    - gpt-5.5
    - gpt-4o-mini-search-preview-2025-03-11
    - text-embedding-3-large
    - gpt-4o-mini-search-preview
    - gpt-4o-2024-05-13
    - gpt-6-sol
    - gpt-5.3-codex
    - gpt-3.5-turbo-instruct
    - sora-2-pro
    - gpt-5
    - gpt-audio-mini-2025-12-15
    - gpt-transcribe
    - o4-mini-deep-research-2025-06-26
    - o1-pro-2025-03-19
    - gpt-4.1-nano-2025-04-14
    - gpt-5-search-api-2025-10-14
    - gpt-4-turbo-2024-04-09
    - gpt-realtime-2.1-mini
    - tts-1
    - gpt-5-mini
    - omni-moderation-2024-09-26
    - gpt-5-mini-2025-08-07
    - gpt-realtime-2025-08-28
    - gpt-5.4-2026-03-05
    - gpt-live-transcribe
    - gpt-3.5-turbo-0125
    - gpt-5-nano
    - gpt-5.6-luna
    - gpt-4
    - gpt-image-2.5-sunburst-2026-09-08
    - whisper-1
    - gpt-5.4-mini
    - gpt-5-pro
    - o1-pro
    - gpt-realtime-whisper
    - gpt-5.2-pro
    - gpt-5-chat-latest
    - gpt-live-1
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-5-2025-08-07
    - tts-1-1106
    - gpt-5.5-pro
    - gpt-5-pro-2025-10-06
    - chat-latest
    - gpt-5.2-pro-2025-12-11
    - gpt-4o-mini-transcribe-2025-03-20
    - gpt-audio-2025-08-28
    - gpt-5.4-pro-2026-03-05
    - gpt-realtime-translate
    - gpt-5.6-sol
    - gpt-audio-mini-2025-10-06
    - gpt-5.5-pro-2026-04-23
    - tts-1-hd-1106
    - gpt-realtime-mini
    - gpt-5.4
    - gpt-4o-transcribe-diarize
    - gpt-5-codex
    - gpt-image-1-mini
    - gpt-image-1.5
    - gpt-5.1-2025-11-13
    - o4-mini-deep-research
    - gpt-4o-search-preview-2025-03-11
    - o3-mini-2025-01-31
    - gpt-4-0613
    - gpt-4o-search-preview
    - gpt-5.6-terra
    - text-embedding-3-small
    - gpt-audio-1.5
    - gpt-3.5-turbo-instruct-0914
    - gpt-6-astra
    - gpt-5.2-codex
    - gpt-3.5-turbo-1106
    - gpt-5-nano-2025-08-07
    - gpt-audio-mini
    - gpt-4o-mini-tts-2025-03-20
    - gpt-5.2
    - gpt-4.1-mini-2025-04-14
    - gpt-4.1-2025-04-14
    - gpt-4o-mini-2024-07-18
    - o4-mini
    - o1
    - gpt-5.4-nano-2026-03-17
    - gpt-5.1-codex-mini
    - gpt-4o-transcribe
    - gpt-image-2.5-flare
    - gpt-6.1-sol
  anthropic:
    - claude-haiku-5-5
    - claude-sonnet-5-5
    - claude-opus-5-5
    - claude-fable-5-1
    - claude-opus-5
    - claude-sonnet-5
    - claude-fable-5
    - claude-opus-4-8
    - claude-opus-4-7
    - claude-sonnet-4-6
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
Provider status:
  anthropic: ok 14 models
  gemini: not_configured (unverified) No Gemini API Key
  openai: ok 135 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Specific Provider Models ===
OpenAI models: o3-2025-04-16, gpt-5.1-chat-latest, gpt-5.3-chat-latest, gpt-5.2-2025-12-11, sora-2, chatgpt-image-latest, gpt-4.1-mini, gpt-3.5-turbo, gpt-4o-mini, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-6-luna, gpt-5.4-mini-2026-03-17, gpt-4-turbo, gpt-4.1-nano, gpt-image-2, gpt-5.4-pro, gpt-realtime-mini-2025-12-15, gpt-3.5-turbo-16k, babbage-002, text-embedding-ada-002, gpt-4o-mini-tts, gpt-4o-2024-08-06, o4-mini-2025-04-16, gpt-4o-2024-11-20, omni-moderation-latest, gpt-realtime, tts-1-hd, gpt-realtime-1.5, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, gpt-5.1, gpt-5.1-codex, gpt-4o, o1-2024-12-17, gpt-5.2-chat-latest, gpt-4o-mini-tts-2025-12-15, gpt-image-1, davinci-002, gpt-image-2-2026-04-21, gpt-4o-mini-transcribe, o3-mini, gpt-audio, gpt-5.4-nano, o3, gpt-5.1-codex-max, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-4o-mini-search-preview-2025-03-11, text-embedding-3-large, gpt-4o-mini-search-preview, gpt-4o-2024-05-13, gpt-6-sol, gpt-5.3-codex, gpt-3.5-turbo-instruct, sora-2-pro, gpt-5, gpt-audio-mini-2025-12-15, gpt-transcribe, o4-mini-deep-research-2025-06-26, o1-pro-2025-03-19, gpt-4.1-nano-2025-04-14, gpt-5-search-api-2025-10-14, gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, tts-1, gpt-5-mini, omni-moderation-2024-09-26, gpt-5-mini-2025-08-07, gpt-realtime-2025-08-28, gpt-5.4-2026-03-05, gpt-live-transcribe, gpt-3.5-turbo-0125, gpt-5-nano, gpt-5.6-luna, gpt-4, gpt-image-2.5-sunburst-2026-09-08, whisper-1, gpt-5.4-mini, gpt-5-pro, o1-pro, gpt-realtime-whisper, gpt-5.2-pro, gpt-5-chat-latest, gpt-live-1, gpt-4o-mini-transcribe-2025-12-15, gpt-5-2025-08-07, tts-1-1106, gpt-5.5-pro, gpt-5-pro-2025-10-06, chat-latest, gpt-5.2-pro-2025-12-11, gpt-4o-mini-transcribe-2025-03-20, gpt-audio-2025-08-28, gpt-5.4-pro-2026-03-05, gpt-realtime-translate, gpt-5.6-sol, gpt-audio-mini-2025-10-06, gpt-5.5-pro-2026-04-23, tts-1-hd-1106, gpt-realtime-mini, gpt-5.4, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-image-1-mini, gpt-image-1.5, gpt-5.1-2025-11-13, o4-mini-deep-research, gpt-4o-search-preview-2025-03-11, o3-mini-2025-01-31, gpt-4-0613, gpt-4o-search-preview, gpt-5.6-terra, text-embedding-3-small, gpt-audio-1.5, gpt-3.5-turbo-instruct-0914, gpt-6-astra, gpt-5.2-codex, gpt-3.5-turbo-1106, gpt-5-nano-2025-08-07, gpt-audio-mini, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-4.1-mini-2025-04-14, gpt-4.1-2025-04-14, gpt-4o-mini-2024-07-18, o4-mini, o1, gpt-5.4-nano-2026-03-17, gpt-5.1-codex-mini, gpt-4o-transcribe, gpt-image-2.5-flare, gpt-6.1-sol

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Q9V8MzC9PXNNJDyURerFSrtbG68qUYa4z-W-XGPa_zEH91wM6311xYXP3l4h6pSqDmiPXNjCjQ7Ma_nGXWhgeg

=== Sending Messages ===
✓ Message 1 sent
  Response: Currently, the available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more information or have further questions, feel free to ask!

✓ Message 2 sent
  Response: The price of the product ekoDB is **$99**. If you have any more questions or need further information, feel free to ask!

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 6lSripj6uVBfBH1_rFqr5AxTuOIXHDPfUo6OG5tuzWRSvP0LgoceMLiQX-rU_ZvSuKuIi_-1nc_eHcBvB0a20w
  Parent: Q9V8MzC9PXNNJDyURerFSrtbG68qUYa4z-W-XGPa_zEH91wM6311xYXP3l4h6pSqDmiPXNjCjQ7Ma_nGXWhgeg

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: 6lSripj6uVBfBH1_rFqr5AxTuOIXHDPfUo6OG5tuzWRSvP0LgoceMLiQX-rU_ZvSuKuIi_-1nc_eHcBvB0a20w (Untitled)
  Session 2: Q9V8MzC9PXNNJDyURerFSrtbG68qUYa4z-W-XGPa_zEH91wM6311xYXP3l4h6pSqDmiPXNjCjQ7Ma_nGXWhgeg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 6lSripj6uVBfBH1_rFqr5AxTuOIXHDPfUo6OG5tuzWRSvP0LgoceMLiQX-rU_ZvSuKuIi_-1nc_eHcBvB0a20w

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: 1bMqYxs-kdQ-RmtaNJ7HPjO79SmsGDt4V2fymXcD93gWmiF0LVpBhqikqJe-sYNi2qRKWhGa9L7_ko5FZW5GsA

=== List Collections ===
Total collections: 23
Sample collections: chat_agent_configs__ek0_testing,chat_goal_templates__ek0_testing,client_collection_management_js,schema_documents_client_go,test_accounts

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: yoNfHBB_WGHBwo3IVxgSNy01HUsP-NHmRYLF5lAt-0IBZNfluCgWX3LQOmMqQNNLr3Q2DdEQ8z19IQgfclpT6w

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 8hnZLY8EB9kAb1GVhKrWOOmjLpmYpzaLeSfSa753UYYKr9BFv1UrEQMw_0hRg-XYfy-G8Vy3HiWlC50tCVTBbA

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
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: _YPlpu30v9blaDJTD6gkzVAlBSJ5kivz_Ahg1yWWpcDlIApSqMQbwXHRWU9fM_ho4jeGovt1dPEheRpP2esYHQ

Call 1: Cache miss (fetches from API)
Response time: 164ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "timezone_abbreviation": "GMT",
          "elevation": 32,
          "longitude": -73.99308,
          "utc_offset_seconds": 0,
          "generationtime_ms": 0.042557716369628906,
          "current": {
            "interval": 900,
            "time": "2026-10-08T05:45",
            "temperature_2m": 13.6
          },
          "current_units": {
            "time": "iso8601",
            "temperature_2m": "°C",
            "interval": "seconds"
          },
          "latitude": 40.710335,
          "timezone": "GMT"
        }
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 6ms (27.3x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "timezone_abbreviation": "GMT",
          "elevation": 32,
          "longitude": -73.99308,
          "utc_offset_seconds": 0,
          "generationtime_ms": 0.042557716369628906,
          "current": {
            "interval": 900,
            "time": "2026-10-08T05:45",
            "temperature_2m": 13.6
          },
          "current_units": {
            "time": "iso8601",
            "temperature_2m": "°C",
            "interval": "seconds"
          },
          "latitude": 40.710335,
          "timezone": "GMT"
        }
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!

=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 60ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "name": "Leanne Graham",
      "phone": "1-770-736-8031 x56442",
      "id": 1,
      "email": "Sincere@april.biz",
      "company": {
       ...

Second call (cache hit - from cache):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "name": "Leanne Graham",
      "phone": "1-770-736-8031 x56442",
      "id": 1,
      "email": "Sincere@april.biz",
      "company": {
        "bs": "harness real-t...
   🚀 Cache speedup: 30.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ All composition examples completed!
🚀 ekoDB Functions Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: d3gGPvxm7SOSx3ESJfm1sgGsa39127I7hOWZhsQgfBeC8NdGO4Q9KEH53wBjBalO2-xspBW3vTW5nvv_dqXhBw
📊 Found 5 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 3 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"count":{"type":"Integer","value":5},"avg_score":{"type":"Float","value":60},"status":{"type":"String","value":"active"}}
   {"count":{"type":"Integer","value":5},"avg_score":{"value":50,"type":"Float"},"status":{"type":"String","value":"inactive"}}
⏱️  Execution time: 0ms

📝 Example 4: Function Management

📋 Total functions: 7
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage function saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count function saved
📊 Total user count: 10
⏱️  Execution time: 0ms

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • ChatMessage helpers
   • Cleaner, more maintainable code
   • Built-in error handling
🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test functions

🚀 ekoDB Advanced Functions Example

📋 Setting up test data...
✅ Created 10 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 10 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Found 2 categories
   Electronics: 6 items (avg $325.67)
   Furniture: 4 items (avg $294.00)
⏱️  Execution time: 0ms

📝 Example 3: Count All Products

✅ Function saved
📊 Total products: 10
⏱️  Execution time: 0ms

📝 Example 4: Multi-Stage Aggregation

✅ Function saved
📊 Category analysis (2 categories):
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Function saved
📊 Product summaries (10 items, showing first 3):
   1. Keyboard - $89 (⭐4.4)
   2. Webcam HD - $119 (⭐4.5)
   3. USB-C Cable - $19 (⭐4.3)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced function examples completed!
🚀 ekoDB AI Functions Example

⚠️  Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data...
✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat function saved
🤖 AI Response:
   1. Scalability: Vector databases can handle large amounts of data efficiently.
2. High Accuracy: Vector data model represents data with high degree of accuracy.
3. Flexibility: Vector models are extremely flexible allowing for efficient data manipulation.
4. Rich Geometric and Topological Operations: Complex operations like polygon overlay, network analysis etc. can be done effectively.
5. Compact Data Structure: Vector data is usually more compact in storage size.
6. Detailed Representation: Vector databases can represent complex geographical structures.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding function saved
📊 Generated embeddings for 3 articles
   1. "Advanced Query Patterns" - 1536D vector
   2. "Getting Started with ekoDB" - 1536D vector
   3. "Draft Article" - 1536D vector
⏱️  Execution time: 0ms

📝 Example 3: List All Articles

✅ Function saved
📊 Found 3 articles
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI examples completed!
🚀 ekoDB JavaScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: iIPT33MeGU61sphfsMun6GXtkbDDWXQH_TdD9f2xjMJJM9I-1pTRfHnVNaeeGR0itJSsyj5myZkC_mKcxOeioQ
📊 Found 2 product groups
   {"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Electronics"},"avg_price":{"type":"Float","value":575.6666666666666}}
   {"category":{"type":"String","value":"Furniture"},"count":{"type":"Integer","value":2},"avg_price":{"type":"Float","value":474}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Electronics"}}
   {"count":{"type":"Integer","value":2},"category":{"value":"Furniture","type":"String"}}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: Function with Parameter Definition

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete function examples finished!

💡 This example demonstrates ekoDB's Function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
🚀 ekoDB CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   active: 7 users
   inactive: 3 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average scores by role:
   admin: 20.0 (3 users)
   user: 70.0 (7 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Function saved
📊 Users (showing first 5 of 10):
   1. User 3 - Score: 30
   2. User 10 - Score: 100
   3. User 4 - Score: 40
   4. User 7 - Score: 70
   5. User 6 - Score: 60
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Function saved
📊 User summary (2 groups):
   inactive: 3 users, Total Score: 180
   active: 7 users, Total Score: 370
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD function examples completed!
🚀 ekoDB JavaScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: NcpqBu9D_xpOErtQM0nKKQrpsqBQIxHFs9JD9lWCgYhRhp55cJTOXfg9jAXQUfMriG-rgEAnvHUSuJYlrV8b_Q
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: PcdpwyMb5GTXOLh7WbXOR8AjoRLnmqfhaafma8cc3J6NTSTtYf0jT3raydicwUoFCw489lRgrZO2x9BUVhAA1w
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"value":{"userId":"user_abc","role":"admin"},"type":"Object"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: g3zuaXZpYfUyl0tHSz_SYHpzv9uY_AXLxql3rPhHwyk6zNC6C5e42ezeA0TkTeCBgVV4n4yVoAK8EyjxadFVkg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: sxoz5mjY1l6gRE7mqfNpXmen51xi6YgPWKEeIjSfXPgB3uLZ9OR41yCGtLPtZUB1SNW6FAazLqNjIMtJ_vK0LQ
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use Field.* helpers for type-safe wrapped values
   ✅ Field.decimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ Stage.kv*() functions work within scripts
   ✅ Combine KV caching with collection inserts for real workflows
🚀 ekoDB Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Introduction to Machine Learning (AI)
   2. Vector Databases Explained (Database)
   3. Getting Started with ekoDB (Database)
   4. Natural Language Processing (AI)
   5. Database Design Principles (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   Database: 3 documents
   AI: 2 documents
⏱️  Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Function saved
📊 Document titles (5 docs):
   1. Introduction to Machine Learning
   2. Vector Databases Explained
   3. Getting Started with ekoDB
   4. Natural Language Processing
   5. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Vector Databases Explained
   3. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Function saved
📊 All documents (5 total, showing first 2):
   1. Introduction to Machine Learning (AI)
   2. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search function examples completed!
=== Join Operations Examples ===

Setting up sample data...
✅ Sample data created

1. Single collection join (users with departments):
Found 2 users with department data:
  - Alice Johnson: Engineering
  - Bob Smith: Sales

2. Join with filtering:
Found 1 users in Engineering:
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data:
  - Alice Johnson: Senior Software Engineer
  - Bob Smith: Sales Manager

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
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { value: { username: 'john_doe', userId: 123 }, type: 'Object' }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { type: 'Object', value: { price: 29.99, name: 'Product 1' } }
cache:product:2: {
  type: 'Object',
  value: { price: 39.989999999999995, name: 'Product 2' }
}
cache:product:3: {
  value: { price: 49.989999999999995, name: 'Product 3' },
  type: 'Object'
}

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
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
=== Search Examples ===

Setting up test data...
✅ Test data created

1. Basic full-text search:
Found 2 results
  1. Score: 12.870, Matched: name, email
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: title, bio
  2. Score: 13.200, Matched: title, bio
  3. Score: 13.200, Matched: bio, title
  4. Score: 13.200, Matched: title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: bio, title
  2. Score: 26.400, Matched: title, bio
  3. Score: 26.400, Matched: title, bio
  4. Score: 26.400, Matched: title, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 6.600, Matched: bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.769, Matched:
  2. Score: 0.737, Matched:
  3. Score: 0.736, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.508, Matched: content, title
  2. Score: 0.895, Matched: content, title
  3. Score: 0.294, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: bio, skills

=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'FYHoCH8O-mCeEOnEyA8h3jqorCIEp9ygLAcW-dv1PEYq2JcqFrEhLs5QCitRX5DMZBs8u7HcKvCOKcTzob-_dA'
}

=== Find by ID ===
Found: {
  value: { value: 42, type: 'Integer' },
  id: 'FYHoCH8O-mCeEOnEyA8h3jqorCIEp9ygLAcW-dv1PEYq2JcqFrEhLs5QCitRX5DMZBs8u7HcKvCOKcTzob-_dA',
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Test Record' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  value: { type: 'Integer', value: 100 },
  id: 'FYHoCH8O-mCeEOnEyA8h3jqorCIEp9ygLAcW-dv1PEYq2JcqFrEhLs5QCitRX5DMZBs8u7HcKvCOKcTzob-_dA',
  active: { value: true, type: 'Boolean' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: OCj2xQZO4V_P_858SoFHNhyGluBHjevwIiq2uDx3BhIe6VQcmibsyQFdEpXQ_I2DTmdfoskWlkw-McAZCYkdJQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
🚀 ekoDB JavaScript Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


🧹 Cleaning up...
✓ Deleted 0 test functions and owned SWR resources

Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native_js (GG9n7aHroopxMTz6XpzPbakCzxLnWrjFSlNvUXwlWhjORCTj9TaJ45ebyDlQDimR9KxaCbQ4ETBwe_Kjtuj1LA)

First call (cache miss - will fetch from GitHub API):
  Response time: 120ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 40.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_js (nu52zuni4-BsEhRm67Oc_pKlJ8ciE-7uJbdzuDNyNtQn0JmzItjeG7c71kAG1-WzFA3YOjzQUHJLbmHqsVi1WQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_js' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_js (hVKCAh9WybfMro2ePpFtWXWdOIT98A1L00E5UAhFa2p1zbvPb4O9lpc6JgpFxQXeob2yg4kY3BhCpMPc4V3CCQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_js' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_js (ru3OMmx5tsISGsSGEEFPBEaX4BjqNDWATzfmmf87Cn_VQnwb_eu-pgO96sjXvJTxS0UI_RQg-R6UNNV-CHQ6dw)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

================================================================================
✅ Key Benefits of Native SWR:
✅ Single function: Replaces 4-function cache-aside pattern
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)
Native SWR:     SWR → Insert (2 functions)
Result:         60% fewer functions, cleaner code, same behavior 🎯

🧹 Cleaning up...
✓ Deleted 4 test functions and owned SWR resources

✅ All examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_js_77344_1791438615178 (70yeFu6DOO2_yqopijaDSJW-70TV19IIdgCDBnOMsqHlmF_2zxC5AYBxk3lrxNj0qCU0f4FnRQqiTcSaHz_Nhw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "id": 1,
          "username": "Bret",
          "email": "Sincere@april.biz",
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "street": "Kulas Light"
          },
          "website": "hildegard.org",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "phone": "1-770-736-8031 x56442",
          "name": "Leanne Graham"
        },
        "type": "Object"
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 3ms (served from cache)
✓ Lightning fast cache hit

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🧹 Cleaning up...
✓ Cleanup complete

✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 6fxEtk3LX1kJaCjcWDilXtCs1Uo_N19xsTCTozkw_K2Pn1uOftLnRSeOmaU5a2CUscpy02tn0yxGfCIkeEzEtg
Created Bob: $500 - ID: faC6LfegeUPm7gVqxZCBdCoILaquu8th87DBAp46nfZVct8jIgbwFSru8iO1VWAw7m9On6rJcVdq3ycQk1JmTw

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): fbc879a2-0309-4d79-a1a5-4f9181d95057

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

✓ Verified committed balances: Alice=$800, Bob=$700

=== Example 5: Rollback Demo ===
New transaction: 8314440d-7582-4d93-9dc5-5a99e846ab47
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Fi1iS01kX3zYVue5sPc3wj4P2ZzfYeHgIx7oNiC0YWDMYWnxd-WBt0FAoQANzZ2WJVwLKmAitvGtWiWW2zfHnw

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB Advanced CRUD Example (TypeScript) ===

--- Inserting base record ---
Inserted: aBz5il0E8sdTk_wD-V9LnJVWZOP9E-4Ev_SBfphpsl9088l_vU1XCBQOeR9nMmKLhlcI4g-ze1DACosFc-4DCg

--- updateWithAction: increment ---
Score after increment: 150

--- updateWithAction: push ---
Tags after push: ["beginner","pro"]

--- updateWithAction: clear ---
temp_data after clear: null

--- updateWithActionSequence: multiple atomic actions ---
After sequence - score: 160, lives: 2, tags: ["beginner","pro","veteran"]

--- Cleanup ---
Deleted collection

=== All advanced CRUD operations completed ===
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
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: gcL-pVcAW4S4FX8G3ys2laryRS-jnwDKDzyPk_P-iRmo6l4v0_d9JNQhSvzEx6BDipuW5E9V3154_divUtvIyw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

Is there anything else you would like to know?

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
✓ Created second session: 8wBSRAfifb4uXr3BlEX_R2u4fNGJaJkk4aPIKB3cLh3TgzGdlLUljQle0qhEGCO-q8KfJO1RD9fY0HUN9TgJjg
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: 8wBSRAfifb4uXr3BlEX_R2u4fNGJaJkk4aPIKB3cLh3TgzGdlLUljQle0qhEGCO-q8KfJO1RD9fY0HUN9TgJjg
✓ Deleted chat session: gcL-pVcAW4S4FX8G3ys2laryRS-jnwDKDzyPk_P-iRmo6l4v0_d9JNQhSvzEx6BDipuW5E9V3154_divUtvIyw
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 09cVkubd4wjIjfsXw1Bc8WmWV1ezshqBgWegepXDKWRl-7pENfTRV7GVNIe48AjGw5VooItyZ85bwLBU-aRLLQ

=== Sending Chat Message ===
Message ID: 5H916ZXc_A152leTKPTFFrfmLEydVC9lX2_7u_pYeFy-DjKNHJXHpevZev0BlrpTqUnNOI1UhuWBvUBvB7qA5g

=== AI Response ===
The available products along with their prices are:

1. **ekoDB** - Price: $99
   - Description: A high-performance database product with AI capabilities.

2. **ekoDB Pro** - Price: $299
   - Description: Enterprise edition product with advanced features.

3. **ekoDB Cloud** - Price: $499
   - Description: Fully managed cloud database service product.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    id: 'JM2hQBors1aqoeNXl6PRs9f3VkL9TOll4mpTj7yC6oa35LeT5KxEWQsXJOZcp6SmchwDtRxKzFkW2aeifV81Rw',
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'Yh__DGjGF8KGQIfEjKmBcskXAjaUSlqKMZetdrqW_05MLoDz56VkDbXvSp5iaSAaqL7zuHgLBDOwbDxmjdYQlQ',
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Cloud',
    id: '_mcCxYt4FoWsSXw6bwqeoUmsKpKH_mrfEucnp6_GUEZYlzkO0kNAjPNfqCkXJ4n9KPSMBGpHi4BXJsqzr3nHDw',
    description: 'Fully managed cloud database service product',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2212ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 88
Total tokens: 3501

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (TypeScript) ===

Created session: GR2Dyo77-T_nJ0qrl-mxcitxNCOZM3ZnTQ5Bi615B1fEFL8LLicDVljNpNKNkX8YW66AmMS663Vg5wx-GnFdug

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}**ekoDB** is an open-source database system specifically designed to optimize for **energy efficiency** in data management systems. Unlike traditional databases that primarily focus on performance or scalability, ekoDB incorporates energy consumption as a core design principle, enabling modern data platforms to substantially reduce their carbon footprint and operational costs.

### Key Features of ekoDB:
- **Energy-Aware Data Management**: Executes queries and manages data with strategies that minimize energy consumption, sometimes by trading off speed for power savings.
- **Custom Storage Engines**: Offers storage layers tailored to save energy based on hardware characteristics and workload patterns.
- **Modular Architecture**: Allows integration with other data systems or embedding within green computing pipelines.
- **Developer Tools**: Provides APIs and visualization tools to monitor and optimize energy consumption for different workloads.

### Use Cases:
- Deploying in **eco-friendly data centers** to meet sustainability goals.
- Power- and cost-sensitive environments, such as edge devices or developing regions.
- Academic research on energy-efficient computing and green IT.

**Status:**
ekoDB is developed and maintained by researchers and practitioners interested in sustainable computing. It’s available on platforms like GitHub and is typically used in research, prototyping, or pilot projects aiming for green technology.

---

**Note:** If you meant “ekoDB” in a different context or a specific product, please clarify, as the term may have other meanings in unrelated fields.

--- Stream complete ---
Message ID: S_1jqHvl2KAp9oj5AZmt8F4_7VvcEAtn9x00q2OIOWlKMrpWQkDR-cmXBX_BtwaZ_wVjJsF-5-yyMf7ui9nGdA
Execution time: 2637ms
Context window: 1000000 tokens

✓ Chat message stream example completed
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
    - o3-2025-04-16
    - gpt-5.1-chat-latest
    - gpt-5.3-chat-latest
    - gpt-5.2-2025-12-11
    - sora-2
    - chatgpt-image-latest
    - gpt-4.1-mini
    - gpt-3.5-turbo
    - gpt-4o-mini
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-6-luna
    - gpt-5.4-mini-2026-03-17
    - gpt-4-turbo
    - gpt-4.1-nano
    - gpt-image-2
    - gpt-5.4-pro
    - gpt-realtime-mini-2025-12-15
    - gpt-3.5-turbo-16k
    - babbage-002
    - text-embedding-ada-002
    - gpt-4o-mini-tts
    - gpt-4o-2024-08-06
    - o4-mini-2025-04-16
    - gpt-4o-2024-11-20
    - omni-moderation-latest
    - gpt-realtime
    - tts-1-hd
    - gpt-realtime-1.5
    - gpt-realtime-2
    - gpt-5-search-api
    - gpt-5.5-2026-04-23
    - gpt-5.1
    - gpt-5.1-codex
    - gpt-4o
    - o1-2024-12-17
    - gpt-5.2-chat-latest
    - gpt-4o-mini-tts-2025-12-15
    - gpt-image-1
    - davinci-002
    - gpt-image-2-2026-04-21
    - gpt-4o-mini-transcribe
    - o3-mini
    - gpt-audio
    - gpt-5.4-nano
    - o3
    - gpt-5.1-codex-max
    - gpt-realtime-2.1
    - gpt-4.1
    - gpt-5.5
    - gpt-4o-mini-search-preview-2025-03-11
    - text-embedding-3-large
    - gpt-4o-mini-search-preview
    - gpt-4o-2024-05-13
    - gpt-6-sol
    - gpt-5.3-codex
    - gpt-3.5-turbo-instruct
    - sora-2-pro
    - gpt-5
    - gpt-audio-mini-2025-12-15
    - gpt-transcribe
    - o4-mini-deep-research-2025-06-26
    - o1-pro-2025-03-19
    - gpt-4.1-nano-2025-04-14
    - gpt-5-search-api-2025-10-14
    - gpt-4-turbo-2024-04-09
    - gpt-realtime-2.1-mini
    - tts-1
    - gpt-5-mini
    - omni-moderation-2024-09-26
    - gpt-5-mini-2025-08-07
    - gpt-realtime-2025-08-28
    - gpt-5.4-2026-03-05
    - gpt-live-transcribe
    - gpt-3.5-turbo-0125
    - gpt-5-nano
    - gpt-5.6-luna
    - gpt-4
    - gpt-image-2.5-sunburst-2026-09-08
    - whisper-1
    - gpt-5.4-mini
    - gpt-5-pro
    - o1-pro
    - gpt-realtime-whisper
    - gpt-5.2-pro
    - gpt-5-chat-latest
    - gpt-live-1
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-5-2025-08-07
    - tts-1-1106
    - gpt-5.5-pro
    - gpt-5-pro-2025-10-06
    - chat-latest
    - gpt-5.2-pro-2025-12-11
    - gpt-4o-mini-transcribe-2025-03-20
    - gpt-audio-2025-08-28
    - gpt-5.4-pro-2026-03-05
    - gpt-realtime-translate
    - gpt-5.6-sol
    - gpt-audio-mini-2025-10-06
    - gpt-5.5-pro-2026-04-23
    - tts-1-hd-1106
    - gpt-realtime-mini
    - gpt-5.4
    - gpt-4o-transcribe-diarize
    - gpt-5-codex
    - gpt-image-1-mini
    - gpt-image-1.5
    - gpt-5.1-2025-11-13
    - o4-mini-deep-research
    - gpt-4o-search-preview-2025-03-11
    - o3-mini-2025-01-31
    - gpt-4-0613
    - gpt-4o-search-preview
    - gpt-5.6-terra
    - text-embedding-3-small
    - gpt-audio-1.5
    - gpt-3.5-turbo-instruct-0914
    - gpt-6-astra
    - gpt-5.2-codex
    - gpt-3.5-turbo-1106
    - gpt-5-nano-2025-08-07
    - gpt-audio-mini
    - gpt-4o-mini-tts-2025-03-20
    - gpt-5.2
    - gpt-4.1-mini-2025-04-14
    - gpt-4.1-2025-04-14
    - gpt-4o-mini-2024-07-18
    - o4-mini
    - o1
    - gpt-5.4-nano-2026-03-17
    - gpt-5.1-codex-mini
    - gpt-4o-transcribe
    - gpt-image-2.5-flare
    - gpt-6.1-sol
  anthropic:
    - claude-haiku-5-5
    - claude-sonnet-5-5
    - claude-opus-5-5
    - claude-fable-5-1
    - claude-opus-5
    - claude-sonnet-5
    - claude-fable-5
    - claude-opus-4-8
    - claude-opus-4-7
    - claude-sonnet-4-6
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
Provider status:
  anthropic: ok 14 models
  gemini: not_configured (unverified) No Gemini API Key
  openai: ok 135 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Specific Provider Models ===
OpenAI models: o3-2025-04-16, gpt-5.1-chat-latest, gpt-5.3-chat-latest, gpt-5.2-2025-12-11, sora-2, chatgpt-image-latest, gpt-4.1-mini, gpt-3.5-turbo, gpt-4o-mini, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-6-luna, gpt-5.4-mini-2026-03-17, gpt-4-turbo, gpt-4.1-nano, gpt-image-2, gpt-5.4-pro, gpt-realtime-mini-2025-12-15, gpt-3.5-turbo-16k, babbage-002, text-embedding-ada-002, gpt-4o-mini-tts, gpt-4o-2024-08-06, o4-mini-2025-04-16, gpt-4o-2024-11-20, omni-moderation-latest, gpt-realtime, tts-1-hd, gpt-realtime-1.5, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, gpt-5.1, gpt-5.1-codex, gpt-4o, o1-2024-12-17, gpt-5.2-chat-latest, gpt-4o-mini-tts-2025-12-15, gpt-image-1, davinci-002, gpt-image-2-2026-04-21, gpt-4o-mini-transcribe, o3-mini, gpt-audio, gpt-5.4-nano, o3, gpt-5.1-codex-max, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-4o-mini-search-preview-2025-03-11, text-embedding-3-large, gpt-4o-mini-search-preview, gpt-4o-2024-05-13, gpt-6-sol, gpt-5.3-codex, gpt-3.5-turbo-instruct, sora-2-pro, gpt-5, gpt-audio-mini-2025-12-15, gpt-transcribe, o4-mini-deep-research-2025-06-26, o1-pro-2025-03-19, gpt-4.1-nano-2025-04-14, gpt-5-search-api-2025-10-14, gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, tts-1, gpt-5-mini, omni-moderation-2024-09-26, gpt-5-mini-2025-08-07, gpt-realtime-2025-08-28, gpt-5.4-2026-03-05, gpt-live-transcribe, gpt-3.5-turbo-0125, gpt-5-nano, gpt-5.6-luna, gpt-4, gpt-image-2.5-sunburst-2026-09-08, whisper-1, gpt-5.4-mini, gpt-5-pro, o1-pro, gpt-realtime-whisper, gpt-5.2-pro, gpt-5-chat-latest, gpt-live-1, gpt-4o-mini-transcribe-2025-12-15, gpt-5-2025-08-07, tts-1-1106, gpt-5.5-pro, gpt-5-pro-2025-10-06, chat-latest, gpt-5.2-pro-2025-12-11, gpt-4o-mini-transcribe-2025-03-20, gpt-audio-2025-08-28, gpt-5.4-pro-2026-03-05, gpt-realtime-translate, gpt-5.6-sol, gpt-audio-mini-2025-10-06, gpt-5.5-pro-2026-04-23, tts-1-hd-1106, gpt-realtime-mini, gpt-5.4, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-image-1-mini, gpt-image-1.5, gpt-5.1-2025-11-13, o4-mini-deep-research, gpt-4o-search-preview-2025-03-11, o3-mini-2025-01-31, gpt-4-0613, gpt-4o-search-preview, gpt-5.6-terra, text-embedding-3-small, gpt-audio-1.5, gpt-3.5-turbo-instruct-0914, gpt-6-astra, gpt-5.2-codex, gpt-3.5-turbo-1106, gpt-5-nano-2025-08-07, gpt-audio-mini, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-4.1-mini-2025-04-14, gpt-4.1-2025-04-14, gpt-4o-mini-2024-07-18, o4-mini, o1, gpt-5.4-nano-2026-03-17, gpt-5.1-codex-mini, gpt-4o-transcribe, gpt-image-2.5-flare, gpt-6.1-sol

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: LBtNdoUOgYJOv7bTWRklLXbrw1s4OdE2oQgCbaid5H94UJriBx4-gmPFnHlP_wSHJn6oIZMw6NHIax3eyw5gQQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need more details or have other questions, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99. If you have any more questions or need further information, feel free to ask!

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 86P716BFAPkMWT3IrxS3KWBkPzZKDHzgthw9KwCNfm2DyyuaTrOLm6Qk71HAv25gipBl6rDt5HGJyMgF8dawGw
  Parent: LBtNdoUOgYJOv7bTWRklLXbrw1s4OdE2oQgCbaid5H94UJriBx4-gmPFnHlP_wSHJn6oIZMw6NHIax3eyw5gQQ

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: 86P716BFAPkMWT3IrxS3KWBkPzZKDHzgthw9KwCNfm2DyyuaTrOLm6Qk71HAv25gipBl6rDt5HGJyMgF8dawGw (Untitled)
  Session 2: LBtNdoUOgYJOv7bTWRklLXbrw1s4OdE2oQgCbaid5H94UJriBx4-gmPFnHlP_wSHJn6oIZMw6NHIax3eyw5gQQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 86P716BFAPkMWT3IrxS3KWBkPzZKDHzgthw9KwCNfm2DyyuaTrOLm6Qk71HAv25gipBl6rDt5HGJyMgF8dawGw

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: iGvqxfK1Pt2kjHTRxlt0j8Jc72E5yim8MvO5oAxjaKCezLoDfxrYN7gU2NillTJfBHH0VzIF9h6DBZY16uEASw

=== List Collections ===
Total collections: 27
Sample collections: chat_agent_configs__ek0_testing,chat_goal_templates__ek0_testing,schema_documents_client_js,schema_documents_client_go,test_accounts

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test_ts' exists: false

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test_ts' exists: true

=== Count Documents ===
Document count in 'collection_utils_test_ts': 5

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_ts'

✓ Collection Utilities example complete
✓ Client created
✓ conc_demo_pay saved
✓ conc_demo_rl_fail saved
✓ conc_demo_rl_skip saved
✓ conc_demo_lock saved

Invoke them like:
  POST /api/functions/conc_demo_pay_ts_77654_1791438637251 { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail_ts_77654_1791438637251 { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip_ts_77654_1791438637251 { "user_id": 42 }
  POST /api/functions/conc_demo_lock_ts_77654_1791438637251 { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'VF7yeTDbYbkKltwgWw-SN-MB7VCQYj8_VYkZ8mhFDZKjYmg4pa_YsLorDWfDKfsNX5vUIyN4SYRTl5HklBRpCA'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: 'VF7yeTDbYbkKltwgWw-SN-MB7VCQYj8_VYkZ8mhFDZKjYmg4pa_YsLorDWfDKfsNX5vUIyN4SYRTl5HklBRpCA',
  age: { value: 29, type: 'Integer' },
  name: { value: 'Alice Johnson', type: 'String' },
  active: { value: true, type: 'Boolean' },
  email: { type: 'String', value: 'alice.j@newdomain.com' }
}
✓ Second upsert (insert): { id: 'new-user-id' }

=== Find One Operation ===
✓ Found user by email: {
  active: { value: true, type: 'Boolean' },
  name: { value: 'Alice Johnson', type: 'String' },
  age: { value: 29, type: 'Integer' },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  id: 'VF7yeTDbYbkKltwgWw-SN-MB7VCQYj8_VYkZ8mhFDZKjYmg4pa_YsLorDWfDKfsNX5vUIyN4SYRTl5HklBRpCA'
}
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: true
✓ Fake record exists: false (should be false)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 7 records (expected ~7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!
✓ Client created
✓ crypto_demo_hmac_ts saved
✓ crypto_demo_aes_ts saved
✓ crypto_demo_uuid_ts saved
✓ crypto_demo_totp_ts saved
✓ crypto_demo_encoding_ts saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac_ts { "payload": "hi" }
  POST /api/functions/crypto_demo_aes_ts { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid_ts
  POST /api/functions/crypto_demo_totp_ts
  POST /api/functions/crypto_demo_encoding_ts { "title": "Héllo World" }

✓ Cleaned up demo functions
=== Distinct Values Example ===

Inserting sample products...
Inserted 8 products

=== Distinct Categories (all products) ===
Found 3 distinct categories:
  - books
  - clothing
  - electronics

=== Distinct Statuses (all products) ===
Found 3 distinct statuses:
  - active
  - archived
  - discontinued

=== Distinct Statuses in Electronics ===
Found 2 distinct statuses for electronics:
  - active
  - discontinued

Cleanup done.
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: b5ZUFaIGY1a3AO1Ex5RvG_Tq6PDjqgZc8GeFsLpFJH6UipixqDe3tdezWEGdwet3lzG0KAJokLPnIpFEcKCIGA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 4ixr1xclCuamfB7iHhO2qdPJ8uM8mA-JJB3jzKgByLVCOklyrHbN37zDkWcLcBbxEKemZH7ShYqvqFjEY85wJg

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
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: TiVmyS7dgk_sQ0khOcN8BmGFRncf-kMjMRy-e2q8xO_6R-0e6h22PFAVkZ3BQSiXH_HLz1IwYBBbJmQsJhEtsw

Call 1: Cache miss (fetches from API)
Response time: 150ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "name": "Leanne Graham",
          "id": 1,
          "username": "Bret",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "address": {
            "suite": "Apt. 556",
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "city": "Gwenborough",
            "zipcode": "92998-3874"
          },
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "email": "Sincere@april.biz"
        },
        "type": "Object"
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 3ms (50x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "name": "Leanne Graham",
          "id": 1,
          "username": "Bret",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "address": {
            "suite": "Apt. 556",
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "city": "Gwenborough",
            "zipcode": "92998-3874"
          },
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "email": "Sincere@april.biz"
        }
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!

=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 69ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "name": "Leanne Graham",
      "username": "Bret",
      "website": "hildegard.org",
      "email": "Sincere@april.biz",
      "address": {
        "suite": "Apt. 5...

Second call (cache hit - from cache):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "name": "Leanne Graham",
      "username": "Bret",
      "website": "hildegard.org",
      "email": "Sincere@april.biz",
      "address": {
        "suite": "Apt. 5...
   🚀 Cache speedup: 23.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ All composition examples completed!
client_function_contract: ok
🚀 ekoDB Functions Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: geAp2kpmcbW2pX-R4LqNOJm2LKSIhXjh3I5-Ej78feX4aWh97ZzjGmjAAsb5pVLGDO01JIKqVzh4cIiqymi_NA
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: KrHzkFUIkCp3yOWb2DodTjI4VfKKa-DA2HP1DPdhl-hevNfXWnCaKDoEjewvvKtLqNxHNsYUgvAEDFViyQogTg
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: qQ5sqG2FKZMxLJ5K3UAxAAUPXXeRcu77yu8xhX3iFOncptm8--XuzOSJ4RUTlFZoLAx2A65As4jKoGyXS25uaQ
📊 Statistics: 2 groups
   {"count":{"value":5,"type":"Integer"},"avg_score":{"value":60,"type":"Float"},"status":{"type":"String","value":"active"}}
   {"count":{"value":5,"type":"Integer"},"avg_score":{"value":50,"type":"Float"},"status":{"value":"inactive","type":"String"}}

📝 Example 4: UserFunction Management

📋 Total scripts: 10
🔍 Retrieved script: Get Active Users
✏️  function updated
🗑️  function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
🚀 ekoDB TypeScript Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   {"count":{"type":"Integer","value":5},"category":{"value":"Electronics","type":"String"},"avg_price":{"type":"Float","value":367}}
   {"count":{"value":3,"type":"Integer"},"category":{"value":"Furniture","type":"String"},"avg_price":{"type":"Float","value":365.6666666666667}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB TypeScript AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits, including:

1. **Efficient Similarity Search**: They enable rapid retrieval of similar items based on vector embeddings, which is ideal for tasks like image, text, and recommendation systems.

2. **Scalability**: They can handle large volumes of high-dimensional data efficiently, making them suitable for big data applications.

3. **Real-Time Queries**: Vector databases support real-time querying, facilitating dynamic applications and interactive AI features.

4. **Multi-Modality Support**: They can integrate and query diverse data types (text, images, audio) by representing them as vectors.

5. **Enhanced Machine Learning Integration**: They seamlessly integrate with machine learning workflows, allowing for direct storage and querying of model outputs.

6. **High Dimensionality Handling**: Designed to manage and optimize for high-dimensional spaces effectively, avoiding the "curse of dimensionality."

7. **Indexing Techniques**: Utilize advanced indexing methods (like HNSW, IVF) to speed up nearest neighbor searches.

8. **Flexibility**: Suitable for a variety of applications, from natural language processing to computer vision.

9. **Improved Accuracy**: More accurate results in finding nearest neighbors compared to traditional databases due to vector representations.

10. **Support for Advanced Analytics**: Enables complex analytics and insights generation by leveraging the underlying vector representations.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
📊 Generated 2 embeddings
   Dimensions: 1536
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text
🚀 ekoDB TypeScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: 7vGqUEha7vbBCjcA55L3O0Gv-1VUZh-xQKeAzP_uQ86YiMJ3hnM_6mzEAcxywv0W7fNNYmFU41j5ixavPQx6ow
📊 Found 2 product groups
   {"avg_price":{"type":"Float","value":575.6666666666666},"category":{"value":"Electronics","type":"String"},"count":{"type":"Integer","value":3}}
   {"avg_price":{"type":"Float","value":474},"category":{"value":"Furniture","type":"String"},"count":{"type":"Integer","value":2}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":{"value":2,"type":"Integer"},"category":{"value":"Furniture","type":"String"}}
   {"category":{"value":"Electronics","type":"String"},"count":{"value":3,"type":"Integer"}}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: UserFunction with Parameter Definition

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
🚀 ekoDB TypeScript CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average score by role:
   {"avg_score":{"type":"Float","value":70},"role":{"value":"user","type":"String"},"count":{"type":"Integer","value":7}}
   {"role":{"type":"String","value":"admin"},"count":{"value":3,"type":"Integer"},"avg_score":{"value":20,"type":"Float"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB TypeScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: ev5rl-Vt4B2K_DhChOLq9rGP6lZhwwd0QNjF3TBhWNIGvTA4AgT02M1X5HR5RZLgZWasNs8ZmgITZmrc-ijFtw
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: tZlGOEx0nNU3glfXFhy2Fq1q1oCNS6JTYN9QW9n83lsrJjV-v5Mc_wn1dCoYffBmywuPAngiQ4VXMIWWqw2D4g
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"role":"admin","userId":"user_abc"}}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: 4WpGwkbCD3kWm6dw7GXtHcMxRJAAvvzjYiftjcIkzFP6Msi5Mk3I9Lfx-t5hcc3_LzDhnawfBFYJipC8ABvMHg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: TZzWfebyTz-cnIQEQuHSAhmQ3my94nv-qldU2NicFNnvQlMvB_yXaXpUBk8skQcC9VebUQ2jEoH9ybQQAPcVWA
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use Field.* helpers for type-safe wrapped values
   ✅ Field.decimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ Stage.kv*() functions work within scripts
   ✅ Combine KV caching with collection inserts for real workflows
🚀 ekoDB TypeScript Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Database Design Principles (Database)
   2. Getting Started with ekoDB (Database)
   3. Natural Language Processing (AI)
   4. Vector Databases Explained (Database)
   5. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"value":"Database","type":"String"},"count":{"type":"Integer","value":3}}
   {"count":{"value":2,"type":"Integer"},"category":{"type":"String","value":"AI"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Goal Template CRUD Example (TypeScript) ===

--- Creating goal template ---
Created template: Data Migration (id: TsSw9U4UDcgcVJjzWjmoHbn7GYqqL6u08FXQjQ_csbEKArEPFTYROQtK3dCwHKk_aVjY2typpz11NNko6ztBZg)

--- Listing templates ---
Templates: {
  count: 1,
  items: [
    {
      description: [Object],
      id: 'TsSw9U4UDcgcVJjzWjmoHbn7GYqqL6u08FXQjQ_csbEKArEPFTYROQtK3dCwHKk_aVjY2typpz11NNko6ztBZg',
      steps: [Object],
      title: [Object]
    }
  ]
}

--- Getting template ---
Fetched: Data Migration

--- Updating template ---
Updated description: Updated: comprehensive data migration workflow

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
=== ekoDB Goals, Tasks & Agents Example (TypeScript) ===

--- Creating goal ---
Created goal: Deploy v2.0 (id: VpVedpa7IugvOujdhZ17lBz8McAFbagb57MdXEqNCF2-BmzTVYi3oicPs61kmdjevICkLts2NQfePNQW7g1B2Q)

--- Listing goals ---
Goals: {
  "count": 1,
  "goals": [
    {
      "created_at": "2026-10-08T05:50:43.724414+00:00",
      "description": "Ship version 2.0 to production",
      "id": "VpVedpa7IugvOujdhZ17lBz8McAFbagb57MdXEqNCF2-BmzTVYi3oicPs61kmdjevICkLts2NQfePNQW7g1B2Q",
      "status": "pending",
      "steps": "[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]",
      "title": "Deploy v2.0",
      "updated_at": "2026-10-08T05:50:43.724414+00:00"
    }
  ]
}

--- Getting goal ---
Fetched: Deploy v2.0

--- Updating goal ---
Updated description: Ship version 2.0 with hot-fix patches

--- Searching goals ---
Search results: {
  "count": 1,
  "items": [
    {
      "_score": 12.870000000000001,
      "created_at": {
        "type": "DateTime",
        "value": "2026-10-08T05:50:43.724414+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 with hot-fix patches"
      },
      "id": "VpVedpa7IugvOujdhZ17lBz8McAFbagb57MdXEqNCF2-BmzTVYi3oicPs61kmdjevICkLts2NQfePNQW7g1B2Q",
      "status": {
        "type": "String",
        "value": "pending"
      },
      "steps": {
        "type": "String",
        "value": "[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"
      },
      "title": {
        "type": "String",
        "value": "Deploy v2.0"
      },
      "updated_at": {
        "type": "DateTime",
        "value": "2026-10-08T05:50:43.736268+00:00"
      }
    }
  ]
}

--- Goal step: start step 0 ---
Step 0 started on goal VpVedpa7IugvOujdhZ17lBz8McAFbagb57MdXEqNCF2-BmzTVYi3oicPs61kmdjevICkLts2NQfePNQW7g1B2Q
--- Goal step: complete step 0 ---
Step 0 completed on goal VpVedpa7IugvOujdhZ17lBz8McAFbagb57MdXEqNCF2-BmzTVYi3oicPs61kmdjevICkLts2NQfePNQW7g1B2Q
--- Goal step: fail step 1 ---
Step 1 failed on goal VpVedpa7IugvOujdhZ17lBz8McAFbagb57MdXEqNCF2-BmzTVYi3oicPs61kmdjevICkLts2NQfePNQW7g1B2Q

--- Completing goal ---
Goal status: pending_review
--- Approving goal ---
Goal status after approve: in_progress

--- Creating goal to reject ---
--- Rejecting goal ---
Goal status after reject: failed


--- Creating task ---
Created task: Hourly Health Check (id: b-ZLzoo7He4v5oHE0E6s6eOQ9XhV1Lt6hwtdWk-dbg49gGxiFpwsWE2uKOXeXYzDiYLQWM3D4sJq2h6vTvG4OQ)

--- Listing tasks ---
Tasks: {
  "count": 1,
  "items": [
    {
      "action": {
        "type": "String",
        "value": "health_check"
      },
      "config": {
        "type": "Object",
        "value": {
          "endpoint": "/health"
        }
      },
      "cron": {
        "type": "String",
        "value": "0 * * * *"
      },
      "id": "b-ZLzoo7He4v5oHE0E6s6eOQ9XhV1Lt6hwtdWk-dbg49gGxiFpwsWE2uKOXeXYzDiYLQWM3D4sJq2h6vTvG4OQ",
      "name": {
        "type": "String",
        "value": "Hourly Health Check"
      }
    }
  ]
}

--- Getting task ---
Fetched: Hourly Health Check

--- Starting task ---
Task status: running

--- Succeeding task ---
Task status after succeed: active

--- Pausing task ---
Task status after pause: paused

--- Resuming task ---
Task status after resume: active

--- Failing task ---
Task status after fail: active

--- Getting due tasks ---
Due tasks: {
  "count": 0,
  "items": []
}

--- Deleting task ---
Task deleted successfully


--- Creating agent ---
Created agent: SupportBot (id: zx-12eeHbW-1vWbsrbPNu8Uccu6AMlUpXHjc9p-zrFy7RS_-dJXrkIy4GQ9z4_y_bSUBdRFAp4HxsDMQJ5RNPg)

--- Listing agents ---
Agents: {
  "count": 1,
  "items": [
    {
      "deployment_id": {
        "type": "String",
        "value": "deploy_prod_1"
      },
      "id": "zx-12eeHbW-1vWbsrbPNu8Uccu6AMlUpXHjc9p-zrFy7RS_-dJXrkIy4GQ9z4_y_bSUBdRFAp4HxsDMQJ5RNPg",
      "llm_model": {
        "type": "String",
        "value": "gpt-4"
      },
      "name": {
        "type": "String",
        "value": "SupportBot"
      },
      "system_prompt": {
        "type": "String",
        "value": "You are a helpful customer support agent."
      }
    }
  ]
}

--- Getting agent by ID ---
Fetched: SupportBot

--- Getting agent by name ---
By name: SupportBot (id: zx-12eeHbW-1vWbsrbPNu8Uccu6AMlUpXHjc9p-zrFy7RS_-dJXrkIy4GQ9z4_y_bSUBdRFAp4HxsDMQJ5RNPg)

--- Updating agent ---
Updated agent: SupportBot

--- Getting agents by deployment ---
Agents in deployment: {
  "count": 0,
  "items": []
}
WARNING: agents-by-deployment omitted created agent zx-12eeHbW-1vWbsrbPNu8Uccu6AMlUpXHjc9p-zrFy7RS_-dJXrkIy4GQ9z4_y_bSUBdRFAp4HxsDMQJ5RNPg; TODO: check/fix the server-side deployment lookup

--- Deleting agent ---
Agent deleted successfully

--- Cleanup: deleting goals ---
Goals deleted successfully

=== All goals, tasks & agents operations completed ===
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


✅ Join operations examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created
✓ ts_users_register saved
✓ ts_users_login saved
✓ ts_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/jwt_register_ts_77704_1791438644228 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_login_ts_77704_1791438644228 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_verify_ts_77704_1791438644228 { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

✓ Cleaned up demo functions
=== ekoDB KV Document Linking Example (TypeScript) ===

--- Setup: creating KV key and documents ---
Set KV key: session:admin
Inserted document 1: T5otXiP3KE7uafMxmOKX46NXefLT1EDewEFA___LMaLcjUsiIXRt9xmV-Vo0LbVEPQfvtu6RcOuAWMWlijjfwQ
Inserted document 2: K3-0yUE03TfiU4tLZ1EDD4ThSBEUf42uWbwursWOyMIKDItqqWd1SuyPcSjzUtlYMdyoNIVCKO8z3SiFQpqJdQ

--- Linking documents to KV key ---
Linked doc T5otXiP3KE7uafMxmOKX46NXefLT1EDewEFA___LMaLcjUsiIXRt9xmV-Vo0LbVEPQfvtu6RcOuAWMWlijjfwQ: null
Linked doc K3-0yUE03TfiU4tLZ1EDD4ThSBEUf42uWbwursWOyMIKDItqqWd1SuyPcSjzUtlYMdyoNIVCKO8z3SiFQpqJdQ: null

--- Getting links for KV key ---
Links: [
  {
    "collection": "kv_links_example_ts_77706_1791438644309",
    "document_id": "T5otXiP3KE7uafMxmOKX46NXefLT1EDewEFA___LMaLcjUsiIXRt9xmV-Vo0LbVEPQfvtu6RcOuAWMWlijjfwQ",
    "field_path": null,
    "created_at": "2026-10-08T05:50:44.389902Z",
    "last_accessed": "2026-10-08T05:50:44.392615Z",
    "metadata": {}
  },
  {
    "collection": "kv_links_example_ts_77706_1791438644309",
    "document_id": "K3-0yUE03TfiU4tLZ1EDD4ThSBEUf42uWbwursWOyMIKDItqqWd1SuyPcSjzUtlYMdyoNIVCKO8z3SiFQpqJdQ",
    "field_path": null,
    "created_at": "2026-10-08T05:50:44.391336Z",
    "last_accessed": "2026-10-08T05:50:44.392615Z",
    "metadata": {}
  }
]

--- Unlinking document ---
Unlinked doc K3-0yUE03TfiU4tLZ1EDD4ThSBEUf42uWbwursWOyMIKDItqqWd1SuyPcSjzUtlYMdyoNIVCKO8z3SiFQpqJdQ: null

--- Verifying remaining links ---
Remaining links: [
  {
    "collection": "kv_links_example_ts_77706_1791438644309",
    "document_id": "T5otXiP3KE7uafMxmOKX46NXefLT1EDewEFA___LMaLcjUsiIXRt9xmV-Vo0LbVEPQfvtu6RcOuAWMWlijjfwQ",
    "field_path": null,
    "created_at": "2026-10-08T05:50:44.389902Z",
    "last_accessed": "2026-10-08T05:50:44.394857Z",
    "metadata": {}
  }
]

=== All KV linking operations completed ===

--- Cleanup ---
Cleanup complete
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { type: 'Object', value: { userId: 123, username: 'john_doe' } }

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_ops_ts_77712_1791438644484:cache:product:1: success
  kv_ops_ts_77712_1791438644484:cache:product:2: success
  kv_ops_ts_77712_1791438644484:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_ts_77712_1791438644484:cache:product:1: { name: 'Product 1', price: 29.99 }
  kv_ops_ts_77712_1791438644484:cache:product:2: { name: 'Product 2', price: 39.99 }
  kv_ops_ts_77712_1791438644484:cache:product:3: { price: 49.99, name: 'Product 3' }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  kv_ops_ts_77712_1791438644484:cache:product:1: deleted
  kv_ops_ts_77712_1791438644484:cache:product:2: deleted
  kv_ops_ts_77712_1791438644484:cache:product:3: deleted

✓ All KV operations completed successfully
=== KV Precision: Float vs Decimal ===

=== Test 1: Using JavaScript Floats (LOSES PRECISION) ===
✓ Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 2: Using Field.decimal() (PRESERVES PRECISION) ===
✓ Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.30000000000000004 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Cleanup ===

=== Summary ===
✅ Use Field.decimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ Field.decimal() stores values as strings internally,
   preserving exact precision across all operations.

=== Cleanup ===
✓ Cleaned up test keys
✓ Client created
✓ ts_route_admin → GET /api/route/users/admin
✓ ts_route_user_by_id → GET /api/route/users/:id
✓ ts_route_user_posts → GET /api/route/users/:id/posts/:post_id
✓ ts_route_org_create_member → POST /api/route/orgs/:org/members

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions
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
=== ekoDB Raw Completion Stream (SSE) Example ===

--- Basic SSE Raw Completion ---
Response: The three primary colors are red, blue, and yellow.

--- Structured Output via SSE ---
JSON response: [
  {
    "name": "Mercury",
    "diameter_km": 4879
  },
  {
    "name": "Venus",
    "diameter_km": 12104
  },
  {
    "name": "Saturn",
    "diameter_km": 116460
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===
=== ekoDB Schedule Management Example (TypeScript) ===

--- Creating schedule ---
Created schedule: Nightly Database Backup (id: 2b2b6bbe-466b-4bc8-842e-69856be32368, cron: 0 0 2 * * *)

--- Listing schedules ---
Schedules: {
  "count": 1,
  "schedules": [
    {
      "created_at": "2026-10-08T05:50:47.727746Z",
      "cron_expression": "0 0 2 * * *",
      "description": null,
      "enabled": true,
      "function_label": "schedule_noop_typescript_77731_1791438647694",
      "id": "2b2b6bbe-466b-4bc8-842e-69856be32368",
      "last_execution": null,
      "name": "Nightly Database Backup",
      "next_execution": "2026-10-09T02:00:00Z",
      "parameters": {},
      "stats": {
        "avg_execution_time_ms": 0,
        "failed_executions": 0,
        "last_error": null,
        "successful_executions": 0,
        "total_executions": 0
      },
      "timezone": "UTC",
      "updated_at": "2026-10-08T05:50:47.727746Z"
    }
  ]
}

--- Getting schedule ---
Fetched: Nightly Database Backup (cron: 0 0 2 * * *)

--- Updating schedule ---
Updated: Nightly Full Backup (new cron: 0 0 3 * * *)

--- Triggering schedule ---
Trigger response: {
  "schedule_id": "2b2b6bbe-466b-4bc8-842e-69856be32368",
  "status": "triggered"
}

--- Pausing schedule ---
Schedule enabled after pause: false

--- Resuming schedule ---
Schedule enabled after resume: true

--- Deleting schedule ---
Schedule deleted successfully

=== All schedule operations completed ===
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
=== Search Examples ===

Setting up test data...
✅ Test data created

1. Basic full-text search:
Found 2 results
  1. Score: 12.870, Matched: name, email
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: bio, title
  2. Score: 13.200, Matched: title, bio
  3. Score: 13.200, Matched: title, bio
  4. Score: 13.200, Matched: title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: bio, title
  2. Score: 26.400, Matched: bio, title
  3. Score: 26.400, Matched: title, bio
  4. Score: 26.400, Matched: title, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 6.600, Matched: bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.765, Matched:
  2. Score: 0.737, Matched:
  3. Score: 0.733, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.506, Matched: content, title
  2. Score: 0.895, Matched: title, content
  3. Score: 0.293, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: bio, skills

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Introduction to Machine Learning (category: ml)
  2. Deep Learning Fundamentals (category: ml)


✅ Search examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'qpxDwlWynU2V1RXgtYBQxpRfYgNCag0mYOjkFl4ocRXaEjnPf5lSaZh2oHMzZcT_uJTKjSnk9bx6qcIcs7ZgnA'
}

=== Find by ID ===
Found: {
  id: 'qpxDwlWynU2V1RXgtYBQxpRfYgNCag0mYOjkFl4ocRXaEjnPf5lSaZh2oHMzZcT_uJTKjSnk9bx6qcIcs7ZgnA',
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  value: { value: 42, type: 'Integer' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  created_at: { type: 'DateTime', value: '2026-10-08T05:50:48.225+00:00' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  name: { value: 'Test Record', type: 'String' },
  price: { type: 'Float', value: 99.99 },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  active: { value: true, type: 'Boolean' },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-10-08T05:50:48.225Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  id: 'qpxDwlWynU2V1RXgtYBQxpRfYgNCag0mYOjkFl4ocRXaEjnPf5lSaZh2oHMzZcT_uJTKjSnk9bx6qcIcs7ZgnA',
  data: 'aGVsbG8gd29ybGQ=',
  value: 42,
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  created_at: '2026-10-08T05:50:48.225+00:00',
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  name: 'Test Record',
  price: 99.99,
  categories: [ 'electronics', 'computers' ],
  active: true,
  metadata: { nested: { deep: true }, key: 'value' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  active: { value: true, type: 'Boolean' },
  created_at: { type: 'DateTime', value: '2026-10-08T05:50:48.225+00:00' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  name: { value: 'Updated Record', type: 'String' },
  id: 'qpxDwlWynU2V1RXgtYBQxpRfYgNCag0mYOjkFl4ocRXaEjnPf5lSaZh2oHMzZcT_uJTKjSnk9bx6qcIcs7ZgnA',
  value: { value: 100, type: 'Integer' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  price: { type: 'Float', value: 99.99 }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: BrunUyurA8ov0v5GDQlx6FbswsleqVTXeC-YqpH31EilIkywCtnnVmNgPeJSvpgGb2WhQbsKHr8TrZBEy7EauA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
🚀 ekoDB TypeScript Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


🧹 Cleaning up...
✓ Deleted 0 test scripts and owned SWR resources

Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native_ts (DKSbuidco99SRWawVi-t06tScHvvVtfQQqOgUDqsJqw3dAh9OLxsFIlG-_OKT8i-6GuP2IAqD9UjHvDDEKCXiQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 128ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 42.7x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_ts (wYTHiMpcOEE73o5-t9KcH1D1z-qM3bHr0oJNkuXZQLXt31rmPLY-fCGjEr9lsqidKpVCApFskyXzTtVkSTNDHw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_ts' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_ts (MYLyrPbsUxVhDjzOVRewtErBgtWiIwXlxqqnS430vk3m6tY8KIgTv0EdKa4WpCK-NeQdnegz8dZbxPlPtET3dw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_ts' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_ts (ufvE92pyHb7DxgQ7L3-NhO9Www0kahHc1xqq61SOEbokJve7ieNFdXP6bxx8LHJgFLU9y8cVL1UGjmKMc2Zyrw)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

================================================================================
✅ Key Benefits of Native SWR:
✅ Single function: Replaces 4-function cache-aside pattern
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)
Native SWR:     SWR → Insert (2 functions)
Result:         60% fewer functions, cleaner code, same behavior 🎯

🧹 Cleaning up...
✓ Deleted 4 test scripts and owned SWR resources

✅ All examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_ts_77743_1791438649451 (0mSUSERcVENQM061_TZ2LCt3Mdf6cSyL7twI6dTztUPJMV5CCvILUef1FkRGR_N6TRb34G8ya1ItwQotx1h8Pw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "address": {
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            }
          },
          "email": "Sincere@april.biz",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "id": 1,
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "website": "hildegard.org"
        },
        "type": "Object"
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 3ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "value": {
          "address": {
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            }
          },
          "email": "Sincere@april.biz",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "id": 1,
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "website": "hildegard.org"
        },
        "type": "Object"
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_reviews_ts_77743_1791438649451 (AXuz_Da6s-90JYjoKpxyuDRuLJjHiIYrD5lJfIC9IwoLKQIcHGSPyWi5LSeuVpk4KWWBqXuYZQb33jpYMOVW8Q)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "value": {
          "category": "beauty",
          "meta": {
            "updatedAt": "2026-05-23T11:27:41.868Z",
            "barcode": "5784719087687",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "createdAt": "2025-10-09T14:47:01.588Z"
          },
          "availabilityStatus": "In Stock",
          "sku": "BEA-ESS-ESS-001",
          "minimumOrderQuantity": 48,
          "shippingInformation": "Ships in 3-5 business days",
          "id": 1,
          "price": 9.99,
          "tags": [
            "beauty",
            "mascara"
          ],
          "title": "Essence Mascara Lash Princess",
          "dimensions": {
            "depth": 22.99,
            "width": 15.14,
            "height": 13.08
          },
          "warrantyInformation": "1 week warranty",
          "rating": 2.56,
          "discountPercentage": 10.48,
          "reviews": [
            {
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "rating": 3,
              "reviewerName": "Eleanor Collins",
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Would not recommend!"
            },
            {
              "rating": 4,
              "comment": "Very satisfied!",
              "reviewerName": "Lucas Gordon",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "date": "2025-04-30T09:41:02.053Z"
            },
            {
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "reviewerName": "Eleanor Collins",
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Highly impressed!",
              "rating": 5
            }
          ],
          "stock": 99,
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "weight": 4,
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "brand": "Essence",
          "returnPolicy": "No return policy",
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ]
        },
        "type": "Object"
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Multi-API data fetched, merged, and cached atomically

=== Why This Is Powerful ===

✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===

1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!

✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 5_za_a1k9HaNZ45G_Wq1-VkS1oZufr-C2OXURIThJzo3RMfjdk6gOxNkAsAJEVf1zSCB9oalnwGEQU1eKIessQ
Created Bob: $500 - ID: ScrMTld1ATE_9RX0Dy499OotcApFYIy3ino9nYAiv9tLQFFYXNRGF-TyT5LlBowesNChUQigZhvhRHZ0MPB_IA

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): d671d860-7e9d-42a0-98b9-abe30e850923

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

✓ Verified committed balances: Alice=$800, Bob=$700

=== Example 5: Rollback Demo ===
New transaction: 929b70ec-8f77-4a7a-bdf8-de50a1c9d41f
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: 5Ik6wrgvEv_JSJxeeQsa2UAVFa8Tg0KGl3r8YHkIhVnfOuuHgvryZu_-bg7I4_T0icyaGnUDAyIcp2qBNCIFNg

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 10 user functions:
  - get_active_users_client_js: Get Active Users (Updated)
  - get_active_users_ts: Get Active Users
  - fetch_product_reviews_ts_77743_1791438649451: Fetch Product with Reviews (Multi-API)
  - conc_demo_rl_skip_ts_75441_1791438514383: Rate-limit (skip mode)
  - get_active_users_client_ts_updated: Get Active Users (Updated)
  - conc_demo_rl_fail_ts_77654_1791438637251: Rate-limit (fail mode)
  - conc_demo_rl_skip_ts_77654_1791438637251: Rate-limit (skip mode)
  - conc_demo_rl_fail_ts_75441_1791438514383: Rate-limit (fail mode)
  - get_active_users_client_ts_updated: Get Active Users (Updated)
  - fetch_product_reviews_ts_76477_1791438537065: Fetch Product with Reviews (Multi-API)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (TypeScript) ===

Created chat session: zD0Rr3L9TArQBsaCIJF6LPB9BKGIMPULhzLrDVLeJWl4AqJ7tC2RheL0HxDG1xolsQZbWoftwzQdkyw4fXTIhw

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: oQOvHd7mTMt3UhCMx5xGfU5iSxYR_TQVA4q2KZ7kAd-UCkZ6cLuhtUQTGvmq-qmVkTo8B-DzSgW2FaaUPc8ZRg
Execution time: 696ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_32b76e21ca5d4c16b83824b4fea856cd)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: hQfRYEpz4J3Jaldp1uK2WDNRXdaZWmbpSQMJgHbiR7lNtWcxF0F4-l6WD4oYkDWbdJfaqIalY77j341PfHbOxQ
  📡 Notification received for hQfRYEpz4J3Jaldp1uK2WDNRXdaZWmbpSQMJgHbiR7lNtWcxF0F4-l6WD4oYkDWbdJfaqIalY77j341PfHbOxQ

Inserting another record...
✓ Inserted record: Gj0esN83-J6JtWD1ikH-ZPQXajQ7vcokzSr384lE9-wjahUMCVYUVMwp-hmIKOWNIoM1PtGatnB-DecW_IAj2g
  📡 Notification received for Gj0esN83-J6JtWD1ikH-ZPQXajQ7vcokzSr384lE9-wjahUMCVYUVMwp-hmIKOWNIoM1PtGatnB-DecW_IAj2g

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_ts'
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: z1Lw3XA19PRiI_XxouJUsekXw3NCdpnsHgFRKR1h74gyWdgYpJ1kDKWXE7u8dlcySZWjKR7_R2WMLZlHcWVRfg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: {"id":"4lsvTh8yhVGhZE54OlU2l2xQtH9CJdjgBFZmzya6zDLT8uQ0N05BOCsL70EM0wQS0qGk6ozeVj1ohS_KNvEGoQ"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"ylshgczaKtY1_GnqTOJ_obpljAbpK0ssOVNtsr0lPKYNEq1p7lLV-uMD6kvyU5Wxrv423bro50gLDnPbVBnUog"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"price":{"type":"Integer","value":150},"name":{"type":"String","value":"Product 1"},"id":"4lsvTh8yhVGhZE54OlU2l2xQtH9CJdjgBFZmzya6zDLT8uQ0N05BOCsL70EM0wQS0qGk6ozeVj1ohS_KNvEGoQ"}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"custom-id"}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["id","name","email"]
  First user: Alice Johnson <alice@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["id","status","created_at","user_role","age","bio","email","avatar_url","name"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - Dave Brown (age 45)
    - Alice Johnson (age 30)
    - Bob Smith (age 25)

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - Carol White: Manager

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["api_key","avatar_url","status","age","email","password","name","secret_token","created_at","id","user_role","bio"]
  Projected query:
    - 3 fields per record
    - Fields: ["id","name","email"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ All JavaScript integration tests complete!
🟣 Building Kotlin client library...
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build

> Configure project :
The com.github.ben-manes.versions plugin id is deprecated; apply io.github.ben-manes.versions instead.

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE
> Task :jar UP-TO-DATE
> Task :assemble UP-TO-DATE
> Task :loadKtlintReporters UP-TO-DATE
> Task :runKtlintCheckOverKotlinScripts UP-TO-DATE
> Task :ktlintKotlinScriptCheck UP-TO-DATE
> Task :runKtlintCheckOverMainSourceSet UP-TO-DATE
> Task :ktlintMainSourceSetCheck UP-TO-DATE
> Task :runKtlintCheckOverTestSourceSet UP-TO-DATE
> Task :ktlintTestSourceSetCheck UP-TO-DATE
> Task :compileTestKotlin UP-TO-DATE
> Task :compileTestJava NO-SOURCE
> Task :processTestResources NO-SOURCE
> Task :testClasses UP-TO-DATE
> Task :test UP-TO-DATE
> Task :check UP-TO-DATE
> Task :build UP-TO-DATE

[Incubating] Problems report is available at: file://ekoDB/ekodb-client/ekodb-client-kt/build/reports/problems/problems-report.html

Deprecated Gradle features were used in this build, making it incompatible with Gradle 10.

You can use '--warning-mode all' to show the individual deprecation warnings and determine if they come from your own scripts or plugins.

For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/command_line_interface.html#sec:command_line_warnings in the Gradle documentation.

BUILD SUCCESSFUL in 5s
11 actionable tasks: 11 up-to-date
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
✅ Kotlin client built!
=== Running Kotlin example: ClientAdvancedCrud.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Advanced CRUD Example ===

--- Inserting base record ---
Inserted: vrBrxgPUFS_-D5K-ZWAspEErgriOfeYTlnjp9wHfbSmE9Jvxk0XcgZmIcb7wxV7n3CXAi65UCy4RIgaIUJx0Vw

--- updateWithAction: increment views ---
After increment: EkoRecord(fields={score=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=100.0)}), views=ObjectValue(value={value=IntegerValue(value=15), type=StringValue(value=Integer)}), id=StringValue(value=vrBrxgPUFS_-D5K-ZWAspEErgriOfeYTlnjp9wHfbSmE9Jvxk0XcgZmIcb7wxV7n3CXAi65UCy4RIgaIUJx0Vw), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example)])}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Counter Record)})})

--- updateWithAction: push to tags ---
After push: EkoRecord(fields={tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example), StringValue(value=advanced)]), type=StringValue(value=Array)}), score=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=100.0)}), views=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=15)}), id=StringValue(value=vrBrxgPUFS_-D5K-ZWAspEErgriOfeYTlnjp9wHfbSmE9Jvxk0XcgZmIcb7wxV7n3CXAi65UCy4RIgaIUJx0Vw), name=ObjectValue(value={value=StringValue(value=Counter Record), type=StringValue(value=String)})})

--- updateWithActionSequence: multiple operations ---
After sequence: EkoRecord(fields={tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example), StringValue(value=advanced), StringValue(value=sequenced)])}), id=StringValue(value=vrBrxgPUFS_-D5K-ZWAspEErgriOfeYTlnjp9wHfbSmE9Jvxk0XcgZmIcb7wxV7n3CXAi65UCy4RIgaIUJx0Vw), views=ObjectValue(value={value=IntegerValue(value=115), type=StringValue(value=Integer)}), score=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=75.0)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Counter Record)})})

--- Cleanup ---
Deleted collection: kotlin_advanced_crud_example

=== Example Complete ===

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientBatchOperations.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Batch Operations Example ===

=== Batch Insert ===
✓ Inserted 5 records
  IDs: wO9J61J6wruJ39zJzCX_Kb7-Sp6iZh5K_mb4qait11IM1uCOlEWarMSZF5qlIhvanOZpQm2Ehb1bLvxF2ElhqA, TjZcIYYw1b7VE1vTQHohAZRtpW8pvuP5RnlUqMKPbaaP-ajaTY6l2rhpabL4NjwZz2N6pLI18E9mB9KMzXtZPA, TGGRgLlBUUdLBOQDsOQQVLCwd0QWC0CQJDM-NPqG0YKzAqHuCVrINlUqXOXQ-hYydtQdIJOSWrkbmJoQ61g2ZA...

=== Batch Update ===
✓ Updated 3 records; 0 failed

=== Batch Delete ===
✓ Deleted 2 records

=== Cleanup ===
✓ Deleted collection: kotlin_batch_example

=== Example Complete ===

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientChatAdvanced.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: iBlD0taGq0KqRxsEQ7N72pDLhRQLvFiip8Hl9bDH0Q4USte5S52iq1gdMWqhq8sW4nXk1r6J-nUyzcVY-WfqWg

=== Sending Initial Message ===
✓ Message sent
  Responses: ["There are currently no products available in the database. Would you like to view any specific details or perform another action?"]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["The price of ekoDB is $99. If you have any more questions or need further information, feel free to ask!"]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: 3UYlaMt6k_qBIu1epL8hrteVR2xP82sEnUFmybbHcPkksxt-cOLXmqHtotoyqd4_ZHtJdY5nKTE6venn4Zkctw

=== Merging Sessions ===
✓ Merged sessions
  Total messages in merged session: 5

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat session: 3UYlaMt6k_qBIu1epL8hrteVR2xP82sEnUFmybbHcPkksxt-cOLXmqHtotoyqd4_ZHtJdY5nKTE6venn4Zkctw
✓ Deleted chat session: iBlD0taGq0KqRxsEQ7N72pDLhRQLvFiip8Hl9bDH0Q4USte5S52iq1gdMWqhq8sW4nXk1r6J-nUyzcVY-WfqWg
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 13s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientChatBasic.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: dmiYdungzHq75hVw0WcljGO0vcoe1t8ZMECHIaRR9MA1IyfBR02ApnBxPNuMFWxYcUSnzKBheJJtVm9EZqp8tQ

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "FAbw5nn8fWTx-hsvDXzDSw3XqCbeWkqUCgsw7k1I-oymaAqUMPIaMn9_0XelZKI59297UtZeQ7XSOX_KuJWZnA"
  Responses: ["ekoDB is a high-performance database that integrates AI capabilities and supports various advanced features. Here are some key features of ekoDB:\n\n1. **AI Chat Integration**: The chat feature allows users to query the database using natural language, providing AI-powered responses with relevant context.\n\n2. **Search Capabilities**: ekoDB supports:\n   - **Full-text search**: Allows for keyword-based searches across text fields.\n   - **Vector search**: Utilizes embeddings for semantic searching.\n   - **Hybrid search**: Combines both full-text and vector search capabilities with automatic context retrieval.\n\n3. **Intelligent Caching**: It has intelligent caching mechanisms designed for performance optimization, ensuring rapid data access.\n\n4. **Real-time Capabilities**: ekoDB provides real-time data processing and availability, making it suitable for applications requiring immediate data responses.\n\nThese features position ekoDB as a versatile and advanced database solution."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 11s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientChatMessageStream.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
=== ekoDB Chat Message Stream (SSE) Example (Kotlin) ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
Created session: VjNAhnX6ijFjjI89Pa_PbVjboxZldrjIvxa-T-hHpYnR9JEDF9bpZLgLczs4Ca3iYk_aNEVfzr7S9M4hjUyANg

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}As of my last knowledge update in October 2023, ekoDB refers to an operational database designed for high-performance applications, especially tailored for use in the fields of finance, telecommunications, and other industries requiring real-time analytics and data processing.

Some key features of ekoDB often include:

1. **Performance**: ekoDB is built to handle large volumes of transactions with low latency, making it suitable for real-time applications.
2. **Scalability**: The architecture generally supports horizontal scaling, allowing it to manage increased loads effectively.
3. **Consistency and Reliability**: It may include features that ensure data consistency and reliability, essential for mission-critical applications.
4. **Flexibility**: The database typically supports various data models, allowing for the integration of different types of data structures.

Please note that specific details and features may vary, and for the latest information, you may want to check the official documentation or announcements related to ekoDB, as new developments could have occurred after my last update.

--- Stream complete ---
Message ID: Rzx7QZ_Gc84P86EfThn_yOJhwAfcbOOef2OxOm9jy1sPC6T9b3YAXS3Qi-I0dJwTpSYl9W2XoCQiQv_C5ZIpqQ
Execution time: 2531ms
Context window: 128000 tokens

✓ Chat message stream example completed

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientChatModels.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Chat Models Example ===

=== List Chat Models ===
Available chat models by provider:
  openai:
    - o3-2025-04-16
    - gpt-5.1-chat-latest
    - gpt-5.3-chat-latest
    - gpt-5.2-2025-12-11
    - sora-2
    - chatgpt-image-latest
    - gpt-4.1-mini
    - gpt-3.5-turbo
    - gpt-4o-mini
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-6-luna
    - gpt-5.4-mini-2026-03-17
    - gpt-4-turbo
    - gpt-4.1-nano
    - gpt-image-2
    - gpt-5.4-pro
    - gpt-realtime-mini-2025-12-15
    - gpt-3.5-turbo-16k
    - babbage-002
    - text-embedding-ada-002
    - gpt-4o-mini-tts
    - gpt-4o-2024-08-06
    - o4-mini-2025-04-16
    - gpt-4o-2024-11-20
    - omni-moderation-latest
    - gpt-realtime
    - tts-1-hd
    - gpt-realtime-1.5
    - gpt-realtime-2
    - gpt-5-search-api
    - gpt-5.5-2026-04-23
    - gpt-5.1
    - gpt-5.1-codex
    - gpt-4o
    - o1-2024-12-17
    - gpt-5.2-chat-latest
    - gpt-4o-mini-tts-2025-12-15
    - gpt-image-1
    - davinci-002
    - gpt-image-2-2026-04-21
    - gpt-4o-mini-transcribe
    - o3-mini
    - gpt-audio
    - gpt-5.4-nano
    - o3
    - gpt-5.1-codex-max
    - gpt-realtime-2.1
    - gpt-4.1
    - gpt-5.5
    - gpt-4o-mini-search-preview-2025-03-11
    - text-embedding-3-large
    - gpt-4o-mini-search-preview
    - gpt-4o-2024-05-13
    - gpt-6-sol
    - gpt-5.3-codex
    - gpt-3.5-turbo-instruct
    - sora-2-pro
    - gpt-5
    - gpt-audio-mini-2025-12-15
    - gpt-transcribe
    - o4-mini-deep-research-2025-06-26
    - o1-pro-2025-03-19
    - gpt-4.1-nano-2025-04-14
    - gpt-5-search-api-2025-10-14
    - gpt-4-turbo-2024-04-09
    - gpt-realtime-2.1-mini
    - tts-1
    - gpt-5-mini
    - omni-moderation-2024-09-26
    - gpt-5-mini-2025-08-07
    - gpt-realtime-2025-08-28
    - gpt-5.4-2026-03-05
    - gpt-live-transcribe
    - gpt-3.5-turbo-0125
    - gpt-5-nano
    - gpt-5.6-luna
    - gpt-4
    - gpt-image-2.5-sunburst-2026-09-08
    - whisper-1
    - gpt-5.4-mini
    - gpt-5-pro
    - o1-pro
    - gpt-realtime-whisper
    - gpt-5.2-pro
    - gpt-5-chat-latest
    - gpt-live-1
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-5-2025-08-07
    - tts-1-1106
    - gpt-5.5-pro
    - gpt-5-pro-2025-10-06
    - chat-latest
    - gpt-5.2-pro-2025-12-11
    - gpt-4o-mini-transcribe-2025-03-20
    - gpt-audio-2025-08-28
    - gpt-5.4-pro-2026-03-05
    - gpt-realtime-translate
    - gpt-5.6-sol
    - gpt-audio-mini-2025-10-06
    - gpt-5.5-pro-2026-04-23
    - tts-1-hd-1106
    - gpt-realtime-mini
    - gpt-5.4
    - gpt-4o-transcribe-diarize
    - gpt-5-codex
    - gpt-image-1-mini
    - gpt-image-1.5
    - gpt-5.1-2025-11-13
    - o4-mini-deep-research
    - gpt-4o-search-preview-2025-03-11
    - o3-mini-2025-01-31
    - gpt-4-0613
    - gpt-4o-search-preview
    - gpt-5.6-terra
    - text-embedding-3-small
    - gpt-audio-1.5
    - gpt-3.5-turbo-instruct-0914
    - gpt-6-astra
    - gpt-5.2-codex
    - gpt-3.5-turbo-1106
    - gpt-5-nano-2025-08-07
    - gpt-audio-mini
    - gpt-4o-mini-tts-2025-03-20
    - gpt-5.2
    - gpt-4.1-mini-2025-04-14
    - gpt-4.1-2025-04-14
    - gpt-4o-mini-2024-07-18
    - o4-mini
    - o1
    - gpt-5.4-nano-2026-03-17
    - gpt-5.1-codex-mini
    - gpt-4o-transcribe
    - gpt-image-2.5-flare
    - gpt-6.1-sol
  anthropic:
    - claude-haiku-5-5
    - claude-sonnet-5-5
    - claude-opus-5-5
    - claude-fable-5-1
    - claude-opus-5
    - claude-sonnet-5
    - claude-fable-5
    - claude-opus-4-8
    - claude-opus-4-7
    - claude-sonnet-4-6
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
Provider status:
  anthropic: ok 14 models
  gemini: not_configured (unverified) No Gemini API Key
  openai: ok 135 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get OpenAI Models ===
OpenAI models: o3-2025-04-16, gpt-5.1-chat-latest, gpt-5.3-chat-latest, gpt-5.2-2025-12-11, sora-2, chatgpt-image-latest, gpt-4.1-mini, gpt-3.5-turbo, gpt-4o-mini, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-6-luna, gpt-5.4-mini-2026-03-17, gpt-4-turbo, gpt-4.1-nano, gpt-image-2, gpt-5.4-pro, gpt-realtime-mini-2025-12-15, gpt-3.5-turbo-16k, babbage-002, text-embedding-ada-002, gpt-4o-mini-tts, gpt-4o-2024-08-06, o4-mini-2025-04-16, gpt-4o-2024-11-20, omni-moderation-latest, gpt-realtime, tts-1-hd, gpt-realtime-1.5, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, gpt-5.1, gpt-5.1-codex, gpt-4o, o1-2024-12-17, gpt-5.2-chat-latest, gpt-4o-mini-tts-2025-12-15, gpt-image-1, davinci-002, gpt-image-2-2026-04-21, gpt-4o-mini-transcribe, o3-mini, gpt-audio, gpt-5.4-nano, o3, gpt-5.1-codex-max, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-4o-mini-search-preview-2025-03-11, text-embedding-3-large, gpt-4o-mini-search-preview, gpt-4o-2024-05-13, gpt-6-sol, gpt-5.3-codex, gpt-3.5-turbo-instruct, sora-2-pro, gpt-5, gpt-audio-mini-2025-12-15, gpt-transcribe, o4-mini-deep-research-2025-06-26, o1-pro-2025-03-19, gpt-4.1-nano-2025-04-14, gpt-5-search-api-2025-10-14, gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, tts-1, gpt-5-mini, omni-moderation-2024-09-26, gpt-5-mini-2025-08-07, gpt-realtime-2025-08-28, gpt-5.4-2026-03-05, gpt-live-transcribe, gpt-3.5-turbo-0125, gpt-5-nano, gpt-5.6-luna, gpt-4, gpt-image-2.5-sunburst-2026-09-08, whisper-1, gpt-5.4-mini, gpt-5-pro, o1-pro, gpt-realtime-whisper, gpt-5.2-pro, gpt-5-chat-latest, gpt-live-1, gpt-4o-mini-transcribe-2025-12-15, gpt-5-2025-08-07, tts-1-1106, gpt-5.5-pro, gpt-5-pro-2025-10-06, chat-latest, gpt-5.2-pro-2025-12-11, gpt-4o-mini-transcribe-2025-03-20, gpt-audio-2025-08-28, gpt-5.4-pro-2026-03-05, gpt-realtime-translate, gpt-5.6-sol, gpt-audio-mini-2025-10-06, gpt-5.5-pro-2026-04-23, tts-1-hd-1106, gpt-realtime-mini, gpt-5.4, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-image-1-mini, gpt-image-1.5, gpt-5.1-2025-11-13, o4-mini-deep-research, gpt-4o-search-preview-2025-03-11, o3-mini-2025-01-31, gpt-4-0613, gpt-4o-search-preview, gpt-5.6-terra, text-embedding-3-small, gpt-audio-1.5, gpt-3.5-turbo-instruct-0914, gpt-6-astra, gpt-5.2-codex, gpt-3.5-turbo-1106, gpt-5-nano-2025-08-07, gpt-audio-mini, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-4.1-mini-2025-04-14, gpt-4.1-2025-04-14, gpt-4o-mini-2024-07-18, o4-mini, o1, gpt-5.4-nano-2026-03-17, gpt-5.1-codex-mini, gpt-4o-transcribe, gpt-image-2.5-flare, gpt-6.1-sol

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

=== Chat Models Example Complete ===

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientChatSessions.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: fQd9wjIJsFCce33XS1YmodWmYxL4TKhvNoa_Om66Lu3OvoNJ_dkMGsVAwX4UkOWOwFwTpdEJwiKbxdKN7hkrJQ

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The available product is:\n\n- **Product**: ekoDB\n  - **Description**: A high-performance database product with AI capabilities\n  - **Price**: $99\n\nIf you need more information or have additional questions, let me know!"]

✓ Message 2 sent
  Responses: ["The price of ekoDB is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"fQd9wjIJsFCce33XS1YmodWmYxL4TKhvNoa_Om66Lu3OvoNJ_dkMGsVAwX4UkOWOwFwTpdEJwiKbxdKN7hkrJQ"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"SwDVAeIaa4Ty1JQbtK9VWce1LPJMw8-eiKjHnKU4E7LCkMGKUJc1FRPZK4so8S1G_Tpl6Rqan1X02NwYWA4C4A","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-10-08T05:52:06.915925+00:00"},"id":"eS5Ns6neItlobS3jpjmJEmWhAC0cRnDWD1x5t6RoWa09JDEnZUq0rbDz3zZ0USdjjCBNkzzoOJ07o0ehHx6RvQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":88,"prompt_tokens":6740,"total_tokens":6828}},"updated_at":{"type":"DateTime","value":"2026-10-08T05:52:06.915925+00:00"}},{"chat_id":{"type":"String","value":"fQd9wjIJsFCce33XS1YmodWmYxL4TKhvNoa_Om66Lu3OvoNJ_dkMGsVAwX4UkOWOwFwTpdEJwiKbxdKN7hkrJQ"},"content":{"type":"String","value":"The available product is:\n\n- **Product**: ekoDB\n  - **Description**: A high-performance database product with AI capabilities\n  - **Price**: $99\n\nIf you need more information or have additional questions, let me know!"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"SwDVAeIaa4Ty1JQbtK9VWce1LPJMw8-eiKjHnKU4E7LCkMGKUJc1FRPZK4so8S1G_Tpl6Rqan1X02NwYWA4C4A","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-10-08T05:52:06.931385+00:00"},"id":"VX-Za75-TOEEZbjsIzoXuQ6TaosKvQ4zYcG6TizzgLPwyWrtasqp32Y6bHjb6CZV9MzP_7RM_r613Sji-7jMLg","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":88,"prompt_tokens":6740,"total_tokens":6828}},"tool_call_count":{"type":"Number","value":2},"tool_call_history":{"type":"Object","value":{"iterations":2,"tool_calls":[{"arguments":{"collection":"kotlin_chat_sessions_example","filter":{"content":{"field":"product","operator":"In","value":["ekoDB"]},"type":"Condition"}},"id":"call_TU8oOq26LVNIQFzCPxLoGjk8","name":"query_collection"}],"tool_results":[{"error":null,"result":{"count":1,"records":[{"description":"A high-performance database product with AI capabilities","id":"SwDVAeIaa4Ty1JQbtK9VWce1LPJMw8-eiKjHnKU4E7LCkMGKUJc1FRPZK4so8S1G_Tpl6Rqan1X02NwYWA4C4A","price":99,"product":"ekoDB"}]},"success":true,"tool_call_id":"call_TU8oOq26LVNIQFzCPxLoGjk8","tool_name":"query_collection"}]}},"updated_at":{"type":"DateTime","value":"2026-10-08T05:52:06.931385+00:00"}},{"chat_id":{"type":"String","value":"fQd9wjIJsFCce33XS1YmodWmYxL4TKhvNoa_Om66Lu3OvoNJ_dkMGsVAwX4UkOWOwFwTpdEJwiKbxdKN7hkrJQ"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"SwDVAeIaa4Ty1JQbtK9VWce1LPJMw8-eiKjHnKU4E7LCkMGKUJc1FRPZK4so8S1G_Tpl6Rqan1X02NwYWA4C4A","price":99,"product":"ekoDB"},"score":2.102564102564102}]},"created_at":{"type":"DateTime","value":"2026-10-08T05:52:08.107920+00:00"},"id":"1jGFYLD3iuMpYZKL1U8MqSWjXy61210FpmVIs_5KzpdcMWYKSDQQu2-BODhKFZ39MrnK1ZGoNdeskWUXmQBE7Q","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":6,"prompt_tokens":3386,"total_tokens":3392}},"updated_at":{"type":"DateTime","value":"2026-10-08T05:52:08.107920+00:00"}},{"chat_id":{"type":"String","value":"fQd9wjIJsFCce33XS1YmodWmYxL4TKhvNoa_Om66Lu3OvoNJ_dkMGsVAwX4UkOWOwFwTpdEJwiKbxdKN7hkrJQ"},"content":{"type":"String","value":"The price of ekoDB is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"SwDVAeIaa4Ty1JQbtK9VWce1LPJMw8-eiKjHnKU4E7LCkMGKUJc1FRPZK4so8S1G_Tpl6Rqan1X02NwYWA4C4A","price":99,"product":"ekoDB"},"score":2.102564102564102}]},"created_at":{"type":"DateTime","value":"2026-10-08T05:52:08.124166+00:00"},"id":"D45nsYKILbSi5DF3oc-pBjuQrtqt88aMmJSoxSAfcrghrqDHraZWdwtHpI0Le8l3Xm0aTDRKvuWNSDCeJNhBmA","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":6,"prompt_tokens":3386,"total_tokens":3392}},"updated_at":{"type":"DateTime","value":"2026-10-08T05:52:08.124166+00:00"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 1

=== Branching Session ===
✓ Created branched session: iNDiPkzi9k1KYTuh1a8DUt_22V9XfR3-VXoH50QW53JhVUok7KIB96c8BXMYnu3wfZm5B7qkI5wZQzyGOt0hdA

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 12s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientCollectionManagement.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Collection Management Example ===

=== List Collections ===
✓ Found 26 collections
  - chat_agent_configs__ek0_testing
  - chat_goal_templates__ek0_testing
  - schema_documents_client_js
  - schema_documents_client_go
  - test_accounts

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-10-08T05:52:15.622899Z","last_modified":"2026-10-08T05:52:15.622900Z","bypass_ripple":false,"primary_key_alias":"id"}

=== Cleanup ===
✓ Deleted collection: kotlin_collection_example

=== Example Complete ===

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientCollectionUtils.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Collection Utilities Example ===

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test_kt' exists: false

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test_kt' exists: true

=== Count Documents ===
Document count in 'collection_utils_test_kt': 5

=== Get Collection Metadata ===
Collection metadata: {"analytics":["collection_utils_test_kt",{"manifest_backed":false,"record_count":5,"resident_record_ids":5,"total_size":675}],"collection":{"bypass_ripple":false,"created_at":"2026-10-08T05:52:23.162507Z","fields":{"index":{"default":null,"enums":[],"field_type":"Integer","max":null,"min":null,"regex":null,"required":false,"unique":false},"name":{"default":null,"enums":[],"field_type":"String","max":null,"min":null,"regex":null,"required":false,"unique":false}},"last_modified":"2026-10-08T05:52:23.162836Z","primary_key_alias":"id","version":1}}

=== List Collections ===
All collections (27):
  - chat_agent_configs__ek0_testing
  - chat_goal_templates__ek0_testing
  - schema_documents_client_js
  - schema_documents_client_go
  - test_accounts
  - audit__ek0_testing
  - chat_tasks__ek0_testing
  - schema_products_client_js
  - agent_function_versions__ek0_testing
  - chat_configurations__ek0_testing
  - chat_messages__ek0_testing
  - chat_raw_completions__ek0_testing
  - collection_utils_test_kt
  - chat_turns__ek0_testing
  - chat_goals__ek0_testing
  - schema_documents_client_ts
  - functions__ek0_testing
  - schema_users_client_ts
  - schema_employees_client_ts
  - schema_users_client_go
  - test_collection
  - schema_users_client_js
  - schema_products_client_ts
  - schema_products_client_go
  - schedules__ek0_testing
  - schema_employees_client_go
  - schema_employees_client_js

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz_1061538969312958' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_kt'

=== Collection Utilities Example Complete ===

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientConcurrencyStages.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created
✓ conc_demo_pay saved
✓ conc_demo_rl_fail saved
✓ conc_demo_rl_skip saved
✓ conc_demo_lock saved

Invoke them like:
  POST /api/functions/conc_demo_pay        { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail    { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip    { "user_id": 42 }
  POST /api/functions/conc_demo_lock       { "resource": "queue:drain" }

✓ Cleaned up demo functions

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientConvenienceMethods.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
=== ekoDB Convenience Methods Example ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== Native Object Creation ===
✓ Created record: EkoRecord(fields={id=StringValue(value=VuqXyYhWhzju-thkatVhaScDUP9LyI7kaOfQpSmF-8rafXv_Jpk4CFYrSPdeGDj0ezujmTvMHYM0Bv8Fi5baiQ)})

=== Upsert Operation ===
✓ First upsert (update): EkoRecord(fields={id=StringValue(value=VuqXyYhWhzju-thkatVhaScDUP9LyI7kaOfQpSmF-8rafXv_Jpk4CFYrSPdeGDj0ezujmTvMHYM0Bv8Fi5baiQ), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice.j@newdomain.com)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)})})
✓ Second upsert (insert): EkoRecord(fields={id=StringValue(value=new-user-id)})

=== Find One Operation ===
✓ Found user by email: EkoRecord(fields={active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), id=StringValue(value=VuqXyYhWhzju-thkatVhaScDUP9LyI7kaOfQpSmF-8rafXv_Jpk4CFYrSPdeGDj0ezujmTvMHYM0Bv8Fi5baiQ), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice.j@newdomain.com)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)})})
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: true
✓ Fake record exists: false (should be false)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 7 records (expected ~7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientCryptoStages.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created
✓ crypto_demo_hmac_kt saved
✓ crypto_demo_aes_kt saved
✓ crypto_demo_uuid_kt saved
✓ crypto_demo_totp_kt saved
✓ crypto_demo_encoding_kt saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac_kt { "payload": "hi" }
  POST /api/functions/crypto_demo_aes_kt { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid_kt
  POST /api/functions/crypto_demo_totp_kt
  POST /api/functions/crypto_demo_encoding_kt { "title": "Héllo World" }

✓ Cleaned up demo functions

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientDistinctValues.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
Kotlin distinct-values example passed

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientDocumentTtl.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Document TTL Example ===

=== Insert with TTL ===
✓ Inserted document with 10s TTL
  Document ID: TqE-Kar9Cov3cmP7JNBhWM7emBk5lrVoHVNIKJM26lBCZirualPxmVESc_3opdu3TAC1undctAdtltmgM_-0VQ

=== Verify Document Exists ===
✓ Document found: session_id, user_id, created_at, id, ttl

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: osHgVrlOwIyozl3Hu3J0cNsONZn0VdSFZ5xA80ETwCD6q88JTieCgXdXQ1QLffz7aGG5r2gEfMojf1d2xnG7kA

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: value, id, cache_key, ttl

=== Delete Document ===
✓ Deleted document

=== Cleanup ===
✓ Deleted collection: kotlin_ttl_example

=== Example Complete ===

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientEdgeCache.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup function...
✓ Edge cache function created: MONiXQlbeqo6JZ55fbrzv_GPChTEHOu4-2x1pIwadkaozoZtpEHR1Wr5GTzMLs7smYHV0n_zkGNexWvt_cMh2A

Call 1: Cache lookup
Found 1 cached entries
Response time: 16ms

Call 2: Cache lookup (connection warm)
Found 1 cached entries
Response time: 4ms

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

🧹 Cleaning up...
✓ Cleanup complete

✓ Example complete!

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionComposition.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"value":"User 1","type":"String"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 118ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 5ms
   📊 Records: 1
   🚀 Cache speedup: 23.6x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow

🧹 Cleaning up...
✅ Cleanup complete

✅ All composition examples completed!

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionContract.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
client_function_contract: ok

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctions.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
🚀 ekoDB Functions Example (Kotlin Client)

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: dZzX3yCjAd9_GdroT6xTbcrdCxwl8c71YzayM7jCqppQqwCfIjGIvvHhnzIX5hD5XRPlqTv5YC1lqnURHjPQzw
📊 Found 5 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 3 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"count":{"value":5,"type":"Integer"},"status":{"value":"active","type":"String"},"avg_score":{"value":60.0,"type":"Float"}}
   {"count":{"value":5,"type":"Integer"},"avg_score":{"value":50.0,"type":"Float"},"status":{"type":"String","value":"inactive"}}
⏱️  Execution time: 0ms

📝 Example 4: function Management

📋 Total functions: 12
🔍 Retrieved function: Get Active Users
✏️  function updated
🗑️  function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage function saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count function saved
📊 Total user count: {"value":10,"type":"Integer"}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted test functions
✅ Deleted collection

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • Built-in error handling

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsAdvanced.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
🚀 ekoDB Kotlin Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   {"avg_price":{"value":367.0,"type":"Float"},"category":{"type":"String","value":"Electronics"},"count":{"value":5,"type":"Integer"}}
   {"count":{"type":"Integer","value":3},"avg_price":{"type":"Float","value":365.6666666666667},"category":{"type":"String","value":"Furniture"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced function examples finished!

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsAi.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
🚀 ekoDB Kotlin AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat function saved
🤖 AI Response:
   Vector databases offer several benefits:

1. **Efficiency in Similarity Searches**: They provide fast proximity searches for high-dimensional vectors, making it easier to find similar items.

2. **Support for Unstructured Data**: Ideal for handling unstructured data like text, images, and audio by representing them as vectors.

3. **Scalability**: Capable of managing large datasets with efficient indexing and retrieval.

4. **Enhanced Machine Learning Integration**: Seamlessly supports machine learning models, particularly in recommendation systems and natural language processing.

5. **Multimodal Data Handling**: Can work with diverse data types, allowing for richer and more complex queries.

6. **Real-Time Processing**: Enables real-time analytics and insights, crucial for applications like chatbots and recommendation engines.

7. **Flexibility in Metrics**: Supports various distance metrics (e.g., Euclidean, cosine) for different use cases.

8. **Improved Performance**: Optimized for vector operations, leading to better performance compared to traditional databases for specific tasks.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed function saved
📊 Generated 2 embeddings
   Dimensions: 1536
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI function examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text

BUILD SUCCESSFUL in 10s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsComplete.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
🚀 ekoDB Kotlin Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: ljN0KADyaL5r8Iu61uV75Tl3AicXy0XIo3F-tVRBSq62b93J4bxHeDj6BohlcoAQXEeyVnxHeO6f5RBa_lFhGw
📊 Found 2 category groups

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories

📝 Example 4: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed with 1 results

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete function examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Function management (save, call, delete)

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsCrud.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
🚀 ekoDB Kotlin CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   {"status":{"type":"String","value":"inactive"},"count":{"value":3,"type":"Integer"}}
   {"status":{"value":"active","type":"String"},"count":{"type":"Integer","value":7}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD function examples finished!

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsKvWrapped.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
🚀 ekoDB Kotlin KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within functions
   • Combined wrapped types + KV workflows

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: StringValue(value=d9qdAwRCDLeBTABx46z90wZrJwpwersaZQUpQThXAbNh6MwpKcl4Fuyk0xcMOYD1odBCWqVNwszbeXmAty65bg)
✅ Inserted 2 products

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: vy1RnVh7MDu-omZc_8wFoZ-Q4u7CSBHe9DtmVgEJg1uDijr26X0wRDhSqGBkd0sosst9jYtFVz0DKA29KZ0Iqw
📊 Created order via function
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"userId":"user_abc","role":"admin"}}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: O2m4kbOO-T7Pjnitj4hCDyPz-iODFt9uBlURbiP4QwfRvsxb_p473OI3aqh22Ua-EpNpuv3WD6XwHBBWkIb9zA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: kAML10ECePLZKr3AigrX1Rx1BN-UXoqV9Gr5VV0SRz87Ni1Hc8gVd6fsuI8_cgqKK5jDhAW9Sah6-60kmoIHfQ
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use field* helpers for type-safe wrapped values
   ✅ fieldDecimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ FunctionStageConfig.Kv* classes work within functions

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsSearch.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
🚀 ekoDB Kotlin Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. {"type":"String","value":"Introduction to Machine Learning"} ({"value":"AI","type":"String"})
   2. {"value":"Natural Language Processing","type":"String"} ({"type":"String","value":"AI"})
   3. {"type":"String","value":"Database Design Principles"} ({"value":"Database","type":"String"})
   4. {"type":"String","value":"Vector Databases Explained"} ({"type":"String","value":"Database"})
   5. {"value":"Getting Started with ekoDB","type":"String"} ({"type":"String","value":"Database"})
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"count":{"type":"Integer","value":2},"category":{"value":"AI","type":"String"}}
   {"category":{"value":"Database","type":"String"},"count":{"value":3,"type":"Integer"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search function examples finished!

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientGoalTemplates.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
=== ekoDB Goal Template CRUD Example (Kotlin) ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
--- Creating goal template ---
Created template: Data Migration (id: UwD84QaK8P6tuZOPCBSyRq89-bm8hfFXfhT1EOpGvTKAgPkXoOL8SDtTvk4PGD2DiQQBWffODyCgJ9ER6ccc8w)

--- Listing templates ---
Templates: {"count":1,"items":[{"description":{"type":"String","value":"Template for migrating data between schemas"},"id":"UwD84QaK8P6tuZOPCBSyRq89-bm8hfFXfhT1EOpGvTKAgPkXoOL8SDtTvk4PGD2DiQQBWffODyCgJ9ER6ccc8w","steps":{"type":"Array","value":[{"description":"Analyze source schema"},{"description":"Create target schema"},{"description":"Migrate records"},{"description":"Validate results"}]},"title":{"type":"String","value":"Data Migration"}}]}

--- Getting template ---
Fetched: {"type":"String","value":"Data Migration"}

--- Updating template ---
Updated description: {"type":"String","value":"Updated: comprehensive data migration workflow"}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientGoalsTasksAgents.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Goals, Tasks & Agents Example ===

--- Creating goal ---
Created goal: EZL--04k2WuqI3Fpj11evKnVjtYJkSPJXQuLGkVc6hWt5AuewshxXtRop1KDSgBjf74HU5rOUORAhx_PYEX8XQ

--- Listing goals ---
Goals: {"count":1,"goals":[{"created_at":"2026-10-08T05:54:42.091438+00:00","description":"Ship the next major release","id":"EZL--04k2WuqI3Fpj11evKnVjtYJkSPJXQuLGkVc6hWt5AuewshxXtRop1KDSgBjf74HU5rOUORAhx_PYEX8XQ","status":"pending","steps":"[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to production\"}]","title":"Deploy v2.0","updated_at":"2026-10-08T05:54:42.091438+00:00"}]}

--- Getting goal ---
Fetched: {"type":"String","value":"Deploy v2.0"}

--- Updating goal ---
Updated description: {"type":"String","value":"Ship v2.0 with full test coverage"}

--- Searching goals ---
Search results: {"count":1,"items":[{"_score":12.870000000000001,"created_at":{"type":"DateTime","value":"2026-10-08T05:54:42.091438+00:00"},"description":{"type":"String","value":"Ship v2.0 with full test coverage"},"id":"EZL--04k2WuqI3Fpj11evKnVjtYJkSPJXQuLGkVc6hWt5AuewshxXtRop1KDSgBjf74HU5rOUORAhx_PYEX8XQ","status":{"type":"String","value":"pending"},"steps":{"type":"String","value":"[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0"},"updated_at":{"type":"DateTime","value":"2026-10-08T05:54:42.118994+00:00"}}]}

--- Goal step lifecycle ---
Step 0 started: "EZL--04k2WuqI3Fpj11evKnVjtYJkSPJXQuLGkVc6hWt5AuewshxXtRop1KDSgBjf74HU5rOUORAhx_PYEX8XQ"
Step 0 completed: "EZL--04k2WuqI3Fpj11evKnVjtYJkSPJXQuLGkVc6hWt5AuewshxXtRop1KDSgBjf74HU5rOUORAhx_PYEX8XQ"
Step 1 started: "EZL--04k2WuqI3Fpj11evKnVjtYJkSPJXQuLGkVc6hWt5AuewshxXtRop1KDSgBjf74HU5rOUORAhx_PYEX8XQ"
Step 1 failed: "EZL--04k2WuqI3Fpj11evKnVjtYJkSPJXQuLGkVc6hWt5AuewshxXtRop1KDSgBjf74HU5rOUORAhx_PYEX8XQ"

--- Completing goal ---
Goal status after complete: {"type":"String","value":"pending_review"}

--- Approving goal ---
Goal status after approve: {"type":"String","value":"in_progress"}

--- Creating goal for rejection ---
Goal status after reject: {"type":"String","value":"failed"}

--- Creating task ---
Created task: Rpj2i7Z_HxZc1PUTWYCflKpu6NwNAiMuwFDWAXQWw9hQSpxvqRJ1To07wrm_EkDXOxEBcdapT2Hyxs0r2kRfEw

--- Listing tasks ---
Tasks: {"count":1,"items":[{"cron":{"type":"String","value":"0 2 * * *"},"description":{"type":"String","value":"Full database backup every night at 2 AM"},"id":"Rpj2i7Z_HxZc1PUTWYCflKpu6NwNAiMuwFDWAXQWw9hQSpxvqRJ1To07wrm_EkDXOxEBcdapT2Hyxs0r2kRfEw","name":{"type":"String","value":"Nightly Backup"}}]}

--- Getting task ---
Task name: {"type":"String","value":"Nightly Backup"}

--- Starting task ---
Task status: {"type":"String","value":"running"}

--- Succeeding task ---
Task status after succeed: {"type":"String","value":"active"}

--- Pausing task ---
Task status: {"type":"String","value":"paused"}

--- Resuming task ---
Task status: {"type":"String","value":"active"}

--- Failing task ---
Task marked as failed

--- Checking due tasks ---
Due tasks: {"count":0,"items":[]}

--- Deleting tasks ---
Tasks deleted

--- Creating agent ---
Created agent: lSqpfd93PuZIlx-DrX0BNahJKs6lbMekyLH4nx5fYFjMDGgkSlA2sOLjvMVWKc53amIobuD1TLYtanG9sF85Ww — null

--- Listing agents ---
Agents: {"count":1,"items":[{"deployment_id":{"type":"String","value":"deploy_kt_example"},"id":"lSqpfd93PuZIlx-DrX0BNahJKs6lbMekyLH4nx5fYFjMDGgkSlA2sOLjvMVWKc53amIobuD1TLYtanG9sF85Ww","llm_model":{"type":"String","value":"gpt-4.1"},"name":{"type":"String","value":"DataBot"},"system_prompt":{"type":"String","value":"You are a data analysis assistant."}}]}

--- Getting agent by ID ---
Agent: {"type":"String","value":"DataBot"}

--- Getting agent by name ---
By name: {"type":"String","value":"DataBot"}

--- Updating agent ---
Updated agent system_prompt

--- Agents by deployment ---
Agents for deployment: {"count":0,"items":[]}
WARNING: agents-by-deployment omitted created agent lSqpfd93PuZIlx-DrX0BNahJKs6lbMekyLH4nx5fYFjMDGgkSlA2sOLjvMVWKc53amIobuD1TLYtanG9sF85Ww; TODO: check/fix the server-side deployment lookup

--- Deleting agent ---
Agent deleted

--- Cleanup: deleting goals ---
Goals deleted

=== Example Complete ===

BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientJoins.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Join Operations Example ===

=== Setting up sample data ===
✓ Sample data created

=== Example 1: Single collection join (users with departments) ===
✓ Found 2 users with department data
  - Bob Smith: Sales
  - Alice Johnson: Engineering

=== Example 2: Join with filtering ===
✓ Found 1 users in Engineering
  - Alice Johnson: Building A

=== Example 3: Join with user profiles ===
✓ Found 2 users with profile data
  - Bob Smith: Sales Manager
  - Alice Johnson: Senior Software Engineer

=== Example 4: Join orders with user data ===
✓ Found 2 completed orders
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

=== Example 5: Complex join with multiple conditions ===
✓ Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✓ Deleted test collections

✓ Join operations example completed successfully

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientJwtAuthFlow.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created
✓ kt_users_register saved
✓ kt_users_login saved
✓ kt_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/kt_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/kt_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/kt_users_verify_token { "token": "<jwt>" }

✓ Cleaned up demo functions

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientKvLinks.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - KV Links Example ===

--- Setting KV entry ---
Set key: user:alice

--- Linking documents ---
Linked order_001: null
Linked order_002: null
Linked inv_100: null

--- Getting links ---
Links for user:alice: [{"collection":"orders","document_id":"order_002","field_path":null,"created_at":"2026-10-08T05:55:05.989929Z","last_accessed":"2026-10-08T05:55:05.999365Z","metadata":{}},{"collection":"orders","document_id":"order_001","field_path":null,"created_at":"2026-10-08T05:55:05.986506Z","last_accessed":"2026-10-08T05:55:05.999365Z","metadata":{}},{"collection":"invoices","document_id":"inv_100","field_path":null,"created_at":"2026-10-08T05:55:05.995585Z","last_accessed":"2026-10-08T05:55:05.999365Z","metadata":{}}]

--- Unlinking document ---
Unlinked order_002: null

--- Verifying remaining links ---
Remaining links: [{"collection":"orders","document_id":"order_001","field_path":null,"created_at":"2026-10-08T05:55:05.986506Z","last_accessed":"2026-10-08T05:55:06.008078Z","metadata":{}},{"collection":"invoices","document_id":"inv_100","field_path":null,"created_at":"2026-10-08T05:55:05.995585Z","last_accessed":"2026-10-08T05:55:06.008078Z","metadata":{}}]

--- Cleanup ---
Unlinked remaining documents
Deleted key: user:alice

=== Example Complete ===

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientKvOperations.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - KV Operations Example ===

=== KV Set ===
✓ Set key: user:123

=== KV Get ===
✓ Retrieved value: {"value":{"role":"admin","name":"Alice","email":"alice@example.com"},"type":"Object"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"type":"Object","value":{"user_id":"123","created_at":1791438914363}}
  (Will expire in 10 seconds)

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_ops_kt_1791438914247:config:db: success
  kv_ops_kt_1791438914247:config:cache: success
  kv_ops_kt_1791438914247:config:api: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_kt_1791438914247:config:db: {"host":"localhost","port":5432}
  kv_ops_kt_1791438914247:config:cache: {"ttl":3600,"enabled":true}
  kv_ops_kt_1791438914247:config:api: {"retries":3,"timeout":30}

=== KV Exists ===
✓ Key exists: true

=== KV Find (Pattern Query) ===
✓ Found 3 keys matching 'config:.*'

=== KV Query (Alias for Find) ===
✓ Total keys in store: 5

=== KV Delete ===
✓ Deleted key: user:123

=== Verify Deletion ===
✓ Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  kv_ops_kt_1791438914247:config:db: deleted
  kv_ops_kt_1791438914247:config:cache: deleted
  kv_ops_kt_1791438914247:config:api: deleted

=== Example Complete ===

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientKvPrecision.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - KV Precision: Float vs Decimal ===

=== Test 1: Using Kotlin Doubles (LOSES PRECISION) ===
Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) MATCH
  Widget B: $39.99 (expected $39.99) MATCH
  Widget C: $49.99 (expected $49.99) MATCH

=== Test 2: Using fieldDecimal() (PRESERVES PRECISION) ===
Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99)
  Widget B: $39.99 (expected $39.99)
  Widget C: $49.99 (expected $49.99)

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.30000000000000004 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Cleanup ===
Cleaned up 8 test keys

=== Summary ===
Use fieldDecimal() for monetary values, percentages, and
any case where floating-point errors are unacceptable.
fieldDecimal() stores values as strings internally,
preserving exact precision across all operations.

=== Example Complete ===

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientPathRoutedFunction.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created
✓ kt_route_admin saved
✓ kt_route_user_by_id saved
✓ kt_route_user_posts saved
✓ kt_route_org_create_member saved

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientProjection.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields
Fetched 3 users with only 3 fields each

Example 2: Exclude sensitive fields
Fetched 2 admins without sensitive data
  Password field excluded: true

Example 3: Complex query with projection
Fetched 3 active users with profile fields

Example 4: Find by ID with projection
Fetched user profile: Alice Johnson

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 3 fields per user

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientQueryBuilder.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 records

=== Query 1: Equality (city = 'NYC') ===
✓ Found 2 records in NYC

=== Query 2: Range (age >= 25 AND age < 32) ===
✓ Found 3 records with age 25-31

=== Query 3: Sort by score (descending) ===
✓ Top 3 scores:
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=95)})
  - Score: ObjectValue(value={value=IntegerValue(value=92), type=StringValue(value=Integer)})
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=88)})

=== Query 4: Complex (score > 80 AND age >= 25) ===
✓ Found 4 high-scoring adults

=== Query 5: IN (city IN ['NYC', 'LA']) ===
✓ Found 4 records in NYC or LA

=== Query 6: Pagination (skip 2, limit 2) ===
✓ Page 2 (2 records):
  - ObjectValue(value={value=StringValue(value=Charlie), type=StringValue(value=String)})
  - ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Diana)})

=== Query 7: Contains (name contains 'a') ===
✓ Found 2 names containing 'a'

=== Cleanup ===
✓ Deleted collection: kotlin_query_example

=== Example Complete ===

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientRawCompletionStream.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Raw Completion Stream (SSE) Example ===

--- Basic SSE Raw Completion ---
Response: The three primary colors are red, blue, and yellow.

--- Structured Output via SSE ---
JSON response: [
  {
    "name": "Mercury",
    "diameter_km": 4879
  },
  {
    "name": "Earth",
    "diameter_km": 12742
  },
  {
    "name": "Jupiter",
    "diameter_km": 139820
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===

BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSchedules.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Schedules Example ===

--- Creating schedule ---
Created schedule: f64ffbe1-0ffe-40c1-84c9-b9a2d5e2ce21 — "Hourly Health Check"

--- Listing schedules ---
Schedules: {"count":1,"schedules":[{"created_at":"2026-10-08T05:56:02.920526Z","cron_expression":"0 0 * * * *","description":"Ping all services every hour","enabled":true,"function_label":"schedule_noop_kotlin_80082_1791438962791","id":"f64ffbe1-0ffe-40c1-84c9-b9a2d5e2ce21","last_execution":null,"name":"Hourly Health Check","next_execution":"2026-10-08T06:00:00Z","parameters":{},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-10-08T05:56:02.920526Z"}]}

--- Getting schedule ---
Fetched: "Hourly Health Check" (cron: "0 0 * * * *")

--- Updating schedule ---
Updated cron: "0 */30 * * * *"

--- Triggering schedule ---
Trigger response: {"schedule_id":"f64ffbe1-0ffe-40c1-84c9-b9a2d5e2ce21","status":"triggered"}

--- Pausing schedule ---
Enabled after pause: false

--- Resuming schedule ---
Enabled after resume: true

--- Deleting schedule ---
Schedule deleted successfully

=== Example Complete ===

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSchemaManagement.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'kotlin_schema_example' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: StringValue(value=X22biT3BAIxBhBXYT_iwygfXYej2kos2nqYV-GQzWvYa2l4w_h_vnccLr1jBDnljTJqfAh3PBVJP8QlBNfYaBA)
✓ Inserted user 2: StringValue(value=7mkdIie8qCfyWBiOmj2bGtlGV2SVsFuaG2PrKAgU8WxjigrAU6c98WgLZPZE_eKCfOjjwytGEQ-qE5DdBok9eg)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 27
  Sample: [chat_agent_configs__ek0_testing, chat_goal_templates__ek0_testing, schema_documents_client_js, schema_documents_client_go, test_accounts]

=== Cleanup ===
✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSearch.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
Text results: 2; execution time: 1 ms
{"value":"Python Programming","type":"String"}: score=13.200000000000001, matched=[category, title]
{"type":"String","value":"Rust Programming"}: score=13.200000000000001, matched=[title, category]
Vector results: 3
Filtered vector results: 2
Custom-weight hybrid results: 3
Raw search: {"results":[{"record":{"category":{"type":"String","value":"programming"},"title":{"type":"String","value":"Python Programming"},"id":"LT0SdEdue9_p8pEo_LX8S2ScMTL4WQYF5gexmjVD4hl2KYU8lqTyL0ZcuA2R1S_rE35Bmq_i3CrkZm7UENmi0w","embedding":{"type":"Vector","value":[0.8,0.2,0.1]}},"score":1.0,"matched_fields":[]},{"record":{"category":{"value":"programming","type":"String"},"id":"kq6pRnfwjPcbjPjd7aKIbBgzrHAYfsI-IXo5HkLWUBmoAGpHeePP44ShngiknKjEsVQhe3N6_40_Y7Qg4LwiwA","title":{"type":"String","value":"Rust Programming"},"embedding":{"type":"Vector","value":[0.9,0.1,0.2]}},"score":1.0,"matched_fields":[]}],"total":2,"execution_time_ms":0}

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSimpleCrud.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Simple CRUD Example ===

=== Create ===
✓ Inserted user: EkoRecord(fields={id=StringValue(value=KMUxF9LgWH-9q_furX9y776bwV1CyNA-Eo91iAZ47j_yolNVJd_iMfp5JODZ2tWnH5uR455Tmli9Ca4n-tWEqw)})
  User ID: KMUxF9LgWH-9q_furX9y776bwV1CyNA-Eo91iAZ47j_yolNVJd_iMfp5JODZ2tWnH5uR455Tmli9Ca4n-tWEqw

=== Read ===
✓ Found user by ID: EkoRecord(fields={data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Thu Oct 08 01:56:26 EDT 2026)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), age=ObjectValue(value={value=IntegerValue(value=28), type=StringValue(value=Integer)}), embedding=VectorValue(value=[0.1, 0.2, 0.3, 0.4, 0.5]), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), id=StringValue(value=KMUxF9LgWH-9q_furX9y776bwV1CyNA-Eo91iAZ47j_yolNVJd_iMfp5JODZ2tWnH5uR455Tmli9Ca4n-tWEqw)})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Thu Oct 08 01:56:26 EDT 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: data, active, email, created_at, price, user_id, tags, name, metadata, age, embedding, categories, id

=== Update ===
✓ Updated user: EkoRecord(fields={price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), embedding=VectorValue(value=[0.1, 0.2, 0.3, 0.4, 0.5]), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), id=StringValue(value=KMUxF9LgWH-9q_furX9y776bwV1CyNA-Eo91iAZ47j_yolNVJd_iMfp5JODZ2tWnH5uR455Tmli9Ca4n-tWEqw), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), created_at=ObjectValue(value={value=StringValue(value=Thu Oct 08 01:56:26 EDT 2026), type=StringValue(value=String)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)})})

=== Query ===
✓ Found 1 users matching query
  - EkoRecord(fields={email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Thu Oct 08 01:56:26 EDT 2026)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), id=StringValue(value=KMUxF9LgWH-9q_furX9y776bwV1CyNA-Eo91iAZ47j_yolNVJd_iMfp5JODZ2tWnH5uR455Tmli9Ca4n-tWEqw), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), embedding=VectorValue(value=[0.1, 0.2, 0.3, 0.4, 0.5])})

=== Delete ===
✓ Deleted user with ID: KMUxF9LgWH-9q_furX9y776bwV1CyNA-Eo91iAZ47j_yolNVJd_iMfp5JODZ2tWnH5uR455Tmli9Ca4n-tWEqw

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

=== Example Complete ===

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSimpleWebsocket.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client ===
✓ WebSocket client created

=== Connect to WebSocket ===
✓ Connected to WebSocket

=== Insert Test Record ===
✓ Inserted test record

=== Find All via WebSocket ===
✓ WebSocket findAll result:
  {"data":[{"name":{"type":"String","value":"Test User"},"id":"YfrW2M8KrC_l5ODAqw9eU2cexeQDSSxxiuyVQPwtOz-Z2OnH7X4ImqQguOXhCRfwiyJHMd3e3dPV2UCkUhVY9w","status":{"type":"String","value":"active"}}]}

=== Close WebSocket ===
✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_example

=== Example Complete ===

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSwrNative.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
🚀 ekoDB Kotlin Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.

🧹 Cleaning up...
✓ Deleted 0 test functions and owned SWR collections

Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR function: github_user_native_kt (dz18EzC9PYB328U325xZrmYWLPcWxtFlzHI_pteyGYm51cCgWAuwHmmwBEwhTfi1RfWLGVpCeyMjIyBkSUtkQA)

First call (cache miss - will fetch from GitHub API):
  Response time: 144ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 6ms
  Speedup: 24.0x faster 🚀


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR function with audit trail: product_swr_audit_kt (C7WjBy0MtV_hPkpDYbDjoFvfkbV1foVHLFtDTlyMZBSsjmgoZG8LqKRTj76ccZaLuOcwiS9NrUd0lzrcjX6eoA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_kt' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_kt (Cdc_cS0tB95Z9oqaDsnyVxpUOlI79sSE1OQPTU8fbzZkUglknpD_i2DMuaJH6ylrISpUfUYGdvnNoxe0Uc6f6g)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_kt' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL function: flexible_cache_kt (tFdArsm6h_fPzCEJX-HmUtEYhC7uTElI6KZruuE6qU0zNsvA_GnfHNO3iO_wtQ8KfOTHfkSs0TQb92XEmuP5dQ)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

================================================================================
✅ Key Benefits of Native SWR:
✅ Single function: Replaces 4-function cache-aside pattern
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)
Native SWR:     SWR → Insert (2 functions)
Result:         60% fewer functions, cleaner code, same behavior 🎯

🧹 Cleaning up...
✓ Deleted 4 test functions and owned SWR collections

✅ All examples completed!

BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSwrPattern.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Setting up cache collection...
✓ Cache entry created

Step 2: Create SWR cache lookup function
✓ Created SWR function: swr_cache_lookup_kt_1791439010252 (Z2Jn-FEFoucpgcIUl0Sq5Y7YENMVxb6y2YjREqv7tSSesLT1QG1Wp5FLx9RDB_ajl8zL4A1vWz3uDEza1N2Fuw)

Step 3: First call - Cache lookup
Found 1 cached entries
✓ Cache lookup complete

Step 4: Second call - Fast cache hit
Response time: 5ms (served from cache)
✓ Lightning fast cache hit

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🧹 Cleaning up...
✓ Cleanup complete


BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientTransactions.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Jv8HdF9GIOTIBhyTRuHZmyag_v0DQ3Vd4z2qO3DbwXZ8ppRsQreBdtzfhOJCpZPWGIwy2V1_eKZaKnK0XvHNow
Created Bob: $500 - ID: -GM78UTU9bN7RJZMFJZORRz8m8ytX4CD6mD91V054TyeQyoWTZFHQsIfmCEN4d0SHlKl79RSOd-pTJgBk_qVwA

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 66d717da-f0c4-4b6b-9040-25df439a0043

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

✓ Verified committed balances: Alice=$800, Bob=$700

=== Example 5: Rollback Demo ===
New transaction: f3f6248c-6ad6-493f-b3a2-13eb44f5cd4b
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientUserFunctions.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - User Functions Example ===

=== Create User Function ===
Created user function with ID: mTQQQTTLO1CwGvnuBjtOlipNKdw1LdYpLBn-zszCf2CQXyLqYwnH9BjzNH9uOibZKdRtuInG84CMA2WmSrEK8Q

=== Get User Function ===
Retrieved: "get_active_users_kt" - "Get Active Users (Kotlin)"
Description: "Fetches all users and filters by active status"

=== List All User Functions ===
Found 11 user functions:
  - "get_active_users_client_js": "Get Active Users (Updated)"
  - "fetch_product_reviews_ts_77743_1791438649451": "Fetch Product with Reviews (Multi-API)"
  - "conc_demo_rl_skip_ts_75441_1791438514383": "Rate-limit (skip mode)"
  - "get_active_users_client_ts_updated": "Get Active Users (Updated)"
  - "conc_demo_rl_fail_ts_77654_1791438637251": "Rate-limit (fail mode)"

=== List User Functions by Tag ===
Found 1 user functions with 'kotlin' tag:
  - "get_active_users_kt"

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

=== User Functions Example Complete ===

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientWebsocketChatStream.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== WebSocket Chat Streaming Example (Kotlin) ===

Created chat session: uJZQ2PhzQSiF6VE-lxKs7wqMMGUBGxKGPKcC0YExLrnR8tmMe77gvHewxT-6vv1sQkirHC3-vNanph7_Z8C7oQ

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: DPeRDU83891ySeAAMPxXGoI5JZVYpCsuBe5HyA-8qmWP2aQz9g2QBmIgk4rR_dvckVQxuRgdsBnd8_RloqGTpQ
Execution time: 865ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris....

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientWebsocketSubscribe.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
=== WebSocket Subscription Example (Kotlin) ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_kt' ===
✓ Subscribed (subscription_id: sub_a24e09fd955a45eb8ff018be7051369c)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "baiMuA_Ki0r8oN5fgZBPrJfIYSFousguENMbpqFsndfOfKG7GFPcVoMBbjkIQpPp_gaX66v96M3UmIIG37S5mw"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_kt"
     Record IDs: ["baiMuA_Ki0r8oN5fgZBPrJfIYSFousguENMbpqFsndfOfKG7GFPcVoMBbjkIQpPp_gaX66v96M3UmIIG37S5mw"]
     Timestamp:  "2026-10-08T05:57:22.903085+00:00"

Inserting record 2...
✓ Inserted: "3jr_3imo6Jbc0zk7RWF0RtaVs7SzdPJ3Sc9PyzvJtYv33Ww7R0OhHuMNXE124J2vriyb5WBlEanOCPHFL3fOhg"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["3jr_3imo6Jbc0zk7RWF0RtaVs7SzdPJ3Sc9PyzvJtYv33Ww7R0OhHuMNXE124J2vriyb5WBlEanOCPHFL3fOhg"]

=== Unsubscribing ===
✓ Unsubscribed: {"payload":{"data":{"collection":"ws_subscribe_example_kt","found":true,"unsubscribed":true}},"type":"Success"}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_kt'

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientWebsocketTtl.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - WebSocket TTL Example ===

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: StringValue(value=gMySGUgvSRSf58nr20EfQvLB_qe7QP-IzXxkDL9X6yBeqNGNM0KhXsd1dQPwBNaqKkN8bxqlXeWtE89TykFBog)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"data":[{"name":{"value":"WebSocket TTL Test","type":"String"},"id":"gMySGUgvSRSf58nr20EfQvLB_qe7QP-IzXxkDL9X6yBeqNGNM0KhXsd1dQPwBNaqKkN8bxqlXeWtE89TykFBog","value":{"type":"Integer","value":42},"created_at":{"value":1791439050034,"type":"Integer"},"ttl":"2026-10-08T06:57:30.141788Z"}]}

✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_ttl_example

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: BypassRippleExample.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
=== Bypass Ripple Example ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
1. Basic insert (ripple enabled):
   Inserted with ripple: EkoRecord(fields={id=StringValue(value=KCPCe6H79ziP9XHyqV8lKh3Kss53q2N522lwHi6vcoStwPjJ0jL2LuZ1VrdA1uncJdeBD112XAD41I83JN6UIQ)})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=KFcQG3uiz_Esl01Xyrs06yxyewtFanplZF4f6OTsKwGRIBbIBOn9KitopzpvbqP35auS_0xRJuwufS4ot6kWCw)})

3. Update with bypass_ripple:
   Updated with bypass_ripple: EkoRecord(fields={price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=150)}), id=StringValue(value=KCPCe6H79ziP9XHyqV8lKh3Kss53q2N522lwHi6vcoStwPjJ0jL2LuZ1VrdA1uncJdeBD112XAD41I83JN6UIQ), name=ObjectValue(value={value=StringValue(value=Product 1), type=StringValue(value=String)})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: EkoRecord(fields={price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=500)}), name=ObjectValue(value={value=StringValue(value=Upsert Product), type=StringValue(value=String)}), id=StringValue(value=KCPCe6H79ziP9XHyqV8lKh3Kss53q2N522lwHi6vcoStwPjJ0jL2LuZ1VrdA1uncJdeBD112XAD41I83JN6UIQ)})

✅ All bypass_ripple operations completed successfully!

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
✅ All Kotlin integration tests complete!
🐍 Building Python client package...
🔧 Ensuring maturin is available in .venv...
🔨 Building wheel...
🍹 Building a mixed python/rust project
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.09s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.27.0-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.27.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.27.0
    Uninstalling ekodb_client-0.27.0:
      Successfully uninstalled ekodb_client-0.27.0
Successfully installed ekodb-client-0.27.0
🧪 Ensuring test dependencies (pytest) in .venv...
✅ Python client package built and installed!
📦 Ensuring Python example dependencies in .venv...

🤖 RAG Conversation System Examples
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Prerequisites:
  1. ekoDB server running (make run in ekodb/)
  2. OPENAI_API_KEY set in server environment
  3. API_BASE_URL and API_BASE_KEY exported in your shell

Building Rust client library...
✓ Rust client built

✓ Python client built and installed into .venv (via build-python-client prerequisite)

Building TypeScript client library...
✓ TypeScript client built

Installing TypeScript client in examples...
✓ TypeScript client installed

Building TypeScript example...
✓ TypeScript example built

Building Go client library...
✓ Go client built

Building Go RAG example...
✓ Go example built

Building Kotlin client library...
✓ Kotlin client built

Building Kotlin RAG example...
✓ Kotlin example built

Running Rust RAG Example...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 615.594959ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 343.947875ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 331.341125ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 271.549416ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 232.252625ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 412.145083ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 430.802375ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 243.381208ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 262.845666ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 257.361125ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 217.931208ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 246.678083ms
    • Function auto-cleaned up by client
✓ Stored performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval ===
User asks: "How do I write memory-safe high-performance database code?"

=== Step 3: Searching Related Context ===
Using hybrid search to find relevant messages from all conversations...


→ Generating embedding for user question...
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 216.712125ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages_rust
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 84.627833ms

✓ Found 5 related messages across all conversations:
  1. From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  2. From conv_database_design
     What is database normalization?

  3. From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  4. From conv_database_design
     When should I use NoSQL over SQL?

  5. From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves several best practices and principles that help you manage resources effectively while ensuring safety against common vulnerabilities and issues. Here are some key guidelines:

### 1. **Use Safe Language Features**
   - **Language Choice:** Choose programming languages or frameworks with built-in memory safety features, such as Rust, Kotlin, or Go. These languages help prevent common issues like buffer overflows and null pointer dereferences.
   - **Immutable Data Structures:** Where possible, employ immutable data structures which can help prevent unintended side effects and make your code more predictable.

### 2. **Connection Management**
   - **Connection Pooling:** Use connection pools to manage database connections efficiently. This minimizes the overhead of establishing and tearing down connections repeatedly.
   - **Resource Cleanup:** Ensure that you properly close database connections and any other resources (like cursors) when they are no longer needed to avoid resource leaks.

### 3. **Efficient Query Design**
   - **Use Prepared Statements:** Prepared statements help mitigate SQL injection risks and can allow the database to optimize query execution plans ahead of time.
   - **Batch Operations:** When inserting or updating data, use batch operations to reduce the number of round trips to the database, which enhances performance.

### 4. **Indexing and Query Optimization**
   - **Indexes:** Properly index your database tables to speed up lookups. Analyze your queries and ensure relevant fields are indexed.
   - **Use EXPLAIN:** Utilize the database’s query planner (such as the EXPLAIN command in SQL databases) to understand the execution plan for your queries and optimize them as necessary.

### 5. **Error Handling**
   - **Graceful Error Handling:** Implement robust error handling that captures errors without exposing sensitive information or crashing the application.
   - **Transaction Management:** Use transactions where appropriate to ensure data consistency and integrity, especially during complex operations.

### 6. **Data Type Usage**
   - **Appropriate Data Types:** Choose appropriate data types in your database schema that match your application needs, which can help optimize performance.
   - **Avoid Large Data Transfers:** Be mindful of the amount of data transferred between your application and the database. Retrieve only the fields and records necessary.

### 7. **Concurrency Management**
   - **Transaction Isolation Levels:** Understand and apply suitable transaction isolation levels that balance performance and data integrity according to your use case.
   - **Locking Mechanisms:** Use efficient locking mechanisms to manage concurrent access to data, avoiding deadlocks and performance bottlenecks.

### 8. **Monitoring and Profiling**
   - **Performance Monitoring:** Routinely monitor performance metrics, such as query execution times and resource usage, to identify bottlenecks.
   - **Profiling Tools:** Use profiling tools to analyze your application’s database interactions and identify areas for optimization.

### 9. **Testing and Validation**
   - **Unit Tests:** Write unit tests for database interactions to ensure that your database code behaves as expected.
   - **Load Testing:** Conduct load tests to simulate high-traffic scenarios and identify how your database and application respond under stress.

### 10. **Documentation and Best Practices**
   - **Document Queries:** Comment and document your complex queries and database logic to aid future developers (or yourself) in understanding.
   - **Follow Conventions:** Adhere to database access patterns and coding conventions as established by your team or the language/framework you are using.

By incorporating these best practices, you'll enhance the safety and performance of your database interactions. It's also valuable to stay updated with the latest trends in database technologies and development practices to continuously improve the efficiency of your code.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 381.235083ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 4079 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 244.706875ms
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


→ Executing text_search()...
  • Collection: rag_messages_rust
  • Query: "ownership system"
  • Limit: 3 results
  • Search method: Full-text with fuzzy matching & stemming
  • No vector embeddings needed - pure keyword search
  ✓ Text search completed in 52.661333ms

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_rust_programming: The borrow checker enforces Rust's ownership rules at compile time. It ensures that references don't outlive the data they point to and prevents data races by allowing either multiple immutable references or one mutable reference.

=== System Statistics ===

→ Querying database statistics...
  • Using find_all() helper - simplified query API

📊 Database Statistics:
  • Total conversations: 4
  • Total messages stored: 14
  • All messages indexed for vector search ✓
  • All messages indexed for text search ✓
  • All messages queryable by metadata ✓

=== Step 8: Dynamic Search Configuration ===
Each conversation can have its own search config...

💡 Conversations can store custom search configurations:
  • Search type: hybrid, text, or vector
  • Relevance thresholds
  • Filter by tags or metadata
  • Collection-specific settings
  • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!


=== Cleanup ===
Deleting example collections...

✅ All done! RAG system demonstrated successfully.

✓ Using search results to enhance AI responses (RAG)
✓ Building a self-improving knowledge base
✓ Dynamic search configurations per conversation

ekoDB provides everything needed for AI-powered applications:
  • Vector search (semantic similarity)
  • Text search (keyword matching)
  • Hybrid search (best of both)
  • AI functions (Chat, Embed)
  • Flexible querying and filtering
  • All in one database - no external dependencies!


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Python RAG Example...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.311s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.219s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.316s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.241s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.203s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.242s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.214s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.229s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.262s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.257s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.249s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.214s
    • Function auto-cleaned up by client
✓ Stored performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval ===
User asks: "How do I write memory-safe high-performance database code?"

=== Step 3: Searching Related Context ===
Using hybrid search to find relevant messages from all conversations...


→ Generating embedding for user question...
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.255s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages_py
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.079s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.504] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  2. [Score: 0.481] From conv_database_design
     What is database normalization?

  3. [Score: 0.479] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  4. [Score: 0.467] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.446] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

To write memory-safe, high-performance database code, it's crucial to integrate principles from different programming paradigms and leverage efficient database access patterns. Below are several key strategies that can help:

### 1. Choose the Right Database
- **SQL vs NoSQL**: Choose the appropriate database based on your data structure and requirements. SQL databases are suitable for structured data with complex queries, while NoSQL databases are better for unstructured data or when you require scalability.

### 2. Utilize Connection Pooling
- Reduce the overhead of establishing database connections by using connection pooling. This keeps a pool of active connections ready for use, which minimizes latency and improves performance.

### 3. Batch Operations
- Instead of executing multiple individual queries, batch your insert, update, or delete operations. This reduces round trips to the database and can significantly improve throughput.

### 4. Prepared Statements
- Use prepared statements to prevent SQL injection attacks and improve performance when executing repeated queries. They also help in memory safety since the inputs are bound to specific types.

### 5. Proper Indexing
- Ensure that the appropriate indexes are in place for frequently queried fields. This improves query performance by reducing the amount of data scanned.

### 6. Utilize Transactions Wisely
- Use database transactions to maintain data integrity while ensuring that you minimize the locking duration. This can improve concurrency while reducing the risk of data corruption.

### 7. Optimize Queries
- Regularly analyze query performance using tools like `EXPLAIN` to identify slow queries and optimize them (using indexing, query restructuring, etc.)

### 8. Data Structures Choice
- Choose memory-efficient data structures according to your application needs. For applications that require speed, prefer data structures that offer O(1) access times.

### 9. Limit Data Fetching
- Use pagination, filtering, and projections to limit the amount of data fetched from the database. Fetch only the necessary columns and rows to reduce memory usage and improve application performance.

### 10. Monitor and Profile
- Use monitoring tools to profile your database queries, cache usage, and connection pool. Optimizing based on live data helps fine-tune performance.

### 11. Safety Measures
- Implement proper error handling for database operations and ensure that resources are released properly (like closing connections). Use try-catch statements to handle exceptions that may arise during data operations.

### 12. Use ORM or Efficient Libraries
- If comfortable with an Object-Relational Mapping (ORM) tool, choose one that emphasizes performance and memory safety. Some ORMs provide lazy loading and built-in caching mechanisms.

### 13. Version Control and Migration Tools
- Use version control for your database schema and migration tools to manage changes over time safely. This reduces the risk of breaking changes that could lead to memory leaks or performance degradation.

### Conclusion
By implementing these techniques, you can develop memory-safe, high-performance database applications:
- Align your database choice with your application’s needs.
- Optimize regularly through profiling and monitoring.
- Ensure that your operations are efficient and secure.

Combining appropriate tech choices with best practices in coding and database design will lead to robust and performant applications.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.272s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3500 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.235s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_rust_programming: The borrow checker enforces Rust's ownership rules at compile time. It ensures that references don't outlive the data they point to and prevents data races by allowing either multiple immutable references or one mutable reference.

=== System Statistics ===
Total conversations: 4
Total messages stored: 14
All messages are indexed for vector search ✓
All messages are indexed for text search ✓
All messages are queryable by metadata ✓

=== Step 8: Dynamic Search Configuration ===
Each conversation can have its own search config...

💡 Conversations can store custom search configurations:
  • Search type: hybrid, text, or vector
  • Relevance thresholds
  • Filter by tags or metadata
  • Collection-specific settings
  • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!


=== 📚 Summary: What This Example Showed ===

🔧 ekoDB Native Capabilities Used:
  ✓ Functions with Embed operation (AI integration)
  ✓ Hybrid Search (text + vector combined)
  ✓ Text Search (full-text with stemming)
  ✓ Automatic embedding generation
  ✓ Cross-collection queries

🚀 New Client Helper Methods:
  • client.embed(text, model) - Generate embeddings
  • client.hybrid_search() - Semantic + keyword search
  • client.text_search() - Full-text search
  • client.find_all() - Query all documents

💡 Key Takeaways:
  1. ekoDB handles AI Functions natively - no external services needed
  2. One-line embedding generation with auto-cleanup
  3. Hybrid search combines semantic understanding + keyword matching
  4. Perfect for RAG: store, search, and retrieve context
  5. All AI capabilities accessible through simple client methods

🎯 Build production RAG systems with ekoDB!
   → Set OPENAI_API_KEY in your ekoDB server environment
   → Use these client helpers to make AI integration simple
   → Scale to millions of documents with native indexing

✓ Cleanup complete


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running TypeScript RAG Example...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.214s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.234s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.234s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.247s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.256s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.250s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.287s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.307s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.244s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.270s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.222s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.230s
    • Function auto-cleaned up by client
✓ Stored performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval ===
User asks: "How do I write memory-safe high-performance database code?"

=== Step 3: Searching Related Context ===
Using hybrid search to find relevant messages from all conversations...


→ Generating embedding for user question...
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.343s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages_ts
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.078s
✓ Found 5 related messages across all conversations:
  1. [Score: 0.474] From conv_performance
     How can I optimize database queries?

  2. [Score: 0.466] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.449] From conv_database_design
     When should I use NoSQL over SQL?

  4. [Score: 0.434] From conv_database_design
     What is database normalization?

  5. [Score: 0.405] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code is crucial to ensure that your application runs efficiently without risking memory leaks or corruption. Here are several best practices to achieve this:

### 1. **Use Prepared Statements**
   - Prepared statements prevent SQL injection attacks and optimize performance by allowing the database to cache the execution plan. Always use parameterized queries where possible.

### 2. **Connection Pooling**
   - Use connection pooling to manage database connections effectively. This avoids the overhead of creating a new connection for every request, which can be expensive and slow.

### 3. **Efficient Query Design**
   - Minimize the amount of data transferred between the database and your application. Use specific columns in `SELECT` statements instead of `*`, and ensure indexes are available for frequently queried fields.

### 4. **Batch Processing**
   - For write operations, use batch inserts or updates to reduce the number of database round trips. This can significantly boost performance when handling large volumes of data.

### 5. **Optimized Indexing**
   - Ensure that your database tables have appropriate indexes. Use indexing strategies that match your query patterns, but be mindful that too many indexes can slow down write operations.

### 6. **Use of Transactions**
   - Wrap multiple related operations in transactions to maintain data integrity. Use isolation levels appropriately to avoid locking issues, but ensure that the level chosen does not degrade performance unnecessarily.

### 7. **Memory Management**
   - Be aware of memory allocation and deallocation, especially in languages like C or C++. Use smart pointers or memory management routines to prevent leaks. In managed languages, ensure that objects are dereferenced properly when no longer needed.

### 8. **Asynchronous Operations**
   - Utilize asynchronous programming techniques where possible to improve application responsiveness, especially for I/O-bound operations like database calls.

### 9. **Profiling and Monitoring**
   - Regularly profile your database queries and application performance. Use tools to monitor slow queries, connection usage, and memory consumption to identify potential bottlenecks.

### 10. **Leveraging ORM Tools Wisely**
   - If you are using Object-Relational Mapping (ORM) libraries, be cautious about their performance implications. While they can simplify data interactions, they also may introduce overhead. Optimize the usage of ORM and avoid fetching unnecessary data.

### 11. **Use Appropriate Data Formats**
   - When dealing with binary data or large objects, choose efficient formats for storage and transfer (like Protocol Buffers or Avro). Compression techniques can also help in reducing size during transit.

### 12. **Maintaining a Clean Codebase**
   - Write clear, maintainable code to simplify future optimizations and debugging. Use meaningful variable names, consistent formatting, and include comments where necessary.

By following these best practices, you'll be able to develop memory-safe, high-performance database code that is robust and efficient.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.259s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3155 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.260s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


→ Executing textSearch()...
  • Collection: rag_messages_ts
  • Query: "ownership system"
  • Limit: 3 results
  • Search method: Full-text with fuzzy matching & stemming
  • No vector embeddings needed - pure keyword search
  ✓ Text search completed in 0.047s
✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_rust_programming: The borrow checker enforces Rust's ownership rules at compile time. It ensures that references don't outlive the data they point to and prevents data races by allowing either multiple immutable references or one mutable reference.

=== System Statistics ===

→ Querying database statistics...
  • Using findAllWithLimit() helper - simplified query API

📊 Database Statistics:
  • Total conversations: 4
  • Total messages stored: 14
  • All messages indexed for vector search ✓
  • All messages indexed for text search ✓
  • All messages queryable by metadata ✓

=== Step 8: Dynamic Search Configuration ===
Each conversation can have its own search config...

💡 Conversations can store custom search configurations:
  • Search type: hybrid, text, or vector
  • Relevance thresholds
  • Filter by tags or metadata
  • Collection-specific settings
  • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!


=== 📚 Summary: What This Example Showed ===

🔧 ekoDB Native Capabilities Used:
  ✓ Functions with Embed operation (AI integration)
  ✓ Hybrid Search (text + vector combined)
  ✓ Text Search (full-text with stemming)
  ✓ Automatic embedding generation
  ✓ Cross-collection queries

🚀 New Client Helper Methods:
  • client.embed(text, model) - Generate embeddings
  • client.hybridSearch() - Semantic + keyword search
  • client.textSearch() - Full-text search
  • client.findAllWithLimit() - Query all documents

💡 Key Takeaways:
  1. ekoDB handles AI Functions natively - no external services needed
  2. One-line embedding generation with auto-cleanup
  3. Hybrid search combines semantic understanding + keyword matching
  4. Perfect for RAG: store, search, and retrieve context
  5. All AI capabilities accessible through simple client methods

🎯 Build production RAG systems with ekoDB!
   → Set OPENAI_API_KEY in your ekoDB server environment
   → Use these client helpers to make AI integration simple
   → Scale to millions of documents with native indexing

=== Cleanup ===
✓ Cleanup complete


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Go RAG Example...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.343s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.275s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.292s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.234s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.253s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.251s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.292s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.211s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.225s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.210s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.263s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.310s
    • Function auto-cleaned up by client
✓ Stored performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval ===
User asks: "How do I write memory-safe high-performance database code?"

=== Step 3: Searching Related Context ===
Using hybrid search to find relevant messages from all conversations...


→ Generating embedding for user question...
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.266s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages_go
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.063s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.474] From conv_performance
     How can I optimize database queries?

  2. [Score: 0.466] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.449] From conv_database_design
     When should I use NoSQL over SQL?

  4. [Score: 0.434] From conv_database_design
     What is database normalization?

  5. [Score: 0.405] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code requires a multi-faceted approach that considers both the language you are using and the database interactions you design. Here's a comprehensive strategy you can follow:

### 1. **Choose the Right Language and Framework**
- **Languages with Strong Safety Features:** Use languages like Rust or Go that provide memory safety guarantees, preventing common issues like null pointer dereferencing and buffer overflows.
- **ORM Libraries:** If using languages like Python, Ruby, or Java, consider an Object-Relational Mapping (ORM) library that emphasizes type safety and reduces direct SQL manipulation.

### 2. **Use Prepared Statements**
- Prepared statements help prevent SQL injection attacks by separating query logic from data input.
- They also allow the database to cache the query plan for improved performance on repeated executions.

### 3. **Efficient Connection Management**
- Implement connection pooling to reuse database connections rather than opening a new connection for every query. This reduces overhead and improves response time.

### 4. **Optimize Queries**
- **Select Only Required Data:** Instead of using SELECT *, specify the columns you actually need. This reduces data transfer overhead and can speed up query execution.
- **Indexing:** Create appropriate indexes on frequently queried columns to speed up data retrieval. Monitor performance and adjust indexes as necessary.

### 5. **Batch Operations**
- When performing multiple insertions or updates, use bulk operations instead of individual statements. This reduces the number of database round trips and can improve performance.

### 6. **Cache Results**
- Implement caching strategies to store frequently accessed data in memory. This can significantly reduce database load and improve application performance.

### 7. **Error Handling**
- Implement robust error handling to gracefully manage exceptions and failures. Use error messages that provide context without exposing sensitive information.

### 8. **Regular Maintenance and Monitoring**
- Regularly monitor database performance, and run maintenance tasks like vacuuming, optimizing, or rebuilding indexes. This helps maintain performance and responsiveness.

### 9. **Use Transactions Wisely**
- Group related operations within transactions to ensure data integrity. Proper use of transactions can also help in optimizing performance by reducing the number of commits.

### 10. **Profiling and Performance Testing**
- Continuously profile your database interactions to identify bottlenecks. Use profiling tools specific to your database management system (DBMS) to analyze query performance and improve efficiency.

### 11. **Follow Best Practices for Resource Management**
- Make sure to properly close connections and release resources once they are no longer needed to prevent memory leaks and exhaustion of database connections.

Following these principles will help you write code that is efficient and safe while interacting with databases. Additionally, leverage specific features offered by your database management system and keep your codebase clean and maintainable.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.288s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3171 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.241s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


→ Executing TextSearch()...
  • Collection: rag_messages_go
  • Query: "ownership system"
  • Limit: 3 results
  • Search method: Full-text with fuzzy matching & stemming
  • No vector embeddings needed - pure keyword search
  ✓ Text search completed in 0.050s

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_new_question: Writing memory-safe, high-performance database code requires a multi-faceted approach that considers both the language you are using and the database interactions you design. Here's a comprehensive strategy you can follow:

### 1. **Choose the Right Language and Framework**
- **Languages with Strong Safety Features:** Use languages like Rust or Go that provide memory safety guarantees, preventing common issues like null pointer dereferencing and buffer overflows.
- **ORM Libraries:** If using languages like Python, Ruby, or Java, consider an Object-Relational Mapping (ORM) library that emphasizes type safety and reduces direct SQL manipulation.

### 2. **Use Prepared Statements**
- Prepared statements help prevent SQL injection attacks by separating query logic from data input.
- They also allow the database to cache the query plan for improved performance on repeated executions.

### 3. **Efficient Connection Management**
- Implement connection pooling to reuse database connections rather than opening a new connection for every query. This reduces overhead and improves response time.

### 4. **Optimize Queries**
- **Select Only Required Data:** Instead of using SELECT *, specify the columns you actually need. This reduces data transfer overhead and can speed up query execution.
- **Indexing:** Create appropriate indexes on frequently queried columns to speed up data retrieval. Monitor performance and adjust indexes as necessary.

### 5. **Batch Operations**
- When performing multiple insertions or updates, use bulk operations instead of individual statements. This reduces the number of database round trips and can improve performance.

### 6. **Cache Results**
- Implement caching strategies to store frequently accessed data in memory. This can significantly reduce database load and improve application performance.

### 7. **Error Handling**
- Implement robust error handling to gracefully manage exceptions and failures. Use error messages that provide context without exposing sensitive information.

### 8. **Regular Maintenance and Monitoring**
- Regularly monitor database performance, and run maintenance tasks like vacuuming, optimizing, or rebuilding indexes. This helps maintain performance and responsiveness.

### 9. **Use Transactions Wisely**
- Group related operations within transactions to ensure data integrity. Proper use of transactions can also help in optimizing performance by reducing the number of commits.

### 10. **Profiling and Performance Testing**
- Continuously profile your database interactions to identify bottlenecks. Use profiling tools specific to your database management system (DBMS) to analyze query performance and improve efficiency.

### 11. **Follow Best Practices for Resource Management**
- Make sure to properly close connections and release resources once they are no longer needed to prevent memory leaks and exhaustion of database connections.

Following these principles will help you write code that is efficient and safe while interacting with databases. Additionally, leverage specific features offered by your database management system and keep your codebase clean and maintainable.

=== System Statistics ===

→ Querying database statistics...
  • Using FindAll() helper - simplified query API

📊 Database Statistics:
  • Total conversations: 4
  • Total messages stored: 14
  • All messages indexed for vector search ✓
  • All messages indexed for text search ✓
  • All messages queryable by metadata ✓

=== Step 8: Dynamic Search Configuration ===
Each conversation can have its own search config...

💡 Conversations can store custom search configurations:
  • Search type: hybrid, text, or vector
  • Relevance thresholds
  • Filter by tags or metadata
  • Collection-specific settings
  • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!


=== 📚 Summary: What This Example Showed ===

🔧 ekoDB Native Capabilities Used:
  ✓ Functions with Embed operation (AI integration)
  ✓ Hybrid Search (text + vector combined)
  ✓ Text Search (full-text with stemming)
  ✓ Automatic embedding generation
  ✓ Cross-collection queries

🚀 New Client Helper Methods:
  • client.Embed(text, model) - Generate embeddings
  • client.HybridSearch() - Semantic + keyword search
  • client.TextSearch() - Full-text search
  • client.FindAll() - Query all documents

💡 Key Takeaways:
  1. ekoDB handles AI Functions natively - no external services needed
  2. One-line embedding generation with auto-cleanup
  3. Hybrid search combines semantic understanding + keyword matching
  4. Perfect for RAG: store, search, and retrieve context
  5. All AI capabilities accessible through simple client methods

🎯 Build production RAG systems with ekoDB!
   → Set OPENAI_API_KEY in your ekoDB server environment
   → Use these client helpers to make AI integration simple
   → Scale to millions of documents with native indexing

=== Cleanup ===
✓ Cleanup complete


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Kotlin RAG Example...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.257s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.233s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.307s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.234s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.249s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.223s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.251s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.231s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.241s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.245s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.263s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.287s
    • Function auto-cleaned up by client
✓ Stored performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval ===
User asks: "How do I write memory-safe high-performance database code?"

=== Step 3: Searching Related Context ===
Using hybrid search to find relevant messages from all conversations...


→ Generating embedding for user question...
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.225s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages_kt
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.077s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.474] From conv_performance
     How can I optimize database queries?

  2. [Score: 0.466] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.449] From conv_database_design
     When should I use NoSQL over SQL?

  4. [Score: 0.434] From conv_database_design
     What is database normalization?

  5. [Score: 0.405] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

=== Step 4: Generating Context-Aware Response ===
✓ Context prepared from search results
✓ AI would use this context to generate comprehensive response

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.245s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


→ Executing textSearch()...
  • Collection: rag_messages_kt
  • Query: "ownership system"
  • Limit: 3 results
  • Search method: Full-text with fuzzy matching & stemming
  • No vector embeddings needed - pure keyword search
  ✓ Text search completed in 0.036s

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_rust_programming: The borrow checker enforces Rust's ownership rules at compile time. It ensures that references don't outlive the data they point to and prevents data races by allowing either multiple immutable references or one mutable reference.

=== System Statistics ===

→ Querying database statistics...
  • Using findAllWithLimit() helper - simplified query API

📊 Database Statistics:
  • Total conversations: 4
  • Total messages stored: 13
  • All messages indexed for vector search ✓
  • All messages indexed for text search ✓
  • All messages queryable by metadata ✓

=== Step 8: Dynamic Search Configuration ===
Each conversation can have its own search config...

💡 Conversations can store custom search configurations:
  • Search type: hybrid, text, or vector
  • Relevance thresholds
  • Filter by tags or metadata
  • Collection-specific settings
  • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!


=== 📚 Summary: What This Example Showed ===

🔧 ekoDB Native Capabilities Used:
  ✓ Functions with Embed operation (AI integration)
  ✓ Hybrid Search (text + vector combined)
  ✓ Text Search (full-text with stemming)
  ✓ Automatic embedding generation
  ✓ Cross-collection queries

🚀 New Client Helper Methods:
  • client.embed(text, model) - Generate embeddings
  • client.hybridSearch() - Semantic + keyword search
  • client.textSearch() - Full-text search
  • client.findAllWithLimit() - Query all documents

💡 Key Takeaways:
  1. ekoDB handles AI Functions natively - no external services needed
  2. One-line embedding generation with auto-cleanup
  3. Hybrid search combines semantic understanding + keyword matching
  4. Perfect for RAG: store, search, and retrieve context
  5. All AI capabilities accessible through simple client methods

🎯 Build production RAG systems with ekoDB!
   → Set OPENAI_API_KEY in your ekoDB server environment
   → Use these client helpers to make AI integration simple
   → Scale to millions of documents with native indexing

=== Cleanup ===
✓ Cleanup complete


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ RAG Examples Complete!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

What you just saw across 5 languages:
  ✓ Embeddings generated via ekoDB Functions
  ✓ Hybrid search (semantic + keyword)
  ✓ Text search with stemming
  ✓ Cross-conversation context retrieval
  ✓ Simple client helpers wrapping powerful AI

Mission: AI for All 🚀 - Making RAG accessible to everyone!

✅ All RAG examples complete! Output saved to test-examples-rag.md

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🌐 Testing SWR (Stale-While-Revalidate) Pattern Examples
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📦 Building TypeScript client library...

> @ekodb/ekodb-client@0.27.0 prepare
> npm run build


> @ekodb/ekodb-client@0.27.0 build
> tsc


up to date, audited 44 packages in 544ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.27.0 build
> tsc

✅ TypeScript client built!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running TypeScript SWR Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_ts_82593_1791439286083 (JYAaHqeOCzHbgjDzryIG3NuJ4c4gjlfaExC77TPhmAUo4k1nwlkxYnna8k6fGdZ5nty4EInIyHrkCRiG-hJMtg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "company": {
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets"
          },
          "website": "hildegard.org",
          "phone": "1-770-736-8031 x56442",
          "id": 1,
          "email": "Sincere@april.biz",
          "username": "Bret",
          "name": "Leanne Graham",
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556",
            "street": "Kulas Light",
            "zipcode": "92998-3874"
          }
        }
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 3ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "company": {
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets"
          },
          "website": "hildegard.org",
          "phone": "1-770-736-8031 x56442",
          "id": 1,
          "email": "Sincere@april.biz",
          "username": "Bret",
          "name": "Leanne Graham",
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556",
            "street": "Kulas Light",
            "zipcode": "92998-3874"
          }
        }
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_reviews_ts_82593_1791439286083 (rCrR55pq_LaBs3T2qu9u51xVEIGKRnGVkqGT-MKDMc-tD6zVErXsXKL8PYNAew4CZfE3ST1MTcKp6JbupTqM6A)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "value": {
          "meta": {
            "barcode": "5784719087687",
            "createdAt": "2025-10-09T14:47:01.588Z",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "updatedAt": "2026-05-23T11:27:41.868Z"
          },
          "minimumOrderQuantity": 48,
          "title": "Essence Mascara Lash Princess",
          "category": "beauty",
          "dimensions": {
            "depth": 22.99,
            "height": 13.08,
            "width": 15.14
          },
          "price": 9.99,
          "reviews": [
            {
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 3,
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "reviewerName": "Eleanor Collins",
              "comment": "Would not recommend!"
            },
            {
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "reviewerName": "Lucas Gordon",
              "comment": "Very satisfied!",
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 4
            },
            {
              "reviewerName": "Eleanor Collins",
              "rating": 5,
              "comment": "Highly impressed!",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com"
            }
          ],
          "stock": 99,
          "warrantyInformation": "1 week warranty",
          "rating": 2.56,
          "returnPolicy": "No return policy",
          "id": 1,
          "availabilityStatus": "In Stock",
          "shippingInformation": "Ships in 3-5 business days",
          "brand": "Essence",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "weight": 4,
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "discountPercentage": 10.48,
          "sku": "BEA-ESS-ESS-001",
          "tags": [
            "beauty",
            "mascara"
          ]
        },
        "type": "Object"
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Multi-API data fetched, merged, and cached atomically

=== Why This Is Powerful ===

✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===

1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!

=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: oJIikJ_o-5dlseGfCNy0uY1w6M0au4d6igv33PdXjYzXxJhQmcxyd-YZhuLogvn2A-AIkRkcKHVhWJdfK2i_HQ

Call 1: Cache miss (fetches from API)
Response time: 57ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "username": "Bret",
          "name": "Leanne Graham",
          "id": 1,
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "website": "hildegard.org",
          "phone": "1-770-736-8031 x56442",
          "address": {
            "zipcode": "92998-3874",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "city": "Gwenborough",
            "street": "Kulas Light",
            "suite": "Apt. 556"
          },
          "email": "Sincere@april.biz"
        },
        "type": "Object"
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 2ms (28.5x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "username": "Bret",
          "name": "Leanne Graham",
          "id": 1,
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "website": "hildegard.org",
          "phone": "1-770-736-8031 x56442",
          "address": {
            "zipcode": "92998-3874",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "city": "Gwenborough",
            "street": "Kulas Light",
            "suite": "Apt. 556"
          },
          "email": "Sincere@april.biz"
        }
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!

✅ TypeScript SWR examples complete!
🐍 Building Python client package...
🔧 Ensuring maturin is available in .venv...
🔨 Building wheel...
🍹 Building a mixed python/rust project
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.09s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.27.0-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.27.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.27.0
    Uninstalling ekodb_client-0.27.0:
      Successfully uninstalled ekodb_client-0.27.0
Successfully installed ekodb-client-0.27.0
🧪 Ensuring test dependencies (pytest) in .venv...
✅ Python client package built and installed!
📦 Ensuring Python example dependencies in .venv...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Python SWR Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user_swr_py (uk-yCufRcLQxfmXvmJe69d9YOlpR3-jKNGEJ0hl_yRnsj5z5Hsp_nolUGeA4kUIyL--g39VW2uty2PB-psjotQ)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 182ms
Result: [
  {
    "cached_at": {
      "type": "String",
      "value": "1791439289"
    },
    "data": {
      "type": "Object",
      "value": {
        "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
        "bio": null,
        "blog": "",
        "company": "Linux Foundation",
        "created_at": "2011-09-03T15:26:22Z",
        "email": null,
        "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
        "followers": 326652,
        "followers_url": "https://api.github.com/users/torvalds/followers",
        "following": 0,
        "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
        "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
        "gravatar_id": "",
        "hireable": null,
        "html_url": "https://github.com/torvalds",
        "id": 1024025,
        "location": "Portland, OR",
        "login": "torvalds",
        "name": "Linus Torvalds",
        "node_id": "MDQ6VXNlcjEwMjQwMjU=",
        "organizations_url": "https://api.github.com/users/torvalds/orgs",
        "public_gists": 1,
        "public_repos": 12,
        "received_events_url": "https://api.github.com/users/torvalds/received_events",
        "repos_url": "https://api.github.com/users/torvalds/repos",
        "site_admin": false,
        "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
        "twitter_username": null,
        "type": "User",
        "updated_at": "2026-07-21T17:42:26Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    },
    "id": "torvalds"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 12ms (15.3x faster!)
Result: [
  {
    "cached_at": {
      "type": "String",
      "value": "1791439289"
    },
    "data": {
      "type": "Object",
      "value": {
        "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
        "bio": null,
        "blog": "",
        "company": "Linux Foundation",
        "created_at": "2011-09-03T15:26:22Z",
        "email": null,
        "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
        "followers": 326652,
        "followers_url": "https://api.github.com/users/torvalds/followers",
        "following": 0,
        "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
        "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
        "gravatar_id": "",
        "hireable": null,
        "html_url": "https://github.com/torvalds",
        "id": 1024025,
        "location": "Portland, OR",
        "login": "torvalds",
        "name": "Linus Torvalds",
        "node_id": "MDQ6VXNlcjEwMjQwMjU=",
        "organizations_url": "https://api.github.com/users/torvalds/orgs",
        "public_gists": 1,
        "public_repos": 12,
        "received_events_url": "https://api.github.com/users/torvalds/received_events",
        "repos_url": "https://api.github.com/users/torvalds/repos",
        "site_admin": false,
        "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
        "twitter_username": null,
        "type": "User",
        "updated_at": "2026-07-21T17:42:26Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    }
  }
]
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched_swr_py (pOV-g__CsZOt0SZGKB9EhtLBx5RxS3FCAZOXeUIYo9w-PSMfrDd_rxYQUjXStMb51gbveDl0w2RAEa53COVSaw)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": {
      "type": "String",
      "value": "1791439289"
    },
    "enriched_data": {
      "type": "Object",
      "value": {
        "availabilityStatus": "In Stock",
        "brand": "Essence",
        "category": "beauty",
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "dimensions": {
          "depth": 22.99,
          "height": 13.08,
          "width": 15.14
        },
        "discountPercentage": 10.48,
        "id": 1,
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "meta": {
          "barcode": "5784719087687",
          "createdAt": "2025-10-09T14:47:01.588Z",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
          "updatedAt": "2026-05-23T11:27:41.868Z"
        },
        "minimumOrderQuantity": 48,
        "price": 9.99,
        "rating": 2.56,
        "returnPolicy": "No return policy",
        "reviews": [
          {
            "comment": "Would not recommend!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 3,
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins"
          },
          {
            "comment": "Very satisfied!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 4,
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "reviewerName": "Lucas Gordon"
          },
          {
            "comment": "Highly impressed!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 5,
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins"
          }
        ],
        "shippingInformation": "Ships in 3-5 business days",
        "sku": "BEA-ESS-ESS-001",
        "stock": 99,
        "tags": [
          "beauty",
          "mascara"
        ],
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "title": "Essence Mascara Lash Princess",
        "warrantyInformation": "1 week warranty",
        "weight": 4
      }
    },
    "id": "1"
  }
]
✓ Data fetched, enriched, and cached atomically

=== Why This Is Powerful ===
✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===
1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!

✅ Python SWR examples complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Go SWR Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (SSHTVjervjLM4-5Wd8zmUuHFctJ221QPqJFap7F6utjIqGp96kVn4VE6z1Sy0v-Rbk96mg-fNFnDZCHY7lO4Bg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 73.136ms
Result: [
  {
    "cached_at": {
      "type": "DateTime",
      "value": "2026-10-08T06:01:30+00:00"
    },
    "data": {
      "type": "Object",
      "value": {
        "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
        "bio": null,
        "blog": "",
        "company": "Linux Foundation",
        "created_at": "2011-09-03T15:26:22Z",
        "email": null,
        "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
        "followers": 326652,
        "followers_url": "https://api.github.com/users/torvalds/followers",
        "following": 0,
        "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
        "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
        "gravatar_id": "",
        "hireable": null,
        "html_url": "https://github.com/torvalds",
        "id": 1024025,
        "location": "Portland, OR",
        "login": "torvalds",
        "name": "Linus Torvalds",
        "node_id": "MDQ6VXNlcjEwMjQwMjU=",
        "organizations_url": "https://api.github.com/users/torvalds/orgs",
        "public_gists": 1,
        "public_repos": 12,
        "received_events_url": "https://api.github.com/users/torvalds/received_events",
        "repos_url": "https://api.github.com/users/torvalds/repos",
        "site_admin": false,
        "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
        "twitter_username": null,
        "type": "User",
        "updated_at": "2026-07-21T17:42:26Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    },
    "id": "torvalds"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 3.940958ms (24.3x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (6PUn1aR-zGkA_qnsPI42E-3dou7WzVV9Wpx9FYkOLpSFLlIoyX2mi_gvHpK8xUnIKrGI5PU133hozisPlg6H6Q)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": {
      "type": "DateTime",
      "value": "2026-10-08T06:01:30+00:00"
    },
    "enriched_data": {
      "type": "Object",
      "value": {
        "availabilityStatus": "In Stock",
        "brand": "Essence",
        "category": "beauty",
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "dimensions": {
          "depth": 22.99,
          "height": 13.08,
          "width": 15.14
        },
        "discountPercentage": 10.48,
        "id": 1,
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "meta": {
          "barcode": "5784719087687",
          "createdAt": "2025-10-09T14:47:01.588Z",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
          "updatedAt": "2026-05-23T11:27:41.868Z"
        },
        "minimumOrderQuantity": 48,
        "price": 9.99,
        "rating": 2.56,
        "returnPolicy": "No return policy",
        "reviews": [
          {
            "comment": "Would not recommend!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 3,
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins"
          },
          {
            "comment": "Very satisfied!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 4,
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "reviewerName": "Lucas Gordon"
          },
          {
            "comment": "Highly impressed!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 5,
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins"
          }
        ],
        "shippingInformation": "Ships in 3-5 business days",
        "sku": "BEA-ESS-ESS-001",
        "stock": 99,
        "tags": [
          "beauty",
          "mascara"
        ],
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "title": "Essence Mascara Lash Princess",
        "warrantyInformation": "1 week warranty",
        "weight": 4
      }
    },
    "id": "1"
  }
]
✓ Data fetched, enriched, and cached atomically

=== Why This Is Powerful ===
✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===
1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!

✅ Go SWR examples complete!
🛠️  Building client library...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
✅ Client build complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Rust SWR Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.54s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (A7zbCfsDOKN2SHrYnMeDGcI86BSwJe2vjJr4UOiRKAh-ogXLnbP3xHUtgGo2lX38oc26jQM9sfhxC5lkowIvcg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 201ms
Result: {
  "data": {
    "type": "Object",
    "value": {
      "node_id": "MDQ6VXNlcjEwMjQwMjU=",
      "site_admin": false,
      "following": 0,
      "bio": null,
      "type": "User",
      "twitter_username": null,
      "name": "Linus Torvalds",
      "followers_url": "https://api.github.com/users/torvalds/followers",
      "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
      "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
      "gravatar_id": "",
      "hireable": null,
      "location": "Portland, OR",
      "login": "torvalds",
      "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
      "company": "Linux Foundation",
      "url": "https://api.github.com/users/torvalds",
      "public_repos": 12,
      "followers": 326652,
      "repos_url": "https://api.github.com/users/torvalds/repos",
      "updated_at": "2026-07-21T17:42:26Z",
      "user_view_type": "public",
      "organizations_url": "https://api.github.com/users/torvalds/orgs",
      "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
      "created_at": "2011-09-03T15:26:22Z",
      "blog": "",
      "received_events_url": "https://api.github.com/users/torvalds/received_events",
      "email": null,
      "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
      "html_url": "https://github.com/torvalds",
      "public_gists": 1,
      "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
      "id": 1024025
    }
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 9ms (22.3x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (8KABn2vmjvxJ713Vwm0oNUFU6h8v2nLVwAIAmI78kN-ZiVZQcxhaubsdFl8vx-7ss3h_Ua0F8DhY86zoQzUeIA)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "enriched_data": {
    "type": "Object",
    "value": {
      "stock": 99,
      "availabilityStatus": "In Stock",
      "price": 9.99,
      "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
      "images": [
        "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
      ],
      "returnPolicy": "No return policy",
      "reviews": [
        {
          "comment": "Would not recommend!",
          "rating": 3,
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerName": "Eleanor Collins"
        },
        {
          "reviewerEmail": "lucas.gordon@x.dummyjson.com",
          "reviewerName": "Lucas Gordon",
          "rating": 4,
          "date": "2025-04-30T09:41:02.053Z",
          "comment": "Very satisfied!"
        },
        {
          "date": "2025-04-30T09:41:02.053Z",
          "comment": "Highly impressed!",
          "rating": 5,
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "reviewerName": "Eleanor Collins"
        }
      ],
      "title": "Essence Mascara Lash Princess",
      "brand": "Essence",
      "minimumOrderQuantity": 48,
      "weight": 4,
      "sku": "BEA-ESS-ESS-001",
      "warrantyInformation": "1 week warranty",
      "discountPercentage": 10.48,
      "rating": 2.56,
      "shippingInformation": "Ships in 3-5 business days",
      "tags": [
        "beauty",
        "mascara"
      ],
      "category": "beauty",
      "dimensions": {
        "depth": 22.99,
        "height": 13.08,
        "width": 15.14
      },
      "meta": {
        "updatedAt": "2026-05-23T11:27:41.868Z",
        "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
        "createdAt": "2025-10-09T14:47:01.588Z",
        "barcode": "5784719087687"
      },
      "id": 1,
      "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp"
    }
  }
}
✓ Data fetched, enriched, and cached atomically

=== Why This Is Powerful ===
✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===
1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!

✅ Rust SWR examples complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Kotlin SWR Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR function: swr_fetch_github_user_kt_1791439333409 (rWuPnXd8L8FBwWqJ7y4a5Yio11njpYBtToN10xa3Gb2NDL7DJWhmKkFt69TEO7ZVfOcRHc5PVkQIcccodOQ_2Q)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 80ms
Result: [{"cached_at":{"type":"String","value":"1791439333419"},"data":{"type":"Object","value":{"following":0,"events_url":"https://api.github.com/users/torvalds/events{/privacy}","following_url":"https://api.github.com/users/torvalds/following{/other_user}","node_id":"MDQ6VXNlcjEwMjQwMjU=","organizations_url":"https://api.github.com/users/torvalds/orgs","location":"Portland, OR","public_gists":1,"blog":"","gravatar_id":"","starred_url":"https://api.github.com/users/torvalds/starred{/owner}{/repo}","gists_url":"https://api.github.com/users/torvalds/gists{/gist_id}","created_at":"2011-09-03T15:26:22Z","user_view_type":"public","repos_url":"https://api.github.com/users/torvalds/repos","subscriptions_url":"https://api.github.com/users/torvalds/subscriptions","type":"User","public_repos":12,"site_admin":false,"followers_url":"https://api.github.com/users/torvalds/followers","hireable":null,"bio":null,"followers":326652,"url":"https://api.github.com/users/torvalds","name":"Linus Torvalds","received_events_url":"https://api.github.com/users/torvalds/received_events","company":"Linux Foundation","login":"torvalds","twitter_username":null,"updated_at":"2026-07-21T17:42:26Z","html_url":"https://github.com/torvalds","avatar_url":"https://avatars.githubusercontent.com/u/1024025?v=4","id":1024025,"email":null}},"id":"torvalds"}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 7ms
Cache hit was 11.4x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment function: swr_fetch_product_kt_1791439333409 (qr0sHgy5PeiBUq1KnzRslAixiXoaja5WFGjzSlaFOXqIahsR2ueAVNlFgoxmmfzUq6DMHC6U-BZ6aoc8cgEQ-Q)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"enriched_data":{"type":"Object","value":{"sku":"BEA-ESS-ESS-001","stock":99,"warrantyInformation":"1 week warranty","title":"Essence Mascara Lash Princess","brand":"Essence","rating":2.56,"returnPolicy":"No return policy","tags":["beauty","mascara"],"discountPercentage":10.48,"dimensions":{"height":13.08,"width":15.14,"depth":22.99},"price":9.99,"reviews":[{"date":"2025-04-30T09:41:02.053Z","rating":3,"reviewerEmail":"eleanor.collins@x.dummyjson.com","comment":"Would not recommend!","reviewerName":"Eleanor Collins"},{"reviewerName":"Lucas Gordon","date":"2025-04-30T09:41:02.053Z","reviewerEmail":"lucas.gordon@x.dummyjson.com","rating":4,"comment":"Very satisfied!"},{"date":"2025-04-30T09:41:02.053Z","rating":5,"reviewerName":"Eleanor Collins","comment":"Highly impressed!","reviewerEmail":"eleanor.collins@x.dummyjson.com"}],"meta":{"createdAt":"2025-10-09T14:47:01.588Z","qrCode":"https://cdn.dummyjson.com/public/qr-code.png","barcode":"5784719087687","updatedAt":"2026-05-23T11:27:41.868Z"},"weight":4,"thumbnail":"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp","images":["https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"],"availabilityStatus":"In Stock","minimumOrderQuantity":48,"description":"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.","id":1,"category":"beauty","shippingInformation":"Ships in 3-5 business days"}},"id":"1","enriched_at":{"value":"1791439333636","type":"String"}}]
✓ Data fetched, enriched, and cached atomically

=== Why This Is Powerful ===
✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===
1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!


BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
✅ Kotlin SWR examples complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ All SWR Examples Complete!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

What you just saw - ekoDB as Edge Cache:
  ✓ FindById → Check cache
  ✓ If/Else → Conditional execution
  ✓ HttpRequest → External API calls
  ✓ Insert with TTL → Auto-expiring cache
  ✓ Sub-millisecond cache hits
  ✓ No Redis, no CDN, no cache invalidation logic needed

Your DATABASE is your EDGE! 🚀


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🔗 Testing Function Composition Examples
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🛠️  Building client library...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
✅ Client build complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Rust Function Composition Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_function_composition`
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user_rs
✅ Saved composed function: get_user_wrapper_rs (calls fetch_user_rs + projects fields)

📊 Result from composed function:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user_rs (uses KV)
✅ Saved SWR function using composition: swr_user_rs

First call (cache miss - will fetch from API):
   ⏱️  Duration: 71.053459ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "email": "Sincere@april.biz",
      "id": 1,
      "website": "hildegard.org",
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net",
 ...

Second call (cache hit - from cache):
   ⏱️  Duration: 3.271291ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "name": "Leanne Graham",
      "company": {
        "bs": "harness real-time e-markets",
        "name": "Romaguera-Crona",
        "catchPhra...
   🚀 Cache speedup: 21.7x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user_rs
✅ Level 2 function: fetch_slim_user_rs (calls validate_user_rs)
✅ Level 3 function: get_verified_user_rs (calls fetch_slim_user_rs)

📊 Result from 3-level nested composition:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_enriched_user: Used in 50 workflows
   - get_engineering_user: Specific workflow


✅ Cleanup complete
✅ All composition examples completed!
✅ Rust function composition examples complete!
📦 Building TypeScript client library...

> @ekodb/ekodb-client@0.27.0 prepare
> npm run build


> @ekodb/ekodb-client@0.27.0 build
> tsc


up to date, audited 44 packages in 581ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.27.0 build
> tsc

✅ TypeScript client built!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running TypeScript Function Composition Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"value":"User 1","type":"String"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 68ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "phone": "1-770-736-8031 x56442",
      "name": "Leanne Graham",
      "company": {
        "bs": "harness real-time e-markets",
        "name": "Romaguera-Crona",
...

Second call (cache hit - from cache):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "phone": "1-770-736-8031 x56442",
      "name": "Leanne Graham",
      "company": {
        "bs": "harness real-time e-markets",
        "name": "Romaguera-Crona",
...
   🚀 Cache speedup: 22.7x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ All composition examples completed!
✅ TypeScript function composition examples complete!
🐍 Building Python client package...
🔧 Ensuring maturin is available in .venv...
🔨 Building wheel...
🍹 Building a mixed python/rust project
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.09s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.27.0-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.27.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.27.0
    Uninstalling ekodb_client-0.27.0:
      Successfully uninstalled ekodb_client-0.27.0
Successfully installed ekodb-client-0.27.0
🧪 Ensuring test dependencies (pytest) in .venv...
✅ Python client package built and installed!
📦 Ensuring Python example dependencies in .venv...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Python Function Composition Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user_py
✅ Saved composed function: get_user_wrapper_py (calls fetch_user_py + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type": "String", "value": "User 1"}
   Department: {"type": "String", "value": "engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user_py (uses KV)
✅ Saved SWR function using composition: swr_user_py

First call (cache miss - will fetch from API):
   ⏱️  Duration: 64.8ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...

Second call (cache hit - from cache):
   ⏱️  Duration: 4.2ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 15.4x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user_py
✅ Level 2 function: fetch_slim_user_py (calls validate_user_py)
✅ Level 3 function: get_verified_user_py (calls fetch_slim_user_py)

📊 Result from 3-level nested composition:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ Cleanup complete
✅ All composition examples completed!
✅ Python function composition examples complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Go Function Composition Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 73.941375ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "type": "Object",
          "value": {
            "address": {
              "city": "Gwenborough",
              "geo": {
                "lat": "-37.3159",
          ...

Second call (cache hit - from cache):
   ⏱️  Duration: 4.687291ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "type": "Object",
          "value": {
            "address": {
              "city": "Gwenborough",
              "geo": {
                "lat": "-37.3159",
          ...
   🚀 Cache speedup: 18.2x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ All composition examples completed!
✅ Go function composition examples complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running JavaScript Function Composition Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"value":"User 1","type":"String"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 53ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "address": {
        "zipcode": "92998-3874",
        "street": "Kulas Light",
        "suite": "Apt. 556",
        "geo": {
          "lng": "81.1496",
          "...

Second call (cache hit - from cache):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "address": {
        "zipcode": "92998-3874",
        "street": "Kulas Light",
        "suite": "Apt. 556",
        "geo": {
          "lng": ...
   🚀 Cache speedup: 26.5x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ All composition examples completed!
✅ JavaScript function composition examples complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ All Function Composition Examples Complete!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

What you just saw - CallFunction composability:
  ✓ Reusable Function building blocks
  ✓ Functions calling other Functions
  ✓ Clean SWR patterns via composition
  ✓ Multi-level nesting (arbitrary depth)
  ✓ No code duplication
  ✓ Single source of truth

Build complex workflows from simple pieces! 🚀


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📡 WebSocket Subscription Tests
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

These tests verify real-time WebSocket subscriptions by:
  1. Authenticating and connecting via WebSocket
  2. Subscribing to a collection
  3. Inserting records via REST to trigger notifications
  4. Verifying MutationNotification push messages arrive
  5. Unsubscribing and cleaning up

🛠️  Building client library...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
✅ Client build complete!

🦀 Rust WebSocket Subscription Test...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_websocket_subscribe`
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_rs' ===
✓ Subscribed (subscription_id: sub_fe01631e7d9c415c8bdbd4e8e7bcbbae)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: U8zM8Iz3teh7wtmoGppj9s320jiWQ5YDfVSrqdpAD0IhIqeKh6wlGNQwX_rj9ja13fpAoHSQH8QxoKJj9wF2yg

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_rs"
     Record IDs: ["U8zM8Iz3teh7wtmoGppj9s320jiWQ5YDfVSrqdpAD0IhIqeKh6wlGNQwX_rj9ja13fpAoHSQH8QxoKJj9wF2yg"]
     Timestamp:  "2026-10-08T06:05:36.635423+00:00"

Inserting record 2...
✓ Inserted: _YZiZ3-gmM_1hxnssnVDnXFOoy5efxLCZDe5P10pN8fRH9IPntBodKLHQyzDRI391qC0US2FReFqLIq5_5WPgA

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["_YZiZ3-gmM_1hxnssnVDnXFOoy5efxLCZDe5P10pN8fRH9IPntBodKLHQyzDRI391qC0US2FReFqLIq5_5WPgA"]

=== Unsubscribing ===
✓ Unsubscribed

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_rs'

✓ WebSocket subscription example completed successfully
✅ Rust subscription test complete!

🔷 Go WebSocket Subscription Test...
=== WebSocket Subscription Example (Go) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_go' ===
✓ Subscribed (subscription_id: sub_452f84c8570b4d57b17193c657458b89)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: UZUr3cXKLsC5V6QbQRPsPSRgBq66k0qpUqRvsUwWyg_NWqhO6d-vCha8ggXTOqZtRiFZNd2vetOKhC-MLICL5w

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_go
     Record IDs: [UZUr3cXKLsC5V6QbQRPsPSRgBq66k0qpUqRvsUwWyg_NWqhO6d-vCha8ggXTOqZtRiFZNd2vetOKhC-MLICL5w]
     Timestamp:  2026-10-08T06:05:37.095989+00:00

Inserting record 2...
✓ Inserted: Ug_5HtD8YYwWq2UrzZME3iIEjIQ17tNeLdf09JsblHhdes16IPMf5a2x6DNDisN3KxomOFKDNvTwrsMu_rzPXg

  📡 Notification received:
     Event:      insert
     Record IDs: [Ug_5HtD8YYwWq2UrzZME3iIEjIQ17tNeLdf09JsblHhdes16IPMf5a2x6DNDisN3KxomOFKDNvTwrsMu_rzPXg]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
✅ Go subscription test complete!
📦 Ensuring Python example dependencies in .venv...

🐍 Python WebSocket Subscription Test...
=== WebSocket Subscription Example (Python) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_py' ===
✓ Subscribed (subscription_id: sub_8cdca383e44a4ef084c1eb9aea6e70b6)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: wijskHkhRbJQXkYdAFQv-VqTjxdCzaXIc1hKoriIpeAVAYFBp8LDN7Hg6hMIJywUex0MQ6345z3YXwsxa5jd3w

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: wijskHkhRbJQXkYdAFQv-VqTjxdCzaXIc1hKoriIpeAVAYFBp8LDN7Hg6hMIJywUex0MQ6345z3YXwsxa5jd3w
     Timestamp:  2026-10-08T06:05:37.633970+00:00

Inserting record 2...
✓ Inserted: so6WilSjrTANuTmuSDg8PvhjY9QsyCPU9_ruoH1GiWN0sL_vuBW7dS3zR14J6T-oH5deYSNrWWw9s7AxPxjVZg

  📡 Notification received:
     Event:      insert
     Record IDs: so6WilSjrTANuTmuSDg8PvhjY9QsyCPU9_ruoH1GiWN0sL_vuBW7dS3zR14J6T-oH5deYSNrWWw9s7AxPxjVZg

=== Unsubscribing ===
✓ Unsubscribed: {'collection': 'ws_subscribe_example_py', 'found': True, 'unsubscribed': True}

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_py'

✓ WebSocket subscription example completed successfully
✅ Python subscription test complete!
📦 Building TypeScript client library...

> @ekodb/ekodb-client@0.27.0 prepare
> npm run build


> @ekodb/ekodb-client@0.27.0 build
> tsc


up to date, audited 44 packages in 591ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.27.0 build
> tsc

✅ TypeScript client built!

📘 TypeScript WebSocket Subscription Test...
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_7beea3a9dda0443bbddbe412f6039e1e)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: IDhDFIoqImHRfBx2ePMwtacI03wSECe8gJ_MWx-ZVH4kc1LqtYea_MSML452D2CH7_pPAWr-1RN2K5GCdwJvkw
  📡 Notification received for IDhDFIoqImHRfBx2ePMwtacI03wSECe8gJ_MWx-ZVH4kc1LqtYea_MSML452D2CH7_pPAWr-1RN2K5GCdwJvkw

Inserting another record...
✓ Inserted record: S5L-BiwnpbHMLM8AfIQvuGqu-OHFM1U2D8Woec8e-JIDeyWLGkhI-J8Hz1PaTCjP3f9JTdkDOrVP7X-s9dbFcg
  📡 Notification received for S5L-BiwnpbHMLM8AfIQvuGqu-OHFM1U2D8Woec8e-JIDeyWLGkhI-J8Hz1PaTCjP3f9JTdkDOrVP7X-s9dbFcg

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_ts'
✅ TypeScript subscription test complete!

🟣 Kotlin WebSocket Subscription Test...
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
=== WebSocket Subscription Example (Kotlin) ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_kt' ===
✓ Subscribed (subscription_id: sub_ad2dc66ada74456f85d587e13ff9f0fb)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "yPXecaMz88idXXzOepTZULnOkCL-tn9GhzkfhblY6YOPh8h1nN7LnPcjZeVoRrpzRVIF_A5F0c2KxuEJ0C5bUQ"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_kt"
     Record IDs: ["yPXecaMz88idXXzOepTZULnOkCL-tn9GhzkfhblY6YOPh8h1nN7LnPcjZeVoRrpzRVIF_A5F0c2KxuEJ0C5bUQ"]
     Timestamp:  "2026-10-08T06:05:44.262608+00:00"

Inserting record 2...
✓ Inserted: "zG10FoD8DKBP5y1JR-DC_gD8Vt14Py5dQtCTA0thR8t0hvBaxHUPDSb9NB5RcOlJxrArdedCGyVGs2liJRa-FA"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["zG10FoD8DKBP5y1JR-DC_gD8Vt14Py5dQtCTA0thR8t0hvBaxHUPDSb9NB5RcOlJxrArdedCGyVGs2liJRa-FA"]

=== Unsubscribing ===
✓ Unsubscribed: {"payload":{"data":{"collection":"ws_subscribe_example_kt","found":true,"unsubscribed":true}},"type":"Success"}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_kt'

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
✅ Kotlin subscription test complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ All WebSocket Subscription Tests Passed!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
