make test-examples-rust
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
