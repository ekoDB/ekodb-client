make test-examples
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
✅ All Rust integration tests complete!
📦 Ensuring Python example dependencies in .venv...
🧪 Running Python examples (direct HTTP/WebSocket)...
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'M8b_EQ6FX89176ofS_mxbUMnHKr7xk6cq3ysDYwaGxDz-m-jZib8wW77I7W7j5-OVSu-pOJbpkqUi3lpvIX9Sg'}

=== Find by ID ===
Found: {'name': {'type': 'String', 'value': 'Test Record'}, 'value': {'type': 'Integer', 'value': 42}, 'active': {'value': True, 'type': 'Boolean'}, 'id': 'M8b_EQ6FX89176ofS_mxbUMnHKr7xk6cq3ysDYwaGxDz-m-jZib8wW77I7W7j5-OVSu-pOJbpkqUi3lpvIX9Sg'}

=== Find with Query ===
Found documents: [{'value': {'type': 'Integer', 'value': 42}, 'name': {'value': 'Test Record', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}, 'id': 'M8b_EQ6FX89176ofS_mxbUMnHKr7xk6cq3ysDYwaGxDz-m-jZib8wW77I7W7j5-OVSu-pOJbpkqUi3lpvIX9Sg'}]

=== Update Document ===
Updated: {'id': 'M8b_EQ6FX89176ofS_mxbUMnHKr7xk6cq3ysDYwaGxDz-m-jZib8wW77I7W7j5-OVSu-pOJbpkqUi3lpvIX9Sg', 'value': {'value': 100, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'active': {'value': True, 'type': 'Boolean'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: i-BoHLxUCpfQhQWQxwsyD402xlYPS6LoD6X_oYCMV_fkoL_K-fB3R6hX9PVSkXNS9gZ3sIGGNLchn2jZnUkZ7A

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "value": {
          "value": 42,
          "type": "Integer"
        },
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "id": "i-BoHLxUCpfQhQWQxwsyD402xlYPS6LoD6X_oYCMV_fkoL_K-fB3R6hX9PVSkXNS9gZ3sIGGNLchn2jZnUkZ7A",
        "active": {
          "type": "Boolean",
          "value": true
        }
      }
    ]
  },
  "messageId": "2148508262"
}
✓ Retrieved 1 record via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Functions Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: g4nhW7FBtFM4pg-a3xN6gKdooKICB428mD0DqMGT2pyNQiuQHjECybkaN8tUhyx8ckc4YZuHjZIlQIXlserdbQ
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: 6T-DDGypM0BT7MnDl192wuPGADbMbRUy1YkoGRPUUisZRIO1Cc_mLhhU_-UUIrJ6pUR1SJGPZFv-8SOGUydeTw
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: uQYYJgx5LwITResE59P7zlFowoQb0zupmIza0iRpOnMPUf70G6tGppq04-ABOJES9qo2M7YBgXcS52w9gT9xHA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'max_score': {'type': 'Integer', 'value': 90}, 'status': {'value': 'inactive', 'type': 'String'}, 'avg_score': {'value': 50.0, 'type': 'Float'}, 'count': {'type': 'Integer', 'value': 5}}
   {'count': {'type': 'Integer', 'value': 5}, 'avg_score': {'value': 60.0, 'type': 'Float'}, 'max_score': {'value': 100, 'type': 'Integer'}, 'status': {'type': 'String', 'value': 'active'}}

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
Retrieved value: {'type': 'Object', 'value': {'userId': 123, 'username': 'john_doe'}}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
kv_operations:direct:py:cache:product:1: {'type': 'Object', 'value': {'price': 29.99, 'name': 'Product 1'}}
kv_operations:direct:py:cache:product:2: {'value': {'name': 'Product 2', 'price': 39.989999999999995}, 'type': 'Object'}
kv_operations:direct:py:cache:product:3: {'type': 'Object', 'value': {'price': 49.989999999999995, 'name': 'Product 3'}}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: cas8-8_Gj5EDfZrpmX0BsMPCZg2ZDuh_2YCg5S4SySyEr9qebg6znlmLpQ9opHPIsKW2XCnoDlSLPSc2I6Gxag

=== List Collections ===
Total collections: 13
Sample collections: ['chat_goals__ek0_testing', 'chat_agent_configs__ek0_testing', 'chat_tasks__ek0_testing', 'chat_goal_templates__ek0_testing', 'chat_configurations__ek0_testing']

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
Created Alice: $1000 - ID: hFLG3ML0dVZ5sQUM76xnA_cT6akSrIy1SpmXTvcvjpej68OOhBfCx1-mmR2txyCjGhMV4Lhd6snkXUswobC6jg
Created Bob: $500 - ID: WcRJm_3eNUlcMXZstHGfRzXnehfwaCqn2nuGwZAU6nxOXMGG7qW-H6DEh_gvkupViW-hzKYu9nJ754aITjCeyw

=== Example 1: Begin Transaction ===
Transaction ID: 55e00123-fcbf-4a39-9478-66f66b408569

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
Bob: ${'type': 'Integer', 'value': 700}

=== Example 5: Rollback ===
New transaction: 2db4ca1d-df85-4be8-ad47-c4c23e838fc8
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: e6vTqGKEXR9j2chZ1ykCa-8aYD1UZx00QhBAlzxZqSlqUMacQVz6Oj61zqPGwWud5qadPoltvaiWhJl1HjoXfg
Created Bob: $500 - ID: eSOSlyssNoOtJUWn2v6s9KQmO1Xvqd5c5uVzyUSjN4cr9K2U4oPxTs67KpbJ_YtnzvQ3ZiGrjlkg-Src0aUQtQ

=== Example 1: Begin Transaction ===
Transaction ID: 14771edf-017c-4666-9388-db6c4129faf4

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
Created Alice: $1000 - ID: L8p8h7Qs2Z7YSWUGYewY-1Frgi_FWLxNhls1-U_GEPMHZwmM6a9NElfuFqp7wcQITxwgQ2Q80aFI0S4x__HTeA
Created Bob: $500 - ID: SdG5SJgBxA03QV-eeI3ZhbYdFGbc-x9vdQvryDoY_4zpbA-Zdn87AKH_UANPkrufQeEw0_aNYNQBdXWzvAqqfA

=== Example 1: Begin Transaction ===
Transaction ID: 34d464a6-7f3d-467b-9e40-ce1adc7f574f

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
Bob: ${'type': 'Integer', 'value': 700}

=== Example 5: Rollback ===
New transaction: 7238079c-dbe4-4990-9d30-4a18bf7b7bac
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: MzuQ0URf50htUnk_KQZ3ZvgoSaYOl5Dac15XjQYIIkJwSbI4AFCv8VEjM5CsSZxQb7SwK4EIEIZa_XCFgqLDTw
Created Bob: $500 - ID: 3nIhP_DJ2KYPorTHNPMPbc7bfbCDYW3AV2l-bURPr9LIVE2SgBAKd_Z7M3pOMqPdbkNVre8VKadc39xDhSy5nw

=== Example 1: Begin Transaction ===
Transaction ID: 65785a0d-5d1a-4fd3-9b55-ec5d628b0cae

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to DURABLE mode...
   ✓ Config updated: durable_operations=true


============================================================
🔥 TEST 3: Durable Mode (durable=true)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 1KkLRtKP3oAsM-z69dvP-qz2Y8Tl0AnFvVpO1UqdCyceO97Qi9pyUSOyQ8o0h7BeXVnAvIQgPpNcDbyR4FWNQQ
Created Bob: $500 - ID: lYPtjV8PyGMIjUOrZZguI8o_GFmBNrtd304LYJ04C5_qUL7mxJQlYCmVuCDGfJKjgPdozBCST3hsYYGdIUWwQw

=== Example 1: Begin Transaction ===
Transaction ID: 9d08783c-e9de-477d-b301-dac3ef77db38

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
New transaction: 19318aef-6516-4eb3-9cd1-aa6f7aa0f128
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: K4XeyRBVS8LjZSmlQlO4TEtu8I5vMvexSklqkXoGytjX-uJB9tPtM5a3dQJ5vvy9fpFU_S7rz66kT1DP_k4Uug
Created Bob: $500 - ID: vqkS3gPegHP9z66_y1GPxMbPRXVotWKEeEsWsj--bgbnrjNyvXE0mkNHzZlMZrb7MbXclX41ZLTPGBrzHkzZLw

=== Example 1: Begin Transaction ===
Transaction ID: 5ccf2939-7ede-48c2-804b-8cc418f04d77

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'value': 800, 'type': 'Integer'}
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
   ✅ Function saved: oBMDdBfuJqICiV3vxfk1fvTqpmgX_fbHLuc6udqpihAb8ZemEfTPK_IlhfxMWNNbiDrqISlniEZkkBLW7KKZaA

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: d_SYEFDmJbIf9MuPPHKkXOzZxbcb8fIL722Z_dgUPPfmFQiSJG9rSFJwaqw9POQWhYcssfbOHHpAEVgqGVE8Jw
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: jPJsox36HpdJeLvEXqCcf_SH9f9nenE_EeAk6nvJ-RmdCRGbVxgnnSZ7ZG_LhVkYWLswO9Ae2NYhyzJjOSWHgQ

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
   ✅ Function saved: KuioFKuRoua0RRyaWzW2DXSlmt_6pWHLOAfHECZUv2LjMgx5nDqvHtFftWdQkyVmINAiRCC_cSWZj6dvbBqjlw

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 100}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: sAX4_6JcBgt0b7F_PkXboABqyHKhVHgEgofAM86pcnepOTmBlJA2Rj6kOsN-se_W7jnujF1QTUjwkjct79_kvA

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: oBMDdBfuJqICiV3vxfk1...
   ✅ Deleted script: jPJsox36HpdJeLvEXqCc...
   ✅ Deleted script: KuioFKuRoua0RRyaWzW2...
   ✅ Deleted script: sAX4_6JcBgt0b7F_PkXb...
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
  Output: Document ID = 5p3RNcUXW3VA7MX17buz_cB45WKfe4anu4yf7TDsIHwpCjRMou-dZ2s8-1PLavZmgNcL1f_0p34wrczTbko02g
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(5p3RNcUXW3VA7MX17buz_cB45WKfe4anu4yf7TDsIHwpCjRMou-dZ2s8-1PLavZmgNcL1f_0p34wrczTbko02g)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(5p3RNcUXW3VA7MX17buz_cB45WKfe4anu4yf7TDsIHwpCjRMou-dZ2s8-1PLavZmgNcL1f_0p34wrczTbko02g)
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
  Output: Document ID = SH3kuHCzszyUZLcFeKVGf7RuxfHPVnEiGhM4N4z952LWKjuGh_ZkwAFrWdBBl10bN6RQU8N1RXX4bakE4hP1Kg
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(SH3kuHCzszyUZLcFeKVGf7RuxfHPVnEiGhM4N4z952LWKjuGh_ZkwAFrWdBBl10bN6RQU8N1RXX4bakE4hP1Kg)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(SH3kuHCzszyUZLcFeKVGf7RuxfHPVnEiGhM4N4z952LWKjuGh_ZkwAFrWdBBl10bN6RQU8N1RXX4bakE4hP1Kg)
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
    Finished `release` profile [optimized] target(s) in 0.16s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.30.1-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.30.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.30.1
    Uninstalling ekodb_client-0.30.1:
      Successfully uninstalled ekodb_client-0.30.1
Successfully installed ekodb-client-0.30.1
🧪 Ensuring test dependencies (pytest) in .venv...
✅ Python client package built and installed!
Admin permissions read successfully
=== ekoDB Advanced CRUD Example (Python) ===

--- insert ---
Inserted: E83KP_UpmvMNP6dxSvfCsOuvqjHIyOfB25SY6J_YRpBv0aW6QecVna1DTdFMWokuDgbDgJK_0nJ7wpHLiqSijw

--- update_with_action (increment) ---
After increment count by 5: {'id': 'E83KP_UpmvMNP6dxSvfCsOuvqjHIyOfB25SY6J_YRpBv0aW6QecVna1DTdFMWokuDgbDgJK_0nJ7wpHLiqSijw', 'tags': {'type': 'Array', 'value': ['initial']}, 'score': {'type': 'Float', 'value': 50.0}, 'name': {'type': 'String', 'value': 'Counter Record'}, 'count': {'type': 'Integer', 'value': 15}}

--- update_with_action (decrement) ---
After decrement score by 10: {'tags': {'value': ['initial'], 'type': 'Array'}, 'name': {'type': 'String', 'value': 'Counter Record'}, 'count': {'value': 15, 'type': 'Integer'}, 'score': {'value': 40.0, 'type': 'Float'}, 'id': 'E83KP_UpmvMNP6dxSvfCsOuvqjHIyOfB25SY6J_YRpBv0aW6QecVna1DTdFMWokuDgbDgJK_0nJ7wpHLiqSijw'}

--- update_with_action_sequence ---
After action sequence: {'count': {'type': 'Integer', 'value': 115}, 'id': 'E83KP_UpmvMNP6dxSvfCsOuvqjHIyOfB25SY6J_YRpBv0aW6QecVna1DTdFMWokuDgbDgJK_0nJ7wpHLiqSijw', 'tags': {'value': ['initial', 'sequenced'], 'type': 'Array'}, 'name': {'type': 'String', 'value': 'Counter Record'}, 'score': {'value': 40.0, 'type': 'Float'}}

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
✓ Created session: DaaaUYUQrr_Dmp7pAgKk-Cnn-B8SPjZ7FGcAx7iEPZeVTmXOLwT9QPLK_eSML8LTMQwZbkBRkFRiorzGZX4g8A

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or have further questions, feel free to ask!

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['token_usage', 'llm_model', 'id', 'chat_id', 'role', 'content', 'context_snippets', 'created_at', 'updated_at', 'llm_provider'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: YWlPPRDDWQFRTx1WvcBGGCO9ReTO7xHtJbIitEUiufYJzXNjp7TK1IsHYGKF55Nx5UD8VWii10_Yve0kSIQoKQ
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: YWlPPRDDWQFRTx1WvcBGGCO9ReTO7xHtJbIitEUiufYJzXNjp7TK1IsHYGKF55Nx5UD8VWii10_Yve0kSIQoKQ
✓ Deleted chat session: DaaaUYUQrr_Dmp7pAgKk-Cnn-B8SPjZ7FGcAx7iEPZeVTmXOLwT9QPLK_eSML8LTMQwZbkBRkFRiorzGZX4g8A
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: A82B4kyPNQVbF7gR1OouhZAUOMRA6mYGLFwNMTvQjXetJcm_gcEdFGa4-xZmXei1iTgJ88Ut7O4mQjqOAnSPjQ

=== Sending Chat Message ===
Message ID: Wzo084xLVg4PsAo8OFWd45LYnjgFfIMiYKVd0TMRFI2DY7YJO_NHhiO-UaN-ad6xB85cyk2Qy-TovfbP8xz8VQ

=== AI Response ===
Here are the products available along with their prices:

1. **ekoDB Pro**
   - Description: Enterprise edition product with advanced features
   - Price: $299

2. **ekoDB Cloud**
   - Description: Fully managed cloud database service product
   - Price: $499

3. **ekoDB**
   - Description: A high-performance database product with AI capabilities
   - Price: $99

Execution Time: 7597ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 91
Total tokens: 3504

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (Python) ===

Created session: MprTxuUL0YCp_S9E-upT826FetfIWhXJuK00zBvHxWYPT0qFW0acOkDLB1zYKuVk7nIHFOoH3-J7a2z9x1NG3w

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}EkoDB is a high-performance, open-source database management system designed for handling large amounts of data and providing efficient querying capabilities. It is particularly focused on supporting key-value and document data models, making it suitable for a variety of applications that require fast access to data.

Key features of ekoDB may include:

1. **Scalability**: EkoDB is designed to scale horizontally, allowing it to handle increased loads by distributing data across multiple nodes.

2. **Performance**: The system emphasizes high performance for both read and write operations, often incorporating advanced indexing and caching strategies.

3. **Flexibility**: As it supports different data models, developers have the flexibility to work with data in ways that are most suitable for their applications.

4. **Open Source**: Being open source means that users can freely access, modify, and use the software, which can facilitate community contributions and enhancements.

5. **Data Durability and Reliability**: EkoDB likely incorporates mechanisms for data integrity and durability, ensuring that data is safely stored and can be recovered in case of failure.

6. **Ease of Use**: The system is designed with usability in mind, often providing libraries or interfaces that make it easier for developers to integrate ekoDB into their applications.

It's important to check the official documentation or the project’s GitHub repository for the most up-to-date information, features, and specific use cases for ekoDB, as the database landscape can evolve rapidly.

--- Stream complete ---
Message ID: EwRdwT-Hq-UyjH1ZnijLRf-Nh9M4RIIyL1hYp9AspnUrYvZpTN4dSel8OX-2RV2FUDLWcdUXyp72lOYLOKiwlA
Execution time: 3000ms
Context window: 128000 tokens

✓ Chat message stream example completed
✓ Client created

=== Get All Chat Models ===
OpenAI models: ['gpt-4-turbo-2024-04-09', 'gpt-realtime-2.1-mini', 'gpt-realtime-2025-08-28', 'gpt-4', 'tts-1-hd', 'gpt-realtime-1.5', 'gpt-realtime', 'gpt-5.2-pro', 'gpt-5-chat-latest', 'gpt-3.5-turbo-0125', 'gpt-5.4-2026-03-05', 'gpt-5-nano', 'gpt-5.4-mini', 'o1-pro', 'gpt-5.1-codex', 'tts-1', 'gpt-5-mini', 'gpt-5-mini-2025-08-07', 'omni-moderation-2024-09-26', 'gpt-live-transcribe', 'gpt-5.6-luna', 'omni-moderation-latest', 'gpt-5-2025-08-07', 'o4-mini-2025-04-16', 'gpt-4o-2024-11-20', 'gpt-5.1', 'whisper-1', 'gpt-5-pro', 'gpt-realtime-whisper', 'gpt-5.5-pro', 'gpt-audio-2025-08-28', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-realtime-2', 'gpt-5-search-api', 'gpt-5.5-2026-04-23', 'chat-latest', 'gpt-realtime-translate', 'gpt-5.4-pro-2026-03-05', 'gpt-5.4-mini-2026-03-17', 'babbage-002', 'gpt-4o-2024-05-13', 'text-embedding-3-large', 'gpt-4o-mini-transcribe-2025-12-15', 'tts-1-1106', 'gpt-5.5-pro-2026-04-23', 'gpt-audio-mini-2025-10-06', 'gpt-5.2-pro-2025-12-11', 'gpt-4-turbo', 'gpt-4o-2024-08-06', 'o3', 'gpt-4o-mini-search-preview-2025-03-11', 'o3-mini-2025-01-31', 'gpt-audio-1.5', 'gpt-image-1.5', 'gpt-5-pro-2025-10-06', 'gpt-5.6-sol', 'gpt-4.1-nano', 'gpt-image-2', 'gpt-3.5-turbo-16k', 'text-embedding-ada-002', 'gpt-realtime-2.1', 'gpt-4.1', 'gpt-5.5', 'gpt-4o-transcribe-diarize', 'gpt-5-codex', 'gpt-4-0613', 'gpt-5.1-2025-11-13', 'gpt-4o-search-preview', 'gpt-4.1-mini', 'gpt-4o-mini', 'gpt-5.4-pro', 'gpt-4o-mini-tts', 'gpt-realtime-mini-2025-12-15', 'gpt-5.1-codex-max', 'gpt-realtime-mini', 'gpt-image-1-mini', 'gpt-5.6-terra', 'gpt-3.5-turbo-instruct-0914', 'o1-2024-12-17', 'sora-2', 'tts-1-hd-1106', 'gpt-4o-mini-tts-2025-12-15', 'gpt-image-1', 'gpt-audio', 'gpt-5.3-codex', 'gpt-5-search-api-2025-10-14', 'gpt-5', 'gpt-4o-mini-transcribe', 'gpt-image-2-2026-04-21', 'gpt-5.2-chat-latest', 'sora-2-pro', 'gpt-3.5-turbo-instruct', 'gpt-transcribe', 'gpt-4.1-nano-2025-04-14', 'o4-mini-deep-research', 'gpt-5.4', 'gpt-4o-search-preview-2025-03-11', 'text-embedding-3-small', 'gpt-5.1-chat-latest', 'gpt-4o-mini-2024-07-18', 'gpt-4.1-2025-04-14', 'gpt-5.3-chat-latest', 'o3-2025-04-16', 'gpt-5.2-2025-12-11', 'chatgpt-image-latest', 'gpt-3.5-turbo', 'gpt-5.2-codex', 'gpt-4o', 'davinci-002', 'gpt-5.4-nano', 'o3-mini', 'o4-mini-deep-research-2025-06-26', 'gpt-audio-mini-2025-12-15', 'o1-pro-2025-03-19', 'gpt-5-nano-2025-08-07', 'gpt-4o-transcribe', 'o4-mini', 'gpt-5.1-codex-mini', 'gpt-3.5-turbo-1106', 'gpt-4o-mini-tts-2025-03-20', 'gpt-5.2', 'gpt-5.4-nano-2026-03-17', 'o1', 'gpt-image-2.5-sunburst-2026-09-08', 'gpt-6-luna', 'gpt-audio-mini', 'gpt-4.1-mini-2025-04-14', 'gpt-image-2.5-flare', 'gpt-4o-mini-search-preview', 'gpt-6-sol', 'gpt-6-astra', 'gpt-6.1-sol', 'gpt-live-1', 'gpt-image-2.5-sunburst', 'gpt-image-2.5-flare-2026-09-08']
Anthropic models: ['claude-haiku-5-5', 'claude-sonnet-5-5', 'claude-opus-5-5', 'claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']
Perplexity models: []
Gemini models: []

=== Get OpenAI Models ===
OpenAI models: ['gpt-4-turbo-2024-04-09', 'gpt-realtime-2.1-mini', 'gpt-realtime-2025-08-28', 'gpt-4', 'tts-1-hd', 'gpt-realtime-1.5', 'gpt-realtime', 'gpt-5.2-pro', 'gpt-5-chat-latest', 'gpt-3.5-turbo-0125', 'gpt-5.4-2026-03-05', 'gpt-5-nano', 'gpt-5.4-mini', 'o1-pro', 'gpt-5.1-codex', 'tts-1', 'gpt-5-mini', 'gpt-5-mini-2025-08-07', 'omni-moderation-2024-09-26', 'gpt-live-transcribe', 'gpt-5.6-luna', 'omni-moderation-latest', 'gpt-5-2025-08-07', 'o4-mini-2025-04-16', 'gpt-4o-2024-11-20', 'gpt-5.1', 'whisper-1', 'gpt-5-pro', 'gpt-realtime-whisper', 'gpt-5.5-pro', 'gpt-audio-2025-08-28', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-realtime-2', 'gpt-5-search-api', 'gpt-5.5-2026-04-23', 'chat-latest', 'gpt-realtime-translate', 'gpt-5.4-pro-2026-03-05', 'gpt-5.4-mini-2026-03-17', 'babbage-002', 'gpt-4o-2024-05-13', 'text-embedding-3-large', 'gpt-4o-mini-transcribe-2025-12-15', 'tts-1-1106', 'gpt-5.5-pro-2026-04-23', 'gpt-audio-mini-2025-10-06', 'gpt-5.2-pro-2025-12-11', 'gpt-4-turbo', 'gpt-4o-2024-08-06', 'o3', 'gpt-4o-mini-search-preview-2025-03-11', 'o3-mini-2025-01-31', 'gpt-audio-1.5', 'gpt-image-1.5', 'gpt-5-pro-2025-10-06', 'gpt-5.6-sol', 'gpt-4.1-nano', 'gpt-image-2', 'gpt-3.5-turbo-16k', 'text-embedding-ada-002', 'gpt-realtime-2.1', 'gpt-4.1', 'gpt-5.5', 'gpt-4o-transcribe-diarize', 'gpt-5-codex', 'gpt-4-0613', 'gpt-5.1-2025-11-13', 'gpt-4o-search-preview', 'gpt-4.1-mini', 'gpt-4o-mini', 'gpt-5.4-pro', 'gpt-4o-mini-tts', 'gpt-realtime-mini-2025-12-15', 'gpt-5.1-codex-max', 'gpt-realtime-mini', 'gpt-image-1-mini', 'gpt-5.6-terra', 'gpt-3.5-turbo-instruct-0914', 'o1-2024-12-17', 'sora-2', 'tts-1-hd-1106', 'gpt-4o-mini-tts-2025-12-15', 'gpt-image-1', 'gpt-audio', 'gpt-5.3-codex', 'gpt-5-search-api-2025-10-14', 'gpt-5', 'gpt-4o-mini-transcribe', 'gpt-image-2-2026-04-21', 'gpt-5.2-chat-latest', 'sora-2-pro', 'gpt-3.5-turbo-instruct', 'gpt-transcribe', 'gpt-4.1-nano-2025-04-14', 'o4-mini-deep-research', 'gpt-5.4', 'gpt-4o-search-preview-2025-03-11', 'text-embedding-3-small', 'gpt-5.1-chat-latest', 'gpt-4o-mini-2024-07-18', 'gpt-4.1-2025-04-14', 'gpt-5.3-chat-latest', 'o3-2025-04-16', 'gpt-5.2-2025-12-11', 'chatgpt-image-latest', 'gpt-3.5-turbo', 'gpt-5.2-codex', 'gpt-4o', 'davinci-002', 'gpt-5.4-nano', 'o3-mini', 'o4-mini-deep-research-2025-06-26', 'gpt-audio-mini-2025-12-15', 'o1-pro-2025-03-19', 'gpt-5-nano-2025-08-07', 'gpt-4o-transcribe', 'o4-mini', 'gpt-5.1-codex-mini', 'gpt-3.5-turbo-1106', 'gpt-4o-mini-tts-2025-03-20', 'gpt-5.2', 'gpt-5.4-nano-2026-03-17', 'o1', 'gpt-image-2.5-sunburst-2026-09-08', 'gpt-6-luna', 'gpt-audio-mini', 'gpt-4.1-mini-2025-04-14', 'gpt-image-2.5-flare', 'gpt-4o-mini-search-preview', 'gpt-6-sol', 'gpt-6-astra', 'gpt-6.1-sol', 'gpt-live-1', 'gpt-image-2.5-sunburst', 'gpt-image-2.5-flare-2026-09-08']

=== Get Anthropic Models ===
Anthropic models: ['claude-haiku-5-5', 'claude-sonnet-5-5', 'claude-opus-5-5', 'claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']

✓ Chat Models API example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: nCNPuKJv8GPtyXLCRvTkHGJYE9kPCha3eUtQo623t7zdHWswiQOEo7XnnWNhZ59T-5xdw9CJY-cGK5ErOAiRDw

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need more information about this product or any others, feel free to ask!

✓ Message 2 sent
  Response: The price of the product **ekoDB** is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: CMWF5zMcVhO_-4tSMpu503Z1S_g-vfQLUBh9er7OqszyExy9vadH6Od-X0MTEQsjRI0ClP0xzGk1LNrULn6Opg
  Parent: nCNPuKJv8GPtyXLCRvTkHGJYE9kPCha3eUtQo623t7zdHWswiQOEo7XnnWNhZ59T-5xdw9CJY-cGK5ErOAiRDw

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: CMWF5zMcVhO_-4tSMpu503Z1S_g-vfQLUBh9er7OqszyExy9vadH6Od-X0MTEQsjRI0ClP0xzGk1LNrULn6Opg (Untitled)
  Session 2: nCNPuKJv8GPtyXLCRvTkHGJYE9kPCha3eUtQo623t7zdHWswiQOEo7XnnWNhZ59T-5xdw9CJY-cGK5ErOAiRDw (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: CMWF5zMcVhO_-4tSMpu503Z1S_g-vfQLUBh9er7OqszyExy9vadH6Od-X0MTEQsjRI0ClP0xzGk1LNrULn6Opg

=== Cleanup ===
✓ Deleted sessions and collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "Ii8jCWNX2g1G9DZoeWnA3l3BcT_8B_1OqP474LxAPk_uG0VIXzOXLbteI4VHxCFaA3P0aBHMxjoTU3m27p5lLA"

=== List Collections ===
Total collections: 13
Sample collections: ['chat_goals__ek0_testing', 'chat_agent_configs__ek0_testing', 'chat_tasks__ek0_testing', 'client_collection_management_python', 'chat_goal_templates__ek0_testing']

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
✓ Created record with plain dict: {'id': 'wA4zWJ24zL0FCo2fk4hHRMvdfgJPAc8ReEHc8VgMts8ZyyM42jAm3xFNbKmGjjMA-RwMOi-MfdX6IOBskCTFFA'}

=== Upsert Operation ===
✓ Upsert (update existing record): wA4zWJ24zL0FCo2fk4hHRMvdfgJPAc8ReEHc8VgMts8ZyyM42jAm3xFNbKmGjjMA-RwMOi-MfdX6IOBskCTFFA
✓ Inserted second record: naY99mZWVdXnuNbnF24mXyB3arbH4SK7iDsD-8HbaMHVBmkq6QHxlodk9iSy3X-Ay2oqzS15vLWvmx90oA2CGg
✓ Upsert (update second record): naY99mZWVdXnuNbnF24mXyB3arbH4SK7iDsD-8HbaMHVBmkq6QHxlodk9iSy3X-Ay2oqzS15vLWvmx90oA2CGg

=== Find One Operation ===
✓ Found user by email: {'id': 'wA4zWJ24zL0FCo2fk4hHRMvdfgJPAc8ReEHc8VgMts8ZyyM42jAm3xFNbKmGjjMA-RwMOi-MfdX6IOBskCTFFA', 'name': {'value': 'Alice Johnson', 'type': 'String'}, 'age': {'type': 'Integer', 'value': 29}, 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}, 'active': {'value': True, 'type': 'Boolean'}}
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
✓ Inserted document: vOSp6OEkK6Pus6meC3P9NDFyqoCBSNGmmMhR45dJ_9R86n-BXayRqIV5XFyn5KDZp27FUg2nzLK52E9-HVEjJQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: a8H4fJG6CIOj6iyI5sSTmfud5W517WahyZLNWc8qCYpjdLcrDYUl_UIsHtUXYdvm70ulY0X9mumDEW_tqGhssQ

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
✓ Edge cache script created: 2Ix1QX585skJmHnmF2F73_9IYx9zL8_OgXCSr_w6JghZls2C8somSSjTUvH2Xj9OwffOPEt0891lrcdo54HIeQ

Call 1: Cache miss (fetches from API)
Response time: 435ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 20.6,
            "time": "2026-10-20T20:00"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.017642974853515625,
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
Response time: 2ms (236.9x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 20.6,
            "time": "2026-10-20T20:00"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.017642974853515625,
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
   ⏱️  Duration: 65.0ms
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
   ⏱️  Duration: 0.5ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 130.0x faster!

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

✅ Function saved: Qt0v6OtplKmVd0KTqlDXhlp1YhS0BuuRFLbL6n6azdUUvcMSm9GgbK3ywbzNDAMCKPIqzwyCsU0reguTxrC1sA
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: 6pAHV_0v4TFGv7Oli0-s8Jb8DaqJk7PksBCQcIGpVpvMNpMWGVmulyRUyFwit7CGugcJqpfQ0MzWb93DO1J-KA
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: SbWsywZBw4SqJLZnS8707RjFEgcqMnKOB_JggzjODzttRadnHTOth7gDUrPAl3ZEvzABr8AoyZ49ZIyplfcJgA
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

1. **Efficient Similarity Searches**: They enable fast searching and retrieval of data based on vector similarity, which is ideal for tasks like recommendation systems and image retrieval.

2. **High-Dimensional Data Handling**: They are optimized for storing and processing high-dimensional vectors, which are common in machine learning applications.

3. **Scalability**: Vector databases can handle large volumes of data efficiently, often leveraging distributed architectures.

4. **Integration with AI and ML**: They support advanced analytics and machine learning model outputs, making it easier to manage and query embeddings.

5. **Real-Time Processing**: Many vector databases are designed for quick data ingestion and real-time querying, suitable for dynamic applications.

6. **Flexibility**: They can accommodate various types of data and use cases, including text, images, and audio embeddings.

7. **Support for Advanced Indexing**: Techniques like HNSW or ANNOY provide optimized indexing methods for faster retrieval times compared to traditional databases.

8. **Rich Query Capabilities**: Users can perform complex queries based on vector distance metrics, enabling nuanced data exploration.

These features make vector databases particularly powerful for applications in AI, natural language processing, and other fields requiring high-performance data analysis.
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

✅ Function saved: o5CYRYm-5NxpOJHxaxNacHsr9D5JEbkwyPD6wh5lVEZiNo_6VaAwGYEgp43PwAlZRAm3C2cpzQ7K4vbfPMYsLQ
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
   {'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 3}}
   {'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 2}}
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

✅ Inserted order: hnAApYsOT4sVKzuvt3cjQ6G9htiQBalQOf1xT_E7EkQfWaSjUOCXPT7_olkpgBfRGlHae2rxtQ9169IWYdu_Vw
✅ Inserted 2 products with wrapped types

📝 Example 2: Querying and Extracting Wrapped Types

📊 Found 2 products
   • Laptop Pro
   • Wireless Mouse

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

✅ Inserted order: 3M1ZQAN5VbYP6fy4FnDcG7V28UNTQs7uJNOCNiQ1qKJK0Xbp-ZDwdUyw5OMfXqYGoycyQFOcHPx_7zqhIP4h2g
✅ Cached order status
📊 Quick status lookup: {'value': '{"type":"Object","value":{"status":"processing","updated_at":"2026-10-20T20:02:49.923939+00:00"}}'}

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
   1. Getting Started with ekoDB (Database)
   2. Introduction to Machine Learning (AI)
   3. Vector Databases Explained (Database)
   4. Database Design Principles (Database)
   5. Natural Language Processing (AI)
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
Created template: Data Migration (id: jm-Rb3OxkcOD5i10hYYCZV2JAHf3TXb-Gn0xhgW-JfrbihCv6DFPiGiwB1b7q8oJaKzWmLO__wNMgn3YaF0BeA)

--- Listing templates ---
Templates: {'count': 1, 'items': [{'description': {'type': 'String', 'value': 'Template for migrating data between schemas'}, 'id': 'jm-Rb3OxkcOD5i10hYYCZV2JAHf3TXb-Gn0xhgW-JfrbihCv6DFPiGiwB1b7q8oJaKzWmLO__wNMgn3YaF0BeA', 'steps': {'type': 'Array', 'value': [{'description': 'Analyze source schema'}, {'description': 'Create target schema'}, {'description': 'Migrate records'}, {'description': 'Validate results'}]}, 'title': {'type': 'String', 'value': 'Data Migration'}}]}

--- Getting template ---
Fetched: {'type': 'String', 'value': 'Data Migration'}

--- Updating template ---
Updated description: {'type': 'String', 'value': 'Updated: comprehensive data migration workflow'}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
=== ekoDB Goals / Tasks / Agents Integration Example (Python) ===

--- goal_create ---
Created goal: Deploy v3 (id: gPzgQkh0gXEtE0JGjkFf7JPGWDIzDxmc_4dkkgZKqSTtrZTKjRR8Wtm-tzfVd5k0Yr5uJf4zlSvR5c2MEZI3wg)

--- goal_list ---
Goals: {'count': 1, 'goals': [{'created_at': '2026-10-20T20:02:50.272828+00:00', 'description': 'Deploy version 3 to production', 'id': 'gPzgQkh0gXEtE0JGjkFf7JPGWDIzDxmc_4dkkgZKqSTtrZTKjRR8Wtm-tzfVd5k0Yr5uJf4zlSvR5c2MEZI3wg', 'status': 'pending', 'steps': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]', 'title': 'Deploy v3', 'updated_at': '2026-10-20T20:02:50.272828+00:00'}]}

--- goal_get ---
Fetched: {'type': 'String', 'value': 'Deploy v3'}

--- goal_update ---
Updated description: {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}

--- goal_search ---
Search results: {'count': 1, 'items': [{'_score': 13.2, 'created_at': {'type': 'DateTime', 'value': '2026-10-20T20:02:50.272828+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'gPzgQkh0gXEtE0JGjkFf7JPGWDIzDxmc_4dkkgZKqSTtrZTKjRR8Wtm-tzfVd5k0Yr5uJf4zlSvR5c2MEZI3wg', 'status': {'type': 'String', 'value': 'pending'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-20T20:02:50.277832+00:00'}}]}

--- goal_step_start (step 0) ---
Step 0 started: {'created_at': {'type': 'DateTime', 'value': '2026-10-20T20:02:50.272828+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'gPzgQkh0gXEtE0JGjkFf7JPGWDIzDxmc_4dkkgZKqSTtrZTKjRR8Wtm-tzfVd5k0Yr5uJf4zlSvR5c2MEZI3wg', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"InProgress"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-20T20:02:50.283185+00:00'}}

--- goal_step_complete (step 0) ---
Step 0 completed: {'created_at': {'type': 'DateTime', 'value': '2026-10-20T20:02:50.272828+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'gPzgQkh0gXEtE0JGjkFf7JPGWDIzDxmc_4dkkgZKqSTtrZTKjRR8Wtm-tzfVd5k0Yr5uJf4zlSvR5c2MEZI3wg', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-20T20:02:50.287431+00:00'}}

--- goal_step_fail (step 1) ---
Step 1 failed: {'created_at': {'type': 'DateTime', 'value': '2026-10-20T20:02:50.272828+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'gPzgQkh0gXEtE0JGjkFf7JPGWDIzDxmc_4dkkgZKqSTtrZTKjRR8Wtm-tzfVd5k0Yr5uJf4zlSvR5c2MEZI3wg', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","error":"Migration failed: column already exists","status":"Failed"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-20T20:02:50.291686+00:00'}}

--- goal_complete ---
Goal completed: {'type': 'String', 'value': 'pending_review'}

--- goal_approve ---
Goal approved: {'type': 'String', 'value': 'in_progress'}

--- goal_reject ---
Goal rejected: {'type': 'String', 'value': 'failed'}

--- task_create ---
Created task: Nightly Backup (id: ldFRnZA6vWvv2hCeeRLqHXkn5JJvUEyy7pedkdovHe5WWv8uyvgg1AglyjiEXhts3QfeooyGNgPGsVbjN7xHdw)

--- task_list ---
Tasks: {'count': 1, 'items': [{'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'ldFRnZA6vWvv2hCeeRLqHXkn5JJvUEyy7pedkdovHe5WWv8uyvgg1AglyjiEXhts3QfeooyGNgPGsVbjN7xHdw', 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}}]}

--- task_get ---
Fetched: {'type': 'String', 'value': 'Nightly Backup'}

--- task_start ---
Task started: {'type': 'String', 'value': 'running'}

--- task_succeed ---
Task succeeded: {'consecutive_failures': {'type': 'Integer', 'value': 0}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'ldFRnZA6vWvv2hCeeRLqHXkn5JJvUEyy7pedkdovHe5WWv8uyvgg1AglyjiEXhts3QfeooyGNgPGsVbjN7xHdw', 'last_error': {'type': 'Null', 'value': None}, 'last_run': {'type': 'DateTime', 'value': '2026-10-20T20:02:50.331956+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-20T20:02:50.331956+00:00'}}

--- task_pause ---
Task paused: {'type': 'String', 'value': 'paused'}

--- task_resume ---
Task resumed: {'type': 'String', 'value': 'active'}

--- task_fail ---
Task failed: {'consecutive_failures': {'type': 'Integer', 'value': 1}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'ldFRnZA6vWvv2hCeeRLqHXkn5JJvUEyy7pedkdovHe5WWv8uyvgg1AglyjiEXhts3QfeooyGNgPGsVbjN7xHdw', 'last_error': {'type': 'String', 'value': 'Disk full'}, 'last_run': {'type': 'DateTime', 'value': '2026-10-20T20:02:50.351113+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'next_run': {'type': 'DateTime', 'value': '2026-03-22T02:00:00+00:00'}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-20T20:02:50.351113+00:00'}}

--- task_due ---
Due tasks: {'count': 0, 'items': []}

--- task_delete ---
Task deleted

--- agent_create ---
Created agent: CodeReviewer (id: ptj5TV5yYQGFVjgjHu5LXXL1_2CDLQq5Z8eJJQy1ba0Kj9MLywmVyiLbadJclySJX9ScD8jckyKcSrC1ciwAAQ)

--- agent_list ---
Agents: {'count': 1, 'items': [{'deployment_id': {'type': 'String', 'value': 'deploy_test'}, 'id': 'ptj5TV5yYQGFVjgjHu5LXXL1_2CDLQq5Z8eJJQy1ba0Kj9MLywmVyiLbadJclySJX9ScD8jckyKcSrC1ciwAAQ', 'llm_model': {'type': 'String', 'value': 'gpt-4o'}, 'name': {'type': 'String', 'value': 'CodeReviewer'}, 'system_prompt': {'type': 'String', 'value': 'You review code for correctness and style.'}, 'tools': {'type': 'Array', 'value': ['web_search', 'file_read']}}]}

--- agent_get ---
Fetched: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_get_by_name ---
By name: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_update ---
Updated agent prompt: {'type': 'String', 'value': 'You review code. Be concise.'}

--- agents_by_deployment ---
Agents in deploy_test: {'count': 0, 'items': []}
WARNING: agents_by_deployment omitted created agent ptj5TV5yYQGFVjgjHu5LXXL1_2CDLQq5Z8eJJQy1ba0Kj9MLywmVyiLbadJclySJX9ScD8jckyKcSrC1ciwAAQ; TODO: check/fix the server-side deployment lookup

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
Inserted document: t6s0ojNhq3qM7GjPh_OC8AZJ_jaAvJKebs4-2eBDWmEJWIxiSowSIRDRD7kP4OhxuYiv5y4RxMLOR93pi2cXrw

--- kv_link ---
Linked: None

--- kv_get_links ---
Links for app:config: [{'collection': 'kv_links_example_py', 'created_at': '2026-10-20T20:02:50.847959Z', 'document_id': 't6s0ojNhq3qM7GjPh_OC8AZJ_jaAvJKebs4-2eBDWmEJWIxiSowSIRDRD7kP4OhxuYiv5y4RxMLOR93pi2cXrw', 'field_path': None, 'last_accessed': '2026-10-20T20:02:50.848215Z', 'metadata': {}}]

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
  kv_operations:py:cache:product:2: {'price': 39.99, 'name': 'Product 2'}
  kv_operations:py:cache:product:3: {'price': 49.99, 'name': 'Product 3'}

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
    "name": "Earth",
    "diameter_km": 12742
  },
  {
    "name": "Mars",
    "diameter_km": 6779
  },
  {
    "name": "Jupiter",
    "diameter_km": 139820
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===
=== ekoDB Schedule Management Example (Python) ===

--- create_schedule ---
Created schedule: Nightly Report (id: 83e8a7a6-f172-4393-83d3-aaa111f50563)

--- list_schedules ---
Schedules: {'count': 1, 'schedules': [{'created_at': '2026-10-20T20:02:54.062803Z', 'cron_expression': '0 0 0 * * *', 'description': 'Generate and email nightly analytics report', 'enabled': True, 'function_label': 'schedule_noop_python_56896', 'id': '83e8a7a6-f172-4393-83d3-aaa111f50563', 'last_execution': None, 'name': 'Nightly Report', 'next_execution': '2026-10-21T00:00:00Z', 'parameters': {}, 'stats': {'avg_execution_time_ms': 0.0, 'failed_executions': 0, 'last_error': None, 'successful_executions': 0, 'total_executions': 0}, 'timezone': 'UTC', 'updated_at': '2026-10-20T20:02:54.062803Z'}]}

--- get_schedule ---
Fetched: Nightly Report cron=0 0 0 * * *

--- update_schedule ---
Updated cron: 0 30 1 * * *

--- trigger_schedule ---
Triggered: {'schedule_id': '83e8a7a6-f172-4393-83d3-aaa111f50563', 'status': 'triggered'}

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
  1. Score: 12.870, Matched: email, name
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: bio, title
  2. Score: 13.200, Matched: bio, title
  3. Score: 13.200, Matched: bio, title
  4. Score: 13.200, Matched: bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: bio, title
  2. Score: 26.400, Matched: bio, title
  3. Score: 26.400, Matched: title, bio
  4. Score: 26.400, Matched: bio, title

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
  1. Score: 0.773
  2. Score: 0.761
  3. Score: 0.754

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.377, Matched: content, title
  2. Score: 0.887, Matched: content, title
  3. Score: 0.381, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 6.600, Matched: title

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Deep Learning Fundamentals (category: ml)
  2. Introduction to Machine Learning (category: ml)

Paired vector search: document=Rust Programming Answer, query=Rust Programming Question
=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': '2D1YzT2utjCqvVcz32kfaS5YaDcrZV2XemqmfEGLSr6QyOCoy55q7veg2jQlxmubElk6B81yc6y4SmlYkmHo1g'}

=== Find by ID ===
Found: {'id': '2D1YzT2utjCqvVcz32kfaS5YaDcrZV2XemqmfEGLSr6QyOCoy55q7veg2jQlxmubElk6B81yc6y4SmlYkmHo1g', 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'name': {'type': 'String', 'value': 'Test Record'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'price': {'value': 99.99, 'type': 'Float'}, 'value': {'type': 'Integer', 'value': 42}, 'created_at': {'type': 'String', 'value': '2026-10-20T16:02:54.514135'}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-10-20 16:02:54.514135
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'id': '2D1YzT2utjCqvVcz32kfaS5YaDcrZV2XemqmfEGLSr6QyOCoy55q7veg2jQlxmubElk6B81yc6y4SmlYkmHo1g', 'data': 'aGVsbG8gd29ybGQ=', 'active': True, 'tags': ['tag1', 'tag2', 'tag3'], 'name': 'Test Record', 'categories': ['electronics', 'computers'], 'price': 99.99, 'value': 42, 'created_at': '2026-10-20T16:02:54.514135', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'metadata': {'key': 'value', 'nested': {'deep': True}}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'id': '2D1YzT2utjCqvVcz32kfaS5YaDcrZV2XemqmfEGLSr6QyOCoy55q7veg2jQlxmubElk6B81yc6y4SmlYkmHo1g', 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'price': {'type': 'Float', 'value': 99.99}, 'created_at': {'type': 'String', 'value': '2026-10-20T16:02:54.514135'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'value': {'type': 'Integer', 'value': 100}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'active': {'value': True, 'type': 'Boolean'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: N0OrindsygZ1R8MovOYIC98CU7BzfjXC0x-kunTE9GL4Onz5liPVmEYoMdd6bDxq2RhIyppDdJW3UjjyCTQ6cA

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
✓ Created native SWR script: github_user_native_py (fjz23WIwTvWGD-xFbbVzN-DvRzoRrownM9rOBdqZb2pDOM2VR3oh_BU6RusOToZn7jcX6IlZ8P3UZEe1T20ctg)

First call (cache miss - will fetch from GitHub API):
  Response time: 133ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 1ms
  Speedup: 212.6x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_py (k3_6ruOBq8_DlUXssBZrdsN00zazRAMpNoOdEs5kKPU2fi8OHraedyHxh-BSx9LwYuAKUXt5HEA9E7vha9tZyg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_py' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_py (vyicxqomYCBV8uES_oRLSM-SKx42nsqt5AI9l65-jIyQjF6pxJRQJ2JIILWn8byoLbcesF1zoPFW8dChSadU3Q)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_py' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_py (tMEUbhFVE3Z4h_o54-Gu6rmXMOOsJ57a-GX8vX3tvCernGqnpgEFVvU1ExnNjlq8Fc_7WJidf9N9-7rljrDgsQ)
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
✓ Created SWR script: fetch_api_user_py (qf8xW49kbWWXkQegqXnUgVim6DcSActbptW6hajQMlXIBPIjsNGgrk8MtCi6Ww3tSFmoiLy5iFOcXAAqpvlGww)

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
Response time: 1ms (served from cache)
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
Created Alice: $1000 - ID: HDkdJOLpa0zxrq7W0u98HcUEQFzKscXF4tDJLwhQmb53CCRfeShMfJlmoKcS9eapcovolTDpNvdS0vabCfSsIA
Created Bob: $500 - ID: FBDWzwcZfB_-bFgXsbx9Ir5pcyRtB2fZTwOSxWSqizqFdmkx9OiIvaQqQqfmmB6YToM2N-lbbAdm6p3A-Bd-Vw

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 925c49fe-3113-456a-9edd-49973fa2371d

=== Example 2: Operations within Transaction ===
Alice in transaction: {'account_id': {'type': 'String', 'value': 'ACC001'}, 'balance': {'type': 'Integer', 'value': 800}, 'name': {'type': 'String', 'value': 'Alice'}, 'id': 'HDkdJOLpa0zxrq7W0u98HcUEQFzKscXF4tDJLwhQmb53CCRfeShMfJlmoKcS9eapcovolTDpNvdS0vabCfSsIA'}
Bob in transaction: {'name': {'type': 'String', 'value': 'Bob'}, 'balance': {'value': 700, 'type': 'Integer'}, 'account_id': {'type': 'String', 'value': 'ACC002'}, 'id': 'FBDWzwcZfB_-bFgXsbx9Ir5pcyRtB2fZTwOSxWSqizqFdmkx9OiIvaQqQqfmmB6YToM2N-lbbAdm6p3A-Bd-Vw'}
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

✓ Verified committed balances: Alice=$800, Bob=$700

=== Example 5: Rollback Demo ===
New transaction: de2fd121-d09c-4824-b283-bd3d7272051c
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: YkGNNRC6qplXPSuXvQrpBjUh10x1JFcuJ1bnZO8mYByK4UnPH5aPZZ8dl5cgcHBeW1l4OemqlBbTQRykGvetHw

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

Created chat session: mFBBvl_69b0CodzumAFRnJ2rjNf5vC1grEGpgZRQMgxkcsx1Y4TnirSPECUsaxN5xccj02U6DyXR-Iok6gwrGA

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: CuCrr1gsAensHYrNeShU7_E2DQoYhZki-sx51-Kzd8pqaOFbFc6G3pcOv_AHCkrsiEn5TYCw4mt3gK2GiPKXqg
Execution time: 516ms
Token usage: {'completion_tokens': 8, 'prompt_tokens': 15, 'total_tokens': 23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example (Python) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_py' ===
✓ Subscribed (subscription_id: sub_5bf5503d1b43466ea2427529bd9faa77)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: CFCb7jAhU78eKhfIePi7Bh5UItdUvJAfBJ1aF-zWOZISw9P643zYokD428_XPR9E4-Dv2MBGb4-2WTNN5z0WJg

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: CFCb7jAhU78eKhfIePi7Bh5UItdUvJAfBJ1aF-zWOZISw9P643zYokD428_XPR9E4-Dv2MBGb4-2WTNN5z0WJg
     Timestamp:  2026-10-20T20:02:56.813309+00:00

Inserting record 2...
✓ Inserted: q4FQsn_zUW2QEnG2ldwu8U9GeJmxUqgZQOmy-USIFesPopj4Fs6fxeyHTl47n8NVjZE4kYMDIpyoSrAik_NMIg

  📡 Notification received:
     Event:      insert
     Record IDs: q4FQsn_zUW2QEnG2ldwu8U9GeJmxUqgZQOmy-USIFesPopj4Fs6fxeyHTl47n8NVjZE4kYMDIpyoSrAik_NMIg

=== Unsubscribing ===
✓ Unsubscribed: {'collection': 'ws_subscribe_example_py', 'found': True, 'unsubscribed': True}

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_py'

✓ WebSocket subscription example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: gAvAV_7yX5lBcsELlC9-s3NyG3VanpU8sf7ZiSNFJg48EK9QK8j3cxcf2pj8D2VALQ-0tC7rVd9chVieS0sM1A

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
   Inserted with ripple: {'id': '9lDrgF3M7zBwrUW1z7YG5JVDvrmg_AknFVt29vdPRvbqjaTyMkOG0i6id4AIEW9KdOCxIkAaOPXAJApRNjERzw'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'oMRUsHmQuiTRyo-ceG4plIW2kPy0wfXOZ8tI4tbgWL9gU7DpYY4iRe1-FlfWXv_0hnr7e7g33UyXGwLj5LW44g'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'name': {'value': 'Product 1', 'type': 'String'}, 'id': '9lDrgF3M7zBwrUW1z7YG5JVDvrmg_AknFVt29vdPRvbqjaTyMkOG0i6id4AIEW9KdOCxIkAaOPXAJApRNjERzw', 'price': {'type': 'Integer', 'value': 150}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'price': {'value': 500, 'type': 'Integer'}, 'id': '9lDrgF3M7zBwrUW1z7YG5JVDvrmg_AknFVt29vdPRvbqjaTyMkOG0i6id4AIEW9KdOCxIkAaOPXAJApRNjERzw', 'name': {'type': 'String', 'value': 'Upsert Product'}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['id', 'name', 'email']
  First user: {'type': 'String', 'value': 'Dave Brown'} <{'value': 'dave@example.com', 'type': 'String'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['id', 'bio', 'age', 'user_role', 'avatar_url', 'name', 'status', 'created_at', 'email']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'type': 'String', 'value': 'Dave Brown'} (age {'type': 'Integer', 'value': 45})
    - {'type': 'String', 'value': 'Alice Johnson'} (age {'value': 30, 'type': 'Integer'})
    - {'type': 'String', 'value': 'Bob Smith'} (age {'value': 25, 'type': 'Integer'})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'value': 'Carol White', 'type': 'String'}: {'value': 'Manager', 'type': 'String'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['id', 'created_at', 'user_role', 'api_key', 'bio', 'email', 'name', 'password', 'age', 'status', 'avatar_url', 'secret_token']
  Projected query:
    - 3 fields per record
    - Fields: ['id', 'email', 'name']
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
Inserted: map[id:D881MFdBfQYW-D2OxbR_fQ61T0MCXTmsjD_-unqkvVddEtQIOq6k8JGTTG1SWX8TfZenE0iq-sYDkV55yQbvPA]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:D881MFdBfQYW-D2OxbR_fQ61T0MCXTmsjD_-unqkvVddEtQIOq6k8JGTTG1SWX8TfZenE0iq-sYDkV55yQbvPA name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: [map[active:map[type:Boolean value:true] id:D881MFdBfQYW-D2OxbR_fQ61T0MCXTmsjD_-unqkvVddEtQIOq6k8JGTTG1SWX8TfZenE0iq-sYDkV55yQbvPA name:map[type:String value:Test Record] value:map[type:Integer value:42]]]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:D881MFdBfQYW-D2OxbR_fQ61T0MCXTmsjD_-unqkvVddEtQIOq6k8JGTTG1SWX8TfZenE0iq-sYDkV55yQbvPA name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.go completed successfully

=== Running simple_websocket.go ===
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: geV_g4HJlFgMeXTdVZkkZGXB6LMI94YYhK0LiqSMw-3GAt79Ba-MimqS-E-sQleWTHPW-QPLtX1m9F78Bu5mvw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1792526579947362000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "geV_g4HJlFgMeXTdVZkkZGXB6LMI94YYhK0LiqSMw-3GAt79Ba-MimqS-E-sQleWTHPW-QPLtX1m9F78Bu5mvw",
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

✅ Function saved: ilxDrkbYzV_cNchKRSzcEVDWOoaCzXSGiQs-NTdnd_v8BnY-yJuVL4Np2IjgGIrfX62HDZJQhvJKD64TnoeLXw
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: 03PPivPNMTBBDXHQys8EBx1jbHzRYqCP9PNZxTiMMLNYpIHXJ_oRoBHSZyYC-4CX7zEAXI9LXQGaRacsEO6Q1A
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: 84VoOCYqA7zvoLBPZYDOV2X7SgUA-8rwummsdeQZonp22CdsT5zHTg-UtwAhzemWaqI0ktMWl_wx8Fh9bHoQiw
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
Collection created with first record: hFu5M2qfE98Gryrt8Q4C0MSVvJjXYgmPVoO-dGNtkj8095yQnaJxsfh3QbI60bLD221JylOdH4qxdibPz4fGCg

=== List Collections ===
Total collections: 13
Sample collections: [chat_goals__ek0_testing chat_agent_configs__ek0_testing chat_tasks__ek0_testing collection_management_go chat_goal_templates__ek0_testing]

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ collection_management.go completed successfully

=== Running transactions.go ===
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: KmNzJ9ZRXzCDrx0E-34dU8_vuQC653N8KeZE41P-whMW5xZ9MkP_8Pf5HV-mIWKTQMPrmttmp4b4UKjFo070Tw
Created Bob: $500 - ID: zUb1iSScUbIb4r5WblaIIIu-xy1lw3CNtgxCreeS8EBtcj6i8RL2v_msmSlpRYVV9jbIKhlo5nv9KHMozTeSbg

=== Example 1: Begin Transaction ===
Transaction ID: 6ec6211c-57e4-45de-8fd0-aaf9f8032d9b

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
New transaction: 8efa362b-e47a-481f-8674-f7a96c0ecfa9
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
   ✅ Function saved: H5ad90yTLr3nRwsvDXmMe_2sz34ydpU4KUPLICfJQ_yx8M8f3stCdDaIdaXFX7ARPyXfNx7q1Dl6NnMEpR2_QQ

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: m14iYMirOJslbGrx8Zsv8nxcqvCMKwU9ZFuYZf66wG2ZJ3fdw9vBMKrDPDk_IPQv2SIXQAkcqmay5ap08rYLZA
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: WA1ZXcOuiOsxNzvUAtisSpmTLmR1ySzQdf6Zj3VDnOsy2YqmMLLTOXUSW0vD15WC7u1zeOZfDonkHH3geBUwNQ

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
   ✅ Function saved: ZOtB9sZW4ArCHPFS2eSSLrYkHvCTJ2vWME_5w7L9tzkTD39gtcSW8sLRCUOoSS0FwD0k9wUrb-x_bJo0P0mCvQ

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
   ✅ Function saved: zrR9aGLyA-IJQu6r6Lc0GdN_ui9UtxlBzvZbw-qFzkksXzdtzIXFLFVGcLEuh6sssxGSGro0Nf2NAnO4HpQNRQ

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: H5ad90yTLr3nRwsvDXmM...
   ✅ Deleted script: WA1ZXcOuiOsxNzvUAtis...
   ✅ Deleted script: ZOtB9sZW4ArCHPFS2eSS...
   ✅ Deleted script: zrR9aGLyA-IJQu6r6Lc0...
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
  Output: Document ID = hu_KdgF-BqrNZs3FXxaLZLhjw-Kb9V-ADdBi5nmrjDugwEsx70kf_czXwHhb5TiRhTEtXwY9UK-hAcXtsAiGGA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(hu_KdgF-BqrNZs3FXxaLZLhjw-Kb9V-ADdBi5nmrjDugwEsx70kf_czXwHhb5TiRhTEtXwY9UK-hAcXtsAiGGA)
  Output: Found document with name = map[type:String value:TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(hu_KdgF-BqrNZs3FXxaLZLhjw-Kb9V-ADdBi5nmrjDugwEsx70kf_czXwHhb5TiRhTEtXwY9UK-hAcXtsAiGGA)
  Output: Error (expected) - request failed with status 404: ��error�Record has been deleted
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
  Output: Document ID = kIMWKYHNLtuMS1pCUFsEn6g_yXweKkCOPmj8Bo_1jdVFXSqXzEz3tSZ_aSyyKKjSVt3cYaJY6Qz3hLOL66rEFw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(kIMWKYHNLtuMS1pCUFsEn6g_yXweKkCOPmj8Bo_1jdVFXSqXzEz3tSZ_aSyyKKjSVt3cYaJY6Qz3hLOL66rEFw)
  Output: Found document with name = map[type:String value:WS TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(kIMWKYHNLtuMS1pCUFsEn6g_yXweKkCOPmj8Bo_1jdVFXSqXzEz3tSZ_aSyyKKjSVt3cYaJY6Qz3hLOL66rEFw)
  Output: Error (expected) - request failed with status 404: ��error�Record has been deleted
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
Admin permissions read successfully
=== ekoDB Advanced CRUD Integration Example (Go) ===

--- Inserting base record ---
Inserted record: FRE06lKhR8PY6JBIZgVrEnS2wB6fJLMRDDRMzM6kJuhsvcbtDqb8i_DLPO77e3FPriQ4qL6nO4wZaoPNGToWng

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
✓ Created session: 2Mx8UE5i-xKLDnZcMY2VeLI0Lpag9ZliPspDudkZE4tFhhX-RIWnFu7NkkiBaQsh5li6gIlaA-tdJFI8nbOhWg

=== Sending Initial Message ===
✓ Message sent
  Response: It appears that there are no available products found in the database at this time. If you need help with something else or want to inquire about specific products, feel free to ask!

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
✓ Created second session: rgMXRcidjV_GLtpbCoKj4gxZd3e_2asq5biZ_fypNOq5jWX0gW8ayyelShUZFSIM3Zlmo6j5sfGusQg3aXy13g
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: rgMXRcidjV_GLtpbCoKj4gxZd3e_2asq5biZ_fypNOq5jWX0gW8ayyelShUZFSIM3Zlmo6j5sfGusQg3aXy13g
✓ Deleted chat session: 2Mx8UE5i-xKLDnZcMY2VeLI0Lpag9ZliPspDudkZE4tFhhX-RIWnFu7NkkiBaQsh5li6gIlaA-tdJFI8nbOhWg
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: ZhxzGoIZYntqkli8EWlkpqyxVljvIWCZr809yQT8gHi7XnLKXlaWcvu0fpGFyHWidWC_A8evKGc-MzF4E3s1eg

=== Sending Chat Message ===
Message ID: AFlumZYk5RYni_wz4FPdBZJOqO_D_iYYDMFqKqOtrNbQOXR4arU5IE3tk_xkQ-s6m2qr3w8fpYDxacb_a1WXsA

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product

2. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities

3. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:33grr6hro2menYfvpuwKjsTLL28xiDLqwvIqWh9vEblCGMmqXTbbE83f2GlwYMQL6EqEMzA8yxJqnzmER8OXXg name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:upQa7k3iPDmEW3NQwdiGRV03Zi6Pm0NK3dLbS9yqxzTQ_DSdrjQ4-Impc0iijvnzKutJ23-kw98ucZWvSDe9eg name:ekoDB price:99] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:v--WsZ7lhg-eEV8g19mTd29rw_0amFhYcMLrtqT2-9BH7VTeo9ZFWtq39oTike4INPCTJ2XiG-aXMUZeqRUbHw name:ekoDB Pro price:299] score:0.1111111111111111]

Execution Time: 3204ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 91
Total tokens: 3504

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (Go) ===

Created session: 86mxDl8StQ9_ldfZBTwJ3GFjgmcswZaR5zrfFLUnMZDW6RQAvTS_Jaw8eU7aTwbIHWKuzSUSNRDifN5weNX5Cw

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}EkoDB is an innovative database designed for high-performance, real-time applications, particularly those involving complex data structures and large datasets. It is optimized for transaction processing, offering features such as high concurrency, low-latency response times, and robust data integrity. EkoDB is often used in scenarios requiring fast data access and reliable data management, such as financial services, telecommunications, and large-scale web applications.

Key features of EkoDB might include:

1. **Scalability**: It can handle increasing amounts of data and transactions efficiently.
2. **ACID Compliance**: Ensures that all database transactions are processed reliably.
3. **High Availability**: Designed to minimize downtime and data loss with features like replication and failover.
4. **Real-time Analytics**: Capable of performing analytics on live data with minimal performance impact.
5. **Data Modeling**: Support for complex data types and structures, enabling flexible schema design.

If you are looking for specific information about its architecture, use cases, or deployment scenarios, feel free to ask!

--- Stream complete ---
Message ID: X8SYZtqsBWQvwgHpfnOA2iRBfY0MDLsM-CF5kQd19epHnSf28bepFq6hFNJvboAtzC5W_HnLq48yQpRhBmWJ4g
Execution time: 2328ms
Context window: 128000 tokens

✓ Chat message stream example completed
✓ Client created

=== Get All Chat Models ===
OpenAI models: [gpt-4-turbo-2024-04-09 gpt-realtime-2.1-mini gpt-realtime-2025-08-28 gpt-4 tts-1-hd gpt-realtime-1.5 gpt-realtime gpt-5.2-pro gpt-5-chat-latest gpt-3.5-turbo-0125 gpt-5.4-2026-03-05 gpt-5-nano gpt-5.4-mini o1-pro gpt-5.1-codex tts-1 gpt-5-mini gpt-5-mini-2025-08-07 omni-moderation-2024-09-26 gpt-live-transcribe gpt-5.6-luna omni-moderation-latest gpt-5-2025-08-07 o4-mini-2025-04-16 gpt-4o-2024-11-20 gpt-5.1 whisper-1 gpt-5-pro gpt-realtime-whisper gpt-5.5-pro gpt-audio-2025-08-28 gpt-4o-mini-transcribe-2025-03-20 gpt-realtime-2 gpt-5-search-api gpt-5.5-2026-04-23 chat-latest gpt-realtime-translate gpt-5.4-pro-2026-03-05 gpt-5.4-mini-2026-03-17 babbage-002 gpt-4o-2024-05-13 text-embedding-3-large gpt-4o-mini-transcribe-2025-12-15 tts-1-1106 gpt-5.5-pro-2026-04-23 gpt-audio-mini-2025-10-06 gpt-5.2-pro-2025-12-11 gpt-4-turbo gpt-4o-2024-08-06 o3 gpt-4o-mini-search-preview-2025-03-11 o3-mini-2025-01-31 gpt-audio-1.5 gpt-image-1.5 gpt-5-pro-2025-10-06 gpt-5.6-sol gpt-4.1-nano gpt-image-2 gpt-3.5-turbo-16k text-embedding-ada-002 gpt-realtime-2.1 gpt-4.1 gpt-5.5 gpt-4o-transcribe-diarize gpt-5-codex gpt-4-0613 gpt-5.1-2025-11-13 gpt-4o-search-preview gpt-4.1-mini gpt-4o-mini gpt-5.4-pro gpt-4o-mini-tts gpt-realtime-mini-2025-12-15 gpt-5.1-codex-max gpt-realtime-mini gpt-image-1-mini gpt-5.6-terra gpt-3.5-turbo-instruct-0914 o1-2024-12-17 sora-2 tts-1-hd-1106 gpt-4o-mini-tts-2025-12-15 gpt-image-1 gpt-audio gpt-5.3-codex gpt-5-search-api-2025-10-14 gpt-5 gpt-4o-mini-transcribe gpt-image-2-2026-04-21 gpt-5.2-chat-latest sora-2-pro gpt-3.5-turbo-instruct gpt-transcribe gpt-4.1-nano-2025-04-14 o4-mini-deep-research gpt-5.4 gpt-4o-search-preview-2025-03-11 text-embedding-3-small gpt-5.1-chat-latest gpt-4o-mini-2024-07-18 gpt-4.1-2025-04-14 gpt-5.3-chat-latest o3-2025-04-16 gpt-5.2-2025-12-11 chatgpt-image-latest gpt-3.5-turbo gpt-5.2-codex gpt-4o davinci-002 gpt-5.4-nano o3-mini o4-mini-deep-research-2025-06-26 gpt-audio-mini-2025-12-15 o1-pro-2025-03-19 gpt-5-nano-2025-08-07 gpt-4o-transcribe o4-mini gpt-5.1-codex-mini gpt-3.5-turbo-1106 gpt-4o-mini-tts-2025-03-20 gpt-5.2 gpt-5.4-nano-2026-03-17 o1 gpt-image-2.5-sunburst-2026-09-08 gpt-6-luna gpt-audio-mini gpt-4.1-mini-2025-04-14 gpt-image-2.5-flare gpt-4o-mini-search-preview gpt-6-sol gpt-6-astra gpt-6.1-sol gpt-live-1 gpt-image-2.5-sunburst gpt-image-2.5-flare-2026-09-08]
Anthropic models: [claude-haiku-5-5 claude-sonnet-5-5 claude-opus-5-5 claude-fable-5-1 claude-opus-5 claude-sonnet-5 claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929]
Perplexity models: []
Gemini models: [gemini-2.5-flash gemini-2.5-pro gemini-2.5-flash-preview-tts gemini-2.5-pro-preview-tts gemma-4-26b-a4b-it gemma-4-31b-it gemini-flash-latest gemini-flash-lite-latest gemini-pro-latest gemini-2.5-flash-lite gemini-2.5-flash-image gemini-3-flash-preview gemini-3.1-pro-preview gemini-3.1-pro-preview-customtools gemini-3.1-flash-lite-preview gemini-3.1-flash-lite gemini-3-pro-image-preview gemini-3-pro-image nano-banana-pro-preview gemini-3.1-flash-image-preview gemini-3.1-flash-image gemini-3.1-flash-lite-image gemini-nano-banana-2.1 gemini-3.5-flash gemini-3.5-flash-lite gemini-omni-flash-preview gemini-omni-1.1-flash gemini-3.5-transcribe gemini-3.6-flash gemini-3.7-flash gemini-3.8-flash gemini-4-argon lyria-3-clip-preview lyria-3-pro-preview lyria-3.5 gemini-3.1-flash-tts-preview gemini-3.8-flash-tts gemini-3.8-flash-lite-tts gemini-robotics-er-2-preview gemini-2.5-computer-use-preview-10-2025 antigravity-preview-05-2026 antigravity-preview-09-2026 antigravity-preview-latest deep-research-max-preview-04-2026 deep-research-preview-04-2026 deep-research-pro-preview-12-2025 gemini-embedding-001 gemini-embedding-2-preview gemini-embedding-2 aqa veo-3.1-generate-preview veo-3.1-fast-generate-preview veo-3.1-lite-generate-preview gemini-3.5-transcribe-live gemini-2.5-flash-native-audio-latest gemini-2.5-flash-native-audio-preview-09-2025 gemini-2.5-flash-native-audio-preview-12-2025 gemini-3.1-flash-live-preview gemini-3.8-live gemini-3.8-live-extended-thinking gemini-robotics-er-2-streaming-preview gemini-3.5-live-translate-preview lyria-realtime-exp]
  anthropic: ok 14 models
  gemini: ok 63 models
  openai: ok 135 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get OpenAI Models ===
OpenAI models: [gpt-4-turbo-2024-04-09 gpt-realtime-2.1-mini gpt-realtime-2025-08-28 gpt-4 tts-1-hd gpt-realtime-1.5 gpt-realtime gpt-5.2-pro gpt-5-chat-latest gpt-3.5-turbo-0125 gpt-5.4-2026-03-05 gpt-5-nano gpt-5.4-mini o1-pro gpt-5.1-codex tts-1 gpt-5-mini gpt-5-mini-2025-08-07 omni-moderation-2024-09-26 gpt-live-transcribe gpt-5.6-luna omni-moderation-latest gpt-5-2025-08-07 o4-mini-2025-04-16 gpt-4o-2024-11-20 gpt-5.1 whisper-1 gpt-5-pro gpt-realtime-whisper gpt-5.5-pro gpt-audio-2025-08-28 gpt-4o-mini-transcribe-2025-03-20 gpt-realtime-2 gpt-5-search-api gpt-5.5-2026-04-23 chat-latest gpt-realtime-translate gpt-5.4-pro-2026-03-05 gpt-5.4-mini-2026-03-17 babbage-002 gpt-4o-2024-05-13 text-embedding-3-large gpt-4o-mini-transcribe-2025-12-15 tts-1-1106 gpt-5.5-pro-2026-04-23 gpt-audio-mini-2025-10-06 gpt-5.2-pro-2025-12-11 gpt-4-turbo gpt-4o-2024-08-06 o3 gpt-4o-mini-search-preview-2025-03-11 o3-mini-2025-01-31 gpt-audio-1.5 gpt-image-1.5 gpt-5-pro-2025-10-06 gpt-5.6-sol gpt-4.1-nano gpt-image-2 gpt-3.5-turbo-16k text-embedding-ada-002 gpt-realtime-2.1 gpt-4.1 gpt-5.5 gpt-4o-transcribe-diarize gpt-5-codex gpt-4-0613 gpt-5.1-2025-11-13 gpt-4o-search-preview gpt-4.1-mini gpt-4o-mini gpt-5.4-pro gpt-4o-mini-tts gpt-realtime-mini-2025-12-15 gpt-5.1-codex-max gpt-realtime-mini gpt-image-1-mini gpt-5.6-terra gpt-3.5-turbo-instruct-0914 o1-2024-12-17 sora-2 tts-1-hd-1106 gpt-4o-mini-tts-2025-12-15 gpt-image-1 gpt-audio gpt-5.3-codex gpt-5-search-api-2025-10-14 gpt-5 gpt-4o-mini-transcribe gpt-image-2-2026-04-21 gpt-5.2-chat-latest sora-2-pro gpt-3.5-turbo-instruct gpt-transcribe gpt-4.1-nano-2025-04-14 o4-mini-deep-research gpt-5.4 gpt-4o-search-preview-2025-03-11 text-embedding-3-small gpt-5.1-chat-latest gpt-4o-mini-2024-07-18 gpt-4.1-2025-04-14 gpt-5.3-chat-latest o3-2025-04-16 gpt-5.2-2025-12-11 chatgpt-image-latest gpt-3.5-turbo gpt-5.2-codex gpt-4o davinci-002 gpt-5.4-nano o3-mini o4-mini-deep-research-2025-06-26 gpt-audio-mini-2025-12-15 o1-pro-2025-03-19 gpt-5-nano-2025-08-07 gpt-4o-transcribe o4-mini gpt-5.1-codex-mini gpt-3.5-turbo-1106 gpt-4o-mini-tts-2025-03-20 gpt-5.2 gpt-5.4-nano-2026-03-17 o1 gpt-image-2.5-sunburst-2026-09-08 gpt-6-luna gpt-audio-mini gpt-4.1-mini-2025-04-14 gpt-image-2.5-flare gpt-4o-mini-search-preview gpt-6-sol gpt-6-astra gpt-6.1-sol gpt-live-1 gpt-image-2.5-sunburst gpt-image-2.5-flare-2026-09-08]

=== Get Anthropic Models ===
Anthropic models: [claude-haiku-5-5 claude-sonnet-5-5 claude-opus-5-5 claude-fable-5-1 claude-opus-5 claude-sonnet-5 claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929]

✓ Chat Models API example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: gfFwx5BKZpDNNEM4wpaHtC2eHJwyemk1ZZKbiIcjn6vc1-NPsXVxkY4VS34bQG3AYfeAgjweeC1GqeQvIIhKQA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

Would you like to know more about this product or anything else?

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: xYbgnulnHKMzTFzvDdKOXJfpUHFvLDlC2KEvPZWs4CXse1cmYfCAAqJL9zrDf_wR92zhtlVmXK8p6_fLkAfLCA
  Parent: gfFwx5BKZpDNNEM4wpaHtC2eHJwyemk1ZZKbiIcjn6vc1-NPsXVxkY4VS34bQG3AYfeAgjweeC1GqeQvIIhKQA

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: xYbgnulnHKMzTFzvDdKOXJfpUHFvLDlC2KEvPZWs4CXse1cmYfCAAqJL9zrDf_wR92zhtlVmXK8p6_fLkAfLCA (Untitled)
  Session 2: gfFwx5BKZpDNNEM4wpaHtC2eHJwyemk1ZZKbiIcjn6vc1-NPsXVxkY4VS34bQG3AYfeAgjweeC1GqeQvIIhKQA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: xYbgnulnHKMzTFzvDdKOXJfpUHFvLDlC2KEvPZWs4CXse1cmYfCAAqJL9zrDf_wR92zhtlVmXK8p6_fLkAfLCA

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: DI_Zv4mbBCUtvtD2nc3x4IyPF4wHLGpG6MJxfMLtjocDy8nt0mkgQNc2_M9z7VXp-ycx7G_DlFdZBIQOJRsLlA

=== List Collections ===
Total collections: 13
Sample collections: [chat_goals__ek0_testing client_collection_management_go chat_agent_configs__ek0_testing chat_tasks__ek0_testing chat_goal_templates__ek0_testing]

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
✓ Created record with native map: map[id:IeqUtlrXunmDuWQD0tydt_bQYsK9KweZensI-ZJjMwH1Dr4PMAY_oOgklzOftX6E0rh86eXStJgkjOFzT1xfGQ]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:IeqUtlrXunmDuWQD0tydt_bQYsK9KweZensI-ZJjMwH1Dr4PMAY_oOgklzOftX6E0rh86eXStJgkjOFzT1xfGQ name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:new-user-1792526642945018000]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:IeqUtlrXunmDuWQD0tydt_bQYsK9KweZensI-ZJjMwH1Dr4PMAY_oOgklzOftX6E0rh86eXStJgkjOFzT1xfGQ name:map[type:String value:Alice Johnson]]
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
✓ Inserted document: PuP3ktRH8WaebKZOxEvnr7ESYHWVm0E3GHuHQonL2aSVMAwU6vGIYmnfkKd9q1cW1To6wQSTdp6SNHaSXPO7Wg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: _NEZP7q6cYjWciUQIhGueMm1rNZL9loyT-_2x25KwdxVPj1rw7HBswRzq7bcxtuaoJ8qW0DefmZezDOc6tcYgQ

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
✓ Edge cache script created: OwfVvuzWGTyZW7B-_2qf6CbrosDsq0x0F9-loukj4nC6RHSK5h9Xd5nR0dF1goR6HEqAwJoYsUv_8pxfEl8uYg

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
   ⏱️  Duration: 55.586958ms
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
   ⏱️  Duration: 473.542µs
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

✅ Function saved: 0O-QGfqXkGzm1d9RGX4mnIyRRz37hzuHQVZVO-be6veu_1gQHjMdqY0iTUzmLvdHBme_Ubx6-ncMThG_nN-auQ
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

✅ Function saved: _nzhtRVpeMcshueH2ogi9p3jsMf26gjfHgCFeXTUAmX-x3yVJhSRAAznTaberjzY3-5SLvUPmkrWaHpw4ThgGg
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

✅ Inserted order: VIzLF6BfkdgnGYOgBbcrXZ_xQeTl6RgAlzQmenzitw-PaNu-Sg2yoaSJOoo5iVa5By90WKMtK9ekSlRrY8ppwg
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: ezl5D6WN8BtrOPwvG3eeugImTDacgljSDplZ1BSUS6JKOBJr2wXm7ev4AoFfg74Rm_Q0uiWe31SXW8y6tcU66g
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[type:Object value:map[role:admin userId:user_abc]]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Functions

✅ Function saved: mroCr0oss37yC8235K85YsWMKfqfpEb8wgd9q94JByzYhCGwoeDSmppjNIM3MvF0VjfON4BaJHYzt-MYPB-Y5g
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: 11vl_6Z5pQnkIh1xndRes5tBY_9hE7oy2fSy507ZxcFUPN0d-egW0GS8MJ0w2TXli12CICYmOkcXjXU_Repolg
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
   1. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   2. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   3. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   4. map[type:String value:Database Design Principles] (map[type:String value:Database])
   5. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
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
Created template: Data Migration (id: KhI4L5zfffg8iglF48ncXEfobeeaJIH8Bn-KroklV5jEygFFHtBYwU5sus6UGywRIn3B-ewIdPIqHmgXbQH79w)

--- Listing templates ---
Templates: map[count:1 items:[map[description:map[type:String value:Template for migrating data between schemas] id:KhI4L5zfffg8iglF48ncXEfobeeaJIH8Bn-KroklV5jEygFFHtBYwU5sus6UGywRIn3B-ewIdPIqHmgXbQH79w steps:map[type:Array value:[map[description:Analyze source schema] map[description:Create target schema] map[description:Migrate records] map[description:Validate results]]] title:map[type:String value:Data Migration]]]]

--- Getting template ---
Fetched: Data Migration

--- Updating template ---
Updated description: Updated: comprehensive data migration workflow

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
=== ekoDB Goals, Tasks & Agents Integration Example (Go) ===

--- Creating goal ---
Created goal: Deploy v2.0 (id: cSvqxMMmUp3QmBgA9ns8qLYU20Mzs6xwdK9DQ88c7vfv9Rix-rzpeI348MMvTKBDKvdqGjEiyvzxtvgHovd_uA)

--- Listing goals ---
Goals response: map[count:1 goals:[map[created_at:2026-10-20T20:04:14.276832+00:00 description:Ship the v2.0 release to production id:cSvqxMMmUp3QmBgA9ns8qLYU20Mzs6xwdK9DQ88c7vfv9Rix-rzpeI348MMvTKBDKvdqGjEiyvzxtvgHovd_uA status:pending steps:[{"description":"Run integration tests"},{"description":"Build release artifacts"},{"description":"Deploy to staging"}] title:Deploy v2.0 updated_at:2026-10-20T20:04:14.276832+00:00]]]

--- Getting goal ---
Fetched goal: Deploy v2.0

--- Updating goal ---
Updated description: Ship the v2.0 release to production (with hotfix)

--- Searching goals ---
Search result: map[count:1 items:[map[_score:12.870000000000001 created_at:map[type:DateTime value:2026-10-20T20:04:14.276832+00:00] description:map[type:String value:Ship the v2.0 release to production (with hotfix)] id:cSvqxMMmUp3QmBgA9ns8qLYU20Mzs6xwdK9DQ88c7vfv9Rix-rzpeI348MMvTKBDKvdqGjEiyvzxtvgHovd_uA status:map[type:String value:pending] steps:map[type:String value:[{"description":"Run integration tests"},{"description":"Build release artifacts"},{"description":"Deploy to staging"}]] title:map[type:String value:Deploy v2.0] updated_at:map[type:DateTime value:2026-10-20T20:04:14.281993+00:00]]]]

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
Created task: Run benchmarks (id: 6WN0_uElGHcInWnwvYKFBRzvfppSKJ_GFvhVCv9flwGRNJy1KH1_sOe6kbwlunX_FxsGY6CIsEX8nU96BBPF4w)

--- Listing tasks ---
Tasks: map[count:1 items:[map[description:map[type:String value:Execute YCSB benchmarks against staging] due_at:map[type:DateTime value:2026-10-21T20:04:14+00:00] id:6WN0_uElGHcInWnwvYKFBRzvfppSKJ_GFvhVCv9flwGRNJy1KH1_sOe6kbwlunX_FxsGY6CIsEX8nU96BBPF4w name:map[type:String value:Run benchmarks]]]]

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
Created agent: benchmark-runner (id: i6Z4Wk0siAzaL10gdRTtwVHUU8IkkDuQRZ0bby22OPPnvgvUzqpZZdW8jjJBMIFrAT0HpEAlcT3BmsVFKcgVIA)

--- Listing agents ---
Agents: map[count:1 items:[map[deployment_id:map[type:String value:deploy_prod_1] description:map[type:String value:Runs periodic benchmarks] id:i6Z4Wk0siAzaL10gdRTtwVHUU8IkkDuQRZ0bby22OPPnvgvUzqpZZdW8jjJBMIFrAT0HpEAlcT3BmsVFKcgVIA llm_model:map[type:String value:gpt-4.1] name:map[type:String value:benchmark-runner]]]]

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
  - Mouse ($25) by Alice Johnson
  - Laptop ($1200) by Alice Johnson

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
Inserted doc1: G5hT0iSa5SLTLqLVWAtYXZc80Rtq6EIKL5xJ6m1l5bWR88TmtwB8aUPrxlJpPJT8ZkI1ryCyLZpdB4a_q1mAzA
Inserted doc2: xZdmoFF37bCar5hP1h3a3vKEyM-hIjkfnFk6C_zfkGvx2SWBgOaq7_nDK9yiyL6TW_RxAwdJ4uykfsCGr4vtHg

--- Setting KV entry ---
KV entry set: team:engineering:go

--- Linking KV key to documents ---
Linked doc1: map[]
Linked doc2: map[]

--- Getting links ---
Links for team:engineering:go: [map[collection:kv_links_example_go created_at:2026-10-20T20:04:16.271072Z document_id:G5hT0iSa5SLTLqLVWAtYXZc80Rtq6EIKL5xJ6m1l5bWR88TmtwB8aUPrxlJpPJT8ZkI1ryCyLZpdB4a_q1mAzA field_path:<nil> last_accessed:2026-10-20T20:04:16.273366Z metadata:map[]] map[collection:kv_links_example_go created_at:2026-10-20T20:04:16.273028Z document_id:xZdmoFF37bCar5hP1h3a3vKEyM-hIjkfnFk6C_zfkGvx2SWBgOaq7_nDK9yiyL6TW_RxAwdJ4uykfsCGr4vtHg field_path:<nil> last_accessed:2026-10-20T20:04:16.273366Z metadata:map[]]]

--- Unlinking doc2 ---
Unlink result: map[]

--- Verifying links after unlink ---
Links after unlink: [map[collection:kv_links_example_go created_at:2026-10-20T20:04:16.271072Z document_id:G5hT0iSa5SLTLqLVWAtYXZc80Rtq6EIKL5xJ6m1l5bWR88TmtwB8aUPrxlJpPJT8ZkI1ryCyLZpdB4a_q1mAzA field_path:<nil> last_accessed:2026-10-20T20:04:16.277233Z metadata:map[]]]

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
    "name": "Mercury",
    "diameter_km": 4879
  },
  {
    "name": "Venus",
    "diameter_km": 12104
  },
  {
    "name": "Earth",
    "diameter_km": 12742
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===
=== ekoDB Schedules Integration Example (Go) ===

--- Creating schedule ---
Created schedule: nightly-backup (id: 8541d748-dabe-46f0-9c59-65eed77e181c)

--- Listing schedules ---
Schedules: map[count:1 schedules:[map[created_at:2026-10-20T20:04:22.354311Z cron_expression:0 0 0 * * * description:Runs a database backup every night at midnight enabled:true function_label:schedule_noop_go_62507 id:8541d748-dabe-46f0-9c59-65eed77e181c last_execution:<nil> name:nightly-backup next_execution:2026-10-21T00:00:00Z parameters:map[] stats:map[avg_execution_time_ms:0 failed_executions:0 last_error:<nil> successful_executions:0 total_executions:0] timezone:UTC updated_at:2026-10-20T20:04:22.354311Z]]]

--- Getting schedule ---
Fetched schedule: nightly-backup (cron: 0 0 0 * * *)

--- Updating schedule ---
Updated cron: 0 0 2 * * *

--- Triggering schedule ---
Trigger response: map[schedule_id:8541d748-dabe-46f0-9c59-65eed77e181c status:triggered]

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
  1. Score: 0.753
  2. Score: 0.748
  3. Score: 0.744

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.501
  2. Score: 0.898
  3. Score: 0.299

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Introduction to Machine Learning (category: ml)
  2. Deep Learning Fundamentals (category: ml)

Paired vector search: document=Rust Programming Answer, query=Rust Programming Question
Document search execution path: index
Query search execution path: index

✅ Search examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: map[id:h6WQeeEe4YMkpwSnYK6j6w8RfBW1LTdt8Ay9B9j8aLmSS6jt1AmUYaVcoKAj858dhsm188oWPPXJX2NjzaOvJQ]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-10-20T20:04:24+00:00] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:h6WQeeEe4YMkpwSnYK6j6w8RfBW1LTdt8Ay9B9j8aLmSS6jt1AmUYaVcoKAj858dhsm188oWPPXJX2NjzaOvJQ metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-10-20 20:04:24 +0000 +0000
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-10-20T20:04:24+00:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:h6WQeeEe4YMkpwSnYK6j6w8RfBW1LTdt8Ay9B9j8aLmSS6jt1AmUYaVcoKAj858dhsm188oWPPXJX2NjzaOvJQ metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-10-20T20:04:24+00:00] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:h6WQeeEe4YMkpwSnYK6j6w8RfBW1LTdt8Ay9B9j8aLmSS6jt1AmUYaVcoKAj858dhsm188oWPPXJX2NjzaOvJQ metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: gHvlw_AiIfyF0xCm7HWhJE5kYMMJtUQxHlPP3MMdKkLem6C2zDWAtYG6-AoOlUMzgd9UGOOY9g0dhNreKa5A2A

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
✓ Created native SWR script: github_user_native_go (42oAgLIT8I67g4qfOlKoXm2fVWnfo0BO0s9lUgfu_Lnv1uPDxEwOM2yqDjiM5_ILrk17JgePvWvCmDGsXVRyWQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 143ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 71.5x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_go (sqxUnWAA2c6EVmasiQsh6P_CHph62yuHynWyq2KHoQ25bkmUFkfBxMYBFbIUkUg5KhcZW6sPU2bkOd_lHM91DQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_go' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_go (mqC1BO66xRcHHDdMizFmvJMZbrSCCobPFV5Du1q80dMx7ifN81Aot6TzWdUubjkMPLPGiiByaxHPBxZq57SjWg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_go' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_go (ijh_fDT7kBVPcTz2Mk-W39pAAlxpf86okYgOD5gJazCXEsG5P7XyEdSM10mrbaqrYk1KQDI2PawD2Ncdw5HCyQ)
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
✓ Created SWR script: fetch_api_user_go (0fjqRf2kYkE66WRZO7yl3UXSR8kQie1zm3Tp4bassG98zM65S5qN_BHDf7StsHoGyDhbYLUm55yTmwGvrPaNSA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": {
        "type": "DateTime",
        "value": "2026-10-20T20:04:26+00:00"
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
Response time: 1ms (served from cache)
✓ Lightning fast cache hit

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🧹 Cleaning up...
✓ Cleanup complete

✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Q-SOQ4aGRYJxraQoxGt5lA0gsoO9n4rLalUvLzxdkQqVgq--eIOdklPaq41egs-6m2ASCjKpqotmDzSptKURIQ
Created Bob: $500 - ID: UE9-Plx3eJyd-NdXcY8wkY3OiJosEjA1MQy7tb2ETGuaPp3EBRbm-YIecxdu0pPOwcAmoL8ValCrHIGiLJNXHw

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 7a0aefa3-3706-4441-88c8-aa63849f63fc

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
New transaction: 58f58c03-59fa-4f39-8db5-51ca01ae2b5f
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: loFZYFwhu0iqZiS7Qb9UJ66eMKp05SQrS7Tf_CG1jZWwql16LSra1T16VpKk363BHUKoGBYWVnmtaxKlA5vOxA

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

Created chat session: b_s648_0bcl000GH7WKGae6RhpaosrBktdhcGuo4AlsBlje1634eeXjsoAdFJoOV62zlKumJGU4ydSh8z_B-Bg

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: z-Nxjrl4LjJCeC9JylgjF5AQv-iNfZhWZNNE9nRYMwvJWRCpQ8_O34m6EkY_EwEceV4GwR4xCk-uY_WLJz5Sdw
Execution time: 515ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example (Go) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_go' ===
✓ Subscribed (subscription_id: sub_44d3bcbd55ca40029830661395cc7bab)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: 4vLb-6bAzs_DUeyF7Rzc9XN5fY31hlKvfo6DilwYUKwAYRhaPFK4UVvHuf02MZoXEU9MEy4AvIkGPzgeaEY14Q

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_go
     Record IDs: [4vLb-6bAzs_DUeyF7Rzc9XN5fY31hlKvfo6DilwYUKwAYRhaPFK4UVvHuf02MZoXEU9MEy4AvIkGPzgeaEY14Q]
     Timestamp:  2026-10-20T20:04:29.109980+00:00

Inserting record 2...
✓ Inserted: UdMJyR2vsmQo2KfNjJK9p5DJpMjndYpQCNz-6j4VdTnWPa4bKJ_MysTo7HK_bgi-zJuoDjXu0qyV-TUc3hB0hA

  📡 Notification received:
     Event:      insert
     Record IDs: [UdMJyR2vsmQo2KfNjJK9p5DJpMjndYpQCNz-6j4VdTnWPa4bKJ_MysTo7HK_bgi-zJuoDjXu0qyV-TUc3hB0hA]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: G56_PaLDZ0Y5z2i1v_DkYi3A7MoS9pofNipdXStkD-FNZiVHigs8DydqTWoCt8hjrEgxUMC93LMlIl2BdTp11A

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
Inserted with ripple: map[id:LSoEr8-Fy7MJRWeYBaICeqcxmRcNt0U7WsyCrSYtdY-4rfyq3gRexEP_e8rIkNUo8R6UXRbFo-6zioixVaIkKA]
Inserted with bypass_ripple: map[id:ojC-7zji4aNeexR-IiA7rHl52CcIPfxD_vQZXyNV1hx36w6epc8Wd5_Db3NFwNq9a2BJreT-jWol_pvc8QeLEA]
Inserted with TTL and bypass_ripple: map[id:IR7ui28Bqw_xbeQ_NuYYZE0CmLEmz0wKMzITcwCed57R2rliLN-oSlUyxB0BaPiKgXlUI4SumJxkPd-JLbW0PA]
Updated with bypass_ripple: map[id:LSoEr8-Fy7MJRWeYBaICeqcxmRcNt0U7WsyCrSYtdY-4rfyq3gRexEP_e8rIkNUo8R6UXRbFo-6zioixVaIkKA name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:custom-id]
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: [email id name]
  First user: Alice Johnson <alice@example.com>

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

> @ekodb/ekodb-client@0.30.1 prepare
> npm run build


> @ekodb/ekodb-client@0.30.1 build
> tsc


up to date, audited 44 packages in 838ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.30.1 build
> tsc

✅ TypeScript client built!

added 1 package, removed 1 package, and audited 13 packages in 474ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
Admin permissions read successfully
=== ekoDB Advanced CRUD Example (TypeScript) ===

--- Inserting base record ---
Inserted: CuHLqz3sakZqp74DHOkndnFyScACJq_SJH_gKpVvd4iZiFz6oeR7oApto6iAFQnAzfZGM7dVnNIXES9Kkr8Xpg

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
✓ Created session: q7tV2d87MyEowLbaBgTW5yCm4xOyR3qJY5XhPF03vnt_uY9Xl5FHH7_sp6DoyyZ_BnI-o4GSsbKv3DkvfwgnLA

=== Sending Initial Message ===
✓ Message sent
  Response: It looks like there are currently no available products listed in the database. The only product context I have is the "High-performance database product" named "ekoDB" with an ID of ZOF0HOhlD2ZXllOEQq5FxdIfx92DwjhU-kp4DKQIS-QRTdUdq87joeYeuKXrGx9Vy4xe4t6ABKwrqPJPKzMkEQ, priced at $99.

If you would like more details or need to perform another action, please let me know!

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
✓ Created second session: pXlwy_3n7NHjsqyUQFlPOk9t2r-szPyGt4Agljkt-l4m26IFFxdzWrBRI6vbaMFixw23U1vcqZv38irlG_gsjw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: pXlwy_3n7NHjsqyUQFlPOk9t2r-szPyGt4Agljkt-l4m26IFFxdzWrBRI6vbaMFixw23U1vcqZv38irlG_gsjw
✓ Deleted chat session: q7tV2d87MyEowLbaBgTW5yCm4xOyR3qJY5XhPF03vnt_uY9Xl5FHH7_sp6DoyyZ_BnI-o4GSsbKv3DkvfwgnLA
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: VYt2UhwMm2tqFT_cqWCKpiGGEdDCgWnQXvV7BKKNZFNXOEm9DgbermPQq-e0inKQ7hUxSaUrj7_GOZN5T_T75A

=== Sending Chat Message ===
Message ID: De_j6nLki9caiBTx4BC_wq7PtkA6DmNjl-Q4-6nfsZefrQUS8750hljSqPK7thdde9pypzXAes00ZJ6O80VfSA

=== AI Response ===
Here are the products available along with their prices:

1. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features

2. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product

3. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 299,
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features',
    id: 'N4m7Umfa6LB3yM8aC_s3u5oQvrh99q-zQouGwUysF_UipT-lDFaiVi3fvtzYUO2VgZOx162yf1Px-EOETIwiYw'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'gjGsoHsUc9531yqTBfklqg_EBk6xZTmI-rz17sKMqvVJ74kouCE5zWxziF5vcIJ86bD-aET9TFLLE3B6E8T7zg',
    price: 499,
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    id: 'oFTUoNTAdK1W_YfcdPwpKSJRa8s7orTaoP1GJZJS5n9IsyR_bHwKNDx_1qhw0Ra1107elPIObAlfJdxrMUWqow',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2980ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 91
Total tokens: 3504

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (TypeScript) ===

Created session: 4lT_7FnzchMBOPc6uf0jqjC1qanH3RzHjuCHXNxq3YMB5FF9WFoBbXaqwrWtzdNOoTPL33nNFmBjktT7tAcQkQ

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}**ekoDB** can refer to a few different things, but the most common contexts are:

1. **ekoDB (Ecological Databases):**
   - This is the broader and more common use. Here, "ekoDB" is shorthand for **ecological database(s)**—collections of data related to environmental science, species, habitats, or ecological phenomena. These databases are used by researchers for biodiversity studies, conservation planning, and environmental monitoring.

2. **ekoDB (Specific Projects):**
   - In some academic or open-source projects, "ekoDB" is used as the proper name for a particular ecological database or data management system. For example:
     - There may be a database or web portal called "ekoDB" devoted to storing and sharing ecological, zoological, or biodiversity data within a certain research community or geographic region.

3. **ekoDB (Bioinformatics/Genomics):**
   - In genetics and bioinformatics, ekoDB is sometimes mentioned as a database or platform for omics data, like gene expression or protein studies, especially involving model organisms.

**Do you have a specific context or region in mind?** (For example, a particular scientific field, a web resource, or a type of data?) If so, please provide more details so I can give you the most accurate answer.

--- Stream complete ---
Message ID: nvJE2JaazmSa0b8K1Gx5wOD8WnEGCJA2c2lWaksMgHvdGFteDKFWTopkrGjD0dI4DIph5FmVs0M1Wmx3jRy1_Q
Execution time: 2629ms
Context window: 1000000 tokens

✓ Chat message stream example completed
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
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
  gemini:
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

=== Get Specific Provider Models ===
OpenAI models: gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, gpt-4, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-5-chat-latest, gpt-3.5-turbo-0125, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.4-mini, o1-pro, gpt-5.1-codex, tts-1, gpt-5-mini, gpt-5-mini-2025-08-07, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, omni-moderation-latest, gpt-5-2025-08-07, o4-mini-2025-04-16, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-5.4-mini-2026-03-17, babbage-002, gpt-4o-2024-05-13, text-embedding-3-large, gpt-4o-mini-transcribe-2025-12-15, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-audio-mini-2025-10-06, gpt-5.2-pro-2025-12-11, gpt-4-turbo, gpt-4o-2024-08-06, o3, gpt-4o-mini-search-preview-2025-03-11, o3-mini-2025-01-31, gpt-audio-1.5, gpt-image-1.5, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-4.1-nano, gpt-image-2, gpt-3.5-turbo-16k, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-4-0613, gpt-5.1-2025-11-13, gpt-4o-search-preview, gpt-4.1-mini, gpt-4o-mini, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-5.1-codex-max, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-3.5-turbo-instruct-0914, o1-2024-12-17, sora-2, tts-1-hd-1106, gpt-4o-mini-tts-2025-12-15, gpt-image-1, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-5.2-chat-latest, sora-2-pro, gpt-3.5-turbo-instruct, gpt-transcribe, gpt-4.1-nano-2025-04-14, o4-mini-deep-research, gpt-5.4, gpt-4o-search-preview-2025-03-11, text-embedding-3-small, gpt-5.1-chat-latest, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-5.3-chat-latest, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, gpt-3.5-turbo, gpt-5.2-codex, gpt-4o, davinci-002, gpt-5.4-nano, o3-mini, o4-mini-deep-research-2025-06-26, gpt-audio-mini-2025-12-15, o1-pro-2025-03-19, gpt-5-nano-2025-08-07, gpt-4o-transcribe, o4-mini, gpt-5.1-codex-mini, gpt-3.5-turbo-1106, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, o1, gpt-image-2.5-sunburst-2026-09-08, gpt-6-luna, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-image-2.5-flare, gpt-4o-mini-search-preview, gpt-6-sol, gpt-6-astra, gpt-6.1-sol, gpt-live-1, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: G8fukzJuvGyVk-OheQ6_zEjXmZUy7qTCWx-WXGTOclN9WG97E9Sa6p5vyGG8t-jjQZY36O5MiU6PW1l3FQbfrA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need information about more products or anything else, let me know!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 1tbAC-SLuAC-XVHXwss8ucSPkiMKDk0qnNPUuvzNUdKFe11_vsMpB0TFL5Y0n3xCKQA6QqQTLpUN0Y4A-80H5A
  Parent: G8fukzJuvGyVk-OheQ6_zEjXmZUy7qTCWx-WXGTOclN9WG97E9Sa6p5vyGG8t-jjQZY36O5MiU6PW1l3FQbfrA

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: 1tbAC-SLuAC-XVHXwss8ucSPkiMKDk0qnNPUuvzNUdKFe11_vsMpB0TFL5Y0n3xCKQA6QqQTLpUN0Y4A-80H5A (Untitled)
  Session 2: G8fukzJuvGyVk-OheQ6_zEjXmZUy7qTCWx-WXGTOclN9WG97E9Sa6p5vyGG8t-jjQZY36O5MiU6PW1l3FQbfrA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 1tbAC-SLuAC-XVHXwss8ucSPkiMKDk0qnNPUuvzNUdKFe11_vsMpB0TFL5Y0n3xCKQA6QqQTLpUN0Y4A-80H5A

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: f8_rOjtoCUX_85JvsFgcdwMNBjm2G5U8q2YqdscAGXMfEtklkkfsLkVazbUl-CiLmSaU95YNGmrdEYEbbiWBmg

=== List Collections ===
Total collections: 17
Sample collections: chat_goals__ek0_testing,chat_agent_configs__ek0_testing,chat_tasks__ek0_testing,schema_employees_client_go,schema_users_client_go

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
  POST /api/functions/conc_demo_pay_ts_12240_1792526703280 { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail_ts_12240_1792526703280 { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip_ts_12240_1792526703280 { "user_id": 42 }
  POST /api/functions/conc_demo_lock_ts_12240_1792526703280 { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'ya-cDEFl57qAIZxMB9s_eSMNmo0GYUu_KnU3iYteWQoJXz-JXrRziQa0Nz-4iScBYFRpCRq0wo1jLsLIxceEhA'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: 'ya-cDEFl57qAIZxMB9s_eSMNmo0GYUu_KnU3iYteWQoJXz-JXrRziQa0Nz-4iScBYFRpCRq0wo1jLsLIxceEhA',
  age: { type: 'Integer', value: 29 },
  name: { value: 'Alice Johnson', type: 'String' },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  active: { type: 'Boolean', value: true }
}
✓ Second upsert (insert): { id: 'new-user-id' }

=== Find One Operation ===
✓ Found user by email: {
  id: 'ya-cDEFl57qAIZxMB9s_eSMNmo0GYUu_KnU3iYteWQoJXz-JXrRziQa0Nz-4iScBYFRpCRq0wo1jLsLIxceEhA',
  age: { type: 'Integer', value: 29 },
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Alice Johnson' },
  email: { type: 'String', value: 'alice.j@newdomain.com' }
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
✓ Inserted document: OMZQDI7BqCnbXeRd48A95FKJ0AHZd3tBWepZJ6SuSHgPHufib4CTKd5VhDc2JUwkDlVAX7-yDCiycr4deboxqw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 4xSDj3W6_Kbl8vdZsHp6HCxW1_BbdWLm3pO3sICl966rJIsVaz1ujOoVhYkqSKNK2ZuRpxWr7Cp2Bd5GkRGmSg

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
✓ Edge cache script created: pBy_KOUqwTTI5i22zCR3sqfNB8BMvxP0uMEzD8f2VPyD5689GvOD8pURh7r7fHmCHyqnJYUkgN7IARcCKwP2Ow

Call 1: Cache miss (fetches from API)
Response time: 66ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "username": "Bret",
          "name": "Leanne Graham",
          "email": "Sincere@april.biz",
          "id": 1,
          "website": "hildegard.org",
          "phone": "1-770-736-8031 x56442",
          "address": {
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "city": "Gwenborough",
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

Call 2: Cache hit (served from ekoDB)
Response time: 1ms (66x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "username": "Bret",
          "name": "Leanne Graham",
          "email": "Sincere@april.biz",
          "id": 1,
          "website": "hildegard.org",
          "phone": "1-770-736-8031 x56442",
          "address": {
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "city": "Gwenborough",
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
   ⏱️  Duration: 55ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "phone": "1-770-736-8031 x56442",
      "email": "Sincere@april.biz",
      "username": "Bret",
      "company": {
        "name": "Romaguera-Crona",
        "catch...

Second call (cache hit - from cache):
   ⏱️  Duration: 1ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "phone": "1-770-736-8031 x56442",
      "email": "Sincere@april.biz",
      "username": "Bret",
      "company": {
        "name": "Romaguera-...
   🚀 Cache speedup: 55.0x faster!

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

✅ Function saved: p1LYynbxigwk2I1vwoN0brvKfK-GQF2PRxZybVRdQsQYCmpjaD_TACzAT3Zeil3hIeGIXkzHj3r77phoUu5boQ
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: iL2pjppkc_sSphS22J3--BJ07yLNmrYXkIMV_pCOa-7ZtiPTDqaVjJJOyjlu5jxnHhXJ8NxYH7VSNtEOP9cNfg
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: mdJP0Lj6xPFaIKuPxB2p34Zr7brkM-4shP6EfSt6wkXhc29Z8HbO9nQdANRcEFXnZRR6VXb9-WXtn7aExjWCTA
📊 Statistics: 2 groups
   {"status":{"value":"inactive","type":"String"},"avg_score":{"value":50,"type":"Float"},"count":{"type":"Integer","value":5}}
   {"count":{"type":"Integer","value":5},"status":{"type":"String","value":"active"},"avg_score":{"type":"Float","value":60}}

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
   {"avg_price":{"type":"Float","value":365.6666666666667},"category":{"type":"String","value":"Furniture"},"count":{"type":"Integer","value":3}}
   {"count":{"value":5,"type":"Integer"},"avg_price":{"value":367,"type":"Float"},"category":{"type":"String","value":"Electronics"}}
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
   Vector databases offer several benefits:

1. **Efficient Similarity Searches**: They excel at high-dimensional similarity searches, enabling fast retrieval of similar items based on vector representations.

2. **Scalability**: Designed to handle large volumes of vector data, scaling efficiently as data size increases.

3. **Real-Time Performance**: Support for real-time querying, making them suitable for applications like recommendation systems and personalization.

4. **Multimodal Data Handling**: Capable of processing various data types (text, images, audio) by converting them into vector formats.

5. **Advanced Machine Learning Integration**: Seamless integration with machine learning models to leverage embeddings for enhanced data analysis.

6. **Dimensionality Reduction**: Techniques like PCA or t-SNE can be easily applied to manage high-dimensional data.

7. **Built-In Indexing**: Specialized indexing methods (e.g., HNSW, Annoy) for efficient nearest-neighbor searches.

8. **Data Analysis and Insights**: Facilitates complex analytics and can uncover patterns through clustering and classifications based on vector similarities.

These features make vector databases especially advantageous for AI-driven applications and data science projects.
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

✅ Function saved: nK-XJgiDod29kH3KD7oCuOAFI6XwO_JFavYf4qkWc2GETY_3yJCn1mfXZPdiyza6CjcyY7QdtbWSNkeRUu7tIQ
📊 Found 2 product groups
   {"avg_price":{"type":"Float","value":575.6666666666666},"count":{"type":"Integer","value":3},"category":{"type":"String","value":"Electronics"}}
   {"avg_price":{"type":"Float","value":474},"count":{"value":2,"type":"Integer"},"category":{"type":"String","value":"Furniture"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":{"value":2,"type":"Integer"},"category":{"value":"Furniture","type":"String"}}
   {"count":{"type":"Integer","value":3},"category":{"value":"Electronics","type":"String"}}
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
   active: 7 users
   inactive: 3 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average score by role:
   {"avg_score":{"type":"Float","value":70},"count":{"value":7,"type":"Integer"},"role":{"type":"String","value":"user"}}
   {"role":{"value":"admin","type":"String"},"avg_score":{"type":"Float","value":20},"count":{"value":3,"type":"Integer"}}
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

✅ Inserted order: PgkU4Q5ggcXpSsTzSmYfQNCx8P9g3ZvmuTEMXvd6ZxJ8a78x81KwjmZIyNvPejOU2G8GeJG1yWg4eKORiwLfaQ
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: VGh8CGVcC2br8xsczjckQwALtLDA9Ggzx4065wzY0JfCZqFdsIumS1K9H8zzJjc8q9YRmSP_kn403nvKL6ISkA
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"role":"admin","userId":"user_abc"}}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: YyC__lOsXYkfDiR_bKsTCNXVbogmFg2z4WmILb48Cg8UyPeqrSkrsIMeh0AWHRY2ZHyDLrM1koXLZI6NwyXysw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: IEcjs6zFNiaF5dz4_pk_OJDMUoFw2AMfUBop8ce8LlOgE18P1aazrPGiae0JMEuPEnkQD2jhQEOCOEnzBhWsQw
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
   1. Introduction to Machine Learning (AI)
   2. Getting Started with ekoDB (Database)
   3. Natural Language Processing (AI)
   4. Database Design Principles (Database)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"type":"String","value":"Database"},"count":{"value":3,"type":"Integer"}}
   {"category":{"value":"AI","type":"String"},"count":{"value":2,"type":"Integer"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Goal Template CRUD Example (TypeScript) ===

--- Creating goal template ---
Created template: Data Migration (id: 7j4fUJOogQZVXROb0TsLhWLEPT1qk6mDsRarGoJ3bHb420OKycbR7ztCjV1l__ovA3QNFf7Zmj0A-Ut-V8oAUw)

--- Listing templates ---
Templates: {
  count: 1,
  items: [
    {
      description: [Object],
      id: '7j4fUJOogQZVXROb0TsLhWLEPT1qk6mDsRarGoJ3bHb420OKycbR7ztCjV1l__ovA3QNFf7Zmj0A-Ut-V8oAUw',
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
Created goal: Deploy v2.0 (id: JZfz8m68k-8Bx4qx5t7KMHoIWuIOPlOxwq4y-JvtZhucJFZ5oDPOLnKQvI1Kk1lv3GRecXYZJkDju70ufF2Faw)

--- Listing goals ---
Goals: {
  "count": 1,
  "goals": [
    {
      "created_at": "2026-10-20T20:05:14.526432+00:00",
      "description": "Ship version 2.0 to production",
      "id": "JZfz8m68k-8Bx4qx5t7KMHoIWuIOPlOxwq4y-JvtZhucJFZ5oDPOLnKQvI1Kk1lv3GRecXYZJkDju70ufF2Faw",
      "status": "pending",
      "steps": "[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]",
      "title": "Deploy v2.0",
      "updated_at": "2026-10-20T20:05:14.526432+00:00"
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
        "value": "2026-10-20T20:05:14.526432+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 with hot-fix patches"
      },
      "id": "JZfz8m68k-8Bx4qx5t7KMHoIWuIOPlOxwq4y-JvtZhucJFZ5oDPOLnKQvI1Kk1lv3GRecXYZJkDju70ufF2Faw",
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
        "value": "2026-10-20T20:05:14.536369+00:00"
      }
    }
  ]
}

--- Goal step: start step 0 ---
Step 0 started on goal JZfz8m68k-8Bx4qx5t7KMHoIWuIOPlOxwq4y-JvtZhucJFZ5oDPOLnKQvI1Kk1lv3GRecXYZJkDju70ufF2Faw
--- Goal step: complete step 0 ---
Step 0 completed on goal JZfz8m68k-8Bx4qx5t7KMHoIWuIOPlOxwq4y-JvtZhucJFZ5oDPOLnKQvI1Kk1lv3GRecXYZJkDju70ufF2Faw
--- Goal step: fail step 1 ---
Step 1 failed on goal JZfz8m68k-8Bx4qx5t7KMHoIWuIOPlOxwq4y-JvtZhucJFZ5oDPOLnKQvI1Kk1lv3GRecXYZJkDju70ufF2Faw

--- Completing goal ---
Goal status: pending_review
--- Approving goal ---
Goal status after approve: in_progress

--- Creating goal to reject ---
--- Rejecting goal ---
Goal status after reject: failed


--- Creating task ---
Created task: Hourly Health Check (id: kZAbOEigj9W7ny6DuGa2suFqg9t5NZjZ2ROdU0bkdMwSWpHgDDNiOVR5XmnTUvw-jmsIt-cylw4wN_dyKKeOjA)

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
      "id": "kZAbOEigj9W7ny6DuGa2suFqg9t5NZjZ2ROdU0bkdMwSWpHgDDNiOVR5XmnTUvw-jmsIt-cylw4wN_dyKKeOjA",
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
Created agent: SupportBot (id: fxZlidia5jCZ_bg5Pc5PNjEGH3vCV9A0791-pF4AClVyO9lNGfevHlYfufctdGoogBOaimwHHamZ6_Oda7L_hQ)

--- Listing agents ---
Agents: {
  "count": 1,
  "items": [
    {
      "deployment_id": {
        "type": "String",
        "value": "deploy_prod_1"
      },
      "id": "fxZlidia5jCZ_bg5Pc5PNjEGH3vCV9A0791-pF4AClVyO9lNGfevHlYfufctdGoogBOaimwHHamZ6_Oda7L_hQ",
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
By name: SupportBot (id: fxZlidia5jCZ_bg5Pc5PNjEGH3vCV9A0791-pF4AClVyO9lNGfevHlYfufctdGoogBOaimwHHamZ6_Oda7L_hQ)

--- Updating agent ---
Updated agent: SupportBot

--- Getting agents by deployment ---
Agents in deployment: {
  "count": 0,
  "items": []
}
WARNING: agents-by-deployment omitted created agent fxZlidia5jCZ_bg5Pc5PNjEGH3vCV9A0791-pF4AClVyO9lNGfevHlYfufctdGoogBOaimwHHamZ6_Oda7L_hQ; TODO: check/fix the server-side deployment lookup

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


✅ Join operations examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created
✓ ts_users_register saved
✓ ts_users_login saved
✓ ts_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/jwt_register_ts_27477_1792526715644 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_login_ts_27477_1792526715644 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_verify_ts_27477_1792526715644 { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

✓ Cleaned up demo functions
=== ekoDB KV Document Linking Example (TypeScript) ===

--- Setup: creating KV key and documents ---
Set KV key: session:admin
Inserted document 1: w-p6nky54MLX7LPjzECR566xlbkv3rt_SKFS3nYD0YExS_mDqKLk7QKbpQdtvwlNRwlKz7iTMZ4DMVqIN6cIpg
Inserted document 2: E8ZCRxcula1VT0kPL65443zsBc-hEvSLqwC0890Ki8rZxeNCpc413x0_Yn-0cmBcg-bsrlrTwaX79hzOpNTm8A

--- Linking documents to KV key ---
Linked doc w-p6nky54MLX7LPjzECR566xlbkv3rt_SKFS3nYD0YExS_mDqKLk7QKbpQdtvwlNRwlKz7iTMZ4DMVqIN6cIpg: null
Linked doc E8ZCRxcula1VT0kPL65443zsBc-hEvSLqwC0890Ki8rZxeNCpc413x0_Yn-0cmBcg-bsrlrTwaX79hzOpNTm8A: null

--- Getting links for KV key ---
Links: [
  {
    "collection": "kv_links_example_ts_28130_1792526716109",
    "document_id": "w-p6nky54MLX7LPjzECR566xlbkv3rt_SKFS3nYD0YExS_mDqKLk7QKbpQdtvwlNRwlKz7iTMZ4DMVqIN6cIpg",
    "field_path": null,
    "created_at": "2026-10-20T20:05:16.194817Z",
    "last_accessed": "2026-10-20T20:05:16.199492Z",
    "metadata": {}
  },
  {
    "collection": "kv_links_example_ts_28130_1792526716109",
    "document_id": "E8ZCRxcula1VT0kPL65443zsBc-hEvSLqwC0890Ki8rZxeNCpc413x0_Yn-0cmBcg-bsrlrTwaX79hzOpNTm8A",
    "field_path": null,
    "created_at": "2026-10-20T20:05:16.197684Z",
    "last_accessed": "2026-10-20T20:05:16.199492Z",
    "metadata": {}
  }
]

--- Unlinking document ---
Unlinked doc E8ZCRxcula1VT0kPL65443zsBc-hEvSLqwC0890Ki8rZxeNCpc413x0_Yn-0cmBcg-bsrlrTwaX79hzOpNTm8A: null

--- Verifying remaining links ---
Remaining links: [
  {
    "collection": "kv_links_example_ts_28130_1792526716109",
    "document_id": "w-p6nky54MLX7LPjzECR566xlbkv3rt_SKFS3nYD0YExS_mDqKLk7QKbpQdtvwlNRwlKz7iTMZ4DMVqIN6cIpg",
    "field_path": null,
    "created_at": "2026-10-20T20:05:16.194817Z",
    "last_accessed": "2026-10-20T20:05:16.204739Z",
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
Retrieved value: { type: 'Object', value: { username: 'john_doe', userId: 123 } }

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_ops_ts_28771_1792526716637:cache:product:1: success
  kv_ops_ts_28771_1792526716637:cache:product:2: success
  kv_ops_ts_28771_1792526716637:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_ts_28771_1792526716637:cache:product:1: { name: 'Product 1', price: 29.99 }
  kv_ops_ts_28771_1792526716637:cache:product:2: { name: 'Product 2', price: 39.99 }
  kv_ops_ts_28771_1792526716637:cache:product:3: { name: 'Product 3', price: 49.99 }

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
  kv_ops_ts_28771_1792526716637:cache:product:1: deleted
  kv_ops_ts_28771_1792526716637:cache:product:2: deleted
  kv_ops_ts_28771_1792526716637:cache:product:3: deleted

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
Created schedule: Nightly Database Backup (id: d572e6fd-bc28-4b5e-9f43-f43f10a68ada, cron: 0 0 2 * * *)

--- Listing schedules ---
Schedules: {
  "count": 1,
  "schedules": [
    {
      "created_at": "2026-10-20T20:05:20.925150Z",
      "cron_expression": "0 0 2 * * *",
      "description": null,
      "enabled": true,
      "function_label": "schedule_noop_typescript_33914_1792526720895",
      "id": "d572e6fd-bc28-4b5e-9f43-f43f10a68ada",
      "last_execution": null,
      "name": "Nightly Database Backup",
      "next_execution": "2026-10-21T02:00:00Z",
      "parameters": {},
      "stats": {
        "avg_execution_time_ms": 0,
        "failed_executions": 0,
        "last_error": null,
        "successful_executions": 0,
        "total_executions": 0
      },
      "timezone": "UTC",
      "updated_at": "2026-10-20T20:05:20.925150Z"
    }
  ]
}

--- Getting schedule ---
Fetched: Nightly Database Backup (cron: 0 0 2 * * *)

--- Updating schedule ---
Updated: Nightly Full Backup (new cron: 0 0 3 * * *)

--- Triggering schedule ---
Trigger response: {
  "schedule_id": "d572e6fd-bc28-4b5e-9f43-f43f10a68ada",
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
  1. Score: 13.200, Matched: bio, title
  2. Score: 13.200, Matched: bio, title
  3. Score: 13.200, Matched: bio, title
  4. Score: 13.200, Matched: title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: title, bio
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
  1. Score: 0.765, Matched:
  2. Score: 0.747, Matched:
  3. Score: 0.735, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.494, Matched: content, title
  2. Score: 0.899, Matched: content, title
  3. Score: 0.306, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: skills, bio

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Deep Learning Fundamentals (category: ml)
  2. Introduction to Machine Learning (category: ml)

Paired vector search: document=Rust Programming Answer, query=Rust Programming Question

✅ Search examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'mTcEmNiwyazoLzlYQlJET1o96BlvEA-sUK7pn-q9PKZaV8zBWk4Fd_AUi5xv79RKl500sz2gAPPg7IJvEHlk8Q'
}

=== Find by ID ===
Found: {
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  value: { type: 'Integer', value: 42 },
  price: { value: 99.99, type: 'Float' },
  name: { type: 'String', value: 'Test Record' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  active: { value: true, type: 'Boolean' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  created_at: { type: 'DateTime', value: '2026-10-20T20:05:22.353+00:00' },
  id: 'mTcEmNiwyazoLzlYQlJET1o96BlvEA-sUK7pn-q9PKZaV8zBWk4Fd_AUi5xv79RKl500sz2gAPPg7IJvEHlk8Q'
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-10-20T20:05:22.353Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  categories: [ 'electronics', 'computers' ],
  tags: [ 'tag1', 'tag2', 'tag3' ],
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  value: 42,
  price: 99.99,
  name: 'Test Record',
  metadata: { key: 'value', nested: { deep: true } },
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  active: true,
  data: 'aGVsbG8gd29ybGQ=',
  created_at: '2026-10-20T20:05:22.353+00:00',
  id: 'mTcEmNiwyazoLzlYQlJET1o96BlvEA-sUK7pn-q9PKZaV8zBWk4Fd_AUi5xv79RKl500sz2gAPPg7IJvEHlk8Q'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  value: { type: 'Integer', value: 100 },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  price: { value: 99.99, type: 'Float' },
  name: { type: 'String', value: 'Updated Record' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  created_at: { value: '2026-10-20T20:05:22.353+00:00', type: 'DateTime' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  active: { value: true, type: 'Boolean' },
  id: 'mTcEmNiwyazoLzlYQlJET1o96BlvEA-sUK7pn-q9PKZaV8zBWk4Fd_AUi5xv79RKl500sz2gAPPg7IJvEHlk8Q'
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: a5fpQ5T9ZN8aJ4T65DKD3ZooqoTw3ArNcpX6vSBlgZjner5IQOO8dvujBuZyFBvMOnKCPgtxeyRmxUERXTpn1g

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
✓ Created native SWR script: github_user_native_ts (fiyiDKmnZVylD461jBPJ8P-1ZyHlb9K4K9mCrS8OHdlnTt43A4ok7Y4hm2ZvYS4iwEnKIQ_hgOZo_KOYhNMK3Q)

First call (cache miss - will fetch from GitHub API):
  Response time: 299ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 8ms
  Speedup: 37.4x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_ts (t0_PVwB_sKINvgPBo4cM_9pri6eWcG8_jj56_tCXsBGzZ9ANvvb7sxGbO6RQ3ctzRShXc5CwdoaTvY3uCaK6eA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_ts' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_ts (HIZDiqda1iZJ02IOEgP771NR8wLUy_g4kL75P6rKmJH0a7rrwQcukBUhfrn8XEQ5FeAH1Wx2Yz0YuWsq3P3mWQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_ts' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_ts (VV4-ZFTXZAGGEKSpFsbLJhm-oUzFHpkcIqx-R4PY-pQ5mD5i0BPQGnFcHhvDHZsoAQNrfHbKIhpzE6kIFygFIg)
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
✓ Created SWR script: fetch_api_user_ts_38404_1792526724530 (kVEqjuDd7VdnBqwJg2ZGoA0J-teQrLcooxUZUSXdxWyvFySYo5YmEyy69yGgeiYdBlJjRdD8DgQ99U2HXi0H7g)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "phone": "1-770-736-8031 x56442",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "id": 1,
          "email": "Sincere@april.biz",
          "website": "hildegard.org",
          "name": "Leanne Graham",
          "username": "Bret",
          "address": {
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "city": "Gwenborough",
            "zipcode": "92998-3874",
            "suite": "Apt. 556"
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
Response time: 1ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "value": {
          "phone": "1-770-736-8031 x56442",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "id": 1,
          "email": "Sincere@april.biz",
          "website": "hildegard.org",
          "name": "Leanne Graham",
          "username": "Bret",
          "address": {
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "city": "Gwenborough",
            "zipcode": "92998-3874",
            "suite": "Apt. 556"
          }
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
✓ Created enrichment script: fetch_product_reviews_ts_38404_1792526724530 (Y_KAfXG_WAldViy4NpOMS67xvuVVIXzXa6eRB-drCwtpgVZkHo8zGJkF0z4km5COImEMCDZlPn_9MTV4oQXrFw)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "value": {
          "sku": "BEA-ESS-ESS-001",
          "title": "Essence Mascara Lash Princess",
          "weight": 4,
          "minimumOrderQuantity": 48,
          "discountPercentage": 10.48,
          "availabilityStatus": "In Stock",
          "meta": {
            "createdAt": "2025-10-09T14:47:01.588Z",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "barcode": "5784719087687",
            "updatedAt": "2026-05-23T11:27:41.868Z"
          },
          "rating": 2.56,
          "reviews": [
            {
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "reviewerName": "Eleanor Collins",
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Would not recommend!",
              "rating": 3
            },
            {
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "comment": "Very satisfied!",
              "reviewerName": "Lucas Gordon",
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 4
            },
            {
              "comment": "Highly impressed!",
              "rating": 5,
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Eleanor Collins"
            }
          ],
          "warrantyInformation": "1 week warranty",
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "category": "beauty",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "tags": [
            "beauty",
            "mascara"
          ],
          "price": 9.99,
          "returnPolicy": "No return policy",
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "id": 1,
          "shippingInformation": "Ships in 3-5 business days",
          "dimensions": {
            "width": 15.14,
            "depth": 22.99,
            "height": 13.08
          },
          "brand": "Essence",
          "stock": 99
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
Created Alice: $1000 - ID: lIzPCAYvvSOARVuKe9U-_2VAa3u5RhdDDmTOUp2YV74zZcjqWlOtWj_hxgT_UTuU8FvzMgDfZuVp7Hj26V-KkQ
Created Bob: $500 - ID: Qy_F97-5pLSWilJPzKYOcWYeovr2fFBrUp9XNKW0iBJVNbwQ82hVC6t8xrjx4L1MWM0LTMt_YyJ5I2MzyzVXdw

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 39177b08-c39a-4c28-8453-7874830c9ee5

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
New transaction: 9ff11377-ab76-485b-81db-a39da6354fe6
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: l6mm4a6QAdL8dEIoII7qrX0u_xab8mYiniT21f9DHoB7kP3VP4MT8IXmBK4YIxRI9VEpP3ID-VgdzJh6tw1GxA

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 5 user functions:
  - conc_demo_rl_skip_ts_12240_1792526703280: Rate-limit (skip mode)
  - conc_demo_rl_fail_ts_12240_1792526703280: Rate-limit (fail mode)
  - fetch_product_reviews_ts_38404_1792526724530: Fetch Product with Reviews (Multi-API)
  - get_active_users_ts: Get Active Users
  - get_active_users_client_ts_updated: Get Active Users (Updated)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (TypeScript) ===

Created chat session: FbM3o0MAL3UUPt1faS5KgZC4hNb6aQ90kK_j5dqMn5GTr44d_kPu4Nb1hvS-pn87hyAl7QeyQIwAtXeKHmz5Kw

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: T99qeabXYwU6p7-nLuJa-ZcyqsgnlfmLj710_xfGvDtAx5BzkaCJOdHk6wfLpRW9BlkdXUdRQ_6W9EBvXpL9Nw
Execution time: 811ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_e416ef6fc3a1458586c1ddb969a73bcd)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: zov7PEHGPoGNoJz3K0k0v-lvm38dC0W3nvs7e3L2gZEGk40wHesR74KU9qbtTR7ie6sBmTsZiKftg72bGKsJPw
  📡 Notification received for zov7PEHGPoGNoJz3K0k0v-lvm38dC0W3nvs7e3L2gZEGk40wHesR74KU9qbtTR7ie6sBmTsZiKftg72bGKsJPw

Inserting another record...
✓ Inserted record: 3TjUDoHG9dVuerDn_4_26KfHYRm2MUXPlfT8Y7RXZnz3TUrCwDtCnXeTyL36eqUDcr-Rg8TSDtdjhgitjygi2Q
  📡 Notification received for 3TjUDoHG9dVuerDn_4_26KfHYRm2MUXPlfT8Y7RXZnz3TUrCwDtCnXeTyL36eqUDcr-Rg8TSDtdjhgitjygi2Q

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_ts'
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: IQIoEVmyrNNCbnvjacWViUJYpt2tBcBj3sJ4Wx2UIATbOxVE5cjKspTzbkS-EPk_BhBG--FmmhEnJsGwdM4xWQ

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
   Inserted with ripple: {"id":"khR4VQ2Ju1xgdEwS5qvOoJ0G-KfaebaEyHf0r3fafoIR-KVqg8thJC-yxOCysQaDlCqqTaGW0pUszj2tI9Iu6w"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"PqXR7Fjjy5UG2GZfo3v2xsI32VQdnrq3mr_kjUYg0xum2AJPRkH2MP4QDtv-8HY9SfPztdoDmk-gNXpmf9SplQ"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"khR4VQ2Ju1xgdEwS5qvOoJ0G-KfaebaEyHf0r3fafoIR-KVqg8thJC-yxOCysQaDlCqqTaGW0pUszj2tI9Iu6w","price":{"value":150,"type":"Integer"},"name":{"type":"String","value":"Product 1"}}

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
  Fields returned: ["name","email","id"]
  First user: Bob Smith <bob@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["created_at","user_role","bio","avatar_url","email","age","id","name","status"]

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
    - Fields: ["age","email","password","name","id","avatar_url","status","created_at","secret_token","bio","api_key","user_role"]
  Projected query:
    - 3 fields per record
    - Fields: ["id","name","email"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ All TypeScript integration tests complete!
🧪 Running JavaScript examples (direct HTTP/WebSocket)...

added 1 package, removed 1 package, and audited 9 packages in 484ms

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
  id: 'Xf3mJyiP7ohsfms7rw9cYZt6X6oJTp-_WOxeUlMdPxbkWHwrBP__FmQZmv6Z1jpPq1Tvn8FRJUyk-WLivoJLSg'
}

=== Find by ID ===
Found: {
  value: { type: 'Integer', value: 42 },
  name: { type: 'String', value: 'Test Record' },
  id: 'Xf3mJyiP7ohsfms7rw9cYZt6X6oJTp-_WOxeUlMdPxbkWHwrBP__FmQZmv6Z1jpPq1Tvn8FRJUyk-WLivoJLSg',
  active: { type: 'Boolean', value: true }
}

=== Find with Query ===
Found documents: [
  {
    active: { type: 'Boolean', value: true },
    value: { value: 42, type: 'Integer' },
    name: { type: 'String', value: 'Test Record' },
    id: 'Xf3mJyiP7ohsfms7rw9cYZt6X6oJTp-_WOxeUlMdPxbkWHwrBP__FmQZmv6Z1jpPq1Tvn8FRJUyk-WLivoJLSg'
  }
]

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  id: 'Xf3mJyiP7ohsfms7rw9cYZt6X6oJTp-_WOxeUlMdPxbkWHwrBP__FmQZmv6Z1jpPq1Tvn8FRJUyk-WLivoJLSg',
  value: { value: 100, type: 'Integer' },
  active: { value: true, type: 'Boolean' }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/simple_websocket.js ===
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: myf81OcLYuV07JOL-SvctJ88jRO8sZOlxX19pkGKtLILR2Ct0UqG-nz2nZzN9VcuUSW2OJXxm5eSyT4_4klx3g

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        },
        "active": {
          "type": "Boolean",
          "value": true
        },
        "value": {
          "value": 42,
          "type": "Integer"
        },
        "id": "myf81OcLYuV07JOL-SvctJ88jRO8sZOlxX19pkGKtLILR2Ct0UqG-nz2nZzN9VcuUSW2OJXxm5eSyT4_4klx3g"
      }
    ]
  },
  "messageId": "1792526731757"
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

✅ Function saved: aTOBOXy5zeFnMi-_DhzbqgLaOfdY-vmh1_3wJsKDynTByhnsUguhpdcWccZtYHRYIsQ2c6_H-Weck71nNxW6kg
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: Up5iK5ZTCcqpJHQPfSCIAVuVpJioHtdqbo9QGVbz56Gx8nplLb1LNQoXkTdsnc4pIYCBxmUNrV51BQ_9lGb7jQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: A_GAhBxoTu-NgE-M0VmKM-ugjLNSbbI5i-9R0ezWRNvNvjHAXU-cYtVd7f10qZP-h80MjFwqZrBOr2M3Nhk4Sg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":{"type":"Integer","value":5},"avg_score":{"type":"Float","value":60},"status":{"type":"String","value":"active"},"max_score":{"value":100,"type":"Integer"}}
   {"count":{"type":"Integer","value":5},"avg_score":{"value":50,"type":"Float"},"max_score":{"type":"Integer","value":90},"status":{"value":"inactive","type":"String"}}

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
cache:product:1: { type: 'Object', value: { price: 29.99, name: 'Product 1' } }
cache:product:2: {
  value: { price: 39.989999999999995, name: 'Product 2' },
  type: 'Object'
}
cache:product:3: {
  value: { price: 49.989999999999995, name: 'Product 3' },
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
Collection created with first record: KA0hEkp9cc7QUptQSgW1sXxXyTyteUyFuL45VSo_CfMYeu_pkV0iJNrh4VkUU71YP2AKu8LMBjqFQKkxe-TQOQ

=== List Collections ===
Total collections: 22
Sample collections: [
  'schema_employees_client_ts',
  'chat_goals__ek0_testing',
  'chat_agent_configs__ek0_testing',
  'chat_tasks__ek0_testing',
  'schema_employees_client_go'
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
Created Alice: $1000 - ID: 0Y4SKxLM9emWXNgv3a4BIWGDMyiWjHFtR4A5b3kYOw2dpqsZ6QQ1MEpOXQyEeErs6chBeMpnuldTZU8IWy8UtQ
Created Bob: $500 - ID: QcKMoJTUs6eWaJvx5HqYIF_-eOI6RMjQaGFoC6Ew7scrzr95mHVVZn2Ig6tx01sRUdntGzAsvzCOJLlKQs2zwg

=== Example 1: Begin Transaction ===
Transaction ID: d20bb1f9-044c-488b-a5cf-3189f5ec3201

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
Bob: {"value":700,"type":"Integer"}

=== Example 5: Rollback ===
New transaction: ad157a41-75cc-4997-b11e-2db1b8c76a50
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

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
   ✅ Function saved: xdzN_FHdbvMtEc4PmJKDePsAQ-fxo-lZujDrMwQcA0yCnnwMfPWnSqLRnttzDdlfI7Xe1vfRJ9koDMXrZslAeQ

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
   ✅ Function saved: nXR1Zc5egBEso3dRzkcJ5t_GmT3tDAiFNE8owyRY9G0hjkNHLfGG7SvgleAz5mILIAilIdzcngQx9hGpLsz9FQ

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
   ✅ Function saved: dKQHWe4aT0VxqzW1pK71HNjgJjJJw4RqB6r8e6QPp3PVZUsVo3X4NqndGW_-FVQaB2kf9gaJlzquAy5xN--IbQ

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
   ✅ Function saved: Rf8n-PoPxQ__D1NdV72DTUAC_zHFkO9g_PEPr2G09foQF4nd_VJkWgm8M2iyVOlgSd2zk_ozJ1smd8vGDV9-tA

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: xdzN_FHdbvMtEc4PmJKD...
   ✅ Deleted function: nXR1Zc5egBEso3dRzkcJ...
   ✅ Deleted function: dKQHWe4aT0VxqzW1pK71...
   ✅ Deleted function: Rf8n-PoPxQ__D1NdV72D...
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
  Output: Document ID = eiiJG0G-g1giro96vhQcSjxduKlDVqbnu6iJYCIqZVbtd2DvTaWjYFOzYuNTz5HQxZdg42H1oquxPztOhze5mA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(eiiJG0G-g1giro96vhQcSjxduKlDVqbnu6iJYCIqZVbtd2DvTaWjYFOzYuNTz5HQxZdg42H1oquxPztOhze5mA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(eiiJG0G-g1giro96vhQcSjxduKlDVqbnu6iJYCIqZVbtd2DvTaWjYFOzYuNTz5HQxZdg42H1oquxPztOhze5mA)
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
  Output: Document ID = 7lWVEf4B988__d1km2jzCVU0ZYevKz7TeSBh_4wTX9c5ySDYUUBSQFFGqm5oMwjJLbbbZ7tO8eLu4T-5zoMm-A
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(7lWVEf4B988__d1km2jzCVU0ZYevKz7TeSBh_4wTX9c5ySDYUUBSQFFGqm5oMwjJLbbbZ7tO8eLu4T-5zoMm-A)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(7lWVEf4B988__d1km2jzCVU0ZYevKz7TeSBh_4wTX9c5ySDYUUBSQFFGqm5oMwjJLbbbZ7tO8eLu4T-5zoMm-A)
  Output: Error (expected) - Request failed with status 404: {"error":"Record has been deleted"}
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

> @ekodb/ekodb-client@0.30.1 prepare
> npm run build


> @ekodb/ekodb-client@0.30.1 build
> tsc


up to date, audited 44 packages in 688ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.30.1 build
> tsc

✅ TypeScript client built!

added 1 package, removed 1 package, and audited 9 packages in 461ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities

added 1 package, removed 1 package, and audited 13 packages in 447ms

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
✓ Created session: 9YVGpqmGEa3PgNsJXZETuNBswRu4ajlOALMv6OIjM7T9yPg6rjoRT2AKSbu-V3vguSDLMY84ds5Snrxz1o4BJA

=== Sending Initial Message ===
✓ Message sent
  Response: It appears that there are currently no products available in the database that match the criteria for availability. The only product I found is `ekoDB`, but it is not listed as available.

If you have specific criteria or different parameters to search for, please let me know!

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
✓ Created second session: _BdioloqanbZNQOqp31ocHyQHRvpj-OKXBk25ETrIgLY0UfscjrKqfr5dvkLxeSB1aveAx6bLau_EOKHyGFAgg
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
✓ Created session: 46NYvo0vE8LlH7FeIt0-TMX20-Ib79WpmWL6rz716lsF30kONGm0F-sCGrBCCznSOCZc-shvjptSBnTGSz0KCg

=== Sending Chat Message ===
Message ID: mjHP9vog998PppcYJHsMInEsbh1GLzMJHsPkTAP_l47RQ_N-32Gkeef6BBddPv9g06VCv7v_IchsyQZqYzP-9Q

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product

2. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities

3. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product',
    id: 'FcjDRUl0791n5q881zHUdJ_gJMnzH3MyVwL92fJp3sHhCwl9lBPchZ7Ul0lDPJA4JrQS0iSejGbu1lm9iIfJuA',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB',
    id: 'OS_TN3pQkdJzpVKFjpdmMoEfX2T78MlAZE4Yz4tuYBk4ngYAvmjixN1fd3tAWAPseU26FX9H-AldeBPEshtp2w',
    description: 'A high-performance database product with AI capabilities',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    price: 299,
    id: 'qXMB8WJAdd2llvesk40ltJzXBJOBWpHxGK6Mv3igOLy1tsgpEpcSXqS7fTNcQePDrDFIcfaB_m0nOnIMpJjDqA',
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3131ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 90
Total tokens: 3503

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
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
  gemini:
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

=== Get Specific Provider Models ===
OpenAI models: gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, gpt-4, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-5-chat-latest, gpt-3.5-turbo-0125, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.4-mini, o1-pro, gpt-5.1-codex, tts-1, gpt-5-mini, gpt-5-mini-2025-08-07, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, omni-moderation-latest, gpt-5-2025-08-07, o4-mini-2025-04-16, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-5.4-mini-2026-03-17, babbage-002, gpt-4o-2024-05-13, text-embedding-3-large, gpt-4o-mini-transcribe-2025-12-15, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-audio-mini-2025-10-06, gpt-5.2-pro-2025-12-11, gpt-4-turbo, gpt-4o-2024-08-06, o3, gpt-4o-mini-search-preview-2025-03-11, o3-mini-2025-01-31, gpt-audio-1.5, gpt-image-1.5, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-4.1-nano, gpt-image-2, gpt-3.5-turbo-16k, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-4-0613, gpt-5.1-2025-11-13, gpt-4o-search-preview, gpt-4.1-mini, gpt-4o-mini, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-5.1-codex-max, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-3.5-turbo-instruct-0914, o1-2024-12-17, sora-2, tts-1-hd-1106, gpt-4o-mini-tts-2025-12-15, gpt-image-1, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-5.2-chat-latest, sora-2-pro, gpt-3.5-turbo-instruct, gpt-transcribe, gpt-4.1-nano-2025-04-14, o4-mini-deep-research, gpt-5.4, gpt-4o-search-preview-2025-03-11, text-embedding-3-small, gpt-5.1-chat-latest, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-5.3-chat-latest, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, gpt-3.5-turbo, gpt-5.2-codex, gpt-4o, davinci-002, gpt-5.4-nano, o3-mini, o4-mini-deep-research-2025-06-26, gpt-audio-mini-2025-12-15, o1-pro-2025-03-19, gpt-5-nano-2025-08-07, gpt-4o-transcribe, o4-mini, gpt-5.1-codex-mini, gpt-3.5-turbo-1106, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, o1, gpt-image-2.5-sunburst-2026-09-08, gpt-6-luna, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-image-2.5-flare, gpt-4o-mini-search-preview, gpt-6-sol, gpt-6-astra, gpt-6.1-sol, gpt-live-1, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: H2jbBt6jkz__-wRM4hHwxHrL0PQ7u1z2tsVjPKPF2xc6WtG9kL0fV-m-7BPldf1kveq5KMBQgzH2ENg2fK96gw

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you have more questions or need further assistance, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99. If you have any more questions or need further assistance, feel free to ask!

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: bNufRHxuwvgpE42NXsSPTgmobjvaeXtDfOin17NDBGuPcc-86gkcrRPvJDNumBHASt5rqg-GJKKBEX9D6azKpA
  Parent: H2jbBt6jkz__-wRM4hHwxHrL0PQ7u1z2tsVjPKPF2xc6WtG9kL0fV-m-7BPldf1kveq5KMBQgzH2ENg2fK96gw

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: bNufRHxuwvgpE42NXsSPTgmobjvaeXtDfOin17NDBGuPcc-86gkcrRPvJDNumBHASt5rqg-GJKKBEX9D6azKpA (Untitled)
  Session 2: H2jbBt6jkz__-wRM4hHwxHrL0PQ7u1z2tsVjPKPF2xc6WtG9kL0fV-m-7BPldf1kveq5KMBQgzH2ENg2fK96gw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: bNufRHxuwvgpE42NXsSPTgmobjvaeXtDfOin17NDBGuPcc-86gkcrRPvJDNumBHASt5rqg-GJKKBEX9D6azKpA

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: 1D2UHS_inm2drZpxVPVG_EpIoS7fX3tSM8_Gf2-qTyHa_kq1OQXXejOdYRg2DPwqhVOTwO6QdPMCkKDBm2ltDw

=== List Collections ===
Total collections: 23
Sample collections: schema_employees_client_ts,chat_goals__ek0_testing,client_collection_management_js,chat_agent_configs__ek0_testing,chat_tasks__ek0_testing

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: rjnpZ1ho1BKhtSpIzGH5LXyXmSPmhhI0AvQMtsu-5-S7DP4eABoaV83YPigff1ZxURqJPA8wM8jdSYjUQKxMWA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: i8anBO6MnU7incUmH8-_K3nBvtDK6Zz1-VYPVwx4kCgRC_zXVSZrvviTx17p8FkfK4oln_uEkQxsq-0cL4QdTQ

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
✓ Edge cache script created: F7IUD7U2Ffg6BYtpCNsn_Hcuh__B1mN2YYWsR0RSWReVBj_qNIiQiN4BRtgAebXp4rwt9lDgxy52zE6zAiR8OA

Call 1: Cache miss (fetches from API)
Response time: 428ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "current": {
            "temperature_2m": 20.6,
            "interval": 900,
            "time": "2026-10-20T20:00"
          },
          "latitude": 40.710335,
          "longitude": -73.99308,
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0,
          "timezone": "GMT",
          "elevation": 32,
          "generationtime_ms": 0.028371810913085938,
          "current_units": {
            "time": "iso8601",
            "temperature_2m": "°C",
            "interval": "seconds"
          }
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
Response time: 5ms (85.6x faster!)
Result: {
  "records": [
    {
      "value": {
        "value": {
          "current": {
            "temperature_2m": 20.6,
            "interval": 900,
            "time": "2026-10-20T20:00"
          },
          "latitude": 40.710335,
          "longitude": -73.99308,
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0,
          "timezone": "GMT",
          "elevation": 32,
          "generationtime_ms": 0.028371810913085938,
          "current_units": {
            "time": "iso8601",
            "temperature_2m": "°C",
            "interval": "seconds"
          }
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
   Name: {"value":"User 1","type":"String"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 75ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "phone": "1-770-736-8031 x56442",
      "name": "Leanne Graham",
      "username": "Bret",
      "address": {
        "zipcode": "92998-3874",...

Second call (cache hit - from cache):
   ⏱️  Duration: 1ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "phone": "1-770-736-8031 x56442",
      "name": "Leanne Graham",
      "username": "Bret",
      "address": {
        "zipcode": "92998-3874",
        "geo": {
    ...
   🚀 Cache speedup: 75.0x faster!

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

✅ Function saved: H2QUOq7Oy6Js05FQNl98iYNOe8ugv5-gI_Ux_34f56ftnXS7g9PGeXHIsTQu-qkEl_f-sqT00yLN4uvbyim2XA
📊 Found 5 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 3 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"count":{"value":5,"type":"Integer"},"avg_score":{"type":"Float","value":50},"status":{"type":"String","value":"inactive"}}
   {"avg_score":{"type":"Float","value":60},"count":{"value":5,"type":"Integer"},"status":{"value":"active","type":"String"}}
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
   1. USB-C Cable - $19 (⭐4.3)
   2. Standing Desk - $599 (⭐4.7)
   3. Desk Chair - $349 (⭐4.2)
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
   1. High Precision: Vector databases maintain a high degree of accuracy. The coordinates of points can be defined precisely in space, maintaining the original details.

2. Scalability: Vector data can be easily scaled. Regardless of how much you zoom in or out, vector data remains clear and undistorted.

3. Small Size: Vector files can often be smaller in size as compared to raster data since they store only vertices and paths information.

4. Easy to Edit: Vector data are easy to edit and manipulate. New elements can be added, and existing ones can be deleted or altered.

5. Selected Analysis: Vector databases are perfect for applications needing analysis on a selected discrete dataset, such as location of stores in a city.

6. Data Attributes: Associated attributes or metadata can be easily attached to the vector objects, allowing for rich, multi-dimensional data.

7. Better Representation: They offer a better representation for features like points (city locations, trees), lines (rivers, roads, rail tracks) and polygons (park boundaries, states).
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding function saved
📊 Generated embeddings for 3 articles
   1. "Getting Started with ekoDB" - 1536D vector
   2. "Advanced Query Patterns" - 1536D vector
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

✅ Function saved: IP5gCrEVb0MrgT-gOpNg_Jrw4LcB9j2hgS14MB-jGI15Tv1z3i1X65yjc0Ln0jHuCVs4NdcHmZgx8E5ehKj8FA
📊 Found 2 product groups
   {"avg_price":{"value":575.6666666666666,"type":"Float"},"category":{"type":"String","value":"Electronics"},"count":{"type":"Integer","value":3}}
   {"avg_price":{"type":"Float","value":474},"category":{"value":"Furniture","type":"String"},"count":{"value":2,"type":"Integer"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":{"type":"String","value":"Electronics"},"count":{"type":"Integer","value":3}}
   {"category":{"type":"String","value":"Furniture"},"count":{"value":2,"type":"Integer"}}
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
   inactive: 3 users
   active: 7 users
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
   3. User 7 - Score: 70
   4. User 2 - Score: 20
   5. User 8 - Score: 80
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

✅ Inserted order: PsR3YrwQwQ4J6jXkclnRZZEqSk_aQ0MOsQiXH7R1b38Or5fjbOqVJmTWgXzoDaw4AymOXdy18h5f7J1BD-bQYA
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: Epx2lyywgzHCeOO40pOUEq6HJbGFJFq8iOkLMMwsrDjvZ_FooZ7IrgWgdm1-42GkfxphE-iKWxNhjiR1fzP1kQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"value":{"role":"admin","userId":"user_abc"},"type":"Object"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: IJ3EGI3inryy1zBX9XbLUR2RdqKEUA_lCVCGBX8--Z-rTPfjiyiiYsQew7XcmDqIR_vQdYVpgMchyeJH60abgQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: pxw6WQ1Hlq8m8AyevzUqkOEkuTD7cFvAfGoWE45w9ogpi47squKTR6p0NVw4B2FB93Qx920wRfboUPXeK-xi-w
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
   2. Natural Language Processing (AI)
   3. Getting Started with ekoDB (Database)
   4. Vector Databases Explained (Database)
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
   2. Natural Language Processing
   3. Getting Started with ekoDB
   4. Vector Databases Explained
   5. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Natural Language Processing
   3. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Function saved
📊 All documents (5 total, showing first 2):
   1. Introduction to Machine Learning (AI)
   2. Natural Language Processing (AI)
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
Retrieved value: { type: 'Object', value: { username: 'john_doe', userId: 123 } }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { type: 'Object', value: { name: 'Product 1', price: 29.99 } }
cache:product:2: {
  type: 'Object',
  value: { price: 39.989999999999995, name: 'Product 2' }
}
cache:product:3: {
  type: 'Object',
  value: { name: 'Product 3', price: 49.989999999999995 }
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
  1. Score: 12.870, Matched: email, name
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
  3. Score: 26.400, Matched: bio, title
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
  1. Score: 0.771, Matched:
  2. Score: 0.755, Matched:
  3. Score: 0.729, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.492, Matched: content, title
  2. Score: 0.909, Matched: title, content
  3. Score: 0.302, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: bio, skills

=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'TWLJuYxWt1f9k3ZvspUTi3TaKbbwkqXR9zF-bvSPg1cuB1JWtVme5hQYNM9SYt_lc1dTrOhVg_pHNlB-NfyKnQ'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  value: { type: 'Integer', value: 42 },
  active: { value: true, type: 'Boolean' },
  id: 'TWLJuYxWt1f9k3ZvspUTi3TaKbbwkqXR9zF-bvSPg1cuB1JWtVme5hQYNM9SYt_lc1dTrOhVg_pHNlB-NfyKnQ'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  id: 'TWLJuYxWt1f9k3ZvspUTi3TaKbbwkqXR9zF-bvSPg1cuB1JWtVme5hQYNM9SYt_lc1dTrOhVg_pHNlB-NfyKnQ',
  active: { value: true, type: 'Boolean' },
  name: { value: 'Updated Record', type: 'String' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: G3QHyf45PDKcGp6XzG-fm97lxZwEiya_RVXSkyda_SSqUvnavdUViDPV3JFbYTDGxo_vt0q8urGqH7iXNfI6SQ

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
✓ Created native SWR script: github_user_native_js (vYBJhOPIw-dmsm465D58xPOKSTqkT1G-mp5MEItsYEwSue8LjIDsabqkFWa26A332OI7AznExCyKNwyvldZUBQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 159ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 79.5x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_js (UT98bXwVJ4xXJkGk6VuFQef650MPrlYuQwAzqJPM44kZh5F6tP_uMUH211QuA2GwhelF_OrEtQivl3quLhlRNA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_js' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_js (UZyRXB-vYHkkyMsvI148MqyTyB5rfsCotPFyhO758xWIUUmtBuinB3kpwg2rdAxyPApblmWtyKWPKZXrcZSTug)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_js' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_js (BK2pRiYodFcfeERuGWRoEtgv-h0FtXHMyzYIDhAicaw_YGGDecEzJisKYdZgIuBniFxFNw2r396hssnNEXlWjQ)
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
✓ Created SWR script: fetch_api_user_js_39714_1792526808655 (N6boBq3JJXH7HMLrTNZlv9FXy3r9PugaPCHS3z8GhSi002WYe6NhK4KfOh9QkfEBbIOL0iS__m31gwoN2wZGvA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "website": "hildegard.org",
          "phone": "1-770-736-8031 x56442",
          "name": "Leanne Graham",
          "email": "Sincere@april.biz",
          "id": 1,
          "address": {
            "zipcode": "92998-3874",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "street": "Kulas Light"
          },
          "username": "Bret"
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
Response time: 1ms (served from cache)
✓ Lightning fast cache hit

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🧹 Cleaning up...
✓ Cleanup complete

✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 7Z3nvoto4Aioppub3bhmkWWZp06kafUxJQhoou5RFCnqTia1i2UFbO3NKijaxiAGCsNo_L9AQ1-zY2KrBTDYsg
Created Bob: $500 - ID: 3w7LC8mHWh6izvDIQZe22x4fIotlLSHwZgmwSYcMSmXZJQ2FTd_-cY9-eXtJ3XHeXiQInib_-6oWDMiNrLreWQ

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): e5c62eb8-80b7-4639-bb1b-8fe365f676db

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
New transaction: 554b0d96-bb1d-4c8e-acce-60daa5d0006d
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: FjmVTkZOdQQmm7xKPHqRiZElmwohmq6rk66bFhyhQtc_FW1c2dlzv-FGtFL-ITCkGi2KcSKdzhhFgV855Lw9XQ

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
Admin permissions read successfully
=== ekoDB Advanced CRUD Example (TypeScript) ===

--- Inserting base record ---
Inserted: bNLKWBzApjUuB0hyP_hrNPH6b_q8umZQdRZXmh-jjjIElNC4QeRsBqZMM19tio9aVqeAS3fz2t6Zr4DUHjUj0w

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
✓ Created session: qEzru4MncNDIkzFOCQLM2cRYvJagwMwXHMRUtpBnDUcaNRys0f-Z4unjyJi_lROGVdJFim9u3t9fJFvIeehllg

=== Sending Initial Message ===
✓ Message sent
  Response: It appears that there are currently no products marked as available in the database. The only product we have context for is "ekoDB," which has a price of $99, but it isn't explicitly indicated as available.

If there's anything else you'd like to know or if you want to perform a different search, please let me know!

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
✓ Created second session: rFTbR8ekEncOOTonIZzK7tAtUdGJzNBzaAfy7Ttt82PljIPDHixlhLeb8agtKMhuWBbnAYUt1EtVCkv-Y9A6Iw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: rFTbR8ekEncOOTonIZzK7tAtUdGJzNBzaAfy7Ttt82PljIPDHixlhLeb8agtKMhuWBbnAYUt1EtVCkv-Y9A6Iw
✓ Deleted chat session: qEzru4MncNDIkzFOCQLM2cRYvJagwMwXHMRUtpBnDUcaNRys0f-Z4unjyJi_lROGVdJFim9u3t9fJFvIeehllg
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: RjjqlbsMVMIoadFgm1AR5oFsUkshg_BOA8oucCOZC-yAxkP9MsfHF1WMQa7C_NUas9NNF-1oa2ZPAyREPAOU0Q

=== Sending Chat Message ===
Message ID: foPUGDz-bJskaLNg_egd4LtKFftYwjHFfF5VBWBAqjYjErroWpD-S2NItsoPZTSwgt7SebC_uBh-SYJzpwUxQA

=== AI Response ===
The available products and their prices are as follows:

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
    name: 'ekoDB',
    id: '4Sw7QBDkyLpanDqa7uPQYOTFS0rBbDIrhpRXwephpWtBKaniTx14136vM9xzrQFhvyczfTPAvgWXkzRThXNo3w',
    price: 99,
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Pro',
    price: 299,
    description: 'Enterprise edition product with advanced features',
    id: 'Vj2-xgR0TOS2j7MiS4iWL3R1fy_MHpAA7QCuoFFkFjOBHsS8Krsp232TQ_D96p5nD2DrNgKINZRkgoP87FtYtA'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    price: 499,
    id: 't7BHfTEB0GyvGcZugw5vitp7GuPKIhOg40JIt85NlauLWZCgtrDuSXl89xlFUi7oAOjWAmu4A_wKWJNfm2ZTUw',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3406ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 93
Total tokens: 3506

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (TypeScript) ===

Created session: Mcr3wJMZM0n3LwB0maSHRgPECmYEqPV67Ev07tz4dLAzyFzJAzNN6Mpg4BXhjNHMjPfo5Mn96uWK14cbrS4oWw

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}**ekoDB** is a database and web portal focused on human gene essentiality and knockout/knockdown data. Its key features include:

- **Comprehensive Data Collection:** ekoDB compiles data from various genome-wide experimental studies, particularly those that use gene knockout (e.g., CRISPR, TALEN) or knockdown (e.g., RNAi) approaches in human cells. The database contains information about which genes are essential (required for cell survival and proliferation) or non-essential under specific conditions or in particular cell types.

- **Gene Essentiality:** Gene essentiality refers to whether loss of a gene impairs cell viability or has significant functional effects. ekoDB aggregates and standardizes essentiality data, helping researchers quickly determine which human genes are considered "essential" based on current experimental evidence.

- **User Tools and Visualization:** The web interface allows users to query specific genes, visualize essentiality scores, compare experimental datasets, and explore gene function and pathways associated with essential genes.

- **Applications:** ekoDB is widely used in genetics, functional genomics, drug target discovery, cancer research, and synthetic lethality studies.

**References:**
- Wang T, et al. "Gene Essentiality Profiling Reveals a Gene Network Underlying Cellular Fitness." *bioRxiv* (2017). [Link](https://www.biorxiv.org/content/10.1101/182691v1)
- [ekoDB official website](https://ekodb.genetics.ac.cn/) (if available)

**In summary:**
ekoDB is a valuable research resource for exploring which human genes are essential, based on large-scale genetic perturbation studies.

--- Stream complete ---
Message ID: Cavkn75GIy-0fvTpSvjZfkUH4sD8khuTBwg1bY5U5ddfyx6L2oZBRUFbQCrO5uqz9llOdrLfvD6a-f_gMnQtfw
Execution time: 4276ms
Context window: 1000000 tokens

✓ Chat message stream example completed
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
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
  gemini:
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

=== Get Specific Provider Models ===
OpenAI models: gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, gpt-4, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-5-chat-latest, gpt-3.5-turbo-0125, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.4-mini, o1-pro, gpt-5.1-codex, tts-1, gpt-5-mini, gpt-5-mini-2025-08-07, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, omni-moderation-latest, gpt-5-2025-08-07, o4-mini-2025-04-16, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-5.4-mini-2026-03-17, babbage-002, gpt-4o-2024-05-13, text-embedding-3-large, gpt-4o-mini-transcribe-2025-12-15, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-audio-mini-2025-10-06, gpt-5.2-pro-2025-12-11, gpt-4-turbo, gpt-4o-2024-08-06, o3, gpt-4o-mini-search-preview-2025-03-11, o3-mini-2025-01-31, gpt-audio-1.5, gpt-image-1.5, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-4.1-nano, gpt-image-2, gpt-3.5-turbo-16k, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-4-0613, gpt-5.1-2025-11-13, gpt-4o-search-preview, gpt-4.1-mini, gpt-4o-mini, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-5.1-codex-max, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-3.5-turbo-instruct-0914, o1-2024-12-17, sora-2, tts-1-hd-1106, gpt-4o-mini-tts-2025-12-15, gpt-image-1, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-5.2-chat-latest, sora-2-pro, gpt-3.5-turbo-instruct, gpt-transcribe, gpt-4.1-nano-2025-04-14, o4-mini-deep-research, gpt-5.4, gpt-4o-search-preview-2025-03-11, text-embedding-3-small, gpt-5.1-chat-latest, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-5.3-chat-latest, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, gpt-3.5-turbo, gpt-5.2-codex, gpt-4o, davinci-002, gpt-5.4-nano, o3-mini, o4-mini-deep-research-2025-06-26, gpt-audio-mini-2025-12-15, o1-pro-2025-03-19, gpt-5-nano-2025-08-07, gpt-4o-transcribe, o4-mini, gpt-5.1-codex-mini, gpt-3.5-turbo-1106, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, o1, gpt-image-2.5-sunburst-2026-09-08, gpt-6-luna, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-image-2.5-flare, gpt-4o-mini-search-preview, gpt-6-sol, gpt-6-astra, gpt-6.1-sol, gpt-live-1, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: y0m8Pi58AkfPqm0WLB5O_Lw2pWX7rmsNc5REXcXkWWmioz8Lz7LiqJIuK8BuWC4YSifPxVRK5NeGP-mGJ7H3LA

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
✓ Created branch: -xg0Q2q5kUryxfODZa0h7B1CX6kiHdme1qi6W9wl-A4REqekSt6T3bc86SrjBxMQ8eiya3xENt7p_DhkQGvCvg
  Parent: y0m8Pi58AkfPqm0WLB5O_Lw2pWX7rmsNc5REXcXkWWmioz8Lz7LiqJIuK8BuWC4YSifPxVRK5NeGP-mGJ7H3LA

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: -xg0Q2q5kUryxfODZa0h7B1CX6kiHdme1qi6W9wl-A4REqekSt6T3bc86SrjBxMQ8eiya3xENt7p_DhkQGvCvg (Untitled)
  Session 2: y0m8Pi58AkfPqm0WLB5O_Lw2pWX7rmsNc5REXcXkWWmioz8Lz7LiqJIuK8BuWC4YSifPxVRK5NeGP-mGJ7H3LA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: -xg0Q2q5kUryxfODZa0h7B1CX6kiHdme1qi6W9wl-A4REqekSt6T3bc86SrjBxMQ8eiya3xENt7p_DhkQGvCvg

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: sRwpNXoOpSMCKm-RDHlGCm3F5bsMsBA03koxjmBauAvDDypHV6lJdlCyOCZ_cmbjqj-h1UQ1hM6I7SmYxxzWTA

=== List Collections ===
Total collections: 27
Sample collections: schema_employees_client_ts,chat_goals__ek0_testing,schema_documents_client_js,chat_agent_configs__ek0_testing,chat_tasks__ek0_testing

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
  POST /api/functions/conc_demo_pay_ts_70491_1792526834418 { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail_ts_70491_1792526834418 { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip_ts_70491_1792526834418 { "user_id": 42 }
  POST /api/functions/conc_demo_lock_ts_70491_1792526834418 { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'ICiAcNnUlM_cbBjmcotPttbv7IHBFQmCGMpkJQsEU6qYMy5VPVuvCOdoAbFwzOY2YO03ZZMR2Rf_Z9XQxEAhKQ'
}

=== Upsert Operation ===
✓ First upsert (update): {
  active: { type: 'Boolean', value: true },
  age: { type: 'Integer', value: 29 },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  name: { type: 'String', value: 'Alice Johnson' },
  id: 'ICiAcNnUlM_cbBjmcotPttbv7IHBFQmCGMpkJQsEU6qYMy5VPVuvCOdoAbFwzOY2YO03ZZMR2Rf_Z9XQxEAhKQ'
}
✓ Second upsert (insert): { id: 'new-user-id' }

=== Find One Operation ===
✓ Found user by email: {
  name: { value: 'Alice Johnson', type: 'String' },
  active: { type: 'Boolean', value: true },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  id: 'ICiAcNnUlM_cbBjmcotPttbv7IHBFQmCGMpkJQsEU6qYMy5VPVuvCOdoAbFwzOY2YO03ZZMR2Rf_Z9XQxEAhKQ',
  age: { type: 'Integer', value: 29 }
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
✓ Inserted document: yR1LLqLQ2I76VBDvhbr5DVQiQtTUEWq9sp0P9EnizoQ5zZnpApD0UI-ntF2hz-BabcguCizReJ7SuLcfgHUcZg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: g-8SZ1MkvVndbfbpFBPhcYngwhtplYDGYafH8uyvPPtHrwMU4cso-JFPmF_J-9duDGzRokqoPJZjcqvhNNAYjg

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
✓ Edge cache script created: enh8NTr72FrEiMKv0AQ8dq6rQyUT-zEzqEHZb3LqZTOSoDp3vr0u0k6Wrv_QaJpyyJZzL0-Zmb7Yoqay-bQJtw

Call 1: Cache miss (fetches from API)
Response time: 72ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "name": "Leanne Graham",
          "username": "Bret",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "address": {
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556",
            "city": "Gwenborough"
          },
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
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
Response time: 1ms (72x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "name": "Leanne Graham",
          "username": "Bret",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "address": {
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556",
            "city": "Gwenborough"
          },
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
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
   Name: {"value":"User 1","type":"String"}
   Department: {"type":"String","value":"engineering"}

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
      "phone": "1-770-736-8031 x56442",
      "company": {
        "bs": "harness real-time e-markets",
        "name": "Romaguera-Crona",
        "catchPhrase": "Multi-l...

Second call (cache hit - from cache):
   ⏱️  Duration: 1ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "phone": "1-770-736-8031 x56442",
      "company": {
        "bs": "harness real-time e-markets",
        "name": "Romaguera-Crona",
        "catchPhrase": "Multi-l...
   🚀 Cache speedup: 53.0x faster!

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

✅ Function saved: 7ie8qxvJ2K4o5tbMzCg68T3P-oINR6puxZXVG3E5QyheEbNvilSSCvrpQWd0F8XMm9JBmQKVh73V7xnX6ql97w
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: pWdGUKxaiF3B4dCoDc9wlsnJS8R-e_cpb6TtxCpbH05AHrobQPeEIhUdXelDWu_Fjh_WYkjtilXeaAUBbg2__g
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: smt-n6WWcyDeroROcMk9qrFAUjLNYG3jRjblL8hpgM9mnqF4NzS58KgVRlximkos_Jcv5dqZbfEIdC1NjO2USQ
📊 Statistics: 2 groups
   {"count":{"type":"Integer","value":5},"status":{"value":"inactive","type":"String"},"avg_score":{"value":50,"type":"Float"}}
   {"status":{"value":"active","type":"String"},"avg_score":{"type":"Float","value":60},"count":{"type":"Integer","value":5}}

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
   {"category":{"value":"Furniture","type":"String"},"count":{"type":"Integer","value":3},"avg_price":{"type":"Float","value":365.6666666666667}}
   {"count":{"value":5,"type":"Integer"},"avg_price":{"value":367,"type":"Float"},"category":{"type":"String","value":"Electronics"}}
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

1. **Efficient Similarity Search**: They enable quick retrieval of similar items by computing distances in high-dimensional spaces, facilitating tasks like recommendation systems and image similarity.

2. **Scalability**: Designed to handle large datasets and high-dimensional vectors, vector databases can scale horizontally, accommodating growing amounts of data.

3. **Flexibility**: They support various data types and embedding techniques, making them suitable for applications like natural language processing and computer vision.

4. **Performance Optimization**: Many vector databases use indexing techniques such as HNSW or Annoy to speed up search times while maintaining accuracy.

5. **Real-time Processing**: They allow for fast querying and data insertion, making them ideal for applications requiring real-time responses.

6. **Integrated Machine Learning**: Vector databases often integrate seamlessly with machine learning workflows, supporting model training and deployment.

7. **Handling Unstructured Data**: They excel in managing unstructured data by converting it into vector representations, making it easier to analyze and retrieve.

8. **Support for Embeddings**: They can easily manage embeddings from various AI models, enabling tasks like semantic search and clustering.

Overall, vector databases are crucial for applications that need to perform similarity searches and handle high-dimensional data efficiently.
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

✅ Function saved: A_WuAc75DgVE-Xa1WQxwIFLX3tQAnnryUSLCbW-xEDFnToRYV96j2xQUkA9lvR_pYmA4QrxEPE_H4DDUE4XhGw
📊 Found 2 product groups
   {"count":{"type":"Integer","value":3},"avg_price":{"type":"Float","value":575.6666666666666},"category":{"type":"String","value":"Electronics"}}
   {"category":{"type":"String","value":"Furniture"},"avg_price":{"value":474,"type":"Float"},"count":{"value":2,"type":"Integer"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":{"value":"Furniture","type":"String"},"count":{"value":2,"type":"Integer"}}
   {"count":{"type":"Integer","value":3},"category":{"type":"String","value":"Electronics"}}
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
   active: 7 users
   inactive: 3 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average score by role:
   {"count":{"value":7,"type":"Integer"},"role":{"type":"String","value":"user"},"avg_score":{"type":"Float","value":70}}
   {"avg_score":{"value":20,"type":"Float"},"role":{"value":"admin","type":"String"},"count":{"type":"Integer","value":3}}
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

✅ Inserted order: 0M-AaLOOC3oz09mWCXunK45X0qo86cNNmK-3onemLkE2itjT71oHlM1iJV7b-dIJMN1YsXfON6d31rqtTtgnRA
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: 9RvoGgeIbhWD46NO44G1o8BO5Dod7vbe6ga1ZTbwtip9yOcMjbxD3EVyNwry4NGOFW8X6gysbeJtk1y4HqUUGQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"value":{"userId":"user_abc","role":"admin"},"type":"Object"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: wfPAbVNY6wO_NietKIuGti7Tz_Nf_eg28Dl6h22Q9dIjjSimmIkKKcczEFr4L-TqUiwHdjtYmhFNNf9pGq4hlg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: BCvE4s5plw21tznRmP-AiKPSFtC6bQeVU--MCa8CEStfNMPradqw5PcseYfkbQpMzUktQHI9fbHDii6NAiy4Lg
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
   1. Introduction to Machine Learning (AI)
   2. Database Design Principles (Database)
   3. Vector Databases Explained (Database)
   4. Natural Language Processing (AI)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"count":{"type":"Integer","value":3},"category":{"value":"Database","type":"String"}}
   {"category":{"value":"AI","type":"String"},"count":{"type":"Integer","value":2}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Goal Template CRUD Example (TypeScript) ===

--- Creating goal template ---
Created template: Data Migration (id: llw8k-5AqDIRCwtQBOTa3Q2k3CsQ8mi0hukqjWvnsY1ekpY4QujPIYnG7aLLFw3sw2eBWyTpIVTD3qprLYckWg)

--- Listing templates ---
Templates: {
  count: 1,
  items: [
    {
      description: [Object],
      id: 'llw8k-5AqDIRCwtQBOTa3Q2k3CsQ8mi0hukqjWvnsY1ekpY4QujPIYnG7aLLFw3sw2eBWyTpIVTD3qprLYckWg',
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
Created goal: Deploy v2.0 (id: OVSvlbHoM-TNnQy3_yaAjk3Dg_8lqvBYz39pI24T9zv0jCqzzF8ckcPK1_xRcJ4vOoXw2KywTm7crwLKnbXDyw)

--- Listing goals ---
Goals: {
  "count": 1,
  "goals": [
    {
      "created_at": "2026-10-20T20:07:20.655004+00:00",
      "description": "Ship version 2.0 to production",
      "id": "OVSvlbHoM-TNnQy3_yaAjk3Dg_8lqvBYz39pI24T9zv0jCqzzF8ckcPK1_xRcJ4vOoXw2KywTm7crwLKnbXDyw",
      "status": "pending",
      "steps": "[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]",
      "title": "Deploy v2.0",
      "updated_at": "2026-10-20T20:07:20.655004+00:00"
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
        "value": "2026-10-20T20:07:20.655004+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 with hot-fix patches"
      },
      "id": "OVSvlbHoM-TNnQy3_yaAjk3Dg_8lqvBYz39pI24T9zv0jCqzzF8ckcPK1_xRcJ4vOoXw2KywTm7crwLKnbXDyw",
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
        "value": "2026-10-20T20:07:20.664626+00:00"
      }
    }
  ]
}

--- Goal step: start step 0 ---
Step 0 started on goal OVSvlbHoM-TNnQy3_yaAjk3Dg_8lqvBYz39pI24T9zv0jCqzzF8ckcPK1_xRcJ4vOoXw2KywTm7crwLKnbXDyw
--- Goal step: complete step 0 ---
Step 0 completed on goal OVSvlbHoM-TNnQy3_yaAjk3Dg_8lqvBYz39pI24T9zv0jCqzzF8ckcPK1_xRcJ4vOoXw2KywTm7crwLKnbXDyw
--- Goal step: fail step 1 ---
Step 1 failed on goal OVSvlbHoM-TNnQy3_yaAjk3Dg_8lqvBYz39pI24T9zv0jCqzzF8ckcPK1_xRcJ4vOoXw2KywTm7crwLKnbXDyw

--- Completing goal ---
Goal status: pending_review
--- Approving goal ---
Goal status after approve: in_progress

--- Creating goal to reject ---
--- Rejecting goal ---
Goal status after reject: failed


--- Creating task ---
Created task: Hourly Health Check (id: 2JoCmgYOOvmOROMd5vgA8i21bOQttx4ogYLU0eYaIP29Lb0M2MwYCkTdYL1PcpZWAFmTQTP18kV6rdiTGTly3A)

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
      "id": "2JoCmgYOOvmOROMd5vgA8i21bOQttx4ogYLU0eYaIP29Lb0M2MwYCkTdYL1PcpZWAFmTQTP18kV6rdiTGTly3A",
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
Created agent: SupportBot (id: cNc14qRQnKZRROjWMx5eIhH7WASyM0bWoRI4DEHre5bt390K915nSd4cfbUlBJFZOtxtyjlgetstE-nbf2UhJQ)

--- Listing agents ---
Agents: {
  "count": 1,
  "items": [
    {
      "deployment_id": {
        "type": "String",
        "value": "deploy_prod_1"
      },
      "id": "cNc14qRQnKZRROjWMx5eIhH7WASyM0bWoRI4DEHre5bt390K915nSd4cfbUlBJFZOtxtyjlgetstE-nbf2UhJQ",
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
By name: SupportBot (id: cNc14qRQnKZRROjWMx5eIhH7WASyM0bWoRI4DEHre5bt390K915nSd4cfbUlBJFZOtxtyjlgetstE-nbf2UhJQ)

--- Updating agent ---
Updated agent: SupportBot

--- Getting agents by deployment ---
Agents in deployment: {
  "count": 0,
  "items": []
}
WARNING: agents-by-deployment omitted created agent cNc14qRQnKZRROjWMx5eIhH7WASyM0bWoRI4DEHre5bt390K915nSd4cfbUlBJFZOtxtyjlgetstE-nbf2UhJQ; TODO: check/fix the server-side deployment lookup

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


✅ Join operations examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created
✓ ts_users_register saved
✓ ts_users_login saved
✓ ts_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/jwt_register_ts_78518_1792526841109 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_login_ts_78518_1792526841109 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_verify_ts_78518_1792526841109 { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

✓ Cleaned up demo functions
=== ekoDB KV Document Linking Example (TypeScript) ===

--- Setup: creating KV key and documents ---
Set KV key: session:admin
Inserted document 1: _ox6sF-jDBjuaFFxRhlEzDpNnlAHU_QC5b8R3uBmrRIaYRoEoXWqYjH3HWQSrpZRUw25cWUwzxSWTrmVSUSfZw
Inserted document 2: VlfO28WAL-JdIY-dbnYNPWoiLvFp9sLOwgo38zZLWq3I7DqYfRWRA9qygaJfYFLpYq2ppashaqEIQM46sBNgMQ

--- Linking documents to KV key ---
Linked doc _ox6sF-jDBjuaFFxRhlEzDpNnlAHU_QC5b8R3uBmrRIaYRoEoXWqYjH3HWQSrpZRUw25cWUwzxSWTrmVSUSfZw: null
Linked doc VlfO28WAL-JdIY-dbnYNPWoiLvFp9sLOwgo38zZLWq3I7DqYfRWRA9qygaJfYFLpYq2ppashaqEIQM46sBNgMQ: null

--- Getting links for KV key ---
Links: [
  {
    "collection": "kv_links_example_ts_78669_1792526841196",
    "document_id": "_ox6sF-jDBjuaFFxRhlEzDpNnlAHU_QC5b8R3uBmrRIaYRoEoXWqYjH3HWQSrpZRUw25cWUwzxSWTrmVSUSfZw",
    "field_path": null,
    "created_at": "2026-10-20T20:07:21.276619Z",
    "last_accessed": "2026-10-20T20:07:21.279357Z",
    "metadata": {}
  },
  {
    "collection": "kv_links_example_ts_78669_1792526841196",
    "document_id": "VlfO28WAL-JdIY-dbnYNPWoiLvFp9sLOwgo38zZLWq3I7DqYfRWRA9qygaJfYFLpYq2ppashaqEIQM46sBNgMQ",
    "field_path": null,
    "created_at": "2026-10-20T20:07:21.277851Z",
    "last_accessed": "2026-10-20T20:07:21.279357Z",
    "metadata": {}
  }
]

--- Unlinking document ---
Unlinked doc VlfO28WAL-JdIY-dbnYNPWoiLvFp9sLOwgo38zZLWq3I7DqYfRWRA9qygaJfYFLpYq2ppashaqEIQM46sBNgMQ: null

--- Verifying remaining links ---
Remaining links: [
  {
    "collection": "kv_links_example_ts_78669_1792526841196",
    "document_id": "_ox6sF-jDBjuaFFxRhlEzDpNnlAHU_QC5b8R3uBmrRIaYRoEoXWqYjH3HWQSrpZRUw25cWUwzxSWTrmVSUSfZw",
    "field_path": null,
    "created_at": "2026-10-20T20:07:21.276619Z",
    "last_accessed": "2026-10-20T20:07:21.282121Z",
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
Retrieved value: { value: { userId: 123, username: 'john_doe' }, type: 'Object' }

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_ops_ts_78847_1792526841376:cache:product:1: success
  kv_ops_ts_78847_1792526841376:cache:product:2: success
  kv_ops_ts_78847_1792526841376:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_ts_78847_1792526841376:cache:product:1: { name: 'Product 1', price: 29.99 }
  kv_ops_ts_78847_1792526841376:cache:product:2: { name: 'Product 2', price: 39.99 }
  kv_ops_ts_78847_1792526841376:cache:product:3: { name: 'Product 3', price: 49.99 }

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
  kv_ops_ts_78847_1792526841376:cache:product:1: deleted
  kv_ops_ts_78847_1792526841376:cache:product:2: deleted
  kv_ops_ts_78847_1792526841376:cache:product:3: deleted

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
    "name": "Earth",
    "diameter_km": 12742
  },
  {
    "name": "Mars",
    "diameter_km": 6779
  },
  {
    "name": "Jupiter",
    "diameter_km": 139820
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===
=== ekoDB Schedule Management Example (TypeScript) ===

--- Creating schedule ---
Created schedule: Nightly Database Backup (id: 464de3d4-b691-4362-956e-fbf8e02ab78f, cron: 0 0 2 * * *)

--- Listing schedules ---
Schedules: {
  "count": 1,
  "schedules": [
    {
      "created_at": "2026-10-20T20:07:24.312030Z",
      "cron_expression": "0 0 2 * * *",
      "description": null,
      "enabled": true,
      "function_label": "schedule_noop_typescript_82326_1792526844277",
      "id": "464de3d4-b691-4362-956e-fbf8e02ab78f",
      "last_execution": null,
      "name": "Nightly Database Backup",
      "next_execution": "2026-10-21T02:00:00Z",
      "parameters": {},
      "stats": {
        "avg_execution_time_ms": 0,
        "failed_executions": 0,
        "last_error": null,
        "successful_executions": 0,
        "total_executions": 0
      },
      "timezone": "UTC",
      "updated_at": "2026-10-20T20:07:24.312030Z"
    }
  ]
}

--- Getting schedule ---
Fetched: Nightly Database Backup (cron: 0 0 2 * * *)

--- Updating schedule ---
Updated: Nightly Full Backup (new cron: 0 0 3 * * *)

--- Triggering schedule ---
Trigger response: {
  "schedule_id": "464de3d4-b691-4362-956e-fbf8e02ab78f",
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
  1. Score: 13.200, Matched: bio, title
  2. Score: 13.200, Matched: bio, title
  3. Score: 13.200, Matched: bio, title
  4. Score: 13.200, Matched: title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: title, bio
  2. Score: 26.400, Matched: title, bio
  3. Score: 26.400, Matched: bio, title
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
  1. Score: 0.783, Matched:
  2. Score: 0.756, Matched:
  3. Score: 0.714, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.513, Matched: title, content
  2. Score: 0.885, Matched: title, content
  3. Score: 0.303, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: bio, skills

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Introduction to Machine Learning (category: ml)
  2. Deep Learning Fundamentals (category: ml)

Paired vector search: document=Rust Programming Answer, query=Rust Programming Question

✅ Search examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'SmTrUvRmT7qAkeXfnHpMSGBquVTLd0H1fPtJalnmTj_Qt4pEFeIVeVoNpVi7B1HpY81QK6GBPnlYPUbuvfLuxA'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  active: { type: 'Boolean', value: true },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  created_at: { type: 'DateTime', value: '2026-10-20T20:07:24.845+00:00' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  id: 'SmTrUvRmT7qAkeXfnHpMSGBquVTLd0H1fPtJalnmTj_Qt4pEFeIVeVoNpVi7B1HpY81QK6GBPnlYPUbuvfLuxA',
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  price: { type: 'Float', value: 99.99 },
  value: { value: 42, type: 'Integer' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-10-20T20:07:24.845Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  name: 'Test Record',
  active: true,
  metadata: { nested: { deep: true }, key: 'value' },
  created_at: '2026-10-20T20:07:24.845+00:00',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  categories: [ 'electronics', 'computers' ],
  id: 'SmTrUvRmT7qAkeXfnHpMSGBquVTLd0H1fPtJalnmTj_Qt4pEFeIVeVoNpVi7B1HpY81QK6GBPnlYPUbuvfLuxA',
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  data: 'aGVsbG8gd29ybGQ=',
  price: 99.99,
  value: 42,
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  value: { value: 100, type: 'Integer' },
  id: 'SmTrUvRmT7qAkeXfnHpMSGBquVTLd0H1fPtJalnmTj_Qt4pEFeIVeVoNpVi7B1HpY81QK6GBPnlYPUbuvfLuxA',
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  name: { type: 'String', value: 'Updated Record' },
  active: { value: true, type: 'Boolean' },
  price: { type: 'Float', value: 99.99 },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  created_at: { type: 'DateTime', value: '2026-10-20T20:07:24.845+00:00' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 16umZM2RiGCwfIZGjv6ULjiiyeW86mFs8VsYq1m64fKkH0oRtL1BRsm1Ssbmr4BtHhaq9Tt9DnFGfj6SZlNsGg

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
✓ Created native SWR script: github_user_native_ts (MrJkSk-sa2VwESNo3ypeV9rldtUezAL-xUHxQDbKRb-SECzHFOIbeDm1Z5ZVjkOpXwTeUygtlFxkVJq6Wk0RVQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 49ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 1ms
  Speedup: 49.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_ts (fupSd_shvZWVnrCawKNqb3qMJk7L5iW4w4MPHtucvMO4RnBlfAd4arD0wuwm9cOAA2VmvvjObG4WF6UFCuecCA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_ts' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_ts (VKocyNriM0OclvLSeuASFAkWgg-RZcS7WUxT9AkwPH16uoiAERHZjl7eu-mqacOrUkRlZw55QGsm0FTypPAk2Q)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_ts' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_ts (F1vGJncTxEyBlbM3m9aAwCAdsyJhr4Bws_0RimR7_8VvGH1o4ai1l9nT5xnZB2mHoLxFiEECQEhAAL_X5Ehg0Q)
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
✓ Created SWR script: fetch_api_user_ts_84316_1792526845924 (N2qpgqIqBLp9EZeC5pr880hYVZMREduwAuqULfDITgZhBsCJl9Kw_Mb3JtUBcXjnxYhjvLuWipAPjjtGhsIbGw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "name": "Leanne Graham",
          "address": {
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "street": "Kulas Light",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "zipcode": "92998-3874"
          },
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "website": "hildegard.org",
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 1ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "value": {
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "name": "Leanne Graham",
          "address": {
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "street": "Kulas Light",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "zipcode": "92998-3874"
          },
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "website": "hildegard.org",
          "id": 1
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
✓ Created enrichment script: fetch_product_reviews_ts_84316_1792526845924 (oKFP34Lq5Ny6nxgLsMXriyng0DJbJWZhxRgRn7F6gDoqxV1bXld9cwVICzOCV-ELcbqpGcGnFI1CL_6SCrymcw)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "value": {
          "availabilityStatus": "In Stock",
          "shippingInformation": "Ships in 3-5 business days",
          "warrantyInformation": "1 week warranty",
          "price": 9.99,
          "weight": 4,
          "sku": "BEA-ESS-ESS-001",
          "reviews": [
            {
              "rating": 3,
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "reviewerName": "Eleanor Collins",
              "comment": "Would not recommend!"
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 4,
              "comment": "Very satisfied!",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "reviewerName": "Lucas Gordon"
            },
            {
              "rating": 5,
              "comment": "Highly impressed!",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Eleanor Collins",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com"
            }
          ],
          "category": "beauty",
          "title": "Essence Mascara Lash Princess",
          "dimensions": {
            "width": 15.14,
            "depth": 22.99,
            "height": 13.08
          },
          "discountPercentage": 10.48,
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "brand": "Essence",
          "id": 1,
          "meta": {
            "barcode": "5784719087687",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "updatedAt": "2026-05-23T11:27:41.868Z",
            "createdAt": "2025-10-09T14:47:01.588Z"
          },
          "stock": 99,
          "tags": [
            "beauty",
            "mascara"
          ],
          "returnPolicy": "No return policy",
          "minimumOrderQuantity": 48,
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
Created Alice: $1000 - ID: ILnvyn9v6pspu3HWnE48fphMbxqWjNIoSzTwDsQijop76WinpR0glTPsA77j9jdYbjDqHwA9OcjGP_vCguUHyA
Created Bob: $500 - ID: JgA-Sy1VMKguAp9D29ucfwCiXbHH56dUuIlpxvrtQR2-zuwypj1j4BJgdvQAszfMzTVUoRNbN4xuZQZjSHi8qg

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 90a4cad4-f9cb-485c-9491-e91102926fb1

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
New transaction: 3c5d630e-3b75-4585-a179-18e395f7a6f0
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: 0GN1SCtK2mAT2lhge9TrfED21l9YGTarrwdjP2pvRMpOUMwJs04RAfRsTdB4k0tnPXCGeONEGxdf1s3dKk6y6A

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 10 user functions:
  - get_active_users_ts: Get Active Users
  - conc_demo_rl_skip_ts_12240_1792526703280: Rate-limit (skip mode)
  - conc_demo_rl_fail_ts_12240_1792526703280: Rate-limit (fail mode)
  - get_active_users_client_js: Get Active Users (Updated)
  - conc_demo_rl_fail_ts_70491_1792526834418: Rate-limit (fail mode)
  - conc_demo_rl_skip_ts_70491_1792526834418: Rate-limit (skip mode)
  - fetch_product_reviews_ts_38404_1792526724530: Fetch Product with Reviews (Multi-API)
  - get_active_users_client_ts_updated: Get Active Users (Updated)
  - fetch_product_reviews_ts_84316_1792526845924: Fetch Product with Reviews (Multi-API)
  - get_active_users_client_ts_updated: Get Active Users (Updated)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (TypeScript) ===

Created chat session: Leu7iZ8HISwLM6fnGJnnswINfN8I8xn-Q0dfPHLmKdKmRuDZmmU9fEOsyLwh4JvGm41CJ1SOmMZYqNOCq-DmyA

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: 0L4al5f12d5S90L3DvJniTJzisxu4YN00cK59RvOs7_9Dubxybs1XINz7qEXGOnFoPX6lls8yl10ceuUbNxqmw
Execution time: 676ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_881c3132b3644e6285daa6e7b5aef2d3)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: ma9gu24xVgTLZnXcehriPXp72SLioVRYtC3hTWaj33MOXVtuTaQuzeeNGAedSzVSVbtSxFpqClFVCpRm5OIy-g
  📡 Notification received for ma9gu24xVgTLZnXcehriPXp72SLioVRYtC3hTWaj33MOXVtuTaQuzeeNGAedSzVSVbtSxFpqClFVCpRm5OIy-g

Inserting another record...
✓ Inserted record: FnHtgjXazBsfC2Zxo7p8edk7f5hSwHL8f4tzz-35CFGgWmZiySvw-hz8yEmOVQWmAPwCLesaRfIsv7eBPFZv3g
  📡 Notification received for FnHtgjXazBsfC2Zxo7p8edk7f5hSwHL8f4tzz-35CFGgWmZiySvw-hz8yEmOVQWmAPwCLesaRfIsv7eBPFZv3g

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_ts'
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: a-t0zWfebHZuIpuxZ9bWmw11r5O-7nSM2xhGUbAQJ5J0TpSY5dn-jbRKwhqDuECVSdtDxMPjmgALMljJphw7zw

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
   Inserted with ripple: {"id":"dShvHVlBYOePf1wwbAPYkoIF7py1tNvGzMTx8EI6-ebl8Q20Xpg2ifkyo4KhUfixEzh-QoCUa_91OlX7rI9cPw"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"n0BrLIhoWH_G7KCMGylpY6uD5vAXr154txAWruQ1kOWGE2y5geZjbyQyoF_-I5FrBrdo4CYtCut8SL13be53DQ"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"price":{"type":"Integer","value":150},"name":{"type":"String","value":"Product 1"},"id":"dShvHVlBYOePf1wwbAPYkoIF7py1tNvGzMTx8EI6-ebl8Q20Xpg2ifkyo4KhUfixEzh-QoCUa_91OlX7rI9cPw"}

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
  Fields returned: ["name","email","id"]
  First user: Bob Smith <bob@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["user_role","status","created_at","avatar_url","age","email","id","name","bio"]

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
    - Fields: ["avatar_url","api_key","age","created_at","bio","email","name","secret_token","id","password","status","user_role"]
  Projected query:
    - 3 fields per record
    - Fields: ["email","id","name"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ All JavaScript integration tests complete!
🟣 Building Kotlin client library...
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
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

BUILD SUCCESSFUL in 6s
11 actionable tasks: 11 up-to-date
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
✅ Kotlin client built!
=== Running Kotlin example: ClientAdmin.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Admin permissions read successfully

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientAdvancedCrud.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Inserted: Zq0YJumT3Pa4oimAE7BGARBoc8VqJ-NYNZHD7OKa_vnp6uK7h9AkN3SN1GQAZNWZa5AuaD0drwl3Q3PNogSSiw

--- updateWithAction: increment views ---
After increment: EkoRecord(fields={name=ObjectValue(value={value=StringValue(value=Counter Record), type=StringValue(value=String)}), id=StringValue(value=Zq0YJumT3Pa4oimAE7BGARBoc8VqJ-NYNZHD7OKa_vnp6uK7h9AkN3SN1GQAZNWZa5AuaD0drwl3Q3PNogSSiw), views=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=15)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example)]), type=StringValue(value=Array)}), score=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=100.0)})})

--- updateWithAction: push to tags ---
After push: EkoRecord(fields={id=StringValue(value=Zq0YJumT3Pa4oimAE7BGARBoc8VqJ-NYNZHD7OKa_vnp6uK7h9AkN3SN1GQAZNWZa5AuaD0drwl3Q3PNogSSiw), name=ObjectValue(value={value=StringValue(value=Counter Record), type=StringValue(value=String)}), score=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=100.0)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example), StringValue(value=advanced)])}), views=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=15)})})

--- updateWithActionSequence: multiple operations ---
After sequence: EkoRecord(fields={tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example), StringValue(value=advanced), StringValue(value=sequenced)])}), score=ObjectValue(value={value=FloatValue(value=75.0), type=StringValue(value=Float)}), name=ObjectValue(value={value=StringValue(value=Counter Record), type=StringValue(value=String)}), id=StringValue(value=Zq0YJumT3Pa4oimAE7BGARBoc8VqJ-NYNZHD7OKa_vnp6uK7h9AkN3SN1GQAZNWZa5AuaD0drwl3Q3PNogSSiw), views=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=115)})})

--- Cleanup ---
Deleted collection: kotlin_advanced_crud_example

=== Example Complete ===

BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientBatchOperations.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
  IDs: _EN2Vbc3tbahQVIQomJ0Yebi4j-ZLTYucR2sM5D2pDuPi9DfzZ9ncCqh5gXCTCc6chbhvkNjqoC3XwUIzxVSSg, 82R4uKzPFfUPXI28nOqBJOWTluEsyJBN4ddeQtOL_pTHlnMKPdQgcRHMrh9-Z2EW2WJjhMl-NRKqEeSZ1Ge0jw, AaZUaVh2sXLJycodG20jVqs7tyfNbhVXnmQmQAI0dqQl7EuZMSwb9wbzItQA-XyoK4sSrRJJA-a8CQ6w4EVgjA...

=== Batch Update ===
✓ Updated 3 records; 0 failed

=== Batch Delete ===
✓ Deleted 2 records

=== Cleanup ===
✓ Deleted collection: kotlin_batch_example

=== Example Complete ===

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientChatAdvanced.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Created session: Hc_JFEIUFWXKJnefaeGSOYrutQzDPcRJkWr98rfCx3Nyc1F8gHyZdrhqvHfpcAB0xV4PRKJsI06hTAD46RmkKA

=== Sending Initial Message ===
✓ Message sent
  Responses: ["It seems that there are currently no available products in the database. Would you like to add any products or check for something else?"]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["The price of ekoDB is $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: ntGUyB3d9JLMpimjXDKa-XPuqNEkoP0T_ufVrqkngG8uZmIxfcn4_mjweX8Bk_n3UEyqFHfQxc2gMUNzx5RCGA

=== Merging Sessions ===
✓ Merged sessions
  Total messages in merged session: 5

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat session: ntGUyB3d9JLMpimjXDKa-XPuqNEkoP0T_ufVrqkngG8uZmIxfcn4_mjweX8Bk_n3UEyqFHfQxc2gMUNzx5RCGA
✓ Deleted chat session: Hc_JFEIUFWXKJnefaeGSOYrutQzDPcRJkWr98rfCx3Nyc1F8gHyZdrhqvHfpcAB0xV4PRKJsI06hTAD46RmkKA
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 17s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientChatBasic.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Created session: BBiND7jyKwbScAnnayo8dsZ0xGe3H0GW80W44XI0QZw9Rz93DxN1CxdmPbgWE0bvtIiV38NwTADgt_GmOVB_0A

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "Qp0u69YHzmXtF867nWO__LBJxNazI7Q5Si-KeQu0ewTxgZnAhUZa_AVFcMJHs5a98vv0SoHSIXRRauVw6J5jzw"
  Responses: ["ekoDB is a high-performance database that integrates intelligent caching and real-time capabilities along with AI features. It allows users to utilize advanced search options including full-text search, vector search, and hybrid search with automatic context retrieval. \n\n### Key Features of ekoDB:\n1. **AI Chat Integration:** ekoDB supports a chat feature that enables users to query the database using natural language, providing AI-powered responses that include relevant context.\n2. **Search Capabilities:** It supports various search methods:\n   - **Full-text Search:** Allows for keyword-based querying.\n   - **Vector Search:** Utilizes vector embeddings for semantic searches.\n   - **Hybrid Search:** Combines both keyword and semantic searching for more effective results. \n\nIn summary, ekoDB is designed for high-performance data handling with intelligent search capabilities and seamless AI integration."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 12s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientChatMessageStream.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Created session: TOvlnJ_JnnXG8n81MIqrogGIRrvyuImgQcf9qeCKpTHwbvtzrUMXKJ_RtQ5KE8J5FfSo-enSqKzZX9Zf10ET_g

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}EkoDB is a database management system designed for high-performance and scalability, often tailored for applications that require efficient data handling and fast access times. Its specific features can vary, but it generally emphasizes efficient data storage, retrieval, and management capabilities that cater to modern application needs.

If you're looking for specific functions, architecture details, or use cases of EkoDB, please provide more context, and I'll do my best to give you the information you need!

--- Stream complete ---
Message ID: 6St0GoXKQUWyoqTmWeW1iU6gSHh3CorC9blYDzULKjZNzGardF88it0JGILTGnT61iEdzJDorTeN5cIDSvmFwg
Execution time: 1456ms
Context window: 128000 tokens

✓ Chat message stream example completed

BUILD SUCCESSFUL in 10s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientChatModels.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
  gemini:
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

=== Get OpenAI Models ===
OpenAI models: gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, gpt-4, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-5-chat-latest, gpt-3.5-turbo-0125, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.4-mini, o1-pro, gpt-5.1-codex, tts-1, gpt-5-mini, gpt-5-mini-2025-08-07, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, omni-moderation-latest, gpt-5-2025-08-07, o4-mini-2025-04-16, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-5.4-mini-2026-03-17, babbage-002, gpt-4o-2024-05-13, text-embedding-3-large, gpt-4o-mini-transcribe-2025-12-15, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-audio-mini-2025-10-06, gpt-5.2-pro-2025-12-11, gpt-4-turbo, gpt-4o-2024-08-06, o3, gpt-4o-mini-search-preview-2025-03-11, o3-mini-2025-01-31, gpt-audio-1.5, gpt-image-1.5, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-4.1-nano, gpt-image-2, gpt-3.5-turbo-16k, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-4-0613, gpt-5.1-2025-11-13, gpt-4o-search-preview, gpt-4.1-mini, gpt-4o-mini, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-5.1-codex-max, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-3.5-turbo-instruct-0914, o1-2024-12-17, sora-2, tts-1-hd-1106, gpt-4o-mini-tts-2025-12-15, gpt-image-1, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-5.2-chat-latest, sora-2-pro, gpt-3.5-turbo-instruct, gpt-transcribe, gpt-4.1-nano-2025-04-14, o4-mini-deep-research, gpt-5.4, gpt-4o-search-preview-2025-03-11, text-embedding-3-small, gpt-5.1-chat-latest, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-5.3-chat-latest, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, gpt-3.5-turbo, gpt-5.2-codex, gpt-4o, davinci-002, gpt-5.4-nano, o3-mini, o4-mini-deep-research-2025-06-26, gpt-audio-mini-2025-12-15, o1-pro-2025-03-19, gpt-5-nano-2025-08-07, gpt-4o-transcribe, o4-mini, gpt-5.1-codex-mini, gpt-3.5-turbo-1106, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, o1, gpt-image-2.5-sunburst-2026-09-08, gpt-6-luna, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-image-2.5-flare, gpt-4o-mini-search-preview, gpt-6-sol, gpt-6-astra, gpt-6.1-sol, gpt-live-1, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

=== Chat Models Example Complete ===

BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientChatSessions.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Created session: 87t_ueZy7PrZuoudOIuC38PHl9K1aseSWJJnt9d24ammXaxSiWlj33P0gutVUu8dvBbwaFsjgFLL-wsfmEwLIQ

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["It appears that there are no products available based on the current query filters. Would you like to search using different criteria or explore more details about any specific product?"]

✓ Message 2 sent
  Responses: ["The price of the product \"ekoDB\" is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"87t_ueZy7PrZuoudOIuC38PHl9K1aseSWJJnt9d24ammXaxSiWlj33P0gutVUu8dvBbwaFsjgFLL-wsfmEwLIQ"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"qxXYVNgYguRcfrNmLoPb-g5E5OoFDV0i53QwHvsxZn8Yemp9P07sY43iW-x9bes85zbsuixHqIh_msYatZSLew","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-10-20T20:09:08.226492+00:00"},"id":"fs0cW_tzj1rJpam2WQgevCA3y1mz_qvNNgkKbQo5E-eTWAXzzq0pt-t1PF1VZD_eTme8Sf8noTIkh3nAfiztlA","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":83,"prompt_tokens":6671,"total_tokens":6754}},"updated_at":{"type":"DateTime","value":"2026-10-20T20:09:08.226492+00:00"}},{"chat_id":{"type":"String","value":"87t_ueZy7PrZuoudOIuC38PHl9K1aseSWJJnt9d24ammXaxSiWlj33P0gutVUu8dvBbwaFsjgFLL-wsfmEwLIQ"},"content":{"type":"String","value":"It appears that there are no products available based on the current query filters. Would you like to search using different criteria or explore more details about any specific product?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"qxXYVNgYguRcfrNmLoPb-g5E5OoFDV0i53QwHvsxZn8Yemp9P07sY43iW-x9bes85zbsuixHqIh_msYatZSLew","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-10-20T20:09:08.237163+00:00"},"id":"B8PDDHGhtl-n9b888Az5IHVUtXIcfKWDfV3FRj048KbRqV2YD3xJhpdPbrEHULvNTrFCNaP-x2Dik8CWnJ37Dg","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":83,"prompt_tokens":6671,"total_tokens":6754}},"tool_call_count":{"type":"Number","value":2},"tool_call_history":{"type":"Object","value":{"iterations":2,"tool_calls":[{"arguments":{"collection":"kotlin_chat_sessions_example","filter":{"content":{"field":"__rag_matched","operator":"Eq","value":"description"},"type":"Condition"}},"id":"call_JwO4u7yQZg3fYqgFFGAyeqqI","name":"query_collection"}],"tool_results":[{"error":null,"result":{"count":0,"records":[]},"success":true,"tool_call_id":"call_JwO4u7yQZg3fYqgFFGAyeqqI","tool_name":"query_collection"}]}},"updated_at":{"type":"DateTime","value":"2026-10-20T20:09:08.237163+00:00"}},{"chat_id":{"type":"String","value":"87t_ueZy7PrZuoudOIuC38PHl9K1aseSWJJnt9d24ammXaxSiWlj33P0gutVUu8dvBbwaFsjgFLL-wsfmEwLIQ"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"qxXYVNgYguRcfrNmLoPb-g5E5OoFDV0i53QwHvsxZn8Yemp9P07sY43iW-x9bes85zbsuixHqIh_msYatZSLew","price":99,"product":"ekoDB"},"score":0.7157894736842105}]},"created_at":{"type":"DateTime","value":"2026-10-20T20:09:09.467195+00:00"},"id":"MpVEfKLvtqLUC_AIuxLscFX71089P1r8egSKLzacuNvqxl5KVOwlkyI5qgN6-tqPxVc4lDmlsla7SM4yRy7Bqw","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":10,"prompt_tokens":3378,"total_tokens":3388}},"updated_at":{"type":"DateTime","value":"2026-10-20T20:09:09.467195+00:00"}},{"chat_id":{"type":"String","value":"87t_ueZy7PrZuoudOIuC38PHl9K1aseSWJJnt9d24ammXaxSiWlj33P0gutVUu8dvBbwaFsjgFLL-wsfmEwLIQ"},"content":{"type":"String","value":"The price of the product \"ekoDB\" is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"qxXYVNgYguRcfrNmLoPb-g5E5OoFDV0i53QwHvsxZn8Yemp9P07sY43iW-x9bes85zbsuixHqIh_msYatZSLew","price":99,"product":"ekoDB"},"score":0.7157894736842105}]},"created_at":{"type":"DateTime","value":"2026-10-20T20:09:09.478855+00:00"},"id":"R6EQPlXqK_YetSUYEHhcG7J_Rwk6oKyjvwUaSlFyqKRitar-TBjhIrHu1pYPn4Mr31bWpV_6PxzZafDcw0eDAA","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":10,"prompt_tokens":3378,"total_tokens":3388}},"updated_at":{"type":"DateTime","value":"2026-10-20T20:09:09.478855+00:00"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 1

=== Branching Session ===
✓ Created branched session: 1EmW4LP3NLnIyocaYdN-mf2-GQczbl34Utd7RwNO48AltQHVNOdK3YpncpcaTHlvOaWXBCasU1CqbGKBR3wsiA

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 15s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientCollectionManagement.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
  - schema_employees_client_ts
  - chat_goals__ek0_testing
  - schema_documents_client_js
  - chat_agent_configs__ek0_testing
  - chat_tasks__ek0_testing

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-10-20T20:09:19.310042Z","last_modified":"2026-10-20T20:09:19.310042Z","bypass_ripple":false,"primary_key_alias":"id"}

=== Cleanup ===
✓ Deleted collection: kotlin_collection_example

=== Example Complete ===

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientCollectionUtils.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Collection metadata: {"analytics":["collection_utils_test_kt",{"manifest_backed":false,"record_count":5,"resident_record_ids":5,"total_size":675}],"collection":{"bypass_ripple":false,"created_at":"2026-10-20T20:09:28.850700Z","fields":{"index":{"default":null,"enums":[],"field_type":"Integer","max":null,"min":null,"regex":null,"required":false,"unique":false},"name":{"default":null,"enums":[],"field_type":"String","max":null,"min":null,"regex":null,"required":false,"unique":false}},"last_modified":"2026-10-20T20:09:28.851048Z","primary_key_alias":"id","version":1}}

=== List Collections ===
All collections (27):
  - schema_employees_client_ts
  - chat_goals__ek0_testing
  - schema_documents_client_js
  - collection_utils_test_kt
  - chat_agent_configs__ek0_testing
  - chat_tasks__ek0_testing
  - test_accounts
  - schema_employees_client_go
  - schema_documents_client_ts
  - schema_users_client_ts
  - schema_users_client_go
  - chat_goal_templates__ek0_testing
  - schema_employees_client_js
  - agent_function_versions__ek0_testing
  - chat_configurations__ek0_testing
  - schema_documents_client_go
  - chat_raw_completions__ek0_testing
  - functions__ek0_testing
  - schedules__ek0_testing
  - chat_turns__ek0_testing
  - test_collection
  - schema_products_client_js
  - schema_users_client_js
  - audit__ek0_testing
  - schema_products_client_go
  - chat_messages__ek0_testing
  - schema_products_client_ts

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz_2148977175491041' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_kt'

=== Collection Utilities Example Complete ===

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientConcurrencyStages.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientConvenienceMethods.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Created record: EkoRecord(fields={id=StringValue(value=fkcSkg4A9X94uICqTDuRE27an0dAL8ucYc9eZ3qc3xRi6N-dG19XsNeoGKwLSIaq8GvYSwiC5NEdT41IE-G7yw)})

=== Upsert Operation ===
✓ First upsert (update): EkoRecord(fields={email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice.j@newdomain.com)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), id=StringValue(value=fkcSkg4A9X94uICqTDuRE27an0dAL8ucYc9eZ3qc3xRi6N-dG19XsNeoGKwLSIaq8GvYSwiC5NEdT41IE-G7yw), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)})})
✓ Second upsert (insert): EkoRecord(fields={id=StringValue(value=new-user-id)})

=== Find One Operation ===
✓ Found user by email: EkoRecord(fields={id=StringValue(value=fkcSkg4A9X94uICqTDuRE27an0dAL8ucYc9eZ3qc3xRi6N-dG19XsNeoGKwLSIaq8GvYSwiC5NEdT41IE-G7yw), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice.j@newdomain.com)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)})})
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

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientCryptoStages.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientDistinctValues.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientDocumentTtl.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
  Document ID: CFJwczzpmZhU32THePV089GREewlVUSfqpcSeAjGbc5_9pm2-p7gaU7lHV0PgWD_PBjiVwkfsn1j1_fw-5R-_g

=== Verify Document Exists ===
✓ Document found: id, session_id, user_id, created_at, ttl

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: cSW24Y3gqM9T2FswOa0yRfvMwGovA5ej540sGEZo6WZp1apFmIwPVs9iqlveKHArwsfnZ-tCDBeFiL0bmh2xSw

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: value, id, cache_key, ttl

=== Delete Document ===
✓ Deleted document

=== Cleanup ===
✓ Deleted collection: kotlin_ttl_example

=== Example Complete ===

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientEdgeCache.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Edge cache function created: IMEX4GN1GttGzwXvrFNyOqRpIZLEV_h1M49zBgZ1UWiaIuTyaRen6ylYbyUM_0wpr_h10YEz9_U7jAS1xO45wg

Call 1: Cache lookup
Found 1 cached entries
Response time: 18ms

Call 2: Cache lookup (connection warm)
Found 1 cached entries
Response time: 9ms

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

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionComposition.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
   Name: {"type":"String","value":"User 1"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 77ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 3ms
   📊 Records: 1
   🚀 Cache speedup: 25.7x faster!

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

🧹 Cleaning up...
✅ Cleanup complete

✅ All composition examples completed!

BUILD SUCCESSFUL in 10s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionContract.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
client_function_contract: ok

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctions.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

✅ Function saved: tIcu0oX4mANyLZKIC_rh181RPuecTbQAImM_ITglIA6XTiYiYoQIKwnp9xRY2rpZCKYI2W7oU0waZWiDKHJaxw
📊 Found 5 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 3 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"count":{"value":5,"type":"Integer"},"avg_score":{"type":"Float","value":50.0},"status":{"value":"inactive","type":"String"}}
   {"avg_score":{"value":60.0,"type":"Float"},"status":{"type":"String","value":"active"},"count":{"value":5,"type":"Integer"}}
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

BUILD SUCCESSFUL in 10s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsAdvanced.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
   {"category":{"value":"Electronics","type":"String"},"avg_price":{"value":367.0,"type":"Float"},"count":{"type":"Integer","value":5}}
   {"avg_price":{"value":365.6666666666667,"type":"Float"},"category":{"type":"String","value":"Furniture"},"count":{"value":3,"type":"Integer"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced function examples finished!

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsAi.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

1. **Efficient Similarity Search**: They enable fast and accurate similarity searches by storing embeddings and supporting nearest neighbor algorithms.

2. **High-dimensional Data Handling**: Ideal for managing high-dimensional data, such as images, audio, and text embeddings.

3. **Scalability**: Designed to handle large datasets, allowing for efficient scaling as data grows.

4. **Real-time Processing**: Support for real-time querying, making them suitable for applications like recommendation systems.

5. **Integration with ML Models**: Seamlessly integrate with machine learning workflows, enhancing model deployment and inference.

6. **Flexibility**: Can accommodate various data types and formats, making them versatile for diverse applications.

7. **Improved Search Relevance**: Provide more contextually relevant search results compared to traditional databases.

8. **Support for Advanced Analytics**: Facilitate complex queries and analytics on unstructured data.

Overall, vector databases significantly enhance the capabilities of applications dealing with large-scale, complex data.
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

BUILD SUCCESSFUL in 12s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsComplete.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

✅ Function saved: DrzOFBss4Y30wMaya_DAY2A7kLBOXBjOtvyhItQpzxUCyW1BWaXKNAFHDje_S85LNswvasPoucijb5U_TJ6dig
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

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsCrud.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
   {"status":{"value":"inactive","type":"String"},"count":{"type":"Integer","value":3}}
   {"status":{"value":"active","type":"String"},"count":{"value":7,"type":"Integer"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD function examples finished!

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsKvWrapped.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

✅ Inserted order: StringValue(value=EGvo2MonsX0PQWoplpTcemugIGdQinlf9PtzYTJVtrWRkRTT_GfaUbZxg32gN3aFx1CbrRdGN0aBTmBv9PcKog)
✅ Inserted 2 products

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: DEjao63okRc_EH7rOWKetcLba8PW1qevwWTrsXMeZy109eVXPc7xugJXWpmfw-Fbf2D-RLTLQIz4M0ohlA4AxQ
📊 Created order via function
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"role":"admin","userId":"user_abc"}}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: Q4nYum2llBWSGYccVfFpt5ucdUTaW0G9ep3a0vG8k0wjxXF3pzITWyq0BOORBYJPJGjYQ567kdj16gfNIdANqg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: dUt184HNB1JxEPcPFWlVwx0LN6JRM1zTs19qWZ8HgDlyXVHjck21TLKerX4kXXdVM37B1LHkaXeDmO5QT43i7A
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

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsSearch.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
   1. {"type":"String","value":"Database Design Principles"} ({"value":"Database","type":"String"})
   2. {"value":"Getting Started with ekoDB","type":"String"} ({"value":"Database","type":"String"})
   3. {"type":"String","value":"Introduction to Machine Learning"} ({"value":"AI","type":"String"})
   4. {"value":"Vector Databases Explained","type":"String"} ({"type":"String","value":"Database"})
   5. {"value":"Natural Language Processing","type":"String"} ({"value":"AI","type":"String"})
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"type":"String","value":"Database"},"count":{"type":"Integer","value":3}}
   {"category":{"value":"AI","type":"String"},"count":{"type":"Integer","value":2}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search function examples finished!

BUILD SUCCESSFUL in 10s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientGoalTemplates.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Created template: Data Migration (id: VeKkrc5bnd3TATPRPlYTXZqAd0dSsGDO4Qf8-1eAXmH_jhAIn5XwphJAfyoi-qA2PiBpPeZcd5Yk8y8xmj3EIA)

--- Listing templates ---
Templates: {"count":1,"items":[{"description":{"type":"String","value":"Template for migrating data between schemas"},"id":"VeKkrc5bnd3TATPRPlYTXZqAd0dSsGDO4Qf8-1eAXmH_jhAIn5XwphJAfyoi-qA2PiBpPeZcd5Yk8y8xmj3EIA","steps":{"type":"Array","value":[{"description":"Analyze source schema"},{"description":"Create target schema"},{"description":"Migrate records"},{"description":"Validate results"}]},"title":{"type":"String","value":"Data Migration"}}]}

--- Getting template ---
Fetched: {"type":"String","value":"Data Migration"}

--- Updating template ---
Updated description: {"type":"String","value":"Updated: comprehensive data migration workflow"}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed

BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientGoalsTasksAgents.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Created goal: _6QjemzdQ8-olPU889KtRfQ7Rn8drSCzCxdo2uRYY5B5yHK-2mT1VroxDwGiMNqpU98-HTcf0WPeKSSg8ejFWA

--- Listing goals ---
Goals: {"count":1,"goals":[{"created_at":"2026-10-20T20:12:23.972009+00:00","description":"Ship the next major release","id":"_6QjemzdQ8-olPU889KtRfQ7Rn8drSCzCxdo2uRYY5B5yHK-2mT1VroxDwGiMNqpU98-HTcf0WPeKSSg8ejFWA","status":"pending","steps":"[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to production\"}]","title":"Deploy v2.0","updated_at":"2026-10-20T20:12:23.972009+00:00"}]}

--- Getting goal ---
Fetched: {"type":"String","value":"Deploy v2.0"}

--- Updating goal ---
Updated description: {"type":"String","value":"Ship v2.0 with full test coverage"}

--- Searching goals ---
Search results: {"count":1,"items":[{"_score":12.870000000000001,"created_at":{"type":"DateTime","value":"2026-10-20T20:12:23.972009+00:00"},"description":{"type":"String","value":"Ship v2.0 with full test coverage"},"id":"_6QjemzdQ8-olPU889KtRfQ7Rn8drSCzCxdo2uRYY5B5yHK-2mT1VroxDwGiMNqpU98-HTcf0WPeKSSg8ejFWA","status":{"type":"String","value":"pending"},"steps":{"type":"String","value":"[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0"},"updated_at":{"type":"DateTime","value":"2026-10-20T20:12:23.995448+00:00"}}]}

--- Goal step lifecycle ---
Step 0 started: "_6QjemzdQ8-olPU889KtRfQ7Rn8drSCzCxdo2uRYY5B5yHK-2mT1VroxDwGiMNqpU98-HTcf0WPeKSSg8ejFWA"
Step 0 completed: "_6QjemzdQ8-olPU889KtRfQ7Rn8drSCzCxdo2uRYY5B5yHK-2mT1VroxDwGiMNqpU98-HTcf0WPeKSSg8ejFWA"
Step 1 started: "_6QjemzdQ8-olPU889KtRfQ7Rn8drSCzCxdo2uRYY5B5yHK-2mT1VroxDwGiMNqpU98-HTcf0WPeKSSg8ejFWA"
Step 1 failed: "_6QjemzdQ8-olPU889KtRfQ7Rn8drSCzCxdo2uRYY5B5yHK-2mT1VroxDwGiMNqpU98-HTcf0WPeKSSg8ejFWA"

--- Completing goal ---
Goal status after complete: {"type":"String","value":"pending_review"}

--- Approving goal ---
Goal status after approve: {"type":"String","value":"in_progress"}

--- Creating goal for rejection ---
Goal status after reject: {"type":"String","value":"failed"}

--- Creating task ---
Created task: tbtxHlmoE9ZOBpchWuS7B9sMdQsjiAd-ZwYypk9gh_giA_alIJYkqv1Qx3rLamwAAky_tgG7ZVXOTOK2O1NdPg

--- Listing tasks ---
Tasks: {"count":1,"items":[{"cron":{"type":"String","value":"0 2 * * *"},"description":{"type":"String","value":"Full database backup every night at 2 AM"},"id":"tbtxHlmoE9ZOBpchWuS7B9sMdQsjiAd-ZwYypk9gh_giA_alIJYkqv1Qx3rLamwAAky_tgG7ZVXOTOK2O1NdPg","name":{"type":"String","value":"Nightly Backup"}}]}

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
Created agent: 8Mi0RgBe-dhuoQYzoiyKnDP8MxM4K8fS4nbQX2o9bAg6zBvuY2vAwt_fkX7upSjDGlsydYxMoqzQtFB06ptHLw — null

--- Listing agents ---
Agents: {"count":1,"items":[{"deployment_id":{"type":"String","value":"deploy_kt_example"},"id":"8Mi0RgBe-dhuoQYzoiyKnDP8MxM4K8fS4nbQX2o9bAg6zBvuY2vAwt_fkX7upSjDGlsydYxMoqzQtFB06ptHLw","llm_model":{"type":"String","value":"gpt-4.1"},"name":{"type":"String","value":"DataBot"},"system_prompt":{"type":"String","value":"You are a data analysis assistant."}}]}

--- Getting agent by ID ---
Agent: {"type":"String","value":"DataBot"}

--- Getting agent by name ---
By name: {"type":"String","value":"DataBot"}

--- Updating agent ---
Updated agent system_prompt

--- Agents by deployment ---
Agents for deployment: {"count":0,"items":[]}
WARNING: agents-by-deployment omitted created agent 8Mi0RgBe-dhuoQYzoiyKnDP8MxM4K8fS4nbQX2o9bAg6zBvuY2vAwt_fkX7upSjDGlsydYxMoqzQtFB06ptHLw; TODO: check/fix the server-side deployment lookup

--- Deleting agent ---
Agent deleted

--- Cleanup: deleting goals ---
Goals deleted

=== Example Complete ===

BUILD SUCCESSFUL in 10s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientJoins.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientJwtAuthFlow.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientKvLinks.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Links for user:alice: [{"collection":"orders","document_id":"order_002","field_path":null,"created_at":"2026-10-20T20:12:54.649053Z","last_accessed":"2026-10-20T20:12:54.657256Z","metadata":{}},{"collection":"invoices","document_id":"inv_100","field_path":null,"created_at":"2026-10-20T20:12:54.653041Z","last_accessed":"2026-10-20T20:12:54.657256Z","metadata":{}},{"collection":"orders","document_id":"order_001","field_path":null,"created_at":"2026-10-20T20:12:54.641679Z","last_accessed":"2026-10-20T20:12:54.657256Z","metadata":{}}]

--- Unlinking document ---
Unlinked order_002: null

--- Verifying remaining links ---
Remaining links: [{"collection":"orders","document_id":"order_001","field_path":null,"created_at":"2026-10-20T20:12:54.641679Z","last_accessed":"2026-10-20T20:12:54.664437Z","metadata":{}},{"collection":"invoices","document_id":"inv_100","field_path":null,"created_at":"2026-10-20T20:12:54.653041Z","last_accessed":"2026-10-20T20:12:54.664437Z","metadata":{}}]

--- Cleanup ---
Unlinked remaining documents
Deleted key: user:alice

=== Example Complete ===

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientKvOperations.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Retrieved value: {"value":{"email":"alice@example.com","name":"Alice","role":"admin"},"type":"Object"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"value":{"created_at":1792527184736,"user_id":"123"},"type":"Object"}
  (Will expire in 10 seconds)

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_ops_kt_1792527184596:config:db: success
  kv_ops_kt_1792527184596:config:cache: success
  kv_ops_kt_1792527184596:config:api: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_kt_1792527184596:config:db: {"host":"localhost","port":5432}
  kv_ops_kt_1792527184596:config:cache: {"ttl":3600,"enabled":true}
  kv_ops_kt_1792527184596:config:api: {"timeout":30,"retries":3}

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
  kv_ops_kt_1792527184596:config:db: deleted
  kv_ops_kt_1792527184596:config:cache: deleted
  kv_ops_kt_1792527184596:config:api: deleted

=== Example Complete ===

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientKvPrecision.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientPathRoutedFunction.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientProjection.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientQueryBuilder.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
  - Score: ObjectValue(value={value=IntegerValue(value=95), type=StringValue(value=Integer)})
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=92)})
  - Score: ObjectValue(value={value=IntegerValue(value=88), type=StringValue(value=Integer)})

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

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientRawCompletionStream.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
    "name": "Venus",
    "diameter_km": 12104
  },
  {
    "name": "Earth",
    "diameter_km": 12742
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===

BUILD SUCCESSFUL in 11s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSchedules.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Created schedule: 707ad563-b078-488f-9f49-cdd3f2910203 — "Hourly Health Check"

--- Listing schedules ---
Schedules: {"count":1,"schedules":[{"created_at":"2026-10-20T20:14:06.348376Z","cron_expression":"0 0 * * * *","description":"Ping all services every hour","enabled":true,"function_label":"schedule_noop_kotlin_58353_1792527246207","id":"707ad563-b078-488f-9f49-cdd3f2910203","last_execution":null,"name":"Hourly Health Check","next_execution":"2026-10-20T21:00:00Z","parameters":{},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-10-20T20:14:06.348376Z"}]}

--- Getting schedule ---
Fetched: "Hourly Health Check" (cron: "0 0 * * * *")

--- Updating schedule ---
Updated cron: "0 */30 * * * *"

--- Triggering schedule ---
Trigger response: {"schedule_id":"707ad563-b078-488f-9f49-cdd3f2910203","status":"triggered"}

--- Pausing schedule ---
Enabled after pause: false

--- Resuming schedule ---
Enabled after resume: true

--- Deleting schedule ---
Schedule deleted successfully

=== Example Complete ===

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSchemaManagement.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Inserted user 1: StringValue(value=NLaancsbWEZnzOkHsmYFhuC7MLpAfEmy684GLDJ3ZbUWb07CEH-HPZfZvcGseIUAncF8E5otTOe8IQm2DfKUxA)
✓ Inserted user 2: StringValue(value=KPnT3RK0VTvY26pTJAZrEAYO28UbiKOOVsS9_nMSedAjQnnClwuvwuMCXqdsPfJXI9UZBbZMfAKKXVSnvH3kPg)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 27
  Sample: [schema_employees_client_ts, chat_goals__ek0_testing, schema_documents_client_js, chat_agent_configs__ek0_testing, chat_tasks__ek0_testing]

=== Cleanup ===
✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSearch.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Text results: 2; execution time: 0 ms
{"type":"String","value":"Rust Programming Question"}: score=0.9162907318741551, matched=[title]
{"type":"String","value":"Rust Programming Answer"}: score=0.9162907318741551, matched=[title]
Paired vector search: document=Rust Programming Answer, query=Rust Programming Question
Filtered vector results: 2
Custom-weight hybrid results: 3
Raw search: {"results":[{"record":{"id":"DXi57bru5wO5LW9WYewQiun9-ccuEOLlv-dWdVSIzWMDHGXhMy9NVMBchL86a_xp650-8lpin4-yakJbaCwO2w","title":{"value":"Rust Programming Question","type":"String"},"category":{"value":"programming","type":"String"},"query_embedding":{"type":"Vector","value":[1.0,0.0,0.0]},"document_embedding":{"type":"Vector","value":[0.0,1.0,0.0]}},"score":1.0,"matched_fields":[]},{"record":{"title":{"value":"Rust Programming Answer","type":"String"},"query_embedding":{"value":[0.0,1.0,0.0],"type":"Vector"},"document_embedding":{"value":[1.0,0.0,0.0],"type":"Vector"},"id":"vozd1orodN8qez8jhDtrtwyz-iSRN43qP_frWyReptWFYiGDgpbIcKPRdukrjsffpytY7wfgijScs4A2qz3-Lw","category":{"value":"programming","type":"String"}},"score":1.0,"matched_fields":[]}],"total":2,"execution_time_ms":0}

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSimpleCrud.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Inserted user: EkoRecord(fields={id=StringValue(value=OhXkKk7qHiGJmeJ_fcZqB-5z6beyBHrS3ov1UNhLshEQJYr4pKkxcGdR_MU71iJswV-JaJCYDhXj-mZtPFWz5Q)})
  User ID: OhXkKk7qHiGJmeJ_fcZqB-5z6beyBHrS3ov1UNhLshEQJYr4pKkxcGdR_MU71iJswV-JaJCYDhXj-mZtPFWz5Q

=== Read ===
✓ Found user by ID: EkoRecord(fields={categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), age=ObjectValue(value={value=IntegerValue(value=28), type=StringValue(value=Integer)}), embedding=VectorValue(value=[0.1, 0.2, 0.3, 0.4, 0.5]), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), id=StringValue(value=OhXkKk7qHiGJmeJ_fcZqB-5z6beyBHrS3ov1UNhLshEQJYr4pKkxcGdR_MU71iJswV-JaJCYDhXj-mZtPFWz5Q), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), created_at=ObjectValue(value={value=StringValue(value=Tue Oct 20 16:14:35 EDT 2026), type=StringValue(value=String)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Tue Oct 20 16:14:35 EDT 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: categories, metadata, data, price, age, embedding, active, id, email, user_id, tags, created_at, name

=== Update ===
✓ Updated user: EkoRecord(fields={user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), embedding=VectorValue(value=[0.1, 0.2, 0.3, 0.4, 0.5]), id=StringValue(value=OhXkKk7qHiGJmeJ_fcZqB-5z6beyBHrS3ov1UNhLshEQJYr4pKkxcGdR_MU71iJswV-JaJCYDhXj-mZtPFWz5Q), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Tue Oct 20 16:14:35 EDT 2026)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})})})

=== Query ===
✓ Found 1 users matching query
  - EkoRecord(fields={data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), embedding=VectorValue(value=[0.1, 0.2, 0.3, 0.4, 0.5]), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), created_at=ObjectValue(value={value=StringValue(value=Tue Oct 20 16:14:35 EDT 2026), type=StringValue(value=String)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), id=StringValue(value=OhXkKk7qHiGJmeJ_fcZqB-5z6beyBHrS3ov1UNhLshEQJYr4pKkxcGdR_MU71iJswV-JaJCYDhXj-mZtPFWz5Q), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)})})

=== Delete ===
✓ Deleted user with ID: OhXkKk7qHiGJmeJ_fcZqB-5z6beyBHrS3ov1UNhLshEQJYr4pKkxcGdR_MU71iJswV-JaJCYDhXj-mZtPFWz5Q

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

=== Example Complete ===

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSimpleWebsocket.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
  {"data":[{"name":{"type":"String","value":"Test User"},"status":{"value":"active","type":"String"},"id":"P15UPIGwDxVxB-SWIGguUTQ_MKIGeR8VZ9YwvdbY5lLiZZH54414-ZRx_e0Wdxy8DA0NmU9_lKp2Rz7JeM_d5g"}]}

=== Close WebSocket ===
✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_example

=== Example Complete ===

BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSwrNative.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Created native SWR function: github_user_native_kt (7-1AEKj7dFQF-yjI_lms8jiXGGwBQhBrg-0ywh6CM8CeQa4fV8CRDuhYd3LZLyfFdbhRZcRj3uYq0u5EjDYQSA)

First call (cache miss - will fetch from GitHub API):
  Response time: 180ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 60.0x faster 🚀


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR function with audit trail: product_swr_audit_kt (mROakmssOUK5dvPwtmCcggJiCXHKhxcV3ntR_Qpnzt1z9KHdQKdo5swd614j7zyaDFyUwp6DU_MpolqE9lXaTw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_kt' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_kt (sRgcuMTeO08jKtviijSCUZClGCeJU4JpHIKb0L6fsZfbm-S-dyjyjFQLz1jgEHPN0KKceBJpb4N2q-xm7hKbtw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_kt' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL function: flexible_cache_kt (BPjKyZIUKOleba3ulrPDBwDpiBwZMgsVWp9dbnWJn7gObbYMasDTv0sHiQ9JZpXDaZDt5eCBHJ5zYHbaac4hHw)
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

BUILD SUCCESSFUL in 10s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSwrPattern.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Created SWR function: swr_cache_lookup_kt_1792527306645 (rJLj-gvI7LVxhAoCLwCxMYizk2z9byxlUFG0iQWK1NpPZFetYHQXZIb30jHM2m2YWNdFn60hQso50DyzswO31g)

Step 3: First call - Cache lookup
Found 1 cached entries
✓ Cache lookup complete

Step 4: Second call - Fast cache hit
Response time: 9ms (served from cache)
✓ Lightning fast cache hit

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🧹 Cleaning up...
✓ Cleanup complete


BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientTransactions.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Created Alice: $1000 - ID: nAiKAKym_IqCu_eW9zHxumGmr5i38VOR_RDnMrZ9t72pj6peimD-xULE1xx_XU7CxA969ueB0ePWHiVjZqKvAg
Created Bob: $500 - ID: KaX2pzqRgsyLLdfePw6Y-XL9hl0N7jX5I4oh3Z6TNvEwe_C8JvG1pmuaoaYQj-9w1gEt79drG8p2IK6de-2d6Q

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 1944d4e3-5714-4206-93f0-e0273fe271b8

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
New transaction: 63b0e99b-9baf-4dc0-9ce9-0ac932cf75e7
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientUserFunctions.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Created user function with ID: 4IoHqPIHej_4X0SSIcbWplKQzJ5h_kRlTPdQX0yMCq3cNtMjb_ngCD5K2M5S6zWLxm7ym93OrSs35sPvttNv_A

=== Get User Function ===
Retrieved: "get_active_users_kt" - "Get Active Users (Kotlin)"
Description: "Fetches all users and filters by active status"

=== List All User Functions ===
Found 11 user functions:
  - "conc_demo_rl_skip_ts_12240_1792526703280": "Rate-limit (skip mode)"
  - "get_active_users_kt": "Get Active Users (Kotlin)"
  - "conc_demo_rl_fail_ts_12240_1792526703280": "Rate-limit (fail mode)"
  - "get_active_users_client_js": "Get Active Users (Updated)"
  - "conc_demo_rl_fail_ts_70491_1792526834418": "Rate-limit (fail mode)"

=== List User Functions by Tag ===
Found 1 user functions with 'kotlin' tag:
  - "get_active_users_kt"

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

=== User Functions Example Complete ===

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientWebsocketChatStream.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

Created chat session: oTycZH3_E2tFTvgfo8GIoYcQ_ayRSgjaN3IvD5PKsRfy1tnESrxPwvHo1x43tRBZU_bUveXB3yG5TW7Mfw5vIQ

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: 462LGn6Gtmh2I8SvQKpAiG3rRw4s8FYHP3ter52LrYQ0Zh6sTJJldIpVSFu8DSIMNrRSqjB_ALbsMBjA6nK8Jw
Execution time: 739ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris....

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientWebsocketSubscribe.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Subscribed (subscription_id: sub_ccf9e5b2e4314242ad52ca88e1a23ad3)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "BR4PlFlEtsffIsHFlV26NyZpLWnulBsG8fOC8WSxGNFvOXsk_eA3wUE0dLaYm6omS7tX7dFYrBpse3WHYhhxyw"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_kt"
     Record IDs: ["BR4PlFlEtsffIsHFlV26NyZpLWnulBsG8fOC8WSxGNFvOXsk_eA3wUE0dLaYm6omS7tX7dFYrBpse3WHYhhxyw"]
     Timestamp:  "2026-10-20T20:15:47.135964+00:00"

Inserting record 2...
✓ Inserted: "Lc9sMJ3u48KOfWDPChTh_FWQAdM4CHMWoaWwQmiW4mwtaYwQIolyNgGn94_3zfPsVDdxh6ysWNUbXLwt19SkSA"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["Lc9sMJ3u48KOfWDPChTh_FWQAdM4CHMWoaWwQmiW4mwtaYwQIolyNgGn94_3zfPsVDdxh6ysWNUbXLwt19SkSA"]

=== Unsubscribing ===
✓ Unsubscribed: {"payload":{"data":{"collection":"ws_subscribe_example_kt","found":true,"unsubscribed":true}},"type":"Success"}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_kt'

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientWebsocketTtl.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Inserted document with TTL: StringValue(value=dKdoqyMs9QGNS8aHoZIyMiYsuvcMiw0HGLIH4tERbDldrxZA4uAJbZiGOVjbLSNhnVWXizYPuYK8rsyIk_deSw)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"data":[{"created_at":{"type":"Integer","value":1792527356273},"value":{"type":"Integer","value":42},"id":"dKdoqyMs9QGNS8aHoZIyMiYsuvcMiw0HGLIH4tERbDldrxZA4uAJbZiGOVjbLSNhnVWXizYPuYK8rsyIk_deSw","name":{"value":"WebSocket TTL Test","type":"String"},"ttl":"2026-10-20T21:15:56.398292Z"}]}

✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_ttl_example

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration

BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: BypassRippleExample.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
   Inserted with ripple: EkoRecord(fields={id=StringValue(value=gpzBwaCXqUtTuB0Hsq9AK4pheREcrOe0F-s9o3XZmqbsVl3M5ewDqwcK-HyMbQipfwwA2qVEI40l9sjq2ncOnA)})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=DiGo0vfYAZDlkfjLgTmV16r1TfoCz9dER0Y5w0QJy68tj3v3pH_XAhYWxvZetUs0b2lUHrEjIEMVT6q4yUie-w)})

3. Update with bypass_ripple:
   Updated with bypass_ripple: EkoRecord(fields={price=ObjectValue(value={value=IntegerValue(value=150), type=StringValue(value=Integer)}), name=ObjectValue(value={value=StringValue(value=Product 1), type=StringValue(value=String)}), id=StringValue(value=gpzBwaCXqUtTuB0Hsq9AK4pheREcrOe0F-s9o3XZmqbsVl3M5ewDqwcK-HyMbQipfwwA2qVEI40l9sjq2ncOnA)})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: EkoRecord(fields={price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=500)}), id=StringValue(value=gpzBwaCXqUtTuB0Hsq9AK4pheREcrOe0F-s9o3XZmqbsVl3M5ewDqwcK-HyMbQipfwwA2qVEI40l9sjq2ncOnA), name=ObjectValue(value={value=StringValue(value=Upsert Product), type=StringValue(value=String)})})

✅ All bypass_ripple operations completed successfully!

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
✅ All Kotlin integration tests complete!
🐍 Building Python client package...
🔧 Ensuring maturin is available in .venv...
🔨 Building wheel...
🍹 Building a mixed python/rust project
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.17s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.30.1-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.30.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.30.1
    Uninstalling ekodb_client-0.30.1:
      Successfully uninstalled ekodb_client-0.30.1
Successfully installed ekodb-client-0.30.1
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
    ✓ Generated embedding: 1536 dimensions in 481.084542ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 290.527666ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 341.709084ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 244.544792ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 278.892083ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 445.423458ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 245.197041ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 208.341458ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 466.821083ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 279.198ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 331.655292ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 711.088167ms
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
    ✓ Generated embedding: 1536 dimensions in 218.290709ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages_rust
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 27.699584ms

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

Writing memory-safe, high-performance database code involves several best practices spanning multiple aspects of software development, database design, and performance optimization. Here’s a comprehensive overview to guide you:

### 1. Choose the Right Database
   - **Understand your use case**: Choose between SQL (e.g., Postgres, MySQL) and NoSQL (e.g., MongoDB, Redis) based on your data structure and access patterns.
   - **Consider database performance**: Look into the database's performance benchmarks and features like indexing, caching, and scalability.

### 2. Use Prepared Statements
   - **Security**: Prepare statements to protect against SQL injection attacks by preventing malformed or malicious input.
   - **Performance**: Reusing prepared statements can be more efficient than creating new statements each time.

### 3. Optimize Queries
   - **Indexing**: Create indexes on columns frequently used in WHERE clauses or JOIN statements to speed up queries.
   - **Query Design**: Write efficient queries by:
     - Limiting the number of records retrieved (use pagination).
     - Selecting only necessary fields.
     - Avoiding SELECT * statements.
   - **Analyze Execution Plans**: Use query analyzers (like `EXPLAIN` in SQL) to understand how your queries are executed and where optimizations can be made.

### 4. Memory Management
   - **Connection Pooling**: Use connection pooling to reuse database connections instead of opening new ones, which can be resource-intensive.
   - **Batch Operations**: For inserts or updates, batch multiple operations in a single command to reduce the number of database calls.

### 5. Error Handling
   - **Graceful Degradation**: Implement robust error handling to manage database connection failures or query errors gracefully.
   - **Logging**: Use logging to track errors and performance issues without leaking sensitive data.

### 6. Proper Resource Management
   - **Close Connections**: Ensure that database connections, cursors, and transactions are properly closed or released after use to prevent resource leaks.
   - **Timeouts**: Set appropriate timeouts for queries and connection attempts to avoid indefinitely hanging requests.

### 7. Data Access Layer
   - **Abstract Data Access**: Use a Data Access Layer (DAL) to encapsulate database logic and facilitate easier testing, maintenance, and changes to the database in the future.
   - **ORM vs. Raw Queries**: Decide when to use an ORM (like Entity Framework, Hibernate) versus raw SQL queries. ORMs offer usability benefits but might introduce performance overhead.

### 8. Monitor and Tune Performance
   - **Performance Monitoring**: Use database monitoring tools to track slow queries, deadlocks, and overall database health.
   - **Regular Tuning**: Periodically review indexes, queries, and data models to adapt to changing usage patterns.

### 9. Utilize Transactions Wisely
   - **Atomic Operations**: Ensure that your data changes are atomic where necessary (use transactions to group operations).
   - **Isolation Levels**: Understand transaction isolation levels (like Read Uncommitted, Read Committed, Repeatable Read, Serializable) to balance performance and consistency.

### 10. Data Normalization vs. Denormalization
   - **Normalization**: Normalize data to reduce redundancy, but balance this with performance needs—sometimes a denormalized approach can improve read performance.
   - **Caching Strategies**: Consider caching frequently accessed data to reduce the load on your database.

### Example Code Snippet (SQL with Prepared Statements)
```python
import sqlite3

def fetch_user_by_id(user_id):
    conn = sqlite3.connect('example.db')
    try:
        cursor = conn.cursor()
        cursor.execute("SELECT name, email FROM users WHERE id = ?", (user_id,))
        user = cursor.fetchone()
        return user
    except sqlite3.Error as e:
        print(f"Database error: {e}")
    finally:
        conn.close()
```

### Conclusion
By combining these practices, you'll be able to write memory-safe, high-performance database code that effectively meets your application's needs while ensuring security and reliability. Regularly revisiting your code and architecture as the application grows will also help maintain performance over time.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 288.186709ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 4293 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 313.180375ms
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
  ✓ Text search completed in 22.331792ms

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

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
    ✓ Generated embedding: 1536 dimensions in 0.271s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.274s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.221s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.348s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.210s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.267s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.312s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.245s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.225s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.230s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.297s
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
    ✓ Generated embedding: 1536 dimensions in 0.352s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages_py
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.020s

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

Writing memory-safe, high-performance database code involves several best practices and techniques that focus on both efficient data handling and thread safety. Here are some key strategies:

### 1. **Use Prepared Statements:**
   - Prepared statements help prevent SQL injection and improve performance by allowing the database to reuse execution plans. They also provide automatic handling of data types.

### 2. **Connection Pooling:**
   - Instead of creating a new database connection for every request, use a connection pool. This helps reduce overhead and improve response times by reusing active connections.

### 3. **Indexing:**
   - Properly index your database tables to speed up read operations. Use indexes on columns that are frequently used in WHERE clauses and JOINs.

### 4. **Batch Processing:**
   - For bulk insert or update operations, use batch processing to minimize the number of transactions. This reduces the overhead associated with transaction management and increases throughput.

### 5. **Lazy Loading:**
   - Implement lazy loading to defer the fetching of related data until it’s actually needed. This helps improve performance by not loading unnecessary data.

### 6. **Transactions:**
   - Use transactions to ensure data integrity and consistency. Handle transactions carefully to avoid deadlocks and ensure they are as short-lived as possible.

### 7. **Error Handling:**
   - Implement robust error handling and logging to manage exceptions gracefully. This includes managing retries for transient errors and ensuring that errors do not lead to memory leaks.

### 8. **Memory Management:**
   - In languages like C or C++, take care of manual memory management. Use smart pointers or memory management libraries to help prevent memory leaks and dangling pointers.

### 9. **Use ORM Wisely:**
   - While Object-Relational Mapping (ORM) can help manage database interactions, be mindful of the performance overhead. Use raw queries when necessary for performance-critical paths.

### 10. **Profiling and Optimization:**
   - Regularly profile your database queries and application code. Use tools to identify slow queries and optimize them. Pay attention to the query execution plans.

### 11. **Concurrency Control:**
   - Use appropriate locking mechanisms to manage concurrent access to shared data. This could be optimistic or pessimistic locking depending on the use case.

### 12. **Asynchronous Operations:**
   - Implement asynchronous database operations where applicable to improve application responsiveness, especially in web applications.

### 13. **Clean Up Resources:**
   - Ensure all database connections, cursors, and other resources are properly closed after use. This includes using language-specific constructs like `try-with-resources` in Java or similar patterns in other languages.

### Conclusion:
By following these best practices, you can write high-performance, memory-safe database code. Each application may have unique requirements, so it’s also important to continually evaluate and adapt your strategies based on the specific use cases and workloads you encounter.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.382s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3133 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.275s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

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
    ✓ Generated embedding: 1536 dimensions in 0.238s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.320s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.265s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.763s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.266s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.256s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.339s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.300s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.318s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.240s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.389s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.290s
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
    ✓ Generated embedding: 1536 dimensions in 0.260s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages_ts
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.024s
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

Writing memory-safe high-performance database code requires a combination of best practices in programming, database design, and system resource management. Here are some key strategies to achieve this goal:

### 1. **Choose the Right Database Technology**
   - Select a database that fits your use case: SQL databases (like Postgres) for complex queries and data integrity, and NoSQL databases (like MongoDB or DynamoDB) for flexibility and scalability.
   - Use tools like ekoDB for handling backend tasks with built-in optimizations.

### 2. **Database Query Optimization**
   - **Use Indexes**: Create appropriate indexes on frequently queried columns to speed up read operations. Use tools like `query_explain` to analyze and optimize queries.
   - **Limit Data Retrieval**: Use pagination and filtering to limit the amount of data retrieved; avoid fetching unnecessary rows or columns.
   - **Batch Operations**: Instead of executing single insert or update operations, use batch inserts or updates to reduce the number of database calls.
   - **Prepared Statements**: Use prepared statements to prevent SQL injection and improve performance by caching query plans.

### 3. **Data Integrity and Safety**
   - **Transactions**: Use transactions to ensure that a series of database operations either all complete successfully or none do, maintaining data integrity.
   - **Error Handling**: Implement robust error handling, ensuring that your application can gracefully handle database errors without crashing. Always validate input data before processing.

### 4. **Connection Management**
   - **Connection Pooling**: Utilize a connection pool to manage database connections efficiently, minimizing the overhead of establishing new connections.
   - **Close Connections**: Ensure that database connections are closed properly after use to avoid memory leaks.

### 5. **Memory Management**
   - **Use Appropriate Data Types**: Use the most appropriate and smallest data types needed for your application to minimize memory usage.
   - **Limit Object Lifetimes**: For application-level languages, ensure that objects are scoped appropriately and released when no longer needed to free up memory.

### 6. **Concurrency Control**
   - **Use Optimistic/Pessimistic Locking**: Implement locking mechanisms as per the application's needs to avoid conflicting updates.
   - **Isolation Levels**: Select an appropriate transaction isolation level based on the use case to balance performance and data integrity.

### 7. **Load Testing and Performance Analysis**
   - Regularly perform load testing to identify performance bottlenecks in your application as usage patterns change.
   - Use profiling tools to analyze performance and identify memory leaks or excessively high memory usage in your application code.

### 8. **Utilize EkoDB Utilities**
   - Utilize ekoDB features like `create_function` to encapsulate complex database operations, improving code readability and reusability.
   - Implement scheduled tasks using `create_schedule` for regular maintenance tasks and performance optimization in your database.

### Conclusion
By combining the right database technology with optimized query patterns, robust error handling, effective connection management, and memory management techniques, you can develop memory-safe high-performance database code that meets your application’s needs. Regular performance analysis and testing further ensure that your application remains efficient as it scales.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.244s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3502 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.233s
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
  ✓ Text search completed in 0.016s
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
    ✓ Generated embedding: 1536 dimensions in 0.285s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.282s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.260s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.393s
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
    ✓ Generated embedding: 1536 dimensions in 0.308s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.414s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.297s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.270s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.544s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.237s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.250s
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
    ✓ Generated embedding: 1536 dimensions in 0.316s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages_go
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.023s

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

Writing memory-safe, high-performance database code involves several best practices and techniques. Here are key strategies that integrate performance optimizations and safety measures:

### 1. **Choose the Right Database Type**
- **SQL vs. NoSQL**: Select the appropriate database system based on your application's needs. SQL is suited for complex queries and data integrity, while NoSQL is better for flexible schema, scalability, and high write throughput.

### 2. **Use Prepared Statements**
- Using prepared statements helps prevent SQL injection attacks and improves performance by allowing the database to cache the execution plan. This reduces the overhead of query preparation for repeated execution.

### 3. **Connection Pooling**
- Implement connection pooling to manage database connections efficiently. This minimizes the overhead associated with opening and closing connections, which can be resource-intensive.

### 4. **Efficient Query Design**
- **Indexing**: Create indexes on frequently queried fields. This can drastically reduce query execution time but be mindful of the trade-offs, as too many indexes can slow down write operations.
- **Batching**: Use batch transactions for multiple insert/update operations to minimize the transaction overhead and reduce the number of roundtrips to the database.
- **SELECT only necessary fields**: Avoid `SELECT *`; specify only the fields you need to reduce the amount of data processed and transmitted.

### 5. **Transaction Management**
- Use transactions to ensure data integrity and consistency, especially in multi-step operations. Ensure that you handle transactions properly to avoid deadlocks and rollbacks that can affect performance.

### 6. **Error Handling**
- Implement robust error handling for database operations to gracefully handle exceptions, which helps avoid resource leaks and improves overall application stability.

### 7. **Memory Management**
- Avoid memory leaks by properly disposing of resources like database connections, result sets, and statements. Use language features (like `using` in C# or try-with-resources in Java) for automatic resource management.

### 8. **Concurrency Control**
- Use optimistic or pessimistic locking strategies to manage concurrent database access and prevent race conditions.

### 9. **Monitor and Optimize Queries**
- Regularly analyze query performance using execution plans. Tools like `EXPLAIN` in SQL databases can help identify slow queries and suggest indexes or restructuring.
- Monitor database performance metrics, tracking query execution times, cache hit ratios, and resource usage to identify bottlenecks.

### 10. **Asynchronous Operations**
- Where applicable, leverage asynchronous database operations to improve the responsiveness of applications, especially in I/O-bound scenarios.

### 11. **Data Caching**
- Implement caching strategies using in-memory data stores or querying caches to reduce the load on the database and improve response times.

### 12. **Use a Framework or ORM**
- Consider using well-established Object-Relational Mapping (ORM) frameworks that handle many best practices and optimizations for you. However, be cautious of performance overhead and understand how they generate queries.

### 13. **Test for Performance and Security**
- Conduct load testing to evaluate how the database and queries perform under stress.
- Review security best practices regularly, including monitoring query patterns for anomalies that could indicate potential attacks.

Incorporating these practices will help you craft high-performance, memory-safe database code that can handle various workloads effectively.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.317s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3659 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.266s
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
  ✓ Text search completed in 0.016s

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_new_question: Writing memory-safe, high-performance database code involves several best practices and techniques. Here are key strategies that integrate performance optimizations and safety measures:

### 1. **Choose the Right Database Type**
- **SQL vs. NoSQL**: Select the appropriate database system based on your application's needs. SQL is suited for complex queries and data integrity, while NoSQL is better for flexible schema, scalability, and high write throughput.

### 2. **Use Prepared Statements**
- Using prepared statements helps prevent SQL injection attacks and improves performance by allowing the database to cache the execution plan. This reduces the overhead of query preparation for repeated execution.

### 3. **Connection Pooling**
- Implement connection pooling to manage database connections efficiently. This minimizes the overhead associated with opening and closing connections, which can be resource-intensive.

### 4. **Efficient Query Design**
- **Indexing**: Create indexes on frequently queried fields. This can drastically reduce query execution time but be mindful of the trade-offs, as too many indexes can slow down write operations.
- **Batching**: Use batch transactions for multiple insert/update operations to minimize the transaction overhead and reduce the number of roundtrips to the database.
- **SELECT only necessary fields**: Avoid `SELECT *`; specify only the fields you need to reduce the amount of data processed and transmitted.

### 5. **Transaction Management**
- Use transactions to ensure data integrity and consistency, especially in multi-step operations. Ensure that you handle transactions properly to avoid deadlocks and rollbacks that can affect performance.

### 6. **Error Handling**
- Implement robust error handling for database operations to gracefully handle exceptions, which helps avoid resource leaks and improves overall application stability.

### 7. **Memory Management**
- Avoid memory leaks by properly disposing of resources like database connections, result sets, and statements. Use language features (like `using` in C# or try-with-resources in Java) for automatic resource management.

### 8. **Concurrency Control**
- Use optimistic or pessimistic locking strategies to manage concurrent database access and prevent race conditions.

### 9. **Monitor and Optimize Queries**
- Regularly analyze query performance using execution plans. Tools like `EXPLAIN` in SQL databases can help identify slow queries and suggest indexes or restructuring.
- Monitor database performance metrics, tracking query execution times, cache hit ratios, and resource usage to identify bottlenecks.

### 10. **Asynchronous Operations**
- Where applicable, leverage asynchronous database operations to improve the responsiveness of applications, especially in I/O-bound scenarios.

### 11. **Data Caching**
- Implement caching strategies using in-memory data stores or querying caches to reduce the load on the database and improve response times.

### 12. **Use a Framework or ORM**
- Consider using well-established Object-Relational Mapping (ORM) frameworks that handle many best practices and optimizations for you. However, be cautious of performance overhead and understand how they generate queries.

### 13. **Test for Performance and Security**
- Conduct load testing to evaluate how the database and queries perform under stress.
- Review security best practices regularly, including monitoring query patterns for anomalies that could indicate potential attacks.

Incorporating these practices will help you craft high-performance, memory-safe database code that can handle various workloads effectively.

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
    ✓ Generated embedding: 1536 dimensions in 0.293s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.322s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.265s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.288s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.271s
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
    ✓ Generated embedding: 1536 dimensions in 0.398s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.26s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.271s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.317s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.304s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.273s
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
    ✓ Generated embedding: 1536 dimensions in 0.4s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages_kt
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.033s

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
    ✓ Generated embedding: 1536 dimensions in 0.279s
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
  ✓ Text search completed in 0.01s

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

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

> @ekodb/ekodb-client@0.30.1 prepare
> npm run build


> @ekodb/ekodb-client@0.30.1 build
> tsc


up to date, audited 44 packages in 856ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.30.1 build
> tsc

✅ TypeScript client built!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running TypeScript SWR Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_ts_44268_1792527488373 (6GHGdKv_Ra36MziW7svi2z272MzCSTB2CTYlRdnx6byBJLjNluauysHRG7zmF_HTuYmPnLOHSEaaFBSnv_TIPg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "id": 1,
          "name": "Leanne Graham",
          "address": {
            "city": "Gwenborough",
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            }
          },
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
          "username": "Bret",
          "phone": "1-770-736-8031 x56442"
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
Response time: 1ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "id": 1,
          "name": "Leanne Graham",
          "address": {
            "city": "Gwenborough",
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            }
          },
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
          "username": "Bret",
          "phone": "1-770-736-8031 x56442"
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
✓ Created enrichment script: fetch_product_reviews_ts_44268_1792527488373 (mtuKzoFF5O1Ng42AaJlIsiKsvjclmStQyc92KL9KmDFJgOns3NQYzXzdEY-iaZVxTTnHWFUNYYYKhPjnSKXHRw)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "sku": "BEA-ESS-ESS-001",
          "id": 1,
          "reviews": [
            {
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "reviewerName": "Eleanor Collins",
              "comment": "Would not recommend!",
              "rating": 3
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "comment": "Very satisfied!",
              "reviewerName": "Lucas Gordon",
              "rating": 4
            },
            {
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "rating": 5,
              "reviewerName": "Eleanor Collins",
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Highly impressed!"
            }
          ],
          "tags": [
            "beauty",
            "mascara"
          ],
          "stock": 99,
          "brand": "Essence",
          "returnPolicy": "No return policy",
          "minimumOrderQuantity": 48,
          "availabilityStatus": "In Stock",
          "discountPercentage": 10.48,
          "rating": 2.56,
          "price": 9.99,
          "meta": {
            "createdAt": "2025-10-09T14:47:01.588Z",
            "updatedAt": "2026-05-23T11:27:41.868Z",
            "barcode": "5784719087687",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
          },
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "warrantyInformation": "1 week warranty",
          "weight": 4,
          "category": "beauty",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "dimensions": {
            "height": 13.08,
            "depth": 22.99,
            "width": 15.14
          },
          "shippingInformation": "Ships in 3-5 business days",
          "title": "Essence Mascara Lash Princess"
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
✓ Edge cache script created: i5ocxJy6YkRvFKusIkbMALXEtQVbi87sq5zI61b8Cz9QUNgHx9-LlWfV8ewoYc1-dmeVf8Jbpb6dRzrzl6Oiag

Call 1: Cache miss (fetches from API)
Response time: 94ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "email": "Sincere@april.biz",
          "username": "Bret",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "address": {
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street": "Kulas Light",
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "suite": "Apt. 556"
          },
          "phone": "1-770-736-8031 x56442",
          "name": "Leanne Graham",
          "id": 1,
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

Call 2: Cache hit (served from ekoDB)
Response time: 1ms (94x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "email": "Sincere@april.biz",
          "username": "Bret",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "address": {
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street": "Kulas Light",
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "suite": "Apt. 556"
          },
          "phone": "1-770-736-8031 x56442",
          "name": "Leanne Graham",
          "id": 1,
          "website": "hildegard.org"
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
    Finished `release` profile [optimized] target(s) in 0.16s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.30.1-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.30.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.30.1
    Uninstalling ekodb_client-0.30.1:
      Successfully uninstalled ekodb_client-0.30.1
Successfully installed ekodb-client-0.30.1
🧪 Ensuring test dependencies (pytest) in .venv...
✅ Python client package built and installed!
📦 Ensuring Python example dependencies in .venv...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Python SWR Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user_swr_py (5FWfr4WTtVwQUOWKgGNS4hShPTm2hVc-CVImpyvjPqHxnn_8fBvJvPWOxi_E0OMpfRKmGG8CrHttE_qUzrBq9Q)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 162ms
Result: [
  {
    "cached_at": {
      "type": "String",
      "value": "1792527492"
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
        "followers": 328628,
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
        "updated_at": "2026-10-16T15:33:53Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    },
    "id": "torvalds"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 1ms (123.9x faster!)
Result: [
  {
    "cached_at": {
      "type": "String",
      "value": "1792527492"
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
        "followers": 328628,
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
        "updated_at": "2026-10-16T15:33:53Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    }
  }
]
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched_swr_py (RkU_L1QbCtVjWgIOT8ll-rj1pycff6rnaFX4ghWWBo32GhWzPYvcMAeBd6rbpOUj-kbaX0arzQldoYpyDvImkw)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": {
      "type": "String",
      "value": "1792527492"
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
✓ Created SWR script: fetch_github_user (NH9Q9GlOrmAkhiqUDHHPWbAmO0wLCLZP2_E1Zq9djsi8Im-hTWTkVOxU-ebwKjbaehV21jOxopkktFhXYz2FiA)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 78.812458ms
Result: [
  {
    "cached_at": {
      "type": "DateTime",
      "value": "2026-10-20T20:18:13+00:00"
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
        "followers": 328628,
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
        "updated_at": "2026-10-16T15:33:53Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    },
    "id": "torvalds"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 1.289625ms (78.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (VVlPLzm0jLQXfiUpB6A4jsHRMK0he2we2IDepWnI_2CfQkQYvfZxFR4ZhDOcrvy0f5rV8Nl0D74UGJ3dhsTEYA)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": {
      "type": "DateTime",
      "value": "2026-10-20T20:18:13+00:00"
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
✅ Client build complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Rust SWR Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.90s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (1qJlmQb3ccfSaUYtHmh6-rhcB-V-2Kl5ysV76m6S8UharTY5F2k5n9Cn697-W2KRwhyA5RsXZ_GL1gLG3rgP2g)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 181ms
Result: {
  "data": {
    "type": "Object",
    "value": {
      "followers": 328628,
      "bio": null,
      "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
      "hireable": null,
      "url": "https://api.github.com/users/torvalds",
      "location": "Portland, OR",
      "received_events_url": "https://api.github.com/users/torvalds/received_events",
      "followers_url": "https://api.github.com/users/torvalds/followers",
      "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
      "public_repos": 12,
      "created_at": "2011-09-03T15:26:22Z",
      "twitter_username": null,
      "public_gists": 1,
      "type": "User",
      "company": "Linux Foundation",
      "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
      "id": 1024025,
      "blog": "",
      "login": "torvalds",
      "site_admin": false,
      "organizations_url": "https://api.github.com/users/torvalds/orgs",
      "html_url": "https://github.com/torvalds",
      "repos_url": "https://api.github.com/users/torvalds/repos",
      "user_view_type": "public",
      "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
      "updated_at": "2026-10-16T15:33:53Z",
      "email": null,
      "name": "Linus Torvalds",
      "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
      "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
      "gravatar_id": "",
      "following": 0,
      "node_id": "MDQ6VXNlcjEwMjQwMjU="
    }
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 4ms (45.2x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (cG-JhcuYCNsEAoH6SHawoa9kq2huqunhLuslIaRcORNB1GXEPb3LqyWV1Fika3JvZP4SsSYoyCIxHNEE9FgwSQ)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "enriched_data": {
    "type": "Object",
    "value": {
      "rating": 2.56,
      "category": "beauty",
      "meta": {
        "barcode": "5784719087687",
        "createdAt": "2025-10-09T14:47:01.588Z",
        "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
        "updatedAt": "2026-05-23T11:27:41.868Z"
      },
      "price": 9.99,
      "shippingInformation": "Ships in 3-5 business days",
      "sku": "BEA-ESS-ESS-001",
      "returnPolicy": "No return policy",
      "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
      "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
      "reviews": [
        {
          "comment": "Would not recommend!",
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "reviewerName": "Eleanor Collins",
          "date": "2025-04-30T09:41:02.053Z",
          "rating": 3
        },
        {
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerName": "Lucas Gordon",
          "comment": "Very satisfied!",
          "rating": 4,
          "reviewerEmail": "lucas.gordon@x.dummyjson.com"
        },
        {
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "date": "2025-04-30T09:41:02.053Z",
          "comment": "Highly impressed!",
          "reviewerName": "Eleanor Collins",
          "rating": 5
        }
      ],
      "warrantyInformation": "1 week warranty",
      "images": [
        "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
      ],
      "discountPercentage": 10.48,
      "stock": 99,
      "weight": 4,
      "minimumOrderQuantity": 48,
      "title": "Essence Mascara Lash Princess",
      "availabilityStatus": "In Stock",
      "tags": [
        "beauty",
        "mascara"
      ],
      "id": 1,
      "brand": "Essence",
      "dimensions": {
        "depth": 22.99,
        "height": 13.08,
        "width": 15.14
      }
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
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Created SWR function: swr_fetch_github_user_kt_1792527541554 (w5rr6h1lm7OgvNKMsHtL4jhFINAFkHiyAO-xwaJ4BNJU61Jo1S4nP607bPGAIru_7royorQ4AJKgX5aX-v4fYw)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 101ms
Result: [{"cached_at":{"type":"String","value":"1792527541565"},"data":{"value":{"followers_url":"https://api.github.com/users/torvalds/followers","created_at":"2011-09-03T15:26:22Z","gists_url":"https://api.github.com/users/torvalds/gists{/gist_id}","hireable":null,"followers":328628,"name":"Linus Torvalds","avatar_url":"https://avatars.githubusercontent.com/u/1024025?v=4","login":"torvalds","public_repos":12,"url":"https://api.github.com/users/torvalds","subscriptions_url":"https://api.github.com/users/torvalds/subscriptions","gravatar_id":"","received_events_url":"https://api.github.com/users/torvalds/received_events","following_url":"https://api.github.com/users/torvalds/following{/other_user}","id":1024025,"html_url":"https://github.com/torvalds","organizations_url":"https://api.github.com/users/torvalds/orgs","starred_url":"https://api.github.com/users/torvalds/starred{/owner}{/repo}","bio":null,"site_admin":false,"email":null,"company":"Linux Foundation","following":0,"node_id":"MDQ6VXNlcjEwMjQwMjU=","repos_url":"https://api.github.com/users/torvalds/repos","type":"User","twitter_username":null,"events_url":"https://api.github.com/users/torvalds/events{/privacy}","public_gists":1,"updated_at":"2026-10-16T15:33:53Z","user_view_type":"public","blog":"","location":"Portland, OR"},"type":"Object"},"id":"torvalds"}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 4ms
Cache hit was 25.3x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment function: swr_fetch_product_kt_1792527541554 (oJJW5hFaRWJ120ElWbP7liPB3-6lJfOOe9AIOOJEPTTBpCSC9ND6qCm2eDF-Uns5djYIzjhCX9nv_pEppTNyCA)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"id":"1","enriched_at":{"type":"String","value":"1792527541835"},"enriched_data":{"value":{"availabilityStatus":"In Stock","images":["https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"],"description":"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.","weight":4,"thumbnail":"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp","title":"Essence Mascara Lash Princess","dimensions":{"depth":22.99,"width":15.14,"height":13.08},"sku":"BEA-ESS-ESS-001","shippingInformation":"Ships in 3-5 business days","minimumOrderQuantity":48,"brand":"Essence","id":1,"meta":{"barcode":"5784719087687","updatedAt":"2026-05-23T11:27:41.868Z","qrCode":"https://cdn.dummyjson.com/public/qr-code.png","createdAt":"2025-10-09T14:47:01.588Z"},"warrantyInformation":"1 week warranty","category":"beauty","returnPolicy":"No return policy","reviews":[{"comment":"Would not recommend!","rating":3,"date":"2025-04-30T09:41:02.053Z","reviewerName":"Eleanor Collins","reviewerEmail":"eleanor.collins@x.dummyjson.com"},{"rating":4,"date":"2025-04-30T09:41:02.053Z","comment":"Very satisfied!","reviewerName":"Lucas Gordon","reviewerEmail":"lucas.gordon@x.dummyjson.com"},{"date":"2025-04-30T09:41:02.053Z","comment":"Highly impressed!","reviewerName":"Eleanor Collins","rating":5,"reviewerEmail":"eleanor.collins@x.dummyjson.com"}],"discountPercentage":10.48,"rating":2.56,"tags":["beauty","mascara"],"stock":99,"price":9.99},"type":"Object"}}]
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


BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
✅ Client build complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Rust Function Composition Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
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
   ⏱️  Duration: 62.510792ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "id": 1,
      "address": {
        "street": "Kulas Light",
        "geo": {
          "lng": "81.1496",
          "lat": "-37.3159"
        ...

Second call (cache hit - from cache):
   ⏱️  Duration: 1.905208ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "website": "hildegard.org",
      "phone": "1-770-736-8031 x56442",
      "email": "Sincere@april.biz",
      "address": {
        "street": "Kulas Light",
        ...
   🚀 Cache speedup: 32.8x faster!

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

> @ekodb/ekodb-client@0.30.1 prepare
> npm run build


> @ekodb/ekodb-client@0.30.1 build
> tsc


up to date, audited 44 packages in 898ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.30.1 build
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
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 58ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "company": {
        "name": "Romaguera-Crona",
        "bs": "harness real-time e-markets",
        "catchPhrase": "Multi-layered client-serv...

Second call (cache hit - from cache):
   ⏱️  Duration: 1ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "company": {
        "name": "Romaguera-Crona",
        "bs": "harness real-time e-markets",
        "catchPhrase": "Multi-layered client-serv...
   🚀 Cache speedup: 58.0x faster!

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
    Finished `release` profile [optimized] target(s) in 0.16s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.30.1-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.30.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.30.1
    Uninstalling ekodb_client-0.30.1:
      Successfully uninstalled ekodb_client-0.30.1
Successfully installed ekodb-client-0.30.1
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
   ⏱️  Duration: 79.4ms
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
   ⏱️  Duration: 1.7ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 48.1x faster!

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
   ⏱️  Duration: 61.006334ms
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
   ⏱️  Duration: 1.982959ms
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
   🚀 Cache speedup: 61.0x faster!

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
   ⏱️  Duration: 113ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "username": "Bret",
      "website": "hildegard.org",
      "address": {
        "zipcode": "92998-3874",
        "suite": "Apt. 556",
       ...

Second call (cache hit - from cache):
   ⏱️  Duration: 1ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "username": "Bret",
      "website": "hildegard.org",
      "address": {
        "zipcode": "92998-3874",
        "suite": "Apt. 556",
       ...
   🚀 Cache speedup: 113.0x faster!

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
✅ Client build complete!

🦀 Rust WebSocket Subscription Test...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/examples/client_websocket_subscribe`
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_rs' ===
✓ Subscribed (subscription_id: sub_b87fa32250b347259870f5151a241a92)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: XOUdVCsW8uaqjH_9-Dm1qmR-mexWn9Us8YXUCVbAODCL4fH07K8i9HVW7XNZ9oaadMlzS1vr5TSuVGbQrpZmUA

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_rs"
     Record IDs: ["XOUdVCsW8uaqjH_9-Dm1qmR-mexWn9Us8YXUCVbAODCL4fH07K8i9HVW7XNZ9oaadMlzS1vr5TSuVGbQrpZmUA"]
     Timestamp:  "2026-10-20T20:21:42.278743+00:00"

Inserting record 2...
✓ Inserted: h9JW1-E3D5_1I5NasjYmjknh8RSTjDag8se9NxgKMmcrU5bqDEa4G6ZKInoF4jn_b7b29XURNYAv3cW-AKzNbg

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["h9JW1-E3D5_1I5NasjYmjknh8RSTjDag8se9NxgKMmcrU5bqDEa4G6ZKInoF4jn_b7b29XURNYAv3cW-AKzNbg"]

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
✓ Subscribed (subscription_id: sub_2a6ec2c271b848b19ea2b2a5d63e3431)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: azlyJ2Ficnsdvw9mlOZev-IqGomZjXxx1i-Ns9kBzIoIvJSxtFgj1QiCZXd1paulgpzHR2ct7df1KRnm1eFnyQ

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_go
     Record IDs: [azlyJ2Ficnsdvw9mlOZev-IqGomZjXxx1i-Ns9kBzIoIvJSxtFgj1QiCZXd1paulgpzHR2ct7df1KRnm1eFnyQ]
     Timestamp:  2026-10-20T20:21:42.535841+00:00

Inserting record 2...
✓ Inserted: jU8DNGuPIhY9gg8GDcqP0vKLQ5uiv8vj3aiY4iUBuqgiErKQgNjIU4mJNdMPfFhOI73VJrQAYPIXYKAm_9EG0Q

  📡 Notification received:
     Event:      insert
     Record IDs: [jU8DNGuPIhY9gg8GDcqP0vKLQ5uiv8vj3aiY4iUBuqgiErKQgNjIU4mJNdMPfFhOI73VJrQAYPIXYKAm_9EG0Q]

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
✓ Subscribed (subscription_id: sub_e47774981da8478baf72c3b5fb66c388)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: MPYTr31Q-j_zhxHj9cpKKipvqFbu4XEF6sFSO3sGmsLPwSgJOCOtUYW6UI80qhcwZfhXMJ3KT9lp-NjmoYukwg

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: MPYTr31Q-j_zhxHj9cpKKipvqFbu4XEF6sFSO3sGmsLPwSgJOCOtUYW6UI80qhcwZfhXMJ3KT9lp-NjmoYukwg
     Timestamp:  2026-10-20T20:21:43.097836+00:00

Inserting record 2...
✓ Inserted: qfRKmQSQZID9LAgE1YmgnoRGTO2ptyflft5ODchu2xaq4qn1bV295slJu6TYYHvvTTuehbpJ864z4AnCSFiZGQ

  📡 Notification received:
     Event:      insert
     Record IDs: qfRKmQSQZID9LAgE1YmgnoRGTO2ptyflft5ODchu2xaq4qn1bV295slJu6TYYHvvTTuehbpJ864z4AnCSFiZGQ

=== Unsubscribing ===
✓ Unsubscribed: {'collection': 'ws_subscribe_example_py', 'found': True, 'unsubscribed': True}

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_py'

✓ WebSocket subscription example completed successfully
✅ Python subscription test complete!
📦 Building TypeScript client library...

> @ekodb/ekodb-client@0.30.1 prepare
> npm run build


> @ekodb/ekodb-client@0.30.1 build
> tsc


up to date, audited 44 packages in 789ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.30.1 build
> tsc

✅ TypeScript client built!

📘 TypeScript WebSocket Subscription Test...
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_9b1f74102717404db47b71c7f3873e6d)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: OUNIdQNDYH9Yait_SNc8HAGwQW-ICJt5w2mEVo-U4oNo97QiKE-2lBYHt23wwoKJT7ozytY7gRdmFhWkA2QXTw
  📡 Notification received for OUNIdQNDYH9Yait_SNc8HAGwQW-ICJt5w2mEVo-U4oNo97QiKE-2lBYHt23wwoKJT7ozytY7gRdmFhWkA2QXTw

Inserting another record...
✓ Inserted record: 2PjwdhTitdt_l4QgJ09DUR9gCQxPbzaV4Ij29Efin9Z7_Hbo7yXGAbtzNRx2IENV7pBsvMixlk1_8WiliMpB9A
  📡 Notification received for 2PjwdhTitdt_l4QgJ09DUR9gCQxPbzaV4Ij29Efin9Z7_Hbo7yXGAbtzNRx2IENV7pBsvMixlk1_8WiliMpB9A

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_ts'
✅ TypeScript subscription test complete!

🟣 Kotlin WebSocket Subscription Test...
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Subscribed (subscription_id: sub_4ed8d2f947ab4630b3810c0fe68772f0)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "Ue7cZOtHGJauQfqDbbUJlVZqfUz2Q-FZfPGXJ1G5HSpetFsbIWY0AUMC9VTlWOIDh3AfHO0tRVoh_-3ZJFlbIA"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_kt"
     Record IDs: ["Ue7cZOtHGJauQfqDbbUJlVZqfUz2Q-FZfPGXJ1G5HSpetFsbIWY0AUMC9VTlWOIDh3AfHO0tRVoh_-3ZJFlbIA"]
     Timestamp:  "2026-10-20T20:21:51.129734+00:00"

Inserting record 2...
✓ Inserted: "pKycgz7JzaecbYXuFJwZ1ARet_9GFlLFkK6noD2kx1SHcy9OkGClxU-0sRHijc4-Vtfl42aXIUfBjV30VtE4_Q"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["pKycgz7JzaecbYXuFJwZ1ARet_9GFlLFkK6noD2kx1SHcy9OkGClxU-0sRHijc4-Vtfl42aXIUfBjV30VtE4_Q"]

=== Unsubscribing ===
✓ Unsubscribed: {"payload":{"data":{"collection":"ws_subscribe_example_kt","found":true,"unsubscribed":true}},"type":"Success"}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_kt'

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
✅ Kotlin subscription test complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ All WebSocket Subscription Tests Passed!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
