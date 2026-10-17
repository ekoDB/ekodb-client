make test-examples-rust
🧪 Running Rust examples (direct HTTP/WebSocket)...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("jWdS5Hu3hiDEzsHCfeKRTHOj3EM3D-R2PSCSYxjz8DjafnyflLvdriqL-azFJXhJ22ZFtXe2zrEbAptSKy-z4A")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("jWdS5Hu3hiDEzsHCfeKRTHOj3EM3D-R2PSCSYxjz8DjafnyflLvdriqL-azFJXhJ22ZFtXe2zrEbAptSKy-z4A"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("jWdS5Hu3hiDEzsHCfeKRTHOj3EM3D-R2PSCSYxjz8DjafnyflLvdriqL-azFJXhJ22ZFtXe2zrEbAptSKy-z4A"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("jWdS5Hu3hiDEzsHCfeKRTHOj3EM3D-R2PSCSYxjz8DjafnyflLvdriqL-azFJXhJ22ZFtXe2zrEbAptSKy-z4A"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: p4coXcYPfeShxvjr7DbHAqzmz7opn8eQz3lFbpUml435E8ro84aTQPnHfY17J6rm1al5vg_VE0O9pJvhLWPh5Q

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1792222977096354000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "p4coXcYPfeShxvjr7DbHAqzmz7opn8eQz3lFbpUml435E8ro84aTQPnHfY17J6rm1al5vg_VE0O9pJvhLWPh5Q",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "qyIGCHvxbue6wRf5OFSuYFmZ8vr9j990pIxcKPjwvCKYLoF-LGX3eJ2Od-bLP0yze6b2h6t49GAg22oW9rbziw"

=== List Collections ===
Total collections: 2
Sample collections: ["audit__ek0_testing", "demo_collection"]

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
✓ Inserted document: 8PPQirPcRp9w5MVJA88Fqc-Vjj4-sQBsKgQfGK2JVnc6J5eDpne0Gun7fk9BihBpMhUlCLYVa5QBnZRFj___Xw

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: CFfzjDpyvR3R9NKkk9WURBkAt8_b0HwyK9jPKoD29S140ZColGNo-jDjgpky0HSbsI9OtaRpFIACf1UKNwXJog

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: 8aMB-e41ZsndFYitCrSvvW9UOzCe2LCTVJzHkscHtcdRGsYYJstzGujY8BaxLt15n-tEfeedTtUfX-CZ5pr8TQ

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("RsTDFEEO6NJRPBsyOdh0_1x6szY2pgNcF5oXFrT7rwJ_JE4oTne6SU8e1FiyARuxj7BqRsiQX4eRzw72VGfOeQ")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record via WebSocket

Record 1:
  id: "RsTDFEEO6NJRPBsyOdh0_1x6szY2pgNcF5oXFrT7rwJ_JE4oTne6SU8e1FiyARuxj7BqRsiQX4eRzw72VGfOeQ"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-10-17T08:46:53.781982Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Functions Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: baZeKdGYvXiMyLNn424F1FjdLNp1OQUoHMdE3gTc4xVS95rpJPIaS3L0MIzFxt3Su_XnKKwM83EwufWc4zTc1A
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: aPmb-ZxhXMWyjll-ZpNsi12wC8l3X73ZG55lTGkroAeM8t39Z4p7GvwlHfT7RP9sdh9jCxJcJu7etElJIbkbRw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Function saved: 7HK_-k6mKHF_EX15cS6EHYfIH6extPF4X7P2FqjLEmwVAhHKN3OdE_OCzj3_7WfywnhULa83-xnXraphO70c_g
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: Gxdp9pils8gAoKZ0lRoBIcJ8q9Aw8eo6rlH-MGNRtls-g6Rwfye9rreKoR-WG-Ls7-C72NMdBIxTTNJu7NgeVw
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
Created Alice: $1000 - ID: RXT38DhorCKNhpbb3GIfwI77oxIl1sCjQOhRaf_ldZnxNR94sflPYHv6IKYSM2qVhsNrPWzjpWUl_NhzcLoXFg
Created Bob: $500 - ID: hZ6MS39_23z58sp4WnqyYsFM0dK1_vxg4jsi-1WWwuMLsDfGKbjkaAt-wii_PkcH70siXc-jDM1cwvwYpUofEw

=== Example 1: Begin Transaction ===
Transaction ID: 7df6e4e0-6572-4273-8f60-1f8815f6956d

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
New transaction: cdc9ec32-a5fa-4ea0-9cb8-ce6c01f9cf2a
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ Rust direct examples complete!
🛠️  Building client library...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.36s
✅ Client build complete!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_advanced_crud`
=== ekoDB Advanced CRUD Example (Rust) ===

--- Inserting base document ---
Inserted: Widget Counter (id: OZeZav7FnxSN78RHof1koIG9m-7gshfEm5p8MM0Qj9MtMbffhk2PUxggl75Xd3Zzb0Uq35_6MTkoX1oFjN967g)

--- update_with_action: increment views by 10 ---
views after increment: Some(Object({"type": String("Integer"), "value": Integer(110)}))

--- update_with_action: decrement views by 3 ---
views after decrement: Some(Object({"type": String("Integer"), "value": Integer(107)}))

--- update_with_action: multiply score by 2 ---
score after multiply: Some(Object({"type": String("Float"), "value": Float(9.0)}))

--- update_with_action: push 'ekodb' to tags ---
tags after push: Some(Object({"type": String("Array"), "value": Array([String("rust"), String("database"), String("ekodb")])}))

--- update_with_action: append '-suffix' to label ---
label after append: Some(Object({"value": String("prefix-suffix"), "type": String("String")}))

--- update_with_action: pop last element from tags ---
tags after pop: Some(Object({"type": String("Array"), "value": Array([String("rust"), String("database")])}))

--- update_with_action: remove 'rust' from tags ---
tags after remove: Some(Object({"type": String("Array"), "value": Array([String("database")])}))

--- update_with_action_sequence: increment views + push tag + append label ---
views after sequence: Some(Object({"value": Integer(157), "type": String("Integer")}))
tags after sequence: Some(Object({"value": Array([String("database"), String("batch")]), "type": String("Array")}))
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: iEhPRlIZ04UABikMSwMcdSMWYG4wimBJegJoSRhaqWuUHiawTj6TyBtLc3idTZH4NFzobVyaXLwSBhRA5cbNsw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or have other inquiries, feel free to ask!

✓ Second message sent
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you need any further information or assistance, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: e8EXQSaSeSAIoKSghHsb-1CuQxarTlYcRYyehO1WlGHIa3eHfNwT8_fzDBbHzsfEkS8wDbUhDDBgidrmSeOAmg
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: e8EXQSaSeSAIoKSghHsb-1CuQxarTlYcRYyehO1WlGHIa3eHfNwT8_fzDBbHzsfEkS8wDbUhDDBgidrmSeOAmg
✓ Deleted chat session: iEhPRlIZ04UABikMSwMcdSMWYG4wimBJegJoSRhaqWuUHiawTj6TyBtLc3idTZH4NFzobVyaXLwSBhRA5cbNsw
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: PjW67vbMvyuXcO9K37ejJV2010OGwWTa_dB6k5IyVyOk3WYAtSq61tPmLYiDprMKvtpr5V36JK0ZIBq5Xmjsjw

=== Sending Chat Message ===
Message ID: fv5GKa1nnVIpr2kTd8YAgwgI4CPRuXFjmb2Y7HboWQjB0rk4vx5FSBDztZ2qrlTmhEcqO51fNaTW6O6Oo6yQsg

=== AI Response ===
Response 1: **ekoDB Overview:**
ekoDB is described as a high-performance database that integrates intelligent caching, real-time capabilities, and AI technologies. This makes it well-suited for applications that require fast data retrieval and AI-enhanced processing.

**Key Features of ekoDB:**
1. **AI Chat Integration:** ekoDB includes a chat feature that allows users to query the database using natural language. This feature provides AI-powered responses along with relevant contextual information.

2. **Search Capabilities:**
   - Supports **full-text search**, allowing for efficient text-based queries.
   - Implements **vector search**, enhancing search capabilities through embeddings.
   - Offers **hybrid search**, which combines both keyword and semantic search methods, with automatic context retrieval.

These features highlight ekoDB's strengths in performance, search functionalities, and integration capabilities with AI.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["category", "title", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("ZH86RHcUWzD62JEKWzZTLCmWBmszopTwVBvAc2G0qCYjRPG7x8-RTuJAzeHJvAq-O62LtiQC8b9emG7YGSMSsw"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("_pM4VbjOve6zxElhmLjjgv535p16e6oZ-lO8T1iINUl9U3cO1YKV4SlqvFVAeH2G_aaxfkHbuuN4D-Q-H1Z6yQ"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("kB6ONCJFogNX7cVezUJe5tMfdxZpg0Jx_rqOMs6g1pR7IHRVqf4Yc8AlF_jZuW3GlW0HZNIHFJCWahY72uCLuA"), "title": String("Search Features")}

Execution Time: 3974ms

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_message_stream`
=== ekoDB Chat Message Stream (SSE) Example (Rust) ===

Created session: iwQ7soC5JvymqBnVwFZtUDPvbKG--OTne0IbaxotVzFJk9XWaMbVB_PgieoGFE8PNMyZhTkrpRX6sCMHzAhoGg

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}**ekoDB** is a comprehensive, open-source database specifically designed for sustainable construction materials. Its primary goal is to aid architects, engineers, builders, and researchers in making informed decisions to reduce the environmental impact of buildings and infrastructure.

### Key Features of ekoDB:

1. **Material Data Repository**:
   ekoDB compiles detailed information about various construction materials, including physical properties, durability, embodied carbon, recyclability, origin, certifications, and other life-cycle analysis data.

2. **Sustainability Metrics**:
   It emphasizes the environmental aspects of materials, such as greenhouse gas emissions (CO2 footprint), energy use, toxicity, and water consumption associated with production and use.

3. **Open-Source and Collaborative**:
   ekoDB is an open-source platform, which means its data and code are freely accessible. This encourages collaboration among professionals and allows users to contribute data, case studies, and improvements.

4. **Decision Support Tools**:
   The database often includes tools for comparing materials, calculating environmental impacts, and generating reports for sustainability certification (e.g., LEED, BREEAM).

5. **Global Scope**:
   While it started with a focus on local or regional materials, ekoDB aims to include data from international sources, making it applicable to a wide range of projects worldwide.

---

**In summary:**
ekoDB is an open, collaborative digital resource supporting sustainable architecture and construction through transparent, accessible information on eco-friendly building materials.

If you meant a different "ekoDB" (such as in another industry), please provide more context!

--- Stream complete ---
Message ID: jcLSC8-PX7XwfzpAOyiF6wGYKWidr_68HcRP9LZhqe-5-jXVqNZnj_JiVaLxD9Iief-cRt4agRCArcHsEIutMQ
Execution time: 2421ms
Context window: 1000000 tokens

✓ Chat message stream example completed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_chat_models`
✓ Client created

=== Get All Chat Models ===
Available chat models by provider:

OpenAI models (135):
  - gpt-3.5-turbo-0125
  - gpt-5.4-2026-03-05
  - gpt-5-nano
  - gpt-5.4-mini
  - o1-pro
  - gpt-5.1-codex
  - tts-1
  - gpt-5-mini
  - gpt-5-mini-2025-08-07
  - gpt-4-turbo-2024-04-09
  - gpt-realtime-2.1-mini
  - gpt-realtime-2025-08-28
  - gpt-4
  - tts-1-hd
  - gpt-realtime-1.5
  - gpt-realtime
  - gpt-5.2-pro
  - gpt-5-chat-latest
  - gpt-5.5-pro
  - gpt-audio-2025-08-28
  - gpt-4o-mini-transcribe-2025-03-20
  - omni-moderation-2024-09-26
  - gpt-live-transcribe
  - gpt-5.6-luna
  - omni-moderation-latest
  - o4-mini-2025-04-16
  - gpt-4o-2024-11-20
  - gpt-5.1
  - whisper-1
  - gpt-5-pro
  - gpt-realtime-whisper
  - gpt-5-2025-08-07
  - gpt-5.4-mini-2026-03-17
  - babbage-002
  - gpt-4o-2024-05-13
  - text-embedding-3-large
  - chat-latest
  - gpt-realtime-translate
  - gpt-5.4-pro-2026-03-05
  - gpt-realtime-2
  - gpt-5-search-api
  - gpt-5.5-2026-04-23
  - tts-1-1106
  - gpt-5.5-pro-2026-04-23
  - gpt-audio-mini-2025-10-06
  - gpt-5.2-pro-2025-12-11
  - o3-mini-2025-01-31
  - gpt-audio-1.5
  - gpt-image-1.5
  - gpt-4-turbo
  - gpt-4o-2024-08-06
  - o3
  - gpt-4o-mini-search-preview-2025-03-11
  - gpt-4o-mini-transcribe-2025-12-15
  - gpt-4.1-mini
  - gpt-4o-mini
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
  - o1-2024-12-17
  - gpt-5-pro-2025-10-06
  - gpt-5.6-sol
  - gpt-4o-mini-tts-2025-12-15
  - gpt-image-1
  - gpt-audio
  - gpt-5.3-codex
  - gpt-5-search-api-2025-10-14
  - gpt-realtime-mini
  - gpt-image-1-mini
  - gpt-5.6-terra
  - gpt-3.5-turbo-instruct-0914
  - sora-2
  - gpt-5.4-pro
  - gpt-4o-mini-tts
  - gpt-realtime-mini-2025-12-15
  - gpt-5.1-codex-max
  - gpt-4o-mini-2024-07-18
  - gpt-4.1-2025-04-14
  - gpt-5
  - gpt-4o-mini-transcribe
  - gpt-image-2-2026-04-21
  - gpt-5.2-chat-latest
  - sora-2-pro
  - gpt-3.5-turbo-instruct
  - gpt-transcribe
  - gpt-4.1-nano-2025-04-14
  - gpt-5.1-chat-latest
  - tts-1-hd-1106
  - gpt-5.2-codex
  - gpt-4o
  - o4-mini-deep-research
  - gpt-5.4
  - gpt-4o-search-preview-2025-03-11
  - text-embedding-3-small
  - gpt-5-nano-2025-08-07
  - gpt-4o-transcribe
  - o4-mini
  - gpt-5.1-codex-mini
  - gpt-image-2.5-sunburst-2026-09-08
  - gpt-5.3-chat-latest
  - o3-2025-04-16
  - gpt-5.2-2025-12-11
  - chatgpt-image-latest
  - gpt-3.5-turbo
  - gpt-3.5-turbo-1106
  - gpt-4o-mini-tts-2025-03-20
  - gpt-5.2
  - gpt-5.4-nano-2026-03-17
  - o1
  - davinci-002
  - gpt-5.4-nano
  - o3-mini
  - o4-mini-deep-research-2025-06-26
  - gpt-audio-mini-2025-12-15
  - o1-pro-2025-03-19
  - gpt-6-luna
  - gpt-image-2.5-flare
  - gpt-6-astra
  - gpt-audio-mini
  - gpt-4.1-mini-2025-04-14
  - gpt-image-2.5-sunburst
  - gpt-image-2.5-flare-2026-09-08
  - gpt-4o-mini-search-preview
  - gpt-6-sol
  - gpt-live-1
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
  - gpt-3.5-turbo-0125
  - gpt-5.4-2026-03-05
  - gpt-5-nano
  - gpt-5.4-mini
  - o1-pro
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: kUeyKtql8cvUcbcZTIutYj71u7o55jqP1_DXM-_oBFi3TfMKYLB_61Ljj9yvVxwBoY8h6rr3VxaZzlnsGoqQag
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Product:** ekoDB
- **Description:** A high-performance database product with AI capabilities
- **Price:** $99

If you have any further questions or need more information, feel free to ask!

✓ Message 2 sent
  Response: The price of the product ekoDB is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: pnTiARv9MdqRt5dN67OH68_lpkCkJ2AO-RQ4dRN9-iVlCc_-ejmHEDT0vwbVuUcG58C-aft6GQNlXDSmkPArvQ
  Parent: kUeyKtql8cvUcbcZTIutYj71u7o55jqP1_DXM-_oBFi3TfMKYLB_61Ljj9yvVxwBoY8h6rr3VxaZzlnsGoqQag

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: pnTiARv9MdqRt5dN67OH68_lpkCkJ2AO-RQ4dRN9-iVlCc_-ejmHEDT0vwbVuUcG58C-aft6GQNlXDSmkPArvQ (Untitled)
  Session 2: kUeyKtql8cvUcbcZTIutYj71u7o55jqP1_DXM-_oBFi3TfMKYLB_61Ljj9yvVxwBoY8h6rr3VxaZzlnsGoqQag (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: pnTiARv9MdqRt5dN67OH68_lpkCkJ2AO-RQ4dRN9-iVlCc_-ejmHEDT0vwbVuUcG58C-aft6GQNlXDSmkPArvQ

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "mmRa0F3z-bkjUXQryy-jl2p67eglA10tGss8REWJQCql4vytITNYA5T6EPARxuJEVGY8WrZ9QHRgTddG9Jn_wQ"

=== List Collections ===
Total collections: 7
Sample collections: ["chat_messages__ek0_testing", "client_collection_management_rust", "audit__ek0_testing", "functions__ek0_testing", "chat_configurations__ek0_testing"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
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
  - chat_messages__ek0_testing
  - audit__ek0_testing
  - functions__ek0_testing
  - chat_configurations__ek0_testing
  - chat_turns__ek0_testing
  - collection_utils_test_rust <-- our test
  - agent_function_versions__ek0_testing

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_rust'

✓ Collection Utilities example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
✓ Created record with fluent builder: Record({"id": String("CucK7lhNBt4dGiwHjzVLeS_xijZEWyC5_-8Wod4ulyhAQD-l_rvl5tfjVCF3ELFqIW2QvsvgEn5a5Edm5YnN7Q")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("bob-user-id")})
✓ Second upsert (update): Record({"age": Object({"value": Integer(36), "type": String("Integer")}), "name": Object({"type": String("String"), "value": String("Bob Smith")}), "email": Object({"type": String("String"), "value": String("bob.smith@newdomain.com")}), "id": String("bob-user-id")})

=== Find One Operation ===
✓ Found user by email: Record({"name": Object({"value": String("Alice Johnson"), "type": String("String")}), "id": String("CucK7lhNBt4dGiwHjzVLeS_xijZEWyC5_-8Wod4ulyhAQD-l_rvl5tfjVCF3ELFqIW2QvsvgEn5a5Edm5YnN7Q"), "email": Object({"type": String("String"), "value": String("alice@example.com")}), "age": Object({"type": String("Integer"), "value": Integer(28)}), "active": Object({"type": String("Boolean"), "value": Boolean(true)})})
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "nAGtjRYlOp__jInBkWy_wYJO938HsHvVKhzgnaMY2tn_ZZ_EcTX7spXjQoe537YptbNvzF2dJE_FGUNKg6y9rw"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("E-Uao5Xd9Q3--mmkzXnZd2T7vcbh0MjgrBA7VdWdosdXNF7SEIwtw1Qtano5AY6vxgpsWfjkInRlGY3gIU3jCg"))

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
✓ Edge cache script created: Na998lmlFEiW-6TGUq2TrRO7un4aMFIfjAlT7SDhoRYH6al8Pv01KU1saci4pT8y7x7CcTijVIv1tnrUY4YS5g

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
   ⏱️  Duration: 95.444292ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "email": "Sincere@april.biz",
      "phone": "1-770-736-8031 x56442",
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net",
        "b...

Second call (cache hit - from cache):
   ⏱️  Duration: 6.290416ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "email": "Sincere@april.biz",
      "website": "hildegard.org",
      "company": {
        "name": "Romaguera-Crona",
        "catchPhrase": "Multi-layered client-s...
   🚀 Cache speedup: 15.2x faster!

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Functions Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: 9aVtM-P4BQnoWtNqECKpEfXhCRSZJ1l2HQIDZgDE0VHYV3BQqYVBx40ecxz3c0gl1NgGBAvgZFYrmAyFuhlL2Q
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: 0I8jfyaEwyA9520vh561Uz5LCgaMcbWwjGINpJDHt4VVpOaW6dAow8ePAu3xjStkbP_Nb4sQg13UhcsKX_yWdw
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: Sv949c_bArcEbSkfYhFjcadw6UZgtuGO4HhOiu3zKLwQs6Gu1aBiBqsVvtpiHtYdTzu3L7JlgOzVN17k_neQUQ
📊 Statistics: 2 groups

📝 Example 4: Function Management

📋 Total functions: 3
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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
   Record({"category": Object({"type": String("String"), "value": String("Electronics")}), "avg_price": Object({"value": Float(367.0), "type": String("Float")}), "count": Object({"value": Integer(5), "type": String("Integer")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits, including:

1. **Efficient Similarity Search**: They enable fast nearest neighbor searches in high-dimensional spaces, making them ideal for applications like image and text similarity.

2. **Scalability**: Designed to handle large volumes of vector data, they can easily scale as data grows.

3. **Support for Unstructured Data**: Vector databases can efficiently manage unstructured data formats, such as images, audio, and text embeddings.

4. **Real-time Processing**: They facilitate real-time querying and updates, essential for dynamic applications like recommendation systems.

5. **Integration with Machine Learning**: They support machine learning model outputs, allowing for seamless integration with AI applications and workflows.

6. **Advanced Query Capabilities**: Users can perform complex queries based on vector similarities, enabling nuanced data retrieval strategies.

7. **Data Compression**: Many vector databases use techniques to reduce storage requirements for high-dimensional data.

8. **Flexibility**: They can work with various data types and support hybrid queries involving both traditional and vector data.
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_functions_complete`
🚀 ekoDB Rust Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: y-xNI1LBXvN9npQOyDJ_saeM7KHR9MNFBCh6fM8It31EZtxOXOnu4vu85QMeBvxcpU1y8Sv-nB4xpfXO0-dRzg
📊 Found 2 product groups
   Record({"category": Object({"type": String("String"), "value": String("Furniture")}), "count": Object({"value": Integer(2), "type": String("Integer")}), "avg_price": Object({"value": Float(474.0), "type": String("Float")})})
   Record({"avg_price": Object({"type": String("Float"), "value": Float(575.6666666666666)}), "count": Object({"value": Integer(3), "type": String("Integer")}), "category": Object({"type": String("String"), "value": String("Electronics")})})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   Record({"count": Object({"value": Integer(3), "type": String("Integer")}), "category": Object({"type": String("String"), "value": String("Electronics")})})
   Record({"count": Object({"type": String("Integer"), "value": Integer(2)}), "category": Object({"value": String("Furniture"), "type": String("String")})})
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
   Record({"count": Object({"type": String("Integer"), "value": Integer(7)}), "status": Object({"value": String("active"), "type": String("String")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("LaOMBbV0SfwHuPceQp5O-tHRQ-rfQ6wP75isc4YUxnVhpW3mjlCr36_gZo_EwA43V2bqLP935Mjlbihmgs5kng"))
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: 4D3Y_d2iVkKf-db2oj-ikQki8Ar_FEuaUGiTyjvQM16ae0KWZuqZJ0Vv5rgkXDyS0GsZy3xSPhSnPTWHI0vQ4A
📊 function executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"type": String("Object"), "value": Object {"role": String("admin"), "userId": String("user_abc")}})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: _13o-qPDj-Mxv2A2WxXS04enD7XvbovRBuoS_ldHbnyLTa5uI1SUjNbBX0NTSMUaWaMvvHLbIh2JsvHuIYYenQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: FU2qt041QQ5an8j0ziTc6iZDus258Fz3a09alXQYrqhdEnZsDD-_OJq1PAhOWNWkSGsiqFdpfMpf_7CiDpCYuQ
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Getting Started with ekoDB (Database)
   2. Natural Language Processing (AI)
   3. Vector Databases Explained (Database)
   4. Database Design Principles (Database)
   5. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   Record({"count": Object({"value": Integer(3), "type": String("Integer")}), "category": Object({"type": String("String"), "value": String("Database")})})
   Record({"category": Object({"type": String("String"), "value": String("AI")}), "count": Object({"value": Integer(2), "type": String("Integer")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_goal_templates`
=== ekoDB Goal Template CRUD Example (Rust) ===

--- Creating goal template ---
Created template: Data Migration (id: IQj38d4yyPRAW24A3IoYb9zLowmSw9pR--5iQoCgFV8R7Y6y5ROTIo_VktxOnygTcm569VQorP5p4ZmJFGEKOw)

--- Listing templates ---
Templates: {"count":1,"items":[{"description":{"type":"String","value":"Template for migrating data between schemas"},"id":"IQj38d4yyPRAW24A3IoYb9zLowmSw9pR--5iQoCgFV8R7Y6y5ROTIo_VktxOnygTcm569VQorP5p4ZmJFGEKOw","steps":{"type":"Array","value":[{"description":"Analyze source schema"},{"description":"Create target schema"},{"description":"Migrate records"},{"description":"Validate results"}]},"title":{"type":"String","value":"Data Migration"}}]}

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
Created goal: "Deploy v2.0 to production" (id: DzmXGUy357k94FQ4ATP6tlXwL_UoknVjAlCPw4Ycezi7ScNdouIUqBoFT1oSR1FTjOS7YAxV3R5-Hx-kBJvU5A)

--- Goal: list ---
Goals: {"count":1,"goals":[{"created_at":"2026-10-17T08:08:57.722876+00:00","description":"Full release cycle for version 2.0","id":"DzmXGUy357k94FQ4ATP6tlXwL_UoknVjAlCPw4Ycezi7ScNdouIUqBoFT1oSR1FTjOS7YAxV3R5-Hx-kBJvU5A","status":"pending","steps":"[{\"description\":\"Run integration tests\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]","title":"Deploy v2.0 to production","updated_at":"2026-10-17T08:08:57.722876+00:00"}]}

--- Goal: get ---
Fetched goal: {"type":"String","value":"Deploy v2.0 to production"}

--- Goal: update ---
Updated description: {"type":"String","value":"Updated: full v2.0 release with rollback plan"}

--- Goal: search ---
Search results: {"count":1,"items":[{"_score":12.87,"created_at":{"type":"DateTime","value":"2026-10-17T08:08:57.722876+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"DzmXGUy357k94FQ4ATP6tlXwL_UoknVjAlCPw4Ycezi7ScNdouIUqBoFT1oSR1FTjOS7YAxV3R5-Hx-kBJvU5A","status":{"type":"String","value":"pending"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-10-17T08:08:57.768748+00:00"}}]}

--- Goal step: start (step 0) ---
Step 0 started: {"created_at":{"type":"DateTime","value":"2026-10-17T08:08:57.722876+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"DzmXGUy357k94FQ4ATP6tlXwL_UoknVjAlCPw4Ycezi7ScNdouIUqBoFT1oSR1FTjOS7YAxV3R5-Hx-kBJvU5A","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"status\":\"InProgress\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-10-17T08:08:57.780028+00:00"}}

--- Goal step: complete (step 0) ---
Step 0 completed: {"created_at":{"type":"DateTime","value":"2026-10-17T08:08:57.722876+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"DzmXGUy357k94FQ4ATP6tlXwL_UoknVjAlCPw4Ycezi7ScNdouIUqBoFT1oSR1FTjOS7YAxV3R5-Hx-kBJvU5A","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-10-17T08:08:57.786128+00:00"}}

--- Goal step: start (step 1) ---
Step 1 started

--- Goal step: fail (step 1) ---
Step 1 failed: {"created_at":{"type":"DateTime","value":"2026-10-17T08:08:57.722876+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"DzmXGUy357k94FQ4ATP6tlXwL_UoknVjAlCPw4Ycezi7ScNdouIUqBoFT1oSR1FTjOS7YAxV3R5-Hx-kBJvU5A","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-10-17T08:08:57.798177+00:00"}}

--- Goal: complete ---
Goal completed (pending review): {"completed_at":{"type":"DateTime","value":"2026-10-17T08:08:57.809700+00:00"},"created_at":{"type":"DateTime","value":"2026-10-17T08:08:57.722876+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"DzmXGUy357k94FQ4ATP6tlXwL_UoknVjAlCPw4Ycezi7ScNdouIUqBoFT1oSR1FTjOS7YAxV3R5-Hx-kBJvU5A","status":{"type":"String","value":"pending_review"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"summary":{"type":"String","value":"Partial completion — step 1 failed"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-10-17T08:08:57.809700+00:00"}}

--- Goal: approve ---
Goal approved: {"completed_at":{"type":"DateTime","value":"2026-10-17T08:08:57.809700+00:00"},"created_at":{"type":"DateTime","value":"2026-10-17T08:08:57.722876+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"DzmXGUy357k94FQ4ATP6tlXwL_UoknVjAlCPw4Ycezi7ScNdouIUqBoFT1oSR1FTjOS7YAxV3R5-Hx-kBJvU5A","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"summary":{"type":"String","value":"Partial completion — step 1 failed"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-10-17T08:08:57.816467+00:00"}}

--- Goal: create (for rejection) ---
Created goal2: "Refactor auth module" (id: dz5WXXqrMUt1HCiIa_CPrmhgJKXwoKcNHqHtTzcxg11uItDEclGioWHmxPaZyFeshc0jCQua-kaibMXNJUuMlg)

--- Goal: complete (goal2) ---

--- Goal: reject ---
Goal rejected: {"completed_at":{"type":"DateTime","value":"2026-10-17T08:08:57.836551+00:00"},"created_at":{"type":"DateTime","value":"2026-10-17T08:08:57.824352+00:00"},"description":{"type":"String","value":"Rewrite JWT handling"},"id":"dz5WXXqrMUt1HCiIa_CPrmhgJKXwoKcNHqHtTzcxg11uItDEclGioWHmxPaZyFeshc0jCQua-kaibMXNJUuMlg","reason":{"type":"String","value":"Breaks backward compatibility"},"status":{"type":"String","value":"failed"},"steps":{"type":"String","value":"[]"},"summary":{"type":"String","value":"Completed refactor"},"title":{"type":"String","value":"Refactor auth module"},"updated_at":{"type":"DateTime","value":"2026-10-17T08:08:57.836551+00:00"}}

--- Task: create ---
Created task: Nightly backup (id: TJqMtZzEcNanCErYtI4Ht6nPN3Fni6Ee4rMU1DHXz80j0tkwXvLamlhoPutEYj63LnS5XX9ZCv2xTWqqJquwcg)

--- Task: list ---
Tasks: {"count":1,"items":[{"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"TJqMtZzEcNanCErYtI4Ht6nPN3Fni6Ee4rMU1DHXz80j0tkwXvLamlhoPutEYj63LnS5XX9ZCv2xTWqqJquwcg","max_retries":{"type":"Integer","value":3},"schedule":{"type":"String","value":"0 2 * * *"},"title":{"type":"String","value":"Nightly backup"}}]}

--- Task: get ---
Fetched task: {"type":"String","value":"Nightly backup"}

--- Task: start ---
Task started: {"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"TJqMtZzEcNanCErYtI4Ht6nPN3Fni6Ee4rMU1DHXz80j0tkwXvLamlhoPutEYj63LnS5XX9ZCv2xTWqqJquwcg","max_retries":{"type":"Integer","value":3},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"running"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-10-17T08:08:57.879543+00:00"}}

--- Task: succeed ---
Task succeeded: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"TJqMtZzEcNanCErYtI4Ht6nPN3Fni6Ee4rMU1DHXz80j0tkwXvLamlhoPutEYj63LnS5XX9ZCv2xTWqqJquwcg","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-10-17T08:08:57.889247+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-10-17T08:08:57.889247+00:00"}}

--- Task: start (round 2) ---

--- Task: pause ---
Task paused: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"TJqMtZzEcNanCErYtI4Ht6nPN3Fni6Ee4rMU1DHXz80j0tkwXvLamlhoPutEYj63LnS5XX9ZCv2xTWqqJquwcg","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-10-17T08:08:57.889247+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"paused"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-10-17T08:08:57.906072+00:00"}}

--- Task: resume ---
Task resumed: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"TJqMtZzEcNanCErYtI4Ht6nPN3Fni6Ee4rMU1DHXz80j0tkwXvLamlhoPutEYj63LnS5XX9ZCv2xTWqqJquwcg","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-10-17T08:08:57.889247+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-10-17T08:08:57.916642+00:00"}}

--- Task: fail ---
Task failed: {"consecutive_failures":{"type":"Integer","value":1},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"TJqMtZzEcNanCErYtI4Ht6nPN3Fni6Ee4rMU1DHXz80j0tkwXvLamlhoPutEYj63LnS5XX9ZCv2xTWqqJquwcg","last_error":{"type":"String","value":"S3 bucket access denied"},"last_run":{"type":"DateTime","value":"2026-10-17T08:08:57.922804+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-10-17T08:08:57.922804+00:00"}}

--- Task: due ---
Due tasks: {"count":0,"items":[]}

--- Task: delete ---
Task deleted

--- Agent: create ---
Created agent: backup-agent (id: FIhcTLLeewVVvVgp5TeRtXJm09oGUuL6sNtOVxlpJhaOM-hoccySVlaD6KaHRdS_dlmXjjtCFtpYjj83U0Sp8w)

--- Agent: list ---
Agents: {"count":1,"items":[{"capabilities":{"type":"Array","value":["backup","restore","verify"]},"deployment_id":{"type":"String","value":"deploy-abc-123"},"description":{"type":"String","value":"Handles nightly backups and restores"},"id":"FIhcTLLeewVVvVgp5TeRtXJm09oGUuL6sNtOVxlpJhaOM-hoccySVlaD6KaHRdS_dlmXjjtCFtpYjj83U0Sp8w","llm_model":{"type":"String","value":"gpt-4.1"},"name":{"type":"String","value":"backup-agent"}}]}

--- Agent: get ---
Fetched agent: {"type":"String","value":"backup-agent"}

--- Agent: get_by_name ---
Found by name: {"type":"String","value":"backup-agent"}

--- Agent: update ---
Updated agent description: {"type":"String","value":"Handles backups, restores, and disaster recovery"}

--- Agent: agents_by_deployment ---
Agents in deployment: {"count":0,"items":[]}
WARNING: agents_by_deployment omitted created agent FIhcTLLeewVVvVgp5TeRtXJm09oGUuL6sNtOVxlpJhaOM-hoccySVlaD6KaHRdS_dlmXjjtCFtpYjj83U0Sp8w; TODO: check/fix the server-side deployment lookup

--- Agent: delete ---
Agent deleted

--- Goal: delete ---
Goal 1 deleted
Goal 2 deleted

=== All goals, tasks & agents operations completed ===
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_kv_links`
=== ekoDB KV Document Linking Example (Rust) ===

--- Inserting documents ---
Inserted doc1: Project Alpha (id: Eyzas9NF5YHvFjtewdqE-qW8WAcL8wcm1IPldgSv4_o2ksFTmYXwtrwlUnOmluvjtwrY9OyWTE9F6h99DMX9fw)
Inserted doc2: Project Beta (id: n2NMIkFIzgALVX8jGnHYzg3OAiCrR2woaZHH0H7Im8J9bfavUQR-scIngziKrQbYKoi3JE0sM_ukw9YRIkKKjA)

--- Setting KV key ---
Set key: kv_links:rs:user:alice:projects

--- Linking documents ---
Linked doc1: null
Linked doc2: null

--- Getting links ---
Links for kv_links:rs:user:alice:projects: [{"collection":"kv_links_example_rs","created_at":"2026-10-17T08:11:31.424103Z","document_id":"Eyzas9NF5YHvFjtewdqE-qW8WAcL8wcm1IPldgSv4_o2ksFTmYXwtrwlUnOmluvjtwrY9OyWTE9F6h99DMX9fw","field_path":null,"last_accessed":"2026-10-17T08:11:31.428394Z","metadata":{}},{"collection":"kv_links_example_rs","created_at":"2026-10-17T08:11:31.426867Z","document_id":"n2NMIkFIzgALVX8jGnHYzg3OAiCrR2woaZHH0H7Im8J9bfavUQR-scIngziKrQbYKoi3JE0sM_ukw9YRIkKKjA","field_path":null,"last_accessed":"2026-10-17T08:11:31.428394Z","metadata":{}}]

--- Unlinking doc1 ---
Unlinked doc1: null

--- Verifying remaining links ---
Remaining links: [{"collection":"kv_links_example_rs","created_at":"2026-10-17T08:11:31.426867Z","document_id":"n2NMIkFIzgALVX8jGnHYzg3OAiCrR2woaZHH0H7Im8J9bfavUQR-scIngziKrQbYKoi3JE0sM_ukw9YRIkKKjA","field_path":null,"last_accessed":"2026-10-17T08:11:31.429438Z","metadata":{}}]

--- Cleanup ---
Unlinked remaining document
Deleted KV key
Deleted documents
Deleted collection

=== All KV linking operations completed ===
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

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
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"value": String("Charlie"), "type": String("String")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
  2. Some(Object({"value": String("David"), "type": String("String")}))
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_raw_completion_stream`
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/examples/client_schedules`
=== ekoDB Schedule Management Example (Rust) ===

--- Creating schedule ---
Created schedule: "nightly-cleanup" (id: 0171a7d8-eb74-4348-b09c-8484486558c7)

--- Listing schedules ---
Schedules: {"count":1,"schedules":[{"created_at":"2026-10-17T08:16:40.805872Z","cron_expression":"0 0 3 * * *","description":"Remove expired sessions and temp files","enabled":true,"function_label":"schedule_noop_rust_48372","id":"0171a7d8-eb74-4348-b09c-8484486558c7","last_execution":null,"name":"nightly-cleanup","next_execution":"2026-10-18T03:00:00Z","parameters":{},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-10-17T08:16:40.805872Z"}]}

--- Getting schedule ---
Fetched schedule: "nightly-cleanup"

--- Updating schedule ---
Updated cron: "0 0 4 * * *"
Updated description: "Remove expired sessions, temp files, and orphaned uploads"

--- Triggering schedule ---
Trigger response: {"schedule_id":"0171a7d8-eb74-4348-b09c-8484486558c7","status":"triggered"}

--- Pausing schedule ---
Schedule paused: {"created_at":"2026-10-17T08:16:40.805872Z","cron_expression":"0 0 4 * * *","description":"Remove expired sessions, temp files, and orphaned uploads","enabled":false,"function_label":"schedule_noop_rust_48372","id":"0171a7d8-eb74-4348-b09c-8484486558c7","last_execution":"2026-10-17T08:16:40.847516Z","name":"nightly-cleanup","next_execution":null,"parameters":{"type":"Object","value":{}},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-10-17T08:16:40.851284Z"}

--- Resuming schedule ---
Schedule resumed: {"created_at":"2026-10-17T08:16:40.805872Z","cron_expression":"0 0 4 * * *","description":"Remove expired sessions, temp files, and orphaned uploads","enabled":true,"function_label":"schedule_noop_rust_48372","id":"0171a7d8-eb74-4348-b09c-8484486558c7","last_execution":"2026-10-17T08:16:40.847516Z","name":"nightly-cleanup","next_execution":"2026-10-18T04:00:00Z","parameters":{"type":"Object","value":{}},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-10-17T08:16:40.854314Z"}

--- Deleting schedule ---
Schedule deleted successfully

=== All schedule operations completed ===
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("shL2yfLbSDSr6c1zUyMIyLDAGV78drW5JBBah33q6Vh52LfKb5LE-v9-_stclbjs1iL0flIy-4KFM5qUzoLUQA"))
✓ Inserted user 2: Some(String("OsZi_cg9A_-EWzqpLqVa2yd_BdRzXBWv3l_TSY5sWS0j3vaH32c9Zp84r7BIKaM4fCJ6QGg0n87Qy8RIoDFCEg"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - title: String
    (required)
  - status: String
  - age: Integer
  - email: String
    (required)

=== Listing Collections ===
✓ Total collections: 13
  Sample: ["chat_goals__ek0_testing", "chat_messages__ek0_testing", "schedules__ek0_testing", "chat_raw_completions__ek0_testing", "schema_client_rust"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
  2. Score: 13.2000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})

=== Search with a metadata pre-filter (category = programming) ===
✓ Found 2 results in category 'programming' (database/ai excluded)
  1. Some(Object {"type": String("String"), "value": String("Rust Programming")}) (category: Some(Object {"type": String("String"), "value": String("programming")}))
  2. Some(Object {"type": String("String"), "value": String("Python for Data Science")}) (category: Some(Object {"type": String("String"), "value": String("programming")}))

Execution time: 2ms
Paired vector search: document=Some("Rust Programming Answer"), query=Some("Rust Programming Question")
=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("R0LvzK_GFXeqntMhXJhpbRvkgoOyjmrGsBKuqEVoSBakzvHsdyURehrikrCQ10fbL9Nw5ykVWQdDHvVrDovSyQ")})

=== Find by ID ===
Found: Record({"value": Object({"value": Integer(42), "type": String("Integer")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "id": String("R0LvzK_GFXeqntMhXJhpbRvkgoOyjmrGsBKuqEVoSBakzvHsdyURehrikrCQ10fbL9Nw5ykVWQdDHvVrDovSyQ"), "created_at": Object({"type": String("DateTime"), "value": String("2026-10-17T08:19:41.277959+00:00")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "name": Object({"type": String("String"), "value": String("Test Record")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-10-17T08:19:41.277959+00:00")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"price": Object({"type": String("Float"), "value": Float(99.99)}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-10-17T08:19:41.277959+00:00")}), "id": String("R0LvzK_GFXeqntMhXJhpbRvkgoOyjmrGsBKuqEVoSBakzvHsdyURehrikrCQ10fbL9Nw5ykVWQdDHvVrDovSyQ"), "name": Object({"type": String("String"), "value": String("Test Record")}), "embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")})})]

=== Update Document ===
Updated: Record({"id": String("R0LvzK_GFXeqntMhXJhpbRvkgoOyjmrGsBKuqEVoSBakzvHsdyURehrikrCQ10fbL9Nw5ykVWQdDHvVrDovSyQ"), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "created_at": Object({"type": String("DateTime"), "value": String("2026-10-17T08:19:41.277959+00:00")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "name": Object({"type": String("String"), "value": String("Updated Record")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "value": Object({"type": String("Integer"), "value": Integer(100)}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: coGQDclSzpodVDSEDRXC2Gs-lXBz2rwbvmSX6TOOR8M_8wNML7Ej3m_Oqz4UYt5Uzos4vTVHQGZ6SOw-hS-A2A

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "coGQDclSzpodVDSEDRXC2Gs-lXBz2rwbvmSX6TOOR8M_8wNML7Ej3m_Oqz4UYt5Uzos4vTVHQGZ6SOw-hS-A2A"
  name: "WebSocket Test Record"
  value: 42
  active: true

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native_rs (N5C_yo5cAGDQBgpQvfa_K6ay90uVrw8goP_h9_X5_Bqlv5WAshcgQaEvEiyTjqyE58yJ6ZXBJN7-so8ZhSMNgg)

First call (cache miss - will fetch from GitHub API):
  Response time: 120ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 6ms
  Speedup: 20.0x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit_rs (A-TbBLghopaWG6RJKGuFerNDV3NMO5ekAR7QpY6MLQ_qt5Y3YYASBrPuOh4lH9ZKG20L4lWmJDuMFfqTkcITNQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_rs' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_rs (qI2s8Q_1YtblzLZSJBubNhPny8qj6XCwjZ-GXPE9VogmDnLbcvwTLAuyWBoHoWgG-IsqspevNiU5EIffNqmDPA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_rs' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache_rs (lU2ICMmu2lr3yf6yx9TEyK9qOR63j4dceHy0kd3R8iUjB8Am4yoQ4YaH3YdYgaJZ0UU3-BtoV-0WaQEvByhKwQ)
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

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_rs (D20wz4D4ZSArp5K7ujF4tDD3yBp121XRXsUdy2KopzHLDJ0P2G01QBJSFhUCho_y-mgz3nCiypFHdfGW8NmDwg)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 9ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 3z7_I75sne-5YWWoIYPfLMCbg4KIQOBbmqDkeKQZlkxIeCArB4jygrwFUFDtOKSfOTWoTV3QegOD83dElV33rQ
Created Bob: $500 - ID: b255IcUEEjfGGZfp5stJSwfn_71Gs7Q8iLrEAK6vDw-7uOfmF8BgjMCzi2pyDgAQu1YfHeYSgXRaWVByiaCtBw

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): c3a9b496-4427-4e6f-b07c-3a07c9cc0091

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
New transaction: e552af97-5aeb-4d40-915b-818c16e1d748
Updated Bob: $700 → $600 (in transaction)
Status before rollback: "Active"
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_user_functions`
✓ Client created

=== Create User Function ===
Created user function with ID: 9otS4N3Q2U2AdAeNSbwtuREp0b5dPfa1rtfuHL5d6C7EkrhAjhgY15Z0zXv-FeuHkmV6No3lXgWY7eZY4PDmGw

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_websocket_chat_stream`
=== WebSocket Chat Streaming Example (Rust) ===

✓ Authentication successful
✓ Created chat session: adkpoS6nptB2B4vM4zT0RAY5fPAepgrpOPzHu1oJoK8FDZH7smAFIZFkeCRVuNsehqWoO8e7ThopJH-zvm2JbQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Sending message: 'What is the capital of France?' ===
✓ Message accepted, streaming...

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: "N-SXYNKUbK3ujSjwjNsFvH67xcOHhgoOpMs6ir0x6VKIfh_dl4ef7lJXeQ3SZrkRzrGIGBgAXslU-_r4W35FSg"
Execution time: 958ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}
Context window: 1000000 tokens

Full response: {"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris....

✓ WebSocket chat streaming example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_websocket_subscribe`
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_rs' ===
✓ Subscribed (subscription_id: sub_b8ef74e7d3814477bf7e3954743e0ac0)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: V0yS4bjHi52MA2nMOKuwPwQSNAWnzTvgFz3yGQlegEOaguCqEEDLOw8rMjc0Ds40mgwpxmcPgR5WqaoHnNdQJg

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_rs"
     Record IDs: ["V0yS4bjHi52MA2nMOKuwPwQSNAWnzTvgFz3yGQlegEOaguCqEEDLOw8rMjc0Ds40mgwpxmcPgR5WqaoHnNdQJg"]
     Timestamp:  "2026-10-17T08:26:24.804715+00:00"

Inserting record 2...
✓ Inserted: 2_oLNUh8bOKLNlxQQj6VNEs5oAq-PGd9Avf3nd1RiMID3w0LrB6NkDXMzNpHY-1ROLtZL3_Xc0ETj4v5U0kySA

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["2_oLNUh8bOKLNlxQQj6VNEs5oAq-PGd9Avf3nd1RiMID3w0LrB6NkDXMzNpHY-1ROLtZL3_Xc0ETj4v5U0kySA"]

=== Unsubscribing ===
✓ Unsubscribed

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_rs'

✓ WebSocket subscription example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("dv0y1l6PzZyU5ACelD5S4snGVEUQEpqJ0dWnUMNYxw01IoYNoo8IPht06aziNNTxNrjE9J3qVgqWrUIe4TJeuQ"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/bypass_ripple_example`
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: Record({"id": String("vMHFY-VNxUB_cZFH9Wr5tJt7-BYuGjLMPfxWRULOosAhrsdGohy9u5JXcb0yJ7ISjWtZmwn8skHvvdISapvfgA")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("eQc4vUMpbeTeFD2NUWJTKB6BxOlp1zadQIc3LfsEpE5Am59MKw1Z_VXlyeU4kjyAb-2-QAJuvAa9qQX0XhH6EQ")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"price": Object({"type": String("Integer"), "value": Integer(150)}), "name": Object({"type": String("String"), "value": String("Product 1")}), "id": String("vMHFY-VNxUB_cZFH9Wr5tJt7-BYuGjLMPfxWRULOosAhrsdGohy9u5JXcb0yJ7ISjWtZmwn8skHvvdISapvfgA")})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("custom-id")})

✅ All bypass_ripple operations completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
  Fields returned: ["id", "avatar_url", "bio", "email", "name", "status", "user_role", "age", "created_at"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["id", "age", "status", "avatar_url", "user_role", "api_key", "password", "created_at", "bio", "name", "email", "secret_token"]
  Projected query:
    - 3 fields per record
    - Fields: ["name", "email", "id"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
