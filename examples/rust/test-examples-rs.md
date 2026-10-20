make test-examples-rust
🧪 Running Rust examples (direct HTTP/WebSocket)...
   Compiling ekodb_client v0.30.1 (ekoDB/ekodb-client/ekodb_client)
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.44s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("HKhBrQJJzEK8kG7JGxbYlT71RRow4NT9OpI7jC0_EauSsXFj3G2jSKAhJlQgl89R7CqYChCWfAE0m0n1mr3xXg")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("HKhBrQJJzEK8kG7JGxbYlT71RRow4NT9OpI7jC0_EauSsXFj3G2jSKAhJlQgl89R7CqYChCWfAE0m0n1mr3xXg"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("HKhBrQJJzEK8kG7JGxbYlT71RRow4NT9OpI7jC0_EauSsXFj3G2jSKAhJlQgl89R7CqYChCWfAE0m0n1mr3xXg"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("HKhBrQJJzEK8kG7JGxbYlT71RRow4NT9OpI7jC0_EauSsXFj3G2jSKAhJlQgl89R7CqYChCWfAE0m0n1mr3xXg"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.46s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: EcwYafXmJ40DQlml3UXCelQ_fgf942pyyeKrRYYWWKv8CKOIJgQcJxNpx2dND0G-4vILL7_N2_r2Dvj_VRvkLQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1792523575324962000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "EcwYafXmJ40DQlml3UXCelQ_fgf942pyyeKrRYYWWKv8CKOIJgQcJxNpx2dND0G-4vILL7_N2_r2Dvj_VRvkLQ",
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.96s
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.94s
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.91s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "TK5RtJ-s6RAKsb2vXkRsJVRYz3BS3CMkPRegSDhqmrTer1kXM7LMgM2_0nb4Wxz7jOfUIVdz0_7KbpfjUiIIVA"

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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.95s
     Running `target/debug/examples/document_ttl`
=== ekoDB Document TTL Example (Rust) ===
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 84SD9ZloVN8cntgwbFZGz9yji9IN1WlPz4ewhsAHMKcrHSk-0VpwL2CBj018myi53mB82YD0yy0F490G5Ki4qA

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: ZwULfat-7ANkQcmFg7vhznWG_gpcla38b_VS2UBXBJZveb48u5EEzX2bEX-7K6qjKPMs89_hiTDjf88jIiWZzA

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: 1KGcEVmQq65vgbXG8rGw11LQk1wmlXDz3RyLwoFmwgcRqkH6IRpyRFSAkWxR_034s9PgTYXnrUfw-jwiVBAPzw

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.41s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("eLY4Ey9hL37_wv-yrd7Hk7ijtEeIyjJWLLYxXDk8zlns83k9bpWTHK_16LhT6vBAWFO10QAudoy91RbHJfIiqQ")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record via WebSocket

Record 1:
  id: "eLY4Ey9hL37_wv-yrd7Hk7ijtEeIyjJWLLYxXDk8zlns83k9bpWTHK_16LhT6vBAWFO10QAudoy91RbHJfIiqQ"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-10-20T20:16:55.067308Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.67s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Functions Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: sYyP9iUTN2AKnkoaVh-j3BMtmwTxulUOW1Wc4YiPwyppCO0ZD_WI-AYRUPnEk6OKJ2npED2G-2E4ultOzBI1dg
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: bKr1IUoU-VANFZJDLS9P3GzsgSKdY8WOfAY0RkFLCGJiX2mhCY-BaexmY5L-0RI0xdTRBchJoWGUMjHRmSsbRg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Function saved: eJ1EX6JsvEiGoEnSPQsIBBOiW7U54yYCIFEvcjrBgfogPZht2XO9Beq9levOBtYgxUMgM3n_UcoL5H5l3imc1g
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: O4lR1KdCjOzB4A62HKy4jeD2Dy4XcilDGAWeCIp9NPwQJ6n276rtn5T-dMst8HAjorZ-m-sZPpdfifTZT9h6pw
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.97s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 7er8zcHILTDpqdmwHFRhFBx7aKPNjpoM7ZezsgOsSvBzq87z97gekpcMXeKFdj5zsQkGDpdcTTMY7Qid0ZFXMg
Created Bob: $500 - ID: 4Vg2EEF_to7_G2sZCSiE8_0gi_Jc89ot2KIS0NbSXn2WxFawAGIc5l1Aw-IEYY3q4L57WEJ_1SyBz6GhiIAWfw

=== Example 1: Begin Transaction ===
Transaction ID: c659ba7a-41a2-4297-aa43-88ea42301e1f

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
New transaction: 35080570-7655-40d6-abcc-52f513900f2f
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ Rust direct examples complete!
🛠️  Building client library...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
✅ Client build complete!
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.28s
     Running `target/debug/examples/client_admin`
Admin permissions read successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.32s
     Running `target/debug/examples/client_advanced_crud`
=== ekoDB Advanced CRUD Example (Rust) ===

--- Inserting base document ---
Inserted: Widget Counter (id: gR_5g5jDN8rTtqwGlDdZaF-fxVA0U95xp7QwvBQTe57xUCu2KrpZ_1FZIksMkqch4DgH4nYHbo5DMG4HbLEJIQ)

--- update_with_action: increment views by 10 ---
views after increment: Some(Object({"value": Integer(110), "type": String("Integer")}))

--- update_with_action: decrement views by 3 ---
views after decrement: Some(Object({"value": Integer(107), "type": String("Integer")}))

--- update_with_action: multiply score by 2 ---
score after multiply: Some(Object({"value": Float(9.0), "type": String("Float")}))

--- update_with_action: push 'ekodb' to tags ---
tags after push: Some(Object({"value": Array([String("rust"), String("database"), String("ekodb")]), "type": String("Array")}))

--- update_with_action: append '-suffix' to label ---
label after append: Some(Object({"type": String("String"), "value": String("prefix-suffix")}))

--- update_with_action: pop last element from tags ---
tags after pop: Some(Object({"type": String("Array"), "value": Array([String("rust"), String("database")])}))

--- update_with_action: remove 'rust' from tags ---
tags after remove: Some(Object({"value": Array([String("database")]), "type": String("Array")}))

--- update_with_action_sequence: increment views + push tag + append label ---
views after sequence: Some(Object({"value": Integer(157), "type": String("Integer")}))
tags after sequence: Some(Object({"type": String("Array"), "value": Array([String("database"), String("batch")])}))
label after sequence: Some(Object({"type": String("String"), "value": String("prefix-suffix-v2")}))

--- Cleanup ---
Deleted collection

=== All advanced CRUD operations completed ===
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.64s
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.48s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 2hUU7aRLdOoLneDPpgtlskQNSixYr1B79c4nSvwF-7jjl6X7K4PUyucS33ThbcKcBaaQbZaFL8U6fg9p9X4jpg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or assistance, feel free to ask!

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
✓ Created second session: 4HYG2QyuN0Lse2HAk72BIkEom1Ab4f-iEIGlk3JJ-zqDN1Y1JYTZ1-_-LmO4W1r_ShcssCbUGYrABp_rHj_8Lw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: 4HYG2QyuN0Lse2HAk72BIkEom1Ab4f-iEIGlk3JJ-zqDN1Y1JYTZ1-_-LmO4W1r_ShcssCbUGYrABp_rHj_8Lw
✓ Deleted chat session: 2hUU7aRLdOoLneDPpgtlskQNSixYr1B79c4nSvwF-7jjl6X7K4PUyucS33ThbcKcBaaQbZaFL8U6fg9p9X4jpg
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.37s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 6lYSUOK2nyS6n2N8AuhZywfIvW6pivfpeZzEoPIPJm3vhmBQaGaec9eG5K2d59jtnH2He7z8ZQnwqZNrOzIxNg

=== Sending Chat Message ===
Message ID: wrBxSZsZk-dOKv85VGNRcNOoccacu7KUAAnfpx371VqdU4b4JHE3uwKUpmOy4goB00EVQ3UzaD76GDS04q52qw

=== AI Response ===
Response 1: ekoDB is a high-performance database that integrates intelligent caching, real-time capabilities, and AI features. It allows users to query their database using natural language and provides AI-powered responses with relevant context.

Key features of ekoDB include:

1. **AI Chat Integration**: The chat feature enables natural language queries to the database, producing AI-powered responses.
2. **Search Capabilities**: ekoDB supports various search methods such as full-text search, vector search, and hybrid search, which include automatic context retrieval to enhance query results.

This combination makes ekoDB a robust and flexible solution for managing data efficiently.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["content", "category", "title"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("3Vogb11r53kSFcJkirZIg9qMMvIFljhgyhPxqqY1oMsPq-jgdgULL_5nMPgJwD4eYTQvAhopfRIswkGLgdEY2Q"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["category", "content", "title"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("ROIjPzyaeP1jaDcPNTxE1flh61XHH2tpHLKDLEJD374KigRU_9a6c4DinzAm8G19jt9OXGEcPfJMAMimVywnHg"), "title": String("Search Features")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("eFyN_9Y6En8wK1SW2zUMKNZCqBtjsNnKcymDyafwMCAXwoastVNGwp6D33XTy8q6OGpAgHrBJvCUkOglXArJMQ"), "title": String("Introduction to ekoDB")}

Execution Time: 2897ms

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.27s
     Running `target/debug/examples/client_chat_message_stream`
=== ekoDB Chat Message Stream (SSE) Example (Rust) ===

Created session: tKUCtm6H1pwFP_qMXAA3sZ6ubpvjd7WpzdgJm2h2yWRe8e2YhiXBTMZqYZPJrtTtedG8Nt5Tj3DJ4_H-bbFbpQ

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}**ekoDB** is an aggregated knowledge database developed by Eko Health, primarily focused on cardiovascular health and digital auscultation. It contains a vast collection of heart and lung sounds, often paired with diagnostic annotations and clinical context. The purpose of ekoDB is to facilitate machine learning research, improve medical device algorithms, and advance clinical decision support tools in cardiology and respiratory care.

Key features of **ekoDB**:

- **Large-scale clinical dataset:** It includes tens of thousands of annotated heart and lung sound recordings from diverse patient populations across multiple clinical sites.
- **Rich metadata:** Each recording is associated with clinical labels (such as heart murmurs, arrhythmias, or respiratory conditions), demographic data, and sometimes imaging results (like echocardiograms).
- **Research and development:** The dataset has been used to develop and validate artificial intelligence (AI) algorithms for detecting cardiovascular diseases, such as murmurs, atrial fibrillation, and other conditions.
- **Academic collaboration:** ekoDB is referenced in peer-reviewed publications and has been made available to researchers under certain conditions for the purpose of advancing medical AI.

In summary, **ekoDB** is a clinical database of digitized bio-signals (mainly heart and lung sounds) instrumental in the advancement of AI-driven stethoscopes and cardiovascular diagnostics.

--- Stream complete ---
Message ID: WCILrqeGXuF8wUr4ipdtMxGYy4BcMuLLKtKBQUefxqac9FkWUy3cDj_-oGzjZYu3YBEmdRQVQ6P7YLdsSpVsxQ
Execution time: 3402ms
Context window: 1000000 tokens

✓ Chat message stream example completed
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.04s
     Running `target/debug/examples/client_chat_models`
✓ Client created

=== Get All Chat Models ===
Available chat models by provider:

OpenAI models (135):
  - gpt-4-turbo-2024-04-09
  - gpt-realtime-2.1-mini
  - gpt-realtime-2025-08-28
  - gpt-4
  - tts-1-hd
  - gpt-realtime-1.5
  - gpt-realtime
  - gpt-5.2-pro
  - gpt-5-chat-latest
  - gpt-3.5-turbo-0125
  - gpt-5.4-2026-03-05
  - gpt-5-nano
  - gpt-5.4-mini
  - o1-pro
  - gpt-5.1-codex
  - tts-1
  - gpt-5-mini
  - gpt-5-mini-2025-08-07
  - omni-moderation-2024-09-26
  - gpt-live-transcribe
  - gpt-5.6-luna
  - omni-moderation-latest
  - gpt-5-2025-08-07
  - o4-mini-2025-04-16
  - gpt-4o-2024-11-20
  - gpt-5.1
  - whisper-1
  - gpt-5-pro
  - gpt-realtime-whisper
  - gpt-5.5-pro
  - gpt-audio-2025-08-28
  - gpt-4o-mini-transcribe-2025-03-20
  - gpt-realtime-2
  - gpt-5-search-api
  - gpt-5.5-2026-04-23
  - chat-latest
  - gpt-realtime-translate
  - gpt-5.4-pro-2026-03-05
  - gpt-5.4-mini-2026-03-17
  - babbage-002
  - gpt-4o-2024-05-13
  - text-embedding-3-large
  - gpt-4o-mini-transcribe-2025-12-15
  - tts-1-1106
  - gpt-5.5-pro-2026-04-23
  - gpt-audio-mini-2025-10-06
  - gpt-5.2-pro-2025-12-11
  - gpt-4-turbo
  - gpt-4o-2024-08-06
  - o3
  - gpt-4o-mini-search-preview-2025-03-11
  - o3-mini-2025-01-31
  - gpt-audio-1.5
  - gpt-image-1.5
  - gpt-5-pro-2025-10-06
  - gpt-5.6-sol
  - gpt-4.1-nano
  - gpt-image-2
  - gpt-3.5-turbo-16k
  - text-embedding-ada-002
  - gpt-realtime-2.1
  - gpt-4.1
  - gpt-5.5
  - gpt-4o-transcribe-diarize
  - gpt-5-codex
  - gpt-4-0613
  - gpt-5.1-2025-11-13
  - gpt-4o-search-preview
  - gpt-4.1-mini
  - gpt-4o-mini
  - gpt-5.4-pro
  - gpt-4o-mini-tts
  - gpt-realtime-mini-2025-12-15
  - gpt-5.1-codex-max
  - gpt-realtime-mini
  - gpt-image-1-mini
  - gpt-5.6-terra
  - gpt-3.5-turbo-instruct-0914
  - o1-2024-12-17
  - sora-2
  - tts-1-hd-1106
  - gpt-4o-mini-tts-2025-12-15
  - gpt-image-1
  - gpt-audio
  - gpt-5.3-codex
  - gpt-5-search-api-2025-10-14
  - gpt-5
  - gpt-4o-mini-transcribe
  - gpt-image-2-2026-04-21
  - gpt-5.2-chat-latest
  - sora-2-pro
  - gpt-3.5-turbo-instruct
  - gpt-transcribe
  - gpt-4.1-nano-2025-04-14
  - o4-mini-deep-research
  - gpt-5.4
  - gpt-4o-search-preview-2025-03-11
  - text-embedding-3-small
  - gpt-5.1-chat-latest
  - gpt-4o-mini-2024-07-18
  - gpt-4.1-2025-04-14
  - gpt-5.3-chat-latest
  - o3-2025-04-16
  - gpt-5.2-2025-12-11
  - chatgpt-image-latest
  - gpt-3.5-turbo
  - gpt-5.2-codex
  - gpt-4o
  - davinci-002
  - gpt-5.4-nano
  - o3-mini
  - o4-mini-deep-research-2025-06-26
  - gpt-audio-mini-2025-12-15
  - o1-pro-2025-03-19
  - gpt-5-nano-2025-08-07
  - gpt-4o-transcribe
  - o4-mini
  - gpt-5.1-codex-mini
  - gpt-3.5-turbo-1106
  - gpt-4o-mini-tts-2025-03-20
  - gpt-5.2
  - gpt-5.4-nano-2026-03-17
  - o1
  - gpt-image-2.5-sunburst-2026-09-08
  - gpt-6-luna
  - gpt-audio-mini
  - gpt-4.1-mini-2025-04-14
  - gpt-image-2.5-flare
  - gpt-4o-mini-search-preview
  - gpt-6-sol
  - gpt-6-astra
  - gpt-6.1-sol
  - gpt-live-1
  - gpt-image-2.5-sunburst
  - gpt-image-2.5-flare-2026-09-08

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

Gemini models (63):
  - gemini-2.5-flash
  - gemini-2.5-pro
  - gemini-2.5-flash-preview-tts
  - gemini-2.5-pro-preview-tts
  - gemma-4-26b-a4b-it
  - gemma-4-31b-it
  - gemini-flash-latest
  - gemini-flash-lite-latest
  - gemini-pro-latest
  - gemini-2.5-flash-lite
  - gemini-2.5-flash-image
  - gemini-3-flash-preview
  - gemini-3.1-pro-preview
  - gemini-3.1-pro-preview-customtools
  - gemini-3.1-flash-lite-preview
  - gemini-3.1-flash-lite
  - gemini-3-pro-image-preview
  - gemini-3-pro-image
  - nano-banana-pro-preview
  - gemini-3.1-flash-image-preview
  - gemini-3.1-flash-image
  - gemini-3.1-flash-lite-image
  - gemini-nano-banana-2.1
  - gemini-3.5-flash
  - gemini-3.5-flash-lite
  - gemini-omni-flash-preview
  - gemini-omni-1.1-flash
  - gemini-3.5-transcribe
  - gemini-3.6-flash
  - gemini-3.7-flash
  - gemini-3.8-flash
  - gemini-4-argon
  - lyria-3-clip-preview
  - lyria-3-pro-preview
  - lyria-3.5
  - gemini-3.1-flash-tts-preview
  - gemini-3.8-flash-tts
  - gemini-3.8-flash-lite-tts
  - gemini-robotics-er-2-preview
  - gemini-2.5-computer-use-preview-10-2025
  - antigravity-preview-05-2026
  - antigravity-preview-09-2026
  - antigravity-preview-latest
  - deep-research-max-preview-04-2026
  - deep-research-preview-04-2026
  - deep-research-pro-preview-12-2025
  - gemini-embedding-001
  - gemini-embedding-2-preview
  - gemini-embedding-2
  - aqa
  - veo-3.1-generate-preview
  - veo-3.1-fast-generate-preview
  - veo-3.1-lite-generate-preview
  - gemini-3.5-transcribe-live
  - gemini-2.5-flash-native-audio-latest
  - gemini-2.5-flash-native-audio-preview-09-2025
  - gemini-2.5-flash-native-audio-preview-12-2025
  - gemini-3.1-flash-live-preview
  - gemini-3.8-live
  - gemini-3.8-live-extended-thinking
  - gemini-robotics-er-2-streaming-preview
  - gemini-3.5-live-translate-preview
  - lyria-realtime-exp

Provider status:
  anthropic: ok 14 models
  gemini: ok 63 models
  openai: ok 135 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Models for Specific Provider ===

openai models (135):
  - gpt-4-turbo-2024-04-09
  - gpt-realtime-2.1-mini
  - gpt-realtime-2025-08-28
  - gpt-4
  - tts-1-hd
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.42s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 3FAQP0_tWKOzw6UDf2jReBM6EkYBL9Mu2oKMalhhmZ0d_71B2htoW619V481Yr_lC5g9LJbRZb7XhkErKjrnfw
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Product Name**: ekoDB
- **Description**: A high-performance database product with AI capabilities
- **Price**: $99

Let me know if you need more information!

✓ Message 2 sent
  Response: The price of the product ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: vJ74IerlDDMIAIypK0eU4nKQVNWCoRLCr06DGkFERceLdNI6WizQQEHellmVXQyu6fOmrZzWGklX0O_Fdr1aBg
  Parent: 3FAQP0_tWKOzw6UDf2jReBM6EkYBL9Mu2oKMalhhmZ0d_71B2htoW619V481Yr_lC5g9LJbRZb7XhkErKjrnfw

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: vJ74IerlDDMIAIypK0eU4nKQVNWCoRLCr06DGkFERceLdNI6WizQQEHellmVXQyu6fOmrZzWGklX0O_Fdr1aBg (Untitled)
  Session 2: 3FAQP0_tWKOzw6UDf2jReBM6EkYBL9Mu2oKMalhhmZ0d_71B2htoW619V481Yr_lC5g9LJbRZb7XhkErKjrnfw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: vJ74IerlDDMIAIypK0eU4nKQVNWCoRLCr06DGkFERceLdNI6WizQQEHellmVXQyu6fOmrZzWGklX0O_Fdr1aBg

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.93s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "Y5LZM9cBoBn2X3kZNuWCToUTIjI2IA4N4YSYx2tc1yw6ms81ewGXnk0gR_PHHgFJ12xngGC0bnoxpi8lONXU4A"

=== List Collections ===
Total collections: 7
Sample collections: ["chat_configurations__ek0_testing", "agent_function_versions__ek0_testing", "functions__ek0_testing", "chat_turns__ek0_testing", "client_collection_management_rust"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.26s
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
  - chat_configurations__ek0_testing
  - agent_function_versions__ek0_testing
  - functions__ek0_testing
  - collection_utils_test_rust <-- our test
  - chat_turns__ek0_testing
  - audit__ek0_testing
  - chat_messages__ek0_testing

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_rust'

✓ Collection Utilities example complete
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.32s
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.39s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("QPCH-jHKsMO9RzorTsrb5Iz_c8_iEkLaQvWLdXrQ-zc1M-t5_JpGoOewgW7zA9Z6AjV4QwGUG9swK60-Z7mrMg")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("bob-user-id")})
✓ Second upsert (update): Record({"email": Object({"value": String("bob.smith@newdomain.com"), "type": String("String")}), "id": String("bob-user-id"), "age": Object({"type": String("Integer"), "value": Integer(36)}), "name": Object({"type": String("String"), "value": String("Bob Smith")})})

=== Find One Operation ===
✓ Found user by email: Record({"email": Object({"value": String("alice@example.com"), "type": String("String")}), "id": String("QPCH-jHKsMO9RzorTsrb5Iz_c8_iEkLaQvWLdXrQ-zc1M-t5_JpGoOewgW7zA9Z6AjV4QwGUG9swK60-Z7mrMg"), "age": Object({"type": String("Integer"), "value": Integer(28)}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "name": Object({"type": String("String"), "value": String("Alice Johnson")})})
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.27s
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.36s
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.39s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "OZVGpSDLcmWfQaKVMVQExvWQDs7x3pO9Mts2-O-_vStiRCoGe4cHr39UNyPa1cHqfGXidMDy-2qtUlQuHGGs9A"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("1x9BKd3Jg5LKBiRJgdmGqPSXYke_FbZVPHq0zC7JVAlcGAk5rWW1mD_7WPUzNJwzFh_iSr-_KpBxYtpQi39A5A"))

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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.00s
     Running `target/debug/examples/client_edge_cache`
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: u-N8QtwyV7K-1PxEj9MVnFHH70INij8gF2ye5vpmZKA4SENA_li7rfE-hSW9qGE6JAjIIK8RDlQ8E62hx_Q5ow

Call 1: Cache lookup
Response time: 0ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 0ms
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.25s
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
   ⏱️  Duration: 92.496958ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "id": 1,
      "address": {
        "geo": {
          "lat": "-37.3159",
          "lng": "81.1496"
        },
        "street": "Kulas Light",
        "suite": "A...

Second call (cache hit - from cache):
   ⏱️  Duration: 4.393166ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "company": {
        "bs": "harness real-time e-markets",
        "catchPhrase": "Multi-layered client-server neural-net",
        "name": "Romaguera-Crona"
      }...
   🚀 Cache speedup: 21.1x faster!

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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.30s
     Running `target/debug/examples/client_function_contract`
client_function_contract: ok
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.62s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Functions Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: -J60q15LGdkd004HzcWo9fSaw8sYLwxKBnFWF7yMJ5IHKGc8LpYwBNNr2EmDTammRLjnjh4uhxCBcnFG2slcyw
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: PNkTqJp8dolJ4ubWavuKcWLZ0CJNeYNkFGnMapubtX_bDYd0rTjVkhep8isYFctKWovC49uVS454LJNfSl11KA
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: DQ9HeVNYBeLl3rbS1kaV-cfBBbACWHPfITRhTLGzNi-AAHme1rf3BQ-fVo_3qKsYhBAH57h7Giop1A4ZR7TUZQ
📊 Statistics: 2 groups

📝 Example 4: Function Management

📋 Total functions: 3
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.04s
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
   Record({"count": Object({"type": String("Integer"), "value": Integer(3)}), "avg_price": Object({"type": String("Float"), "value": Float(365.6666666666667)}), "category": Object({"type": String("String"), "value": String("Furniture")})})
   Record({"count": Object({"type": String("Integer"), "value": Integer(5)}), "category": Object({"value": String("Electronics"), "type": String("String")}), "avg_price": Object({"type": String("Float"), "value": Float(367.0)})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.11s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They excel at finding similar items quickly using vector embeddings, making them ideal for applications like image and text retrieval.

2. **Scalability**: They can handle large volumes of high-dimensional data, allowing for efficient storage and processing as datasets grow.

3. **Real-Time Performance**: Optimized for quick query responses, supporting real-time applications like recommendation systems.

4. **Advanced Machine Learning Integration**: They work well with machine learning models that generate embeddings, facilitating seamless integration in AI workflows.

5. **Flexible Data Types**: Capable of handling various data modalities, including text, images, and audio.

6. **Built-in Indexing**: Features specialized indexing techniques (e.g., HNSW, Annoy) that enhance search speed and accuracy.

7. **Reduced Dimensionality**: Simplifies complex data relationships into lower-dimensional vector space, making it easier to analyze and visualize.

Overall, vector databases are particularly beneficial for AI and ML-driven applications requiring effective similarity searches and high-dimensional data management.
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.15s
     Running `target/debug/examples/client_functions_complete`
🚀 ekoDB Rust Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: d0Kj-EzEMKbBGYftBWkyvze_wMNKwcgtPEme0YUKKamD4QPOiUjEcwg8iTpDufrWvSKCjPEgM1p60jKEFUaY0w
📊 Found 2 product groups
   Record({"category": Object({"type": String("String"), "value": String("Furniture")}), "avg_price": Object({"type": String("Float"), "value": Float(474.0)}), "count": Object({"type": String("Integer"), "value": Integer(2)})})
   Record({"count": Object({"type": String("Integer"), "value": Integer(3)}), "category": Object({"type": String("String"), "value": String("Electronics")}), "avg_price": Object({"type": String("Float"), "value": Float(575.6666666666666)})})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   Record({"count": Object({"type": String("Integer"), "value": Integer(2)}), "category": Object({"value": String("Furniture"), "type": String("String")})})
   Record({"count": Object({"type": String("Integer"), "value": Integer(3)}), "category": Object({"value": String("Electronics"), "type": String("String")})})
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.94s
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
   Record({"count": Object({"type": String("Integer"), "value": Integer(3)}), "status": Object({"type": String("String"), "value": String("inactive")})})
   Record({"count": Object({"value": Integer(7), "type": String("Integer")}), "status": Object({"type": String("String"), "value": String("active")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.12s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("LgAZ5Zau27PvOXdkoW3cRVm33R-yhxITeHjITGqS2BhxSlYgLRjfE9gejNlWxsssAx4iCcTGe_5EbZsRnLgaLg"))
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: oNE2irTcRDu1uEaotbJ_NOWek8M-UesXVHmSFel2I_vc7CqrmIfk10Pz1zMbSk-yaXOiXfzISuWU6YWpmdnXFw
📊 function executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"type": String("Object"), "value": Object {"role": String("admin"), "userId": String("user_abc")}})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: UVWohLodjAVmo9C-PUGimjR6kGpdREOOE4Kkabth4tS-ijBTYvvZXz0IT1ZuXYzTP3fo1smnsSTa9WWc6jWr0A
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: V7aQb4pYJLk4aHIZbXM0G1y5262cx2vJH83-cIhCcEcdjmIkde2S-CgAnmam9y9mcG1MXgcDfshZkSNxd4YjsQ
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.99s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Getting Started with ekoDB (Database)
   2. Natural Language Processing (AI)
   3. Database Design Principles (Database)
   4. Vector Databases Explained (Database)
   5. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   Record({"category": Object({"value": String("Database"), "type": String("String")}), "count": Object({"type": String("Integer"), "value": Integer(3)})})
   Record({"count": Object({"type": String("Integer"), "value": Integer(2)}), "category": Object({"value": String("AI"), "type": String("String")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.30s
     Running `target/debug/examples/client_goal_templates`
=== ekoDB Goal Template CRUD Example (Rust) ===

--- Creating goal template ---
Created template: Data Migration (id: 8AuD7TPbGbzcoYx-y2clwf41KbZgXwIxdTrBA9J-RDWxnNp2CcAdDI0EgJXVELBEU99L4Kb4jfsVOgUz42sLOA)

--- Listing templates ---
Templates: {"count":1,"items":[{"description":{"type":"String","value":"Template for migrating data between schemas"},"id":"8AuD7TPbGbzcoYx-y2clwf41KbZgXwIxdTrBA9J-RDWxnNp2CcAdDI0EgJXVELBEU99L4Kb4jfsVOgUz42sLOA","steps":{"type":"Array","value":[{"description":"Analyze source schema"},{"description":"Create target schema"},{"description":"Migrate records"},{"description":"Validate results"}]},"title":{"type":"String","value":"Data Migration"}}]}

--- Getting template ---
Fetched: {"type":"String","value":"Data Migration"}

--- Updating template ---
Updated description: {"type":"String","value":"Updated: comprehensive data migration workflow"}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.40s
     Running `target/debug/examples/client_goals_tasks_agents`
=== ekoDB Goals, Tasks & Agents Integration Example (Rust) ===

--- Goal: create ---
Created goal: "Deploy v2.0 to production" (id: 3dVAjP5KHwsSrbda0o-xkl29fFnmmr1rGKPDSc7cnuxNO14GAzfouRn62ApQDJnsBxAS2NigvSZQqKgb729LPg)

--- Goal: list ---
Goals: {"count":1,"goals":[{"created_at":"2026-10-20T19:42:15.031806+00:00","description":"Full release cycle for version 2.0","id":"3dVAjP5KHwsSrbda0o-xkl29fFnmmr1rGKPDSc7cnuxNO14GAzfouRn62ApQDJnsBxAS2NigvSZQqKgb729LPg","status":"pending","steps":"[{\"description\":\"Run integration tests\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]","title":"Deploy v2.0 to production","updated_at":"2026-10-20T19:42:15.031806+00:00"}]}

--- Goal: get ---
Fetched goal: {"type":"String","value":"Deploy v2.0 to production"}

--- Goal: update ---
Updated description: {"type":"String","value":"Updated: full v2.0 release with rollback plan"}

--- Goal: search ---
Search results: {"count":1,"items":[{"_score":12.87,"created_at":{"type":"DateTime","value":"2026-10-20T19:42:15.031806+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"3dVAjP5KHwsSrbda0o-xkl29fFnmmr1rGKPDSc7cnuxNO14GAzfouRn62ApQDJnsBxAS2NigvSZQqKgb729LPg","status":{"type":"String","value":"pending"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-10-20T19:42:15.056976+00:00"}}]}

--- Goal step: start (step 0) ---
Step 0 started: {"created_at":{"type":"DateTime","value":"2026-10-20T19:42:15.031806+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"3dVAjP5KHwsSrbda0o-xkl29fFnmmr1rGKPDSc7cnuxNO14GAzfouRn62ApQDJnsBxAS2NigvSZQqKgb729LPg","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"status\":\"InProgress\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-10-20T19:42:15.062050+00:00"}}

--- Goal step: complete (step 0) ---
Step 0 completed: {"created_at":{"type":"DateTime","value":"2026-10-20T19:42:15.031806+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"3dVAjP5KHwsSrbda0o-xkl29fFnmmr1rGKPDSc7cnuxNO14GAzfouRn62ApQDJnsBxAS2NigvSZQqKgb729LPg","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-10-20T19:42:15.066813+00:00"}}

--- Goal step: start (step 1) ---
Step 1 started

--- Goal step: fail (step 1) ---
Step 1 failed: {"created_at":{"type":"DateTime","value":"2026-10-20T19:42:15.031806+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"3dVAjP5KHwsSrbda0o-xkl29fFnmmr1rGKPDSc7cnuxNO14GAzfouRn62ApQDJnsBxAS2NigvSZQqKgb729LPg","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-10-20T19:42:15.076611+00:00"}}

--- Goal: complete ---
Goal completed (pending review): {"completed_at":{"type":"DateTime","value":"2026-10-20T19:42:15.081566+00:00"},"created_at":{"type":"DateTime","value":"2026-10-20T19:42:15.031806+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"3dVAjP5KHwsSrbda0o-xkl29fFnmmr1rGKPDSc7cnuxNO14GAzfouRn62ApQDJnsBxAS2NigvSZQqKgb729LPg","status":{"type":"String","value":"pending_review"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"summary":{"type":"String","value":"Partial completion — step 1 failed"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-10-20T19:42:15.081566+00:00"}}

--- Goal: approve ---
Goal approved: {"completed_at":{"type":"DateTime","value":"2026-10-20T19:42:15.081566+00:00"},"created_at":{"type":"DateTime","value":"2026-10-20T19:42:15.031806+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"3dVAjP5KHwsSrbda0o-xkl29fFnmmr1rGKPDSc7cnuxNO14GAzfouRn62ApQDJnsBxAS2NigvSZQqKgb729LPg","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"summary":{"type":"String","value":"Partial completion — step 1 failed"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-10-20T19:42:15.086275+00:00"}}

--- Goal: create (for rejection) ---
Created goal2: "Refactor auth module" (id: TpljrBocwCc1iGNvrvJr3FJ0UF4sXxbpSadOfuv5HEztH5Wo0Z8VQQXgSIuYcpAa7z5KLGnb9Gaag1rIwTiMAg)

--- Goal: complete (goal2) ---

--- Goal: reject ---
Goal rejected: {"completed_at":{"type":"DateTime","value":"2026-10-20T19:42:15.101172+00:00"},"created_at":{"type":"DateTime","value":"2026-10-20T19:42:15.091148+00:00"},"description":{"type":"String","value":"Rewrite JWT handling"},"id":"TpljrBocwCc1iGNvrvJr3FJ0UF4sXxbpSadOfuv5HEztH5Wo0Z8VQQXgSIuYcpAa7z5KLGnb9Gaag1rIwTiMAg","reason":{"type":"String","value":"Breaks backward compatibility"},"status":{"type":"String","value":"failed"},"steps":{"type":"String","value":"[]"},"summary":{"type":"String","value":"Completed refactor"},"title":{"type":"String","value":"Refactor auth module"},"updated_at":{"type":"DateTime","value":"2026-10-20T19:42:15.101172+00:00"}}

--- Task: create ---
Created task: Nightly backup (id: RCe75t0udm__Zz-Q5BZETWVknnK8MxNL2uqQPT9AOC-v3i_uNsH3fmo9O2Jlmd7Ag2vmIKVaGsIFbZJ_iFqr8g)

--- Task: list ---
Tasks: {"count":1,"items":[{"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"RCe75t0udm__Zz-Q5BZETWVknnK8MxNL2uqQPT9AOC-v3i_uNsH3fmo9O2Jlmd7Ag2vmIKVaGsIFbZJ_iFqr8g","max_retries":{"type":"Integer","value":3},"schedule":{"type":"String","value":"0 2 * * *"},"title":{"type":"String","value":"Nightly backup"}}]}

--- Task: get ---
Fetched task: {"type":"String","value":"Nightly backup"}

--- Task: start ---
Task started: {"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"RCe75t0udm__Zz-Q5BZETWVknnK8MxNL2uqQPT9AOC-v3i_uNsH3fmo9O2Jlmd7Ag2vmIKVaGsIFbZJ_iFqr8g","max_retries":{"type":"Integer","value":3},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"running"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-10-20T19:42:15.129811+00:00"}}

--- Task: succeed ---
Task succeeded: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"RCe75t0udm__Zz-Q5BZETWVknnK8MxNL2uqQPT9AOC-v3i_uNsH3fmo9O2Jlmd7Ag2vmIKVaGsIFbZJ_iFqr8g","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-10-20T19:42:15.134331+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-10-20T19:42:15.134331+00:00"}}

--- Task: start (round 2) ---

--- Task: pause ---
Task paused: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"RCe75t0udm__Zz-Q5BZETWVknnK8MxNL2uqQPT9AOC-v3i_uNsH3fmo9O2Jlmd7Ag2vmIKVaGsIFbZJ_iFqr8g","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-10-20T19:42:15.134331+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"paused"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-10-20T19:42:15.143485+00:00"}}

--- Task: resume ---
Task resumed: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"RCe75t0udm__Zz-Q5BZETWVknnK8MxNL2uqQPT9AOC-v3i_uNsH3fmo9O2Jlmd7Ag2vmIKVaGsIFbZJ_iFqr8g","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-10-20T19:42:15.134331+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-10-20T19:42:15.150163+00:00"}}

--- Task: fail ---
Task failed: {"consecutive_failures":{"type":"Integer","value":1},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"RCe75t0udm__Zz-Q5BZETWVknnK8MxNL2uqQPT9AOC-v3i_uNsH3fmo9O2Jlmd7Ag2vmIKVaGsIFbZJ_iFqr8g","last_error":{"type":"String","value":"S3 bucket access denied"},"last_run":{"type":"DateTime","value":"2026-10-20T19:42:15.156555+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-10-20T19:42:15.156555+00:00"}}

--- Task: due ---
Due tasks: {"count":0,"items":[]}

--- Task: delete ---
Task deleted

--- Agent: create ---
Created agent: backup-agent (id: Ar39Jb2PxDDjJ4hlplfZzzTz7KWh7REGeBQtTaeEAXJDEehgiSAX7Rutf1QmtZ2VhX9KRpk7S9ZDtWib3wcShw)

--- Agent: list ---
Agents: {"count":1,"items":[{"capabilities":{"type":"Array","value":["backup","restore","verify"]},"deployment_id":{"type":"String","value":"deploy-abc-123"},"description":{"type":"String","value":"Handles nightly backups and restores"},"id":"Ar39Jb2PxDDjJ4hlplfZzzTz7KWh7REGeBQtTaeEAXJDEehgiSAX7Rutf1QmtZ2VhX9KRpk7S9ZDtWib3wcShw","llm_model":{"type":"String","value":"gpt-4.1"},"name":{"type":"String","value":"backup-agent"}}]}

--- Agent: get ---
Fetched agent: {"type":"String","value":"backup-agent"}

--- Agent: get_by_name ---
Found by name: {"type":"String","value":"backup-agent"}

--- Agent: update ---
Updated agent description: {"type":"String","value":"Handles backups, restores, and disaster recovery"}

--- Agent: agents_by_deployment ---
Agents in deployment: {"count":0,"items":[]}
WARNING: agents_by_deployment omitted created agent Ar39Jb2PxDDjJ4hlplfZzzTz7KWh7REGeBQtTaeEAXJDEehgiSAX7Rutf1QmtZ2VhX9KRpk7S9ZDtWib3wcShw; TODO: check/fix the server-side deployment lookup

--- Agent: delete ---
Agent deleted

--- Goal: delete ---
Goal 1 deleted
Goal 2 deleted

=== All goals, tasks & agents operations completed ===
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.32s
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.26s
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.40s
     Running `target/debug/examples/client_kv_links`
=== ekoDB KV Document Linking Example (Rust) ===

--- Inserting documents ---
Inserted doc1: Project Alpha (id: lFnmXkwkbQRbYWHatqFkfuvR_xVGTY3eWYpitLmE2Y9p13Z-E4XVSfnhVjR6Ui38RX5TUvM-JPDNT2zljeytjg)
Inserted doc2: Project Beta (id: nmdjiHG_A62F9d3lqsQPQ8TvolqQobvaoPZPUEhFPH55qyrxx5HUln8aD07Csg8yZR8QCEdM9ri3S3VfH3rMpA)

--- Setting KV key ---
Set key: kv_links:rs:user:alice:projects

--- Linking documents ---
Linked doc1: null
Linked doc2: null

--- Getting links ---
Links for kv_links:rs:user:alice:projects: [{"collection":"kv_links_example_rs","created_at":"2026-10-20T19:44:46.942667Z","document_id":"lFnmXkwkbQRbYWHatqFkfuvR_xVGTY3eWYpitLmE2Y9p13Z-E4XVSfnhVjR6Ui38RX5TUvM-JPDNT2zljeytjg","field_path":null,"last_accessed":"2026-10-20T19:44:46.943238Z","metadata":{}},{"collection":"kv_links_example_rs","created_at":"2026-10-20T19:44:46.942966Z","document_id":"nmdjiHG_A62F9d3lqsQPQ8TvolqQobvaoPZPUEhFPH55qyrxx5HUln8aD07Csg8yZR8QCEdM9ri3S3VfH3rMpA","field_path":null,"last_accessed":"2026-10-20T19:44:46.943238Z","metadata":{}}]

--- Unlinking doc1 ---
Unlinked doc1: null

--- Verifying remaining links ---
Remaining links: [{"collection":"kv_links_example_rs","created_at":"2026-10-20T19:44:46.942966Z","document_id":"nmdjiHG_A62F9d3lqsQPQ8TvolqQobvaoPZPUEhFPH55qyrxx5HUln8aD07Csg8yZR8QCEdM9ri3S3VfH3rMpA","field_path":null,"last_accessed":"2026-10-20T19:44:46.943793Z","metadata":{}}]

--- Cleanup ---
Unlinked remaining document
Deleted KV key
Deleted documents
Deleted collection

=== All KV linking operations completed ===
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.20s
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
  kv_operations:rs:cache:product:2: Record({"name": String("Product 2"), "price": Float(39.989999999999995)})
  kv_operations:rs:cache:product:3: Record({"price": Float(49.989999999999995), "name": String("Product 3")})

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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.28s
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.54s
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.29s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Prefix Query (StartsWith) ===
✓ Found 1 users with names starting with A
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"type": String("String"), "value": String("Charlie")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
  2. Some(Object({"type": String("String"), "value": String("David")}))
  3. Some(Object({"type": String("String"), "value": String("Alice")}))

=== Pagination (page 2, size 2) ===
✓ Page 2 results:
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== NOT Operator ===
✓ Found 4 users NOT inactive

=== Complex Chained Query ===
✓ Found 3 users with all conditions

=== Cleanup ===
✓ Deleted collection

✓ All query builder operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.31s
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
    "name": "Earth",
    "diameter_km": 12742
  },
  {
    "name": "Jupiter",
    "diameter_km": 139820
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you’re having a great day.

=== Done ===
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.32s
     Running `target/debug/examples/client_schedules`
=== ekoDB Schedule Management Example (Rust) ===

--- Creating schedule ---
Created schedule: "nightly-cleanup" (id: 1d1e001b-c974-47f0-be1a-85ad714345c1)

--- Listing schedules ---
Schedules: {"count":1,"schedules":[{"created_at":"2026-10-20T19:49:49.792445Z","cron_expression":"0 0 3 * * *","description":"Remove expired sessions and temp files","enabled":true,"function_label":"schedule_noop_rust_68525","id":"1d1e001b-c974-47f0-be1a-85ad714345c1","last_execution":null,"name":"nightly-cleanup","next_execution":"2026-10-21T03:00:00Z","parameters":{},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-10-20T19:49:49.792445Z"}]}

--- Getting schedule ---
Fetched schedule: "nightly-cleanup"

--- Updating schedule ---
Updated cron: "0 0 4 * * *"
Updated description: "Remove expired sessions, temp files, and orphaned uploads"

--- Triggering schedule ---
Trigger response: {"schedule_id":"1d1e001b-c974-47f0-be1a-85ad714345c1","status":"triggered"}

--- Pausing schedule ---
Schedule paused: {"created_at":"2026-10-20T19:49:49.792445Z","cron_expression":"0 0 4 * * *","description":"Remove expired sessions, temp files, and orphaned uploads","enabled":false,"function_label":"schedule_noop_rust_68525","id":"1d1e001b-c974-47f0-be1a-85ad714345c1","last_execution":"2026-10-20T19:49:49.825388Z","name":"nightly-cleanup","next_execution":null,"parameters":{"type":"Object","value":{}},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-10-20T19:49:49.826130Z"}

--- Resuming schedule ---
Schedule resumed: {"created_at":"2026-10-20T19:49:49.792445Z","cron_expression":"0 0 4 * * *","description":"Remove expired sessions, temp files, and orphaned uploads","enabled":true,"function_label":"schedule_noop_rust_68525","id":"1d1e001b-c974-47f0-be1a-85ad714345c1","last_execution":"2026-10-20T19:49:49.825388Z","name":"nightly-cleanup","next_execution":"2026-10-21T04:00:00Z","parameters":{"type":"Object","value":{}},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-10-20T19:49:49.826881Z"}

--- Deleting schedule ---
Schedule deleted successfully

=== All schedule operations completed ===
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.39s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("h7_wJV_-ZFDdeDZEVmdIEiqR81gAXm5On6tAKtWY4pPVJBJ6OJo0owwEJYT-MxHxUV0zaQKwaxvFml_jLliB5g"))
✓ Inserted user 2: Some(String("tyoePh8leXCWbv7RvxpHRdtdKK-bI46keG3Va6Q8h_P8H0fWvz8MYXt_XwbXym8DNJtv8tXAclZPRNwSiGeD8w"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - status: String
  - title: String
    (required)
  - email: String
    (required)
  - age: Integer

=== Listing Collections ===
✓ Total collections: 13
  Sample: ["chat_goals__ek0_testing", "chat_agent_configs__ek0_testing", "chat_tasks__ek0_testing", "chat_goal_templates__ek0_testing", "chat_configurations__ek0_testing"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.44s
     Running `target/debug/examples/client_search`
=== ekoDB Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Found 3 results for 'programming'
  1. Score: 26.4000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 13.2000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})
  3. Score: 13.2000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})

=== Fuzzy Search ===
✓ Found 3 results for 'progamming' (typo)
  1. Score: 2.2000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 1.1000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})
  3. Score: 1.1000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})

=== Field-Specific Search ===
✓ Found 4 results in title/description
  1. Score: 2.0000
     Title: Some(Object {"type": String("String"), "value": String("Machine Learning Basics")})
     Matched: ["description", "title"]
  2. Score: 1.0000
     Title: Some(Object {"type": String("String"), "value": String("Python for Data Science")})
     Matched: ["description"]
  3. Score: 0.5000
     Title: Some(Object {"type": String("String"), "value": String("Rust Programming")})
     Matched: ["description"]
  4. Score: 0.5000
     Title: Some(Object {"type": String("String"), "value": String("Database Design")})
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
  2. Score: 13.2000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})

=== Search with a metadata pre-filter (category = programming) ===
✓ Found 2 results in category 'programming' (database/ai excluded)
  1. Some(Object {"type": String("String"), "value": String("Rust Programming")}) (category: Some(Object {"type": String("String"), "value": String("programming")}))
  2. Some(Object {"type": String("String"), "value": String("Python for Data Science")}) (category: Some(Object {"type": String("String"), "value": String("programming")}))

Execution time: 0ms
Paired vector search: document=Some("Rust Programming Answer"), query=Some("Rust Programming Question")
=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.61s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("hqM5TQphRHAZi3QMuBeAEU5KwO76ggOBtKy78tPtOR9xYnf3UuldKiSjekIItCfqmlFAR-QAab_zxkX0Ork7bA")})

=== Find by ID ===
Found: Record({"active": Object({"value": Boolean(true), "type": String("Boolean")}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "value": Object({"value": Integer(42), "type": String("Integer")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "name": Object({"type": String("String"), "value": String("Test Record")}), "id": String("hqM5TQphRHAZi3QMuBeAEU5KwO76ggOBtKy78tPtOR9xYnf3UuldKiSjekIItCfqmlFAR-QAab_zxkX0Ork7bA"), "created_at": Object({"value": String("2026-10-20T19:52:21.732376+00:00"), "type": String("DateTime")})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-10-20T19:52:21.732376+00:00")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"price": Object({"type": String("Float"), "value": Float(99.99)}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "created_at": Object({"value": String("2026-10-20T19:52:21.732376+00:00"), "type": String("DateTime")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "value": Object({"type": String("Integer"), "value": Integer(42)}), "name": Object({"type": String("String"), "value": String("Test Record")}), "id": String("hqM5TQphRHAZi3QMuBeAEU5KwO76ggOBtKy78tPtOR9xYnf3UuldKiSjekIItCfqmlFAR-QAab_zxkX0Ork7bA"), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])})})]

=== Update Document ===
Updated: Record({"active": Object({"type": String("Boolean"), "value": Boolean(true)}), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "value": Object({"type": String("Integer"), "value": Integer(100)}), "id": String("hqM5TQphRHAZi3QMuBeAEU5KwO76ggOBtKy78tPtOR9xYnf3UuldKiSjekIItCfqmlFAR-QAab_zxkX0Ork7bA"), "name": Object({"type": String("String"), "value": String("Updated Record")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-10-20T19:52:21.732376+00:00")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.61s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: MO3gi0cUx9FNoHm2blUyJXFaDYXFqJTu8DlKWC5YBLX0V0hTaWhlng9URqhhOfjJtEo27p_KsZepaBOtoLClOQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  value: 42
  name: "WebSocket Test Record"
  id: "MO3gi0cUx9FNoHm2blUyJXFaDYXFqJTu8DlKWC5YBLX0V0hTaWhlng9URqhhOfjJtEo27p_KsZepaBOtoLClOQ"
  active: true

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.87s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native_rs (GFlbhtHU3Lioq_uAEN91coTxh_prZa0Ly6yvCBSSS5-35THelGDHKaabH41Z1LEkTqqnQnvtVmzzKlOPIQ5HNg)

First call (cache miss - will fetch from GitHub API):
  Response time: 141ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 70.5x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit_rs (gHgWIBxgWqXYXyTR2aHJqoVCfkRjjxxmf2sC1xPnvfmrHdHd5TRTWHZWpbGSicUE1FQrIG8dEk8qJgawVm6t_Q)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_rs' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_rs (99Zhns7cHLk5iN2RDlehuXUAG-e998a1rV_ynFQUneMbAaupGGLOGMZQKEOFJU1c5NCJUvq8Y5gyzPIiERcV0A)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_rs' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache_rs (AKJiGVWeoWKO5mbhGWx7HaPLGaYYsRoSeYRvzn7rJe-dfY_VM92-NsdBEpAAVH4HVEUovW73ExzNaQVqEGzl2g)
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.82s
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_rs (bnrwYv8VNDexmtqPcHjBr0epyHYk6NoYTXA2DFNqstJqqRDRQhofVBx59M_me4NWEV68Vo3ybhBEobaUB0XglQ)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 1ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.42s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 3mswn9Gg-TPY5nZjt6Yu3mD6RVfjEIbMvu7LPAr1Rx3w8SYY9CTrFkWJuisRJ8HBpzCQaLMCWxz6hnQhFdNB3Q
Created Bob: $500 - ID: 9cew3x5IqJGsrC_1Z-sOJFMEdP51YkBJLj8VGHDGfQHdfg_ENLddimlYSBMy6QY_xUr9ustKh8oNC3mAFC1M3w

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 28a95037-ed3a-4181-9c0e-b765e280e871

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
New transaction: bb0cb8ff-fc4b-42be-b9e3-748c6c3fffd0
Updated Bob: $700 → $600 (in transaction)
Status before rollback: "Active"
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.73s
     Running `target/debug/examples/client_user_functions`
✓ Client created

=== Create User Function ===
Created user function with ID: KiTTyxnSSbqM9uT48mvEGlVVlfCaA0rotdkCJ51Jl4SzjHoFBtoGXzkQNa2ARrDOoNowmYs7izdYjGfLEygnMw

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.35s
     Running `target/debug/examples/client_websocket_chat_stream`
=== WebSocket Chat Streaming Example (Rust) ===

✓ Authentication successful
✓ Created chat session: gmtGw56IwApiLqX50FOTm9kKsoXMxKn_uqyxk4YX1DXeHV6Q-B9UhzOTE8eEFcxBV9e2CXBzLwqgK4jp6FmiGQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Sending message: 'What is the capital of France?' ===
✓ Message accepted, streaming...

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: "CPBAdLBVxKD9nPfBgD8MXRQp2u7C-LwwXO12rmxe4ZiNb7VR5pByWEiGaTPVKoXGHkxEJITbfgzrnqIQLvlqvQ"
Execution time: 1105ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}
Context window: 1000000 tokens

Full response: {"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris....

✓ WebSocket chat streaming example completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.28s
     Running `target/debug/examples/client_websocket_subscribe`
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_rs' ===
✓ Subscribed (subscription_id: sub_01ec0c41cc6742468e63ca4f7b627404)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: CJVsx9TayGexpbwiPVBCbju9Uq1ejKl56SYtgSxNt7iJkPcHB4WniVsCDyMS1Lihewpmw0L0onlbU-WdTK5Y3Q

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_rs"
     Record IDs: ["CJVsx9TayGexpbwiPVBCbju9Uq1ejKl56SYtgSxNt7iJkPcHB4WniVsCDyMS1Lihewpmw0L0onlbU-WdTK5Y3Q"]
     Timestamp:  "2026-10-20T19:59:10.394894+00:00"

Inserting record 2...
✓ Inserted: A9NXd-MYXWVeos1hazRfiwhBDVV-tIAC_vlNctAchLquYPq5swYKm2wh0clUp07R-sqT91kSWbBCNeOOYEZApw

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["A9NXd-MYXWVeos1hazRfiwhBDVV-tIAC_vlNctAchLquYPq5swYKm2wh0clUp07R-sqT91kSWbBCNeOOYEZApw"]

=== Unsubscribing ===
✓ Unsubscribed

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_rs'

✓ WebSocket subscription example completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.64s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("6LKrBkLa7URH5egM5O7vrDzxeUfGrC0Wko255zZwfkoEHsJu6FLEMCTMbQy3XReKYqqxXnHstOF_tkKWnaQt9w"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.30s
     Running `target/debug/examples/bypass_ripple_example`
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: Record({"id": String("d_11IXlWGJN7Npr-b7vJqU8q5bPoaG9C3XQ4HcL-pSTM2hioDWZBdtcF8EvJ9e7gyK5OAA5RCObt9uiPUKfceg")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("hHxlcqcGLRNv97P5wwcDUV2J6tLajLClEPPwAU6-dZJ3Oj-2gVSD35quX6VZfLToxcFmO5U5H1ZpRzziJDHbbA")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"id": String("d_11IXlWGJN7Npr-b7vJqU8q5bPoaG9C3XQ4HcL-pSTM2hioDWZBdtcF8EvJ9e7gyK5OAA5RCObt9uiPUKfceg"), "price": Object({"value": Integer(150), "type": String("Integer")}), "name": Object({"type": String("String"), "value": String("Product 1")})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("custom-id")})

✅ All bypass_ripple operations completed successfully!
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.32s
     Running `target/debug/examples/projection_example`
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["id", "name", "email"]

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["email", "user_role", "created_at", "bio", "name", "id", "age", "status", "avatar_url"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["password", "api_key", "avatar_url", "bio", "status", "name", "created_at", "age", "id", "secret_token", "email", "user_role"]
  Projected query:
    - 3 fields per record
    - Fields: ["email", "name", "id"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
