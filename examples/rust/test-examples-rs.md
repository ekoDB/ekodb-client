make test-examples-rust
🧪 Running Rust examples (direct HTTP/WebSocket)...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("6FEBJZbcQnUeO5uqW_pHxf-UqH4Ssh4COjrXAzUO5dGqzvstlI1Ks2OPEfIzdYlJ58NwuZMlFMM5tQtYcx9_MQ")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("6FEBJZbcQnUeO5uqW_pHxf-UqH4Ssh4COjrXAzUO5dGqzvstlI1Ks2OPEfIzdYlJ58NwuZMlFMM5tQtYcx9_MQ"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("6FEBJZbcQnUeO5uqW_pHxf-UqH4Ssh4COjrXAzUO5dGqzvstlI1Ks2OPEfIzdYlJ58NwuZMlFMM5tQtYcx9_MQ"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("6FEBJZbcQnUeO5uqW_pHxf-UqH4Ssh4COjrXAzUO5dGqzvstlI1Ks2OPEfIzdYlJ58NwuZMlFMM5tQtYcx9_MQ"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: vzKXXYsbLxLlz66EqpnJfwtf5ZFlw32fcVXU9FITUYxr0XvbEXTaMD6J34I2jkXliIkFU7Sdq3J-_wv-dJ-j3Q

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1789587979656746000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "vzKXXYsbLxLlz66EqpnJfwtf5ZFlw32fcVXU9FITUYxr0XvbEXTaMD6J34I2jkXliIkFU7Sdq3J-_wv-dJ-j3Q",
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
Collection created with first record: "c9a_UR6-DRJ7ng-X9UjYHVl2RWuaQRAqW4lBPVyHT7qcUW2WUQTakAve1VgAyG8sGhjWqBQavSGEYKUL-rCTJw"

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
✓ Inserted document: RTwoJD9QpZxBOs0vrY2G6Acgyb7K8iWGV61SHeYd_QmNtIWva9WJgO5jSdQ4cASI83ktbRMmxQBpYQQiQLWGow

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: yDotFN8dhBwNuR7MzEQd0TXLf28qk2FXcOTrSHpzSQaKQWaGzAW1IbPcQlfxfwt_-vIfMnPpRpkzLUF4YAWFBw

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: aC6IzqVPn5JJlIwdZMxnbxEDu2vdjmJskbWsM91GixHs8m7Ntkb9Zs7_AxVcOq3-32f9hzdXsnr6u3xU8GKAzQ

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
✓ Inserted document with TTL: String("RYr0JB-QinzUKHlaRoRKn8GPczCrzJ9ZkfH1CwV-TXKYNFNM1cdFZEUcvB0MdH6OzsVDG95ncmDqinCTEjW8Mw")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record via WebSocket

Record 1:
  id: "RYr0JB-QinzUKHlaRoRKn8GPczCrzJ9ZkfH1CwV-TXKYNFNM1cdFZEUcvB0MdH6OzsVDG95ncmDqinCTEjW8Mw"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-09-16T20:52:35.263156Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Functions Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: GsufPPw9hSFzqd7pdieHmpUERO266lLct4M7y3azLFussBqR_6I9yn2i7tCXzvVj5l8-KXuE5CJiS9rZEGoOQQ
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: 0GgML4jy0yCFkM8Fh8jqeSP50-_HlvGV6PEiaA_lgPk4r7duc-i72vzODUUu2tQpw09YtHfOKTi0v8n2Jl0ilw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Function saved: VxjT1Zg8XigIfDbWCtCWC7baN6dvkSFwqKxTBIjtzVC7sh2jDj3ycSiZP_D98vzDQBuz7oLP-NRf6oKM43HOiQ
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: no6-Ve7QP9dQH1QPm0HJDnFnBQjdYWoHUIwqL7vXV5QZC0qxLWtrIBcZwsAF_ylMEnO3xYHIbNANqdqM0q3SLw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":{"type":"Float","value":60.0},"count":{"type":"Integer","value":5},"max_score":{"type":"Integer","value":100},"status":{"type":"String","value":"active"}}
   {"avg_score":{"type":"Float","value":50.0},"count":{"type":"Integer","value":5},"max_score":{"type":"Integer","value":90},"status":{"type":"String","value":"inactive"}}

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
Created Alice: $1000 - ID: 7q5WAWmzAZuf5aI4S8YFv-4Auapzeh9W514k8E2ArcGJjjt-yjS9eXZRjxCPCUpafxjw0l4NJz6BQlnn-2POeg
Created Bob: $500 - ID: CdAkYUmsGqDaaF83Uss9BZZcZCjYDIWWgojHG0YkmXK_GbYLPVUpxlMTQe0bc08euyk1wAkhCjVfj3FVjCBS1g

=== Example 1: Begin Transaction ===
Transaction ID: 24d13bee-1b5d-4a24-8c8c-f8e342477fa6

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
New transaction: f4c6045c-9377-4424-8d9f-9aaadf51ae1f
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ Rust direct examples complete!
🛠️  Building client library...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
✅ Client build complete!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_advanced_crud`
=== ekoDB Advanced CRUD Example (Rust) ===

--- Inserting base document ---
Inserted: Widget Counter (id: cXTpb6KVCG9lHExTDYMOhEimqCIEg-3K3oOvSXVmGp8XABLKTVKzkeVCsGvI8JkDOrz1OajtMA5IKcrrdnyCmQ)

--- update_with_action: increment views by 10 ---
views after increment: Some(Object({"type": String("Integer"), "value": Integer(110)}))

--- update_with_action: decrement views by 3 ---
views after decrement: Some(Object({"type": String("Integer"), "value": Integer(107)}))

--- update_with_action: multiply score by 2 ---
score after multiply: Some(Object({"type": String("Float"), "value": Float(9.0)}))

--- update_with_action: push 'ekodb' to tags ---
tags after push: Some(Object({"value": Array([String("rust"), String("database"), String("ekodb")]), "type": String("Array")}))

--- update_with_action: append '-suffix' to label ---
label after append: Some(Object({"type": String("String"), "value": String("prefix-suffix")}))

--- update_with_action: pop last element from tags ---
tags after pop: Some(Object({"type": String("Array"), "value": Array([String("rust"), String("database")])}))

--- update_with_action: remove 'rust' from tags ---
tags after remove: Some(Object({"value": Array([String("database")]), "type": String("Array")}))

--- update_with_action_sequence: increment views + push tag + append label ---
views after sequence: Some(Object({"type": String("Integer"), "value": Integer(157)}))
tags after sequence: Some(Object({"type": String("Array"), "value": Array([String("database"), String("batch")])}))
label after sequence: Some(Object({"type": String("String"), "value": String("prefix-suffix-v2")}))

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
✓ Created session: F4D0-fLWITvW9DIMVXJ4jRuSwrweiX30vQjumHYFXnm5mifYCq0hM9lzX-YaMMNMgC_n0SIdGaEcC1GJbTU_Cg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more details or have other questions, feel free to ask!

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
✓ Created second session: W4zzRx_Ade7qJbQh_O1Qq_SUcdylCg2zafWVQT9oV9M8lKpFswSthoyEZBOnTdnqu16EvhUaid7k910ZIBXEGA
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: W4zzRx_Ade7qJbQh_O1Qq_SUcdylCg2zafWVQT9oV9M8lKpFswSthoyEZBOnTdnqu16EvhUaid7k910ZIBXEGA
✓ Deleted chat session: F4D0-fLWITvW9DIMVXJ4jRuSwrweiX30vQjumHYFXnm5mifYCq0hM9lzX-YaMMNMgC_n0SIdGaEcC1GJbTU_Cg
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: YAPEkCok-ZnyynHd_fGLa6MYWwT4LJIsHWP2VKzZvBrhyy5-Q6fFjsTa7ntXU5xj7PJW1SjuDWtKqFNHF_BZZg

=== Sending Chat Message ===
Message ID: JxRdeubIEnMTXIqmjMeKG1qQI5Mz4tcx0sPfVNaWtnWF-W-nl2P8MOLeDJkyWHcTdQMREqVV5PGAEHqfv6fHCQ

=== AI Response ===
Response 1: ekoDB is a high-performance database designed for intelligent caching, real-time capabilities, and integrating AI. Here are some key features of ekoDB:

1. **AI Chat Integration**: It allows you to query your database using natural language, providing AI-powered responses with relevant context.

2. **Search Capabilities**: ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval, making it versatile for different querying needs.

You can leverage these features to enhance your applications and make data interactions more natural and efficient.


=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("Qkp9_hAmQHdbXcxxs-Go8-uHqWFs24TVuFu9P3UVm61uDBT6VgjF1Y2P174AgR1J5bRGZLilmSvkAGi-Cx346A"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("mZf_cnHdTYCLBLdQmSy8Mznyfi7TyeEw4YZtPjKx7Z70u2fZOvdPoiUPmMnqNWhn4AoKVHoMlPyxJJgJfRPzIQ"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("bcB2xRpurqVqHLpmaeV9Pu8UrZzy3SMI-opIVWBj3lititZClERQPrD387AcuPmdtlKbs3eGZUEY9xV9-xfuJQ"), "title": String("Search Features")}

Execution Time: 2738ms

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_message_stream`
=== ekoDB Chat Message Stream (SSE) Example (Rust) ===

Created session: T3xnZLyi2r2O56lAqNrwe2VVd_jPva_Vxo1j479OYvSCEhkjj9_wCZJWrGsKb6nNG4XNSnicpLBzmuUd-nUGrQ

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}**ekoDB** refers to a knowledge base or database developed and maintained by **Eko**, a company that specializes in digital health technology. Eko is best known for its smart stethoscopes and digital auscultation tools used to record, visualize, and analyze heart and lung sounds. While there is no widely recognized, standalone product named "ekoDB," the term is sometimes used to refer to:

1. **Eko's Sound Database**:
   Eko has compiled a large, high-quality database of annotated heart and lung sounds—sometimes called “ekoDB” in research papers and technical materials. This database collects recordings from a diverse patient population, with each recording labeled by clinical experts for features such as murmurs, arrhythmias, crackles, etc. This database is used to train and validate artificial intelligence (AI) algorithms for automated cardiac and pulmonary sound analysis.

2. **Open Research Resource**:
   Eko has partnered with academic and healthcare institutions to share anonymized portions of its sound database (ekoDB) with researchers to promote innovation in digital auscultation and machine learning for healthcare.

**Key Points about ekoDB:**
- Contains tens of thousands of heart and lung sound recordings.
- Labeled with physician annotation for clinical findings.
- Used for developing, training, and validating AI diagnostic tools.
- Available to some researchers and partners for advancing medical AI and telehealth.

**References:**
- Eko Health official website: [ekohealth.com](https://www.ekohealth.com/)
- Research papers such as:
  - "A University-Scale Data Set for Cardiac Sound Classification" (published using ekoDB)
  - "Heart Murmur Detection Using a State-of-the-Art Digital Stethoscope" (uses ekoDB)

**Summary:**
ekoDB is a clinical sound database from Eko used for research and developing AI-based diagnostic solutions in digital auscultation.

--- Stream complete ---
Message ID: 5RfBrsrwwWs81hxKHXNBfqOFSAFc4L7zDmKt9b3zNVGH2qaibtbx47oiMfAHn2Jy2Voq-9ERIr30f3_IuZ6AAw
Execution time: 5030ms
Context window: 1000000 tokens

✓ Chat message stream example completed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_models`
✓ Client created

=== Get All Chat Models ===
Available chat models by provider:

OpenAI models (132):
  - text-embedding-ada-002
  - whisper-1
  - gpt-3.5-turbo
  - tts-1
  - gpt-3.5-turbo-16k
  - gpt-4-0613
  - gpt-4
  - davinci-002
  - babbage-002
  - gpt-3.5-turbo-instruct
  - gpt-3.5-turbo-instruct-0914
  - gpt-3.5-turbo-1106
  - tts-1-hd
  - tts-1-1106
  - tts-1-hd-1106
  - text-embedding-3-small
  - text-embedding-3-large
  - gpt-3.5-turbo-0125
  - gpt-4-turbo
  - gpt-4-turbo-2024-04-09
  - gpt-4o
  - gpt-4o-2024-05-13
  - gpt-4o-mini-2024-07-18
  - gpt-4o-mini
  - gpt-4o-2024-08-06
  - omni-moderation-latest
  - omni-moderation-2024-09-26
  - o1-2024-12-17
  - o1
  - o3-mini
  - o3-mini-2025-01-31
  - gpt-4o-2024-11-20
  - gpt-4o-mini-search-preview-2025-03-11
  - gpt-4o-mini-search-preview
  - gpt-4o-transcribe
  - gpt-4o-mini-transcribe
  - o1-pro-2025-03-19
  - o1-pro
  - gpt-4o-mini-tts
  - o3-2025-04-16
  - o4-mini-2025-04-16
  - o3
  - o4-mini
  - gpt-4.1-2025-04-14
  - gpt-4.1
  - gpt-4.1-mini-2025-04-14
  - gpt-4.1-mini
  - gpt-4.1-nano-2025-04-14
  - gpt-4.1-nano
  - gpt-image-1
  - o4-mini-deep-research
  - gpt-4o-transcribe-diarize
  - o4-mini-deep-research-2025-06-26
  - gpt-5-chat-latest
  - gpt-5-2025-08-07
  - gpt-5
  - gpt-5-mini-2025-08-07
  - gpt-5-mini
  - gpt-5-nano-2025-08-07
  - gpt-5-nano
  - gpt-audio-2025-08-28
  - gpt-realtime
  - gpt-realtime-2025-08-28
  - gpt-audio
  - gpt-5-codex
  - gpt-image-1-mini
  - gpt-5-pro-2025-10-06
  - gpt-5-pro
  - gpt-audio-mini
  - gpt-audio-mini-2025-10-06
  - gpt-5-search-api
  - gpt-realtime-mini
  - sora-2
  - sora-2-pro
  - gpt-5-search-api-2025-10-14
  - gpt-5.1-chat-latest
  - gpt-5.1-2025-11-13
  - gpt-5.1
  - gpt-5.1-codex
  - gpt-5.1-codex-mini
  - gpt-5.1-codex-max
  - gpt-image-1.5
  - gpt-5.2-2025-12-11
  - gpt-5.2
  - gpt-5.2-pro-2025-12-11
  - gpt-5.2-pro
  - gpt-5.2-chat-latest
  - gpt-4o-mini-transcribe-2025-12-15
  - gpt-4o-mini-transcribe-2025-03-20
  - gpt-4o-mini-tts-2025-03-20
  - gpt-4o-mini-tts-2025-12-15
  - gpt-realtime-mini-2025-12-15
  - gpt-audio-mini-2025-12-15
  - chatgpt-image-latest
  - gpt-5.2-codex
  - gpt-5.3-codex
  - gpt-realtime-1.5
  - gpt-audio-1.5
  - gpt-4o-search-preview
  - gpt-4o-search-preview-2025-03-11
  - gpt-5.3-chat-latest
  - gpt-5.4-2026-03-05
  - gpt-5.4-pro
  - gpt-5.4-pro-2026-03-05
  - gpt-5.4
  - gpt-5.4-nano-2026-03-17
  - gpt-5.4-nano
  - gpt-5.4-mini-2026-03-17
  - gpt-5.4-mini
  - gpt-image-2
  - gpt-image-2-2026-04-21
  - gpt-5.5
  - gpt-5.5-2026-04-23
  - gpt-5.5-pro
  - gpt-5.5-pro-2026-04-23
  - chat-latest
  - gpt-realtime-translate
  - gpt-realtime-2
  - gpt-realtime-whisper
  - gpt-5.6-sol
  - gpt-5.6-terra
  - gpt-5.6-luna
  - gpt-realtime-2.1
  - gpt-realtime-2.1-mini
  - gpt-transcribe
  - gpt-live-transcribe
  - gpt-6-astra
  - gpt-image-2.5-flare
  - gpt-image-2.5-sunburst
  - gpt-image-2.5-flare-2026-09-08
  - gpt-image-2.5-sunburst-2026-09-08
  - gpt-live-1

Anthropic models (11):
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
  anthropic: ok 11 models
  gemini: not_configured (unverified) No Gemini API Key
  openai: ok 132 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Models for Specific Provider ===

openai models (132):
  - text-embedding-ada-002
  - whisper-1
  - gpt-3.5-turbo
  - tts-1
  - gpt-3.5-turbo-16k
  ... and 127 more

anthropic models (11):
  - claude-fable-5-1
  - claude-opus-5
  - claude-sonnet-5
  - claude-fable-5
  - claude-opus-4-8
  ... and 6 more
GetChatModel(perplexity) error: Record not found

✓ Chat Models API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: _67apxP34co1JFZjZXW-CFO4FYqO6Zwj_2eiz9HQYmNYML8alJANXlogV9sWIKdy7EBn-8LxHAoqWvfkZGFVkg
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Product:** ekoDB
- **Description:** A high-performance database product with AI capabilities
- **Price:** $99

If you need more details or additional products, please let me know!

✓ Message 2 sent
  Response: The price of the product ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: MeREZRFzRiStK3rVnIK6okjwS1qvbYSLd_AJobjLiqclq-ZgfNSo6Ep_67XHJm_KS2LCXeNkv5mvAbn2qJCawQ
  Parent: _67apxP34co1JFZjZXW-CFO4FYqO6Zwj_2eiz9HQYmNYML8alJANXlogV9sWIKdy7EBn-8LxHAoqWvfkZGFVkg

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: MeREZRFzRiStK3rVnIK6okjwS1qvbYSLd_AJobjLiqclq-ZgfNSo6Ep_67XHJm_KS2LCXeNkv5mvAbn2qJCawQ (Untitled)
  Session 2: _67apxP34co1JFZjZXW-CFO4FYqO6Zwj_2eiz9HQYmNYML8alJANXlogV9sWIKdy7EBn-8LxHAoqWvfkZGFVkg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: MeREZRFzRiStK3rVnIK6okjwS1qvbYSLd_AJobjLiqclq-ZgfNSo6Ep_67XHJm_KS2LCXeNkv5mvAbn2qJCawQ

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "ztalfV1wdo1bV3H--BIAnyyNmW-PwNuGHa-kdABF6DI99P4Rn7OPaTu-TDMzAFlRU986J8NqZEwic_ZwJlVeMQ"

=== List Collections ===
Total collections: 7
Sample collections: ["agent_function_versions__ek0_testing", "client_collection_management_rust", "chat_turns__ek0_testing", "audit__ek0_testing", "chat_messages__ek0_testing"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
  - collection_utils_test_rust <-- our test
  - agent_function_versions__ek0_testing
  - chat_turns__ek0_testing
  - audit__ek0_testing
  - chat_messages__ek0_testing
  - functions__ek0_testing
  - chat_configurations__ek0_testing

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_rust'

✓ Collection Utilities example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
✓ Created record with fluent builder: Record({"id": String("O5zfbL0DfpRNNTLxBuAnvFjD50XBqqKobrpUCpCz2Mm_WfIKX-JopGtjgU37JzeWP7FkAIY5WOqq8bYlhrtgzg")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("55pVAYY7QR3bBOtu66ofnYzaraghlBINl_5zyCBqeys58aBDTHiaV4veO8J9qmxXC1OqjCDcLcifiXn9z7FBRA")})
✓ Second upsert (update): Record({"age": Object({"type": String("Integer"), "value": Integer(36)}), "name": Object({"type": String("String"), "value": String("Bob Smith")}), "email": Object({"type": String("String"), "value": String("bob.smith@newdomain.com")}), "id": String("55pVAYY7QR3bBOtu66ofnYzaraghlBINl_5zyCBqeys58aBDTHiaV4veO8J9qmxXC1OqjCDcLcifiXn9z7FBRA")})

=== Find One Operation ===
✓ Found user by email: Record({"id": String("O5zfbL0DfpRNNTLxBuAnvFjD50XBqqKobrpUCpCz2Mm_WfIKX-JopGtjgU37JzeWP7FkAIY5WOqq8bYlhrtgzg"), "age": Object({"type": String("Integer"), "value": Integer(28)}), "email": Object({"type": String("String"), "value": String("alice@example.com")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "name": Object({"type": String("String"), "value": String("Alice Johnson")})})
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
✓ Inserted document: "AQcATJP9DZD4yB8udszhwUSNA3gNUJAqaZ93cHNi0zMKjxThBOqPzebM88JET40dslJR0HsJjY1K0ILg0DKT5w"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("seiahmDjLalD8M-8wkcP8AjFfx8h3EZco0IQXD2xzVx3wU2n7o4R8tIIXZVFI3Iss4gJgesfVdFSH3L8ck9A2w"))

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
✓ Edge cache script created: AQDRHFie-EguPxGiOJ5jGNR6L0H5SWhipfVXXKqgvnT6eQX-N6uD_R7fogwDgrOJV3OIM2TWG3GCxfMiIYtmEg

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
   ⏱️  Duration: 84.873166ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "email": "Sincere@april.biz",
      "address": {
        "zipcode": "92998-3874",
        "geo": {
          "lat": "-37.3159",
          "lng": "81.1496"
        }...

Second call (cache hit - from cache):
   ⏱️  Duration: 2.854542ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "website": "hildegard.org",
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net",
        "name": "Romaguera-Cro...
   🚀 Cache speedup: 29.7x faster!

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

✅ Function saved: rY1FlKeNOep4Ttx8dF4lt469_MVaThQgGC6cMzEna3q2e0ilTcAiyBEOy2QXfyJYMF55uZS0KoQigr7QCZDnyQ
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: WWs9-ifjQx0l-ubQe9ApIa2xQkWi-nRLRgNFl2jPaeLlGGH2x4pJUQRuaJ-ljh-J0ZU5wQiaoADDL58UfAUjFw
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: qXPqR3RYIPVkpAblNjB7ZmFBKhipycqB0XVK75W7SExKDa00cjIGHDWYatL5V02wNgXFhP2uXQ4Bd4hkJy3dhw
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
   Record({"category": Object({"type": String("String"), "value": String("Electronics")}), "count": Object({"type": String("Integer"), "value": Integer(5)}), "avg_price": Object({"value": Float(367.0), "type": String("Float")})})
   Record({"count": Object({"type": String("Integer"), "value": Integer(3)}), "avg_price": Object({"type": String("Float"), "value": Float(365.6666666666667)}), "category": Object({"type": String("String"), "value": String("Furniture")})})
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

1. **Efficient Similarity Search**: They excel at retrieving similar items based on vector representations, making them ideal for tasks like image retrieval and recommendation systems.

2. **Scalability**: Designed to handle large volumes of high-dimensional data efficiently, allowing for quick indexing and retrieval.

3. **Real-time Queries**: Support for fast, real-time querying and updates, essential for applications needing immediate results.

4. **Handling Unstructured Data**: Can process unstructured data like text, images, and audio by converting them into vector embeddings.

5. **Integration with Machine Learning**: Works well with ML models to facilitate tasks such as classification, clustering, and anomaly detection.

6. **Flexibility**: Support for various distance metrics (e.g., cosine, Euclidean) to suit different application needs.

7. **Enhanced Search Capabilities**: Offers semantic search capabilities beyond traditional keyword-based approaches, providing contextually relevant results.

8. **Improved User Experience**: Enables more personalized and relevant responses in applications, enhancing user satisfaction.
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_complete`
🚀 ekoDB Rust Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: jZxqirrcDXe5QpXcyA_vTTaaG0YW8fdIlNVhzf64KbYTRIJxq8lWnM8ymQM6w5MbpeElv2XcsU5hCb6efNzJfA
📊 Found 2 product groups
   Record({"count": Object({"type": String("Integer"), "value": Integer(3)}), "avg_price": Object({"type": String("Float"), "value": Float(575.6666666666666)}), "category": Object({"value": String("Electronics"), "type": String("String")})})
   Record({"category": Object({"type": String("String"), "value": String("Furniture")}), "count": Object({"type": String("Integer"), "value": Integer(2)}), "avg_price": Object({"value": Float(474.0), "type": String("Float")})})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   Record({"count": Object({"value": Integer(2), "type": String("Integer")}), "category": Object({"value": String("Furniture"), "type": String("String")})})
   Record({"count": Object({"type": String("Integer"), "value": Integer(3)}), "category": Object({"type": String("String"), "value": String("Electronics")})})
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
   Record({"count": Object({"value": Integer(7), "type": String("Integer")}), "status": Object({"value": String("active"), "type": String("String")})})
   Record({"status": Object({"type": String("String"), "value": String("inactive")}), "count": Object({"type": String("Integer"), "value": Integer(3)})})
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

✅ Inserted order: Some(String("N1qy_7is3Mp8xkcPgvzlGxIVOjzVYXoKRiyNVxWDK_lp_zTOd_792z4BuaFLpGlsX-eTQh0PCdJtCRIYUOcYRQ"))
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: mva4g8h8dH6mub4VKmU4Vtb8QUD0Xa4fuzjUXJDRY3FFcMTf6JR7zS3X6JP5QcypmXfHa5nplmWJS3sW7QCUsA
📊 function executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"type": String("Object"), "value": Object {"role": String("admin"), "userId": String("user_abc")}})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: 2v4KpleNLu12qSswBa19EOCjIBkH14t6HBMAodzsIMiQMt73DWX0KvjKFDlfXr9OUAnfuEYl85X-yaLAhHaVnQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: jSR6_ntKHqt2Z5iweJ3XVyUnSTKI0pBjBgkpDPKHr3OFANAiy3GcIUgzhJmgTtRuEZN5t14Ch3joTWZj2dEGnQ
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
   2. Vector Databases Explained (Database)
   3. Introduction to Machine Learning (AI)
   4. Natural Language Processing (AI)
   5. Database Design Principles (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   Record({"count": Object({"value": Integer(2), "type": String("Integer")}), "category": Object({"type": String("String"), "value": String("AI")})})
   Record({"count": Object({"type": String("Integer"), "value": Integer(3)}), "category": Object({"value": String("Database"), "type": String("String")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_goal_templates`
=== ekoDB Goal Template CRUD Example (Rust) ===

--- Creating goal template ---
Created template: Data Migration (id: 2GfFEWlYRQw5A09Hr_QHsxDZuqeEXjv1L2M0ccBRYny4akOR8wdR77f4BJz8mn872lC_X0nFMYAvp5nT-l-QGw)

--- Listing templates ---
Templates: {"count":1,"items":[{"description":{"type":"String","value":"Template for migrating data between schemas"},"id":"2GfFEWlYRQw5A09Hr_QHsxDZuqeEXjv1L2M0ccBRYny4akOR8wdR77f4BJz8mn872lC_X0nFMYAvp5nT-l-QGw","steps":{"type":"Array","value":[{"description":"Analyze source schema"},{"description":"Create target schema"},{"description":"Migrate records"},{"description":"Validate results"}]},"title":{"type":"String","value":"Data Migration"}}]}

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
Created goal: "Deploy v2.0 to production" (id: pMr_P3hffv2XhVmQqGAdBkrNqaCxeIC2RiNGJuNEmLZH96kj9sGKud87wcgcSkowon_L4LJHP7Crx__vhtYW7w)

--- Goal: list ---
Goals: {"count":1,"goals":[{"created_at":"2026-09-16T20:20:38.360522+00:00","description":"Full release cycle for version 2.0","id":"pMr_P3hffv2XhVmQqGAdBkrNqaCxeIC2RiNGJuNEmLZH96kj9sGKud87wcgcSkowon_L4LJHP7Crx__vhtYW7w","status":"pending","steps":"[{\"description\":\"Run integration tests\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]","title":"Deploy v2.0 to production","updated_at":"2026-09-16T20:20:38.360522+00:00"}]}

--- Goal: get ---
Fetched goal: {"type":"String","value":"Deploy v2.0 to production"}

--- Goal: update ---
Updated description: {"type":"String","value":"Updated: full v2.0 release with rollback plan"}

--- Goal: search ---
Search results: {"count":1,"items":[{"_score":12.87,"created_at":{"type":"DateTime","value":"2026-09-16T20:20:38.360522+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"pMr_P3hffv2XhVmQqGAdBkrNqaCxeIC2RiNGJuNEmLZH96kj9sGKud87wcgcSkowon_L4LJHP7Crx__vhtYW7w","status":{"type":"String","value":"pending"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-09-16T20:20:38.402160+00:00"}}]}

--- Goal step: start (step 0) ---
Step 0 started: {"created_at":{"type":"DateTime","value":"2026-09-16T20:20:38.360522+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"pMr_P3hffv2XhVmQqGAdBkrNqaCxeIC2RiNGJuNEmLZH96kj9sGKud87wcgcSkowon_L4LJHP7Crx__vhtYW7w","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"status\":\"InProgress\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-09-16T20:20:38.412332+00:00"}}

--- Goal step: complete (step 0) ---
Step 0 completed: {"created_at":{"type":"DateTime","value":"2026-09-16T20:20:38.360522+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"pMr_P3hffv2XhVmQqGAdBkrNqaCxeIC2RiNGJuNEmLZH96kj9sGKud87wcgcSkowon_L4LJHP7Crx__vhtYW7w","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-09-16T20:20:38.419545+00:00"}}

--- Goal step: start (step 1) ---
Step 1 started

--- Goal step: fail (step 1) ---
Step 1 failed: {"created_at":{"type":"DateTime","value":"2026-09-16T20:20:38.360522+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"pMr_P3hffv2XhVmQqGAdBkrNqaCxeIC2RiNGJuNEmLZH96kj9sGKud87wcgcSkowon_L4LJHP7Crx__vhtYW7w","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-09-16T20:20:38.433342+00:00"}}

--- Goal: complete ---
Goal completed (pending review): {"completed_at":{"type":"DateTime","value":"2026-09-16T20:20:38.440196+00:00"},"created_at":{"type":"DateTime","value":"2026-09-16T20:20:38.360522+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"pMr_P3hffv2XhVmQqGAdBkrNqaCxeIC2RiNGJuNEmLZH96kj9sGKud87wcgcSkowon_L4LJHP7Crx__vhtYW7w","status":{"type":"String","value":"pending_review"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"summary":{"type":"String","value":"Partial completion — step 1 failed"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-09-16T20:20:38.440196+00:00"}}

--- Goal: approve ---
Goal approved: {"completed_at":{"type":"DateTime","value":"2026-09-16T20:20:38.440196+00:00"},"created_at":{"type":"DateTime","value":"2026-09-16T20:20:38.360522+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"pMr_P3hffv2XhVmQqGAdBkrNqaCxeIC2RiNGJuNEmLZH96kj9sGKud87wcgcSkowon_L4LJHP7Crx__vhtYW7w","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"summary":{"type":"String","value":"Partial completion — step 1 failed"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-09-16T20:20:38.447125+00:00"}}

--- Goal: create (for rejection) ---
Created goal2: "Refactor auth module" (id: NlDaez3SW9NeI8ct0XGh3KUf-kz5F9r9uTTIY787X6TIo3RhEjuB1ZcDjuVb4gN0vwAtZhgef_YqPKi_9IOrXw)

--- Goal: complete (goal2) ---

--- Goal: reject ---
Goal rejected: {"completed_at":{"type":"DateTime","value":"2026-09-16T20:20:38.466951+00:00"},"created_at":{"type":"DateTime","value":"2026-09-16T20:20:38.453892+00:00"},"description":{"type":"String","value":"Rewrite JWT handling"},"id":"NlDaez3SW9NeI8ct0XGh3KUf-kz5F9r9uTTIY787X6TIo3RhEjuB1ZcDjuVb4gN0vwAtZhgef_YqPKi_9IOrXw","reason":{"type":"String","value":"Breaks backward compatibility"},"status":{"type":"String","value":"failed"},"steps":{"type":"String","value":"[]"},"summary":{"type":"String","value":"Completed refactor"},"title":{"type":"String","value":"Refactor auth module"},"updated_at":{"type":"DateTime","value":"2026-09-16T20:20:38.466951+00:00"}}

--- Task: create ---
Created task: Nightly backup (id: QSPC8osd5fYLo7RyleME9bLUdkMnrCuHaO2wIq30vP3w6foiq62tJ3_NyEuDXkZB6u8LxduhqMZBa01D0xCzKg)

--- Task: list ---
Tasks: {"count":1,"items":[{"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"QSPC8osd5fYLo7RyleME9bLUdkMnrCuHaO2wIq30vP3w6foiq62tJ3_NyEuDXkZB6u8LxduhqMZBa01D0xCzKg","max_retries":{"type":"Integer","value":3},"schedule":{"type":"String","value":"0 2 * * *"},"title":{"type":"String","value":"Nightly backup"}}]}

--- Task: get ---
Fetched task: {"type":"String","value":"Nightly backup"}

--- Task: start ---
Task started: {"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"QSPC8osd5fYLo7RyleME9bLUdkMnrCuHaO2wIq30vP3w6foiq62tJ3_NyEuDXkZB6u8LxduhqMZBa01D0xCzKg","max_retries":{"type":"Integer","value":3},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"running"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-09-16T20:20:38.501302+00:00"}}

--- Task: succeed ---
Task succeeded: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"QSPC8osd5fYLo7RyleME9bLUdkMnrCuHaO2wIq30vP3w6foiq62tJ3_NyEuDXkZB6u8LxduhqMZBa01D0xCzKg","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-09-16T20:20:38.507945+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-09-16T20:20:38.507945+00:00"}}

--- Task: start (round 2) ---

--- Task: pause ---
Task paused: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"QSPC8osd5fYLo7RyleME9bLUdkMnrCuHaO2wIq30vP3w6foiq62tJ3_NyEuDXkZB6u8LxduhqMZBa01D0xCzKg","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-09-16T20:20:38.507945+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"paused"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-09-16T20:20:38.521502+00:00"}}

--- Task: resume ---
Task resumed: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"QSPC8osd5fYLo7RyleME9bLUdkMnrCuHaO2wIq30vP3w6foiq62tJ3_NyEuDXkZB6u8LxduhqMZBa01D0xCzKg","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-09-16T20:20:38.507945+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-09-16T20:20:38.528670+00:00"}}

--- Task: fail ---
Task failed: {"consecutive_failures":{"type":"Integer","value":1},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"QSPC8osd5fYLo7RyleME9bLUdkMnrCuHaO2wIq30vP3w6foiq62tJ3_NyEuDXkZB6u8LxduhqMZBa01D0xCzKg","last_error":{"type":"String","value":"S3 bucket access denied"},"last_run":{"type":"DateTime","value":"2026-09-16T20:20:38.535900+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-09-16T20:20:38.535900+00:00"}}

--- Task: due ---
Due tasks: {"count":0,"items":[]}

--- Task: delete ---
Task deleted

--- Agent: create ---
Created agent: backup-agent (id: oxl39uQJxoyyIwsv8FMCU9_tyifaHbJXfabw_RDdITNJe_vBOfS9qgHYXfkDeabqNYeMBYAX5gHYtFF0waE7Cg)

--- Agent: list ---
Agents: {"count":1,"items":[{"capabilities":{"type":"Array","value":["backup","restore","verify"]},"deployment_id":{"type":"String","value":"deploy-abc-123"},"description":{"type":"String","value":"Handles nightly backups and restores"},"id":"oxl39uQJxoyyIwsv8FMCU9_tyifaHbJXfabw_RDdITNJe_vBOfS9qgHYXfkDeabqNYeMBYAX5gHYtFF0waE7Cg","llm_model":{"type":"String","value":"gpt-4.1"},"name":{"type":"String","value":"backup-agent"}}]}

--- Agent: get ---
Fetched agent: {"type":"String","value":"backup-agent"}

--- Agent: get_by_name ---
Found by name: {"type":"String","value":"backup-agent"}

--- Agent: update ---
Updated agent description: {"type":"String","value":"Handles backups, restores, and disaster recovery"}

--- Agent: agents_by_deployment ---
Agents in deployment: {"count":0,"items":[]}
WARNING: agents_by_deployment omitted created agent oxl39uQJxoyyIwsv8FMCU9_tyifaHbJXfabw_RDdITNJe_vBOfS9qgHYXfkDeabqNYeMBYAX5gHYtFF0waE7Cg; TODO: check/fix the server-side deployment lookup

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_kv_links`
=== ekoDB KV Document Linking Example (Rust) ===

--- Inserting documents ---
Inserted doc1: Project Alpha (id: LacJGxxhBCaKIOHtZ99F1Z6FxR-t85_qhU-X0251q6ZsUEM2F-QmoJDuRTT1nYIpM-OMoI25BXixKVtlAojOrQ)
Inserted doc2: Project Beta (id: ZMbK40zpPCIGy8BvTzJPTnRXsTVOu_OWmk5kshGBOlWcsNvVz2jwNKs8RO3eC3nkI9nnstrrpWPvtIeXlmY5HA)

--- Setting KV key ---
Set key: kv_links:rs:user:alice:projects

--- Linking documents ---
Linked doc1: null
Linked doc2: null

--- Getting links ---
Links for kv_links:rs:user:alice:projects: [{"collection":"kv_links_example_rs","created_at":"2026-09-16T20:24:02.699585Z","document_id":"LacJGxxhBCaKIOHtZ99F1Z6FxR-t85_qhU-X0251q6ZsUEM2F-QmoJDuRTT1nYIpM-OMoI25BXixKVtlAojOrQ","field_path":null,"last_accessed":"2026-09-16T20:24:02.700985Z","metadata":{}},{"collection":"kv_links_example_rs","created_at":"2026-09-16T20:24:02.700345Z","document_id":"ZMbK40zpPCIGy8BvTzJPTnRXsTVOu_OWmk5kshGBOlWcsNvVz2jwNKs8RO3eC3nkI9nnstrrpWPvtIeXlmY5HA","field_path":null,"last_accessed":"2026-09-16T20:24:02.700985Z","metadata":{}}]

--- Unlinking doc1 ---
Unlinked doc1: null

--- Verifying remaining links ---
Remaining links: [{"collection":"kv_links_example_rs","created_at":"2026-09-16T20:24:02.700345Z","document_id":"ZMbK40zpPCIGy8BvTzJPTnRXsTVOu_OWmk5kshGBOlWcsNvVz2jwNKs8RO3eC3nkI9nnstrrpWPvtIeXlmY5HA","field_path":null,"last_accessed":"2026-09-16T20:24:02.702389Z","metadata":{}}]

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

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
  1. Some(Object({"value": String("Bob"), "type": String("String")}))
  2. Some(Object({"type": String("String"), "value": String("David")}))
  3. Some(Object({"value": String("Alice"), "type": String("String")}))

=== Pagination (page 2, size 2) ===
✓ Page 2 results:
  - Some(Object({"value": String("Charlie"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_schedules`
=== ekoDB Schedule Management Example (Rust) ===

--- Creating schedule ---
Created schedule: "nightly-cleanup" (id: 6e47d965-f6f1-444a-9fe7-f405e554b4df)

--- Listing schedules ---
Schedules: {"count":1,"schedules":[{"created_at":"2026-09-16T20:29:45.953811Z","cron_expression":"0 0 3 * * *","description":"Remove expired sessions and temp files","enabled":true,"function_label":"schedule_noop_rust_48376","id":"6e47d965-f6f1-444a-9fe7-f405e554b4df","last_execution":null,"name":"nightly-cleanup","next_execution":"2026-09-17T03:00:00Z","parameters":{},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-09-16T20:29:45.953811Z"}]}

--- Getting schedule ---
Fetched schedule: "nightly-cleanup"

--- Updating schedule ---
Updated cron: "0 0 4 * * *"
Updated description: "Remove expired sessions, temp files, and orphaned uploads"

--- Triggering schedule ---
Trigger response: {"schedule_id":"6e47d965-f6f1-444a-9fe7-f405e554b4df","status":"triggered"}

--- Pausing schedule ---
Schedule paused: {"created_at":"2026-09-16T20:29:45.953811Z","cron_expression":"0 0 4 * * *","description":"Remove expired sessions, temp files, and orphaned uploads","enabled":false,"function_label":"schedule_noop_rust_48376","id":"6e47d965-f6f1-444a-9fe7-f405e554b4df","last_execution":"2026-09-16T20:29:45.982718Z","name":"nightly-cleanup","next_execution":null,"parameters":{"type":"Object","value":{}},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-09-16T20:29:45.985530Z"}

--- Resuming schedule ---
Schedule resumed: {"created_at":"2026-09-16T20:29:45.953811Z","cron_expression":"0 0 4 * * *","description":"Remove expired sessions, temp files, and orphaned uploads","enabled":true,"function_label":"schedule_noop_rust_48376","id":"6e47d965-f6f1-444a-9fe7-f405e554b4df","last_execution":"2026-09-16T20:29:45.982718Z","name":"nightly-cleanup","next_execution":"2026-09-17T04:00:00Z","parameters":{"type":"Object","value":{}},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-09-16T20:29:45.988255Z"}

--- Deleting schedule ---
Schedule deleted successfully

=== All schedule operations completed ===
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("zgjxAOPOkOIbnY8pWADa7yfeTtXhPY-sVThJQmtjqhvURIRyswbuONx2f7Rkmj2CCnll35h-THcoCcgcVrNi4w"))
✓ Inserted user 2: Some(String("CmApzEP2pD81-mlzWS39XfT8fbjPCPhw0B0WgPcoSsskq_Qer_bJ6X7tdcCfdsrqsz4sgCK2fsba2lkbE4hrCA"))

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
  Sample: ["chat_goals__ek0_testing", "schedules__ek0_testing", "schema_client_rust", "agent_function_versions__ek0_testing", "chat_raw_completions__ek0_testing"]

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
  2. Score: 13.2000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})

=== Search with a metadata pre-filter (category = programming) ===
✓ Found 2 results in category 'programming' (database/ai excluded)
  1. Some(Object {"type": String("String"), "value": String("Python for Data Science")}) (category: Some(Object {"type": String("String"), "value": String("programming")}))
  2. Some(Object {"type": String("String"), "value": String("Rust Programming")}) (category: Some(Object {"type": String("String"), "value": String("programming")}))

Execution time: 2ms
=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("tvT2dKJMeEfbBInNpRXgT69ie3AMyoXhQxv1ZfFpZRgNvdtii2PCVyDCtvZyww_Phb394Wa8Dcj8JfLU98KVfg")})

=== Find by ID ===
Found: Record({"name": Object({"value": String("Test Record"), "type": String("String")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-09-16T20:32:48.012295+00:00")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "id": String("tvT2dKJMeEfbBInNpRXgT69ie3AMyoXhQxv1ZfFpZRgNvdtii2PCVyDCtvZyww_Phb394Wa8Dcj8JfLU98KVfg"), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "price": Object({"value": Float(99.99), "type": String("Float")}), "value": Object({"type": String("Integer"), "value": Integer(42)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-09-16T20:32:48.012295+00:00")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"id": String("tvT2dKJMeEfbBInNpRXgT69ie3AMyoXhQxv1ZfFpZRgNvdtii2PCVyDCtvZyww_Phb394Wa8Dcj8JfLU98KVfg"), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "price": Object({"value": Float(99.99), "type": String("Float")}), "value": Object({"value": Integer(42), "type": String("Integer")}), "name": Object({"value": String("Test Record"), "type": String("String")}), "created_at": Object({"value": String("2026-09-16T20:32:48.012295+00:00"), "type": String("DateTime")}), "embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "active": Object({"type": String("Boolean"), "value": Boolean(true)})})]

=== Update Document ===
Updated: Record({"user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-09-16T20:32:48.012295+00:00")}), "id": String("tvT2dKJMeEfbBInNpRXgT69ie3AMyoXhQxv1ZfFpZRgNvdtii2PCVyDCtvZyww_Phb394Wa8Dcj8JfLU98KVfg"), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "value": Object({"type": String("Integer"), "value": Integer(100)}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "price": Object({"type": String("Float"), "value": Float(99.99)}), "name": Object({"value": String("Updated Record"), "type": String("String")})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 9FZcjPDA48f3nvU9w0iOCuA6dW8JQSG8tA6G9nSZ8igJ92KM9jMfCJvE7ZNGC85j_WNUhh5WPBrYHA63ATP5VA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  name: "WebSocket Test Record"
  id: "9FZcjPDA48f3nvU9w0iOCuA6dW8JQSG8tA6G9nSZ8igJ92KM9jMfCJvE7ZNGC85j_WNUhh5WPBrYHA63ATP5VA"
  active: true
  value: 42

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native_rs (4CwiTTV9zpAkuo-Vb7Qz1-49j3Eb9hLwxwG19U5XrMio2NKsKbB-V6F4TI79cKBjb3mAVFaJPTsHs-KCgsQF-g)

First call (cache miss - will fetch from GitHub API):
  Response time: 169ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 5ms
  Speedup: 33.8x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit_rs (dYhKXIcfWZCmcgDiYqq5vcDYgicV7ZTB3NyxaDBppyLItcrwJGQastG0VqHF9YOmNz2FcmyK-xszznhZsReh8Q)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_rs' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_rs (-mcesHyYMjqOLJ2_KHdtLhOE1b21esztj0Iym8RU6jH81R9Ej-tAqXeV-A4AlMgXJpOXqo7_gIbbZEZ3RQbx5w)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_rs' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache_rs (36SQ9pxPKrMDVscmlTTQdNWoaKzTl0gGQirmnQl_febkz-UBdQfNebgJz-wXaf3nTHjIvIw59vQynK1DDjN2LQ)
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

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_rs (JfjjIBF5UgF4DMhyyaSWC6ZxItiO1w3lWz2U24F1PVyEoxpV5wuZW83QeaewjvvviJQx5n_PscBGjFebV6quYw)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 3ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: DtMj_mG-AJOsCJSjyTJJXes28yhnh57XwB4B-OE9yQoZSvU8M2jOOc-7pvhXxdXGpeFwbkurSYXy5GTagATxaQ
Created Bob: $500 - ID: CntzzaVK84pWyzeMwA9OI1ASN5_N52_39dIoxTB3QCRRl5KaqCa0Iz0YcMC17XGEjg4k2b_YBdWtCMM_kiD0eA

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 95597353-d77f-44cd-a82b-b2373b031061

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
New transaction: 9211e85b-f91e-49a5-a2b7-c7d6a501c255
Updated Bob: $700 → $600 (in transaction)
Status before rollback: "Active"
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_user_functions`
✓ Client created

=== Create User Function ===
Created user function with ID: PYC3-ndog8dc8KjyJCjuXs9BpecBCXYZzl7MRqq3vdzooiUxwQoCDNeBDynN2MRcrdGtTwP0tNwXsF7RJzncdQ

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
✓ Created chat session: tBlwv1YCayD6d4aDNNRijzDIusrZW90hvOGbBsq8ZokyHP_SSQe2q61OwqMjlxkX8JtDVJe21nPUKq4586S-Lw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Sending message: 'What is the capital of France?' ===
✓ Message accepted, streaming...

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: "22-dqVbz_hR9Hgk2UxPui6Gu1fuQvVMOP43Dv_SNbgIw_TkQC8tVG-zjvRzAk3chwDX49HwQDdTL6XOFD0jFhw"
Execution time: 731ms
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
✓ Subscribed (subscription_id: sub_9883a255f3064fcb9e69c6093cf3c8b3)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: AB-kLSp0n_oFFMCpdCqAvFsv5pB3AOC-44Li5NIIusMsZkQFLv2EzNlOLgeJf__4kBufqoYRTjzaTXHuW9VS3w

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_rs"
     Record IDs: ["AB-kLSp0n_oFFMCpdCqAvFsv5pB3AOC-44Li5NIIusMsZkQFLv2EzNlOLgeJf__4kBufqoYRTjzaTXHuW9VS3w"]
     Timestamp:  "2026-09-16T20:40:00.201971+00:00"

Inserting record 2...
✓ Inserted: exGmD9qeeMEBqmvd2bQvIdCrubVNMEAE-mwO9ZNzYujbeZdwq1Qja9BsmmYhaaAz0vkyRt4CcrynJK4r5TEf6w

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["exGmD9qeeMEBqmvd2bQvIdCrubVNMEAE-mwO9ZNzYujbeZdwq1Qja9BsmmYhaaAz0vkyRt4CcrynJK4r5TEf6w"]

=== Unsubscribing ===
✓ Unsubscribed

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_rs'

✓ WebSocket subscription example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("-omaBpZVZUWC9-KRPwxPA-VFKc6f-cq2oV8DMlVd12wSlTXGJl3QQyXam3DnmxSali4wDt9NuOGurlP3mhq7rQ"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/bypass_ripple_example`
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: Record({"id": String("5k2Lw8axvjtm7tjy79oWHiyXaQRDp-5wc_3BvxL3y7YJZOCW56h5TOSDeIOurFtCCmr68PsKucyn81M68mIt-Q")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("5gYPrEEXLAawQMBtLEtGK3Q87fBNRaR2vIdGXSHnM5-e3c2-EhVt_qrVhQWfuFLIvAg-CcxU_OE0gLnh1Z4Rig")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"price": Object({"type": String("Integer"), "value": Integer(150)}), "name": Object({"value": String("Product 1"), "type": String("String")}), "id": String("5k2Lw8axvjtm7tjy79oWHiyXaQRDp-5wc_3BvxL3y7YJZOCW56h5TOSDeIOurFtCCmr68PsKucyn81M68mIt-Q")})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("BL2BVkepmYr7J6-nsEIS9T4Jyk-ZuR9kwJ4zGtb_cQmxjqgCeHsuhG0FJsrX45VjYamMB9MjYmB58uULJjzvuQ")})

✅ All bypass_ripple operations completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/projection_example`
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["email", "id", "name"]

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["created_at", "age", "avatar_url", "user_role", "id", "email", "name", "bio", "status"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["password", "age", "name", "status", "api_key", "email", "id", "secret_token", "bio", "created_at", "avatar_url", "user_role"]
  Projected query:
    - 3 fields per record
    - Fields: ["name", "email", "id"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
