make test-examples-rust
🧪 Running Rust examples (direct HTTP/WebSocket)...
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("Z0h1T97ETrikMhyfrWmHW8CGIlga_7zb1AoCAb-IZxfIDLtFyUrYE-yMojia-QX81j2TbZYGK4C9gv6GQAij4A")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("Z0h1T97ETrikMhyfrWmHW8CGIlga_7zb1AoCAb-IZxfIDLtFyUrYE-yMojia-QX81j2TbZYGK4C9gv6GQAij4A"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("Z0h1T97ETrikMhyfrWmHW8CGIlga_7zb1AoCAb-IZxfIDLtFyUrYE-yMojia-QX81j2TbZYGK4C9gv6GQAij4A"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("Z0h1T97ETrikMhyfrWmHW8CGIlga_7zb1AoCAb-IZxfIDLtFyUrYE-yMojia-QX81j2TbZYGK4C9gv6GQAij4A"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: BuJyoP-A5m_zxZfcrNSPehBg5psmQyacvEQ9RYr-DyEzrJnDC4Rq0o-CIG9JMcIEzIgr2lOmIKiXZmhlV8nPRA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1789563129210903000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "BuJyoP-A5m_zxZfcrNSPehBg5psmQyacvEQ9RYr-DyEzrJnDC4Rq0o-CIG9JMcIEzIgr2lOmIKiXZmhlV8nPRA",
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "s8PLj482_CytEmrNSk0Jhd0q5DjzX4GjBKncwS78rKtnvt4MIax1MoWUsKWS4xQ0q919Pxqd53KsMNirGEdPzw"

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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/document_ttl`
=== ekoDB Document TTL Example (Rust) ===
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 0bITwwl4DvHD_-g-uTNg7IsNozmnwx7XCXEpKbNG2sJ1zP41N6vL-kGJh7kP-zB3RlObGguD1kEviC6082Bxzw

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: 5nDApqj6-uzFa3XZFVbOzzlTuw-6rO9zJb_aDCfJT9-_q3YIeP752xSGTusU9ZvDL4IVTF1TVXGEdntnIfzGBQ

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: b58e_ckLprMvLjNKXN7-xjQQjClijzdx_2yWGm2PSCli6QstLHEekacZTZcrsFVjKkrf2nhxnhdhjmkgycfTRw

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("zDu__FXDajqDs532gfO9Kg7FwAs_5-QjAh4VfED83oMhr_cF5u_RIQew-mOJUE0wZ9CeM_-RxyNj7nrY_y3qkg")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record via WebSocket

Record 1:
  id: "zDu__FXDajqDs532gfO9Kg7FwAs_5-QjAh4VfED83oMhr_cF5u_RIQew-mOJUE0wZ9CeM_-RxyNj7nrY_y3qkg"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-09-16T13:56:53.289744Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Functions Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: 6wquNjJM3_pMc66OQ0xt4MEdyF51oL10tMCAita4BHTYrxQTPYFKn7i0tCikvs2Hjdne0OOkrdQCPiwTs9PevA
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: DECwFOpsFqy4UyFjtPpeeXDKcGCkxLTuSxBi1dED9bJ9O3N_JiF_AyclBoqnxNNbeEl1xrq6YwTAJ9C7Uyv8dQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Function saved: eQCXq-sihZIS5lBS5zbH4jMOkTV0thINY4_YGK8gnBqwBq-bPsmIvQApPruG6WDbuS7MqX780AXQsxE5brkOYg
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: ySOzgrPnI6lVmsMpfUiFSiBcSYhbpZapQwn3vAGzZBDnQaJIb7ssnJK_OHI_WmxeemRpT7PwNh39TMnF8c28WQ
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: gyYAeRRzaJ4678zHhc42jlmQKLNzZexCP9ZzrONE-9MmnmIz2RVP1ZD3lBmw2cgu21M-Y_kfrZysSfgR_r09jQ
Created Bob: $500 - ID: 2-YfTFXkA7CujBaxJU2CaaBYPYUkngjfei8laQIhIcNsYk82RY2Isq7nSqr3mLsKwFfo4AZSJhiqAT6y43ixBw

=== Example 1: Begin Transaction ===
Transaction ID: 4ce9a49a-eb18-4e5a-a86e-fc63b61c0102

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
New transaction: 83d7dcce-80a8-415f-b98a-6e0b997248c0
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ Rust direct examples complete!
🛠️  Building client library...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
✅ Client build complete!
bash: warning: setlocale: LC_ALL: cannot change locale (C.UTF-8): No such file or directory
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_advanced_crud`
=== ekoDB Advanced CRUD Example (Rust) ===

--- Inserting base document ---
Inserted: Widget Counter (id: S7eanqhlZAh9ei-K8vORXUHRv9s1UiT-pHzpDHA5MV0e9xh2IUIQvHQe5GtrABIDEzJ6tHBALZk6Lq4fY2FSEA)

--- update_with_action: increment views by 10 ---
views after increment: Some(Object({"type": String("Integer"), "value": Integer(110)}))

--- update_with_action: decrement views by 3 ---
views after decrement: Some(Object({"value": Integer(107), "type": String("Integer")}))

--- update_with_action: multiply score by 2 ---
score after multiply: Some(Object({"value": Float(9.0), "type": String("Float")}))

--- update_with_action: push 'ekodb' to tags ---
tags after push: Some(Object({"type": String("Array"), "value": Array([String("rust"), String("database"), String("ekodb")])}))

--- update_with_action: append '-suffix' to label ---
label after append: Some(Object({"type": String("String"), "value": String("prefix-suffix")}))

--- update_with_action: pop last element from tags ---
tags after pop: Some(Object({"value": Array([String("rust"), String("database")]), "type": String("Array")}))

--- update_with_action: remove 'rust' from tags ---
tags after remove: Some(Object({"type": String("Array"), "value": Array([String("database")])}))

--- update_with_action_sequence: increment views + push tag + append label ---
views after sequence: Some(Object({"type": String("Integer"), "value": Integer(157)}))
tags after sequence: Some(Object({"type": String("Array"), "value": Array([String("database"), String("batch")])}))
label after sequence: Some(Object({"type": String("String"), "value": String("prefix-suffix-v2")}))

--- Cleanup ---
Deleted collection

=== All advanced CRUD operations completed ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: CJFDxZ7ySXKr2xupDkPpSbSRYCDuSrznhhHBEOINRPtnDwqtPJzrqLE9f9aqDiS4bAz0WuMoJrvgCT6dqgucUA

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more details or additional products, let me know!

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
✓ Created second session: x94-ex96XM6G9sItMBgu-Pjb2NywpJkANmMeY2K6Tpq-gQb3GbiVNUZoBhJZVzbFhvuj7j-wiNwVfFMQX6bHgA
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: x94-ex96XM6G9sItMBgu-Pjb2NywpJkANmMeY2K6Tpq-gQb3GbiVNUZoBhJZVzbFhvuj7j-wiNwVfFMQX6bHgA
✓ Deleted chat session: CJFDxZ7ySXKr2xupDkPpSbSRYCDuSrznhhHBEOINRPtnDwqtPJzrqLE9f9aqDiS4bAz0WuMoJrvgCT6dqgucUA
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: ziikrS8SSoT8XnnY80XN31mOw9x_br3cXewzlYilC6aEZ4c1tim5bMT_bik9hZo0nWExqMmInxjfaW2MSvQ1wQ

=== Sending Chat Message ===
Message ID: i5slv-rqzJCkUz41uzrW6cObLiMsKJjSPOvPOHCvAHQP4t1qU0W0gAXN5mKVfuZSTj8MICWugo8y7rdlqDkKIg

=== AI Response ===
Response 1: ekoDB is a high-performance database designed with intelligent caching and real-time capabilities, along with AI integration.

### Key Features:
1. **AI Chat Integration**: The chat feature allows you to query your database using natural language, providing AI-powered responses with relevant context.

2. **Search Capabilities**:
   - **Full-text Search**: Enables querying of text data efficiently.
   - **Vector Search**: Allows for semantic searches using vector embeddings.
   - **Hybrid Search**: Combines both full-text and vector searching with automatic context retrieval.

These features make ekoDB a versatile and powerful tool for managing and querying data effectively.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("Uc5yGDp_U06VPAhiEg3yMiWtjiO54uT3nX9VRX9IsRgb3cVtdJ-G39BNEj3_GEezHIAATUu87ahen_cjUVFKSQ"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["category", "title", "content"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("lBEkOWr3s40PvtyTuxBXRoNgfzfe3JbYOcj_YLAuy6rhoKS6YYJbkXJxJ3QV4R0tIl64V-luACGAPJ5JDLiGpw"), "title": String("Search Features")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("-41IQXXmuqHyIFDphWjnE6RgU-9pXZVwH8ZWaFsTZ9gP5ezgxJzucB4rX4MruG2k03ec1wojjOi0GVUQmWFZkA"), "title": String("Introduction to ekoDB")}

Execution Time: 3470ms

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_chat_message_stream`
=== ekoDB Chat Message Stream (SSE) Example (Rust) ===

Created session: Yml_AosYcMSL_uitCWAmtTLFICgyIyBvbapxMNZ4YIs7zapOQQw-8XOKtAdv3-GEYCdL-y0XoTGEPw9Bcujuug

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}**ekoDB** is a publicly accessible, comprehensive database that integrates and organizes ecological knowledge, particularly data on how species interact with their environment (eco-evolutionary responses), traits, and biotic interactions. Its main features are:

- **Purpose**: To facilitate research in ecology and evolutionary biology by providing a centralized source for ecological response data, trait information, and species interactions.
- **Content**: Includes published and curated data on ecological traits (like growth rate, diet, or thermal tolerance), how those traits respond to environmental factors, and how species interact with one another (e.g., predation, mutualism).
- **Utility**: Helps researchers analyze ecological patterns, develop and test ecological models, and perform meta-analyses, especially in the context of global change or biodiversity studies.

ekoDB is similar in concept to other knowledge bases like TRY (for plant traits) and the Global Biotic Interactions database, but aims for broader integration, harmonizing different kinds of ecological data in one place.

**References:**
- Online documentation and database: [https://www.ekodb.org/](https://www.ekodb.org/)
- See also: Brodersen, J., et al. (2023). ekoDB: Ecological knowledge database for eco‐evolutionary analysis. *Ecology and Evolution*, 13(2), e9926. [https://doi.org/10.1002/ece3.9926](https://doi.org/10.1002/ece3.9926)

--- Stream complete ---
Message ID: 0yDQXyQxMRzbAvNUI9Madzn2KD7mVWFLIqV4GMM3uWS5rs1FiRoiR79g6ha2fUVJZEPSPgDHJpVPxyH9kTOWHQ
Execution time: 3743ms
Context window: 1000000 tokens

✓ Chat message stream example completed
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 6cWoPtMLf7xz-iAQKNNfe3zmkUQXC22ZTumfXp6Q8O9cDhtl66SOo_ap5Z8N9n7w3R9k64vMYa-8jKrFNlLqBw
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Product Name:** ekoDB
- **Description:** A high-performance database product with AI capabilities
- **Price:** $99

If you need more details or additional products, let me know!

✓ Message 2 sent
  Response: The price of the product ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: nKqdZso8cfgChZmV3w8z57mPSc8cy4BFxf_sAun42DlayyjnUTqaj46SAC-drdJg72QKrM-dFkaOAibBOXuVUw
  Parent: 6cWoPtMLf7xz-iAQKNNfe3zmkUQXC22ZTumfXp6Q8O9cDhtl66SOo_ap5Z8N9n7w3R9k64vMYa-8jKrFNlLqBw

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: nKqdZso8cfgChZmV3w8z57mPSc8cy4BFxf_sAun42DlayyjnUTqaj46SAC-drdJg72QKrM-dFkaOAibBOXuVUw (Untitled)
  Session 2: 6cWoPtMLf7xz-iAQKNNfe3zmkUQXC22ZTumfXp6Q8O9cDhtl66SOo_ap5Z8N9n7w3R9k64vMYa-8jKrFNlLqBw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: nKqdZso8cfgChZmV3w8z57mPSc8cy4BFxf_sAun42DlayyjnUTqaj46SAC-drdJg72QKrM-dFkaOAibBOXuVUw

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "w48Ns5c78s8OAnW3jbjuNtUHCi9TPTJTpECeaey8Qn-3uxX2FxEJOvu0C_65fiTDRoLOYdSAhiKy9ZibXx4SDQ"

=== List Collections ===
Total collections: 7
Sample collections: ["agent_function_versions__ek0_testing", "functions__ek0_testing", "chat_configurations__ek0_testing", "client_collection_management_rust", "audit__ek0_testing"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
  - agent_function_versions__ek0_testing
  - functions__ek0_testing
  - chat_configurations__ek0_testing
  - audit__ek0_testing
  - collection_utils_test_rust <-- our test
  - chat_messages__ek0_testing
  - chat_turns__ek0_testing

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_rust'

✓ Collection Utilities example complete
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("xHHO_SZO9GSvmZUbN-MSxUaMk8B6IEL9JmrEU3ID1qdAycZbY4cIr7BnzNSvHqJ9jWjnKqL5spB8XwBkGVHvGw")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("JOS5Tj5n2ueYUtHz0JVW2gdLAvZqTdCOLGr9x0LHY-yoUKzk9fnHsm8QJnnrWmvPI5RCistOSyPlHpaeIK4_TQ")})
✓ Second upsert (update): Record({"id": String("JOS5Tj5n2ueYUtHz0JVW2gdLAvZqTdCOLGr9x0LHY-yoUKzk9fnHsm8QJnnrWmvPI5RCistOSyPlHpaeIK4_TQ"), "email": Object({"value": String("bob.smith@newdomain.com"), "type": String("String")}), "name": Object({"type": String("String"), "value": String("Bob Smith")}), "age": Object({"type": String("Integer"), "value": Integer(36)})})

=== Find One Operation ===
✓ Found user by email: Record({"name": Object({"value": String("Alice Johnson"), "type": String("String")}), "email": Object({"type": String("String"), "value": String("alice@example.com")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "age": Object({"type": String("Integer"), "value": Integer(28)}), "id": String("xHHO_SZO9GSvmZUbN-MSxUaMk8B6IEL9JmrEU3ID1qdAycZbY4cIr7BnzNSvHqJ9jWjnKqL5spB8XwBkGVHvGw")})
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "Ppb-0tgxNy3qEKcTbd4CimSY4UN170faXhlEBm_yA7nyBNQhCB22kRgA_Kx5LE7zzeguqZ2Yk1yShmUsxNGjPQ"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("hk04mrw6-pAq4Zu33LOsbUNvrYNLGasZxWiIANhZDqSKxjhbJQfJa8o0zZNY_bJ5II-5uIRaTi_Dq5k9lIHCAg"))

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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_edge_cache`
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: qIcr8gwEutVxrA-6MXDRU9T183eeC0GCmOA53_mIxc3IVoqFQlQlCzhcEm6a1aspbQ51lqnQj4o8BSxnU02fBQ

Call 1: Cache lookup
Response time: 7ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 5ms
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
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
   ⏱️  Duration: 91.863875ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "email": "Sincere@april.biz",
      "name": "Leanne Graham",
      "id": 1,
      "address": {
        "zipcode": "92998-3874",
        "city"...

Second call (cache hit - from cache):
   ⏱️  Duration: 5.721584ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "email": "Sincere@april.biz",
      "phone": "1-770-736-8031 x56442",
      "name": "Leanne Graham",
      "username": "Bret",
      "address": {
        "geo": {
 ...
   🚀 Cache speedup: 16.1x faster!

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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_function_contract`
client_function_contract: ok
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Functions Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: PJ2MKyrqMIXnN0VzFBS18JByfCmKLkoBzdwyL6vmKUeW1Wk0buQG71iZj-D-a8W2VHtlVqS3TJ9xDkc9QFoO5g
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: TVA2xrfxRMgG9MItEEnTiqbFrTRQYMVnG8aPuNrEtm-vBW3267aPK_TCbdzV5yzRYjW4ZuLnP7S46-IBH-DflQ
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: TsqE2h6QOwFDcmfmgLfXfUPi5QP4QafGXAMdDymkD_GfXe7vvVm0YXjmDWEkRMVwnFXteq2qUQMJxfgY63jugQ
📊 Statistics: 2 groups

📝 Example 4: Function Management

📋 Total functions: 3
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
   Record({"category": Object({"type": String("String"), "value": String("Electronics")}), "count": Object({"type": String("Integer"), "value": Integer(5)}), "avg_price": Object({"type": String("Float"), "value": Float(367.0)})})
   Record({"avg_price": Object({"value": Float(365.6666666666667), "type": String("Float")}), "category": Object({"value": String("Furniture"), "type": String("String")}), "count": Object({"type": String("Integer"), "value": Integer(3)})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They excel at finding similar items quickly using vector embeddings, ideal for tasks like recommendation and image retrieval.

2. **Scalability**: Designed to handle large datasets with high dimensionality, vector databases can scale horizontally with distributed computing.

3. **Real-time Performance**: They provide fast indexing and querying, making real-time applications feasible.

4. **Supports Complex Data Types**: They can efficiently manage unstructured data like images, text, and audio by converting them into vector representations.

5. **Advanced Algorithms**: Vector databases utilize sophisticated algorithms like Approximate Nearest Neighbors (ANN) to enhance search performance.

6. **Integration with Machine Learning**: They work well with machine learning workflows, allowing for easy storage and retrieval of feature vectors.

7. **Flexibility**: Many vector databases support various distance metrics, enabling tailored similarity searches for different applications.

8. **Reduced Latency**: They minimize the time taken to fetch related data, improving user experience in applications like chatbots or personalized content delivery.
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_functions_complete`
🚀 ekoDB Rust Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: NvSepuMRVP9sdxRUgTs98MWWz6zhycomTszJfyc4wvhjFdR5wl4Yd59s4rXFpFMVLmmoDtryDFJUWvTUXGPJag
📊 Found 2 product groups
   Record({"category": Object({"type": String("String"), "value": String("Electronics")}), "count": Object({"value": Integer(3), "type": String("Integer")}), "avg_price": Object({"value": Float(575.6666666666666), "type": String("Float")})})
   Record({"avg_price": Object({"type": String("Float"), "value": Float(474.0)}), "count": Object({"value": Integer(2), "type": String("Integer")}), "category": Object({"type": String("String"), "value": String("Furniture")})})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   Record({"category": Object({"type": String("String"), "value": String("Furniture")}), "count": Object({"value": Integer(2), "type": String("Integer")})})
   Record({"count": Object({"value": Integer(3), "type": String("Integer")}), "category": Object({"value": String("Electronics"), "type": String("String")})})
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
   Record({"count": Object({"type": String("Integer"), "value": Integer(3)}), "status": Object({"value": String("inactive"), "type": String("String")})})
   Record({"status": Object({"value": String("active"), "type": String("String")}), "count": Object({"value": Integer(7), "type": String("Integer")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("r_PrrGWlWLADT630OIrkEM6wxT_GgGInaTrJSMfSxpc5v3YyQqXARxsotoHAcKBla-6azgU2cn9NwQE9R6F0AQ"))
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: MEjNoK5vk3ZVbNkgC3bg-ZbwegJ0BKsPhnYFJiICJzSrWjYEyQ-UIr9WiNk7_Sg6L8wqhrSW-o9P8Y8bpG_69g
📊 function executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"type": String("Object"), "value": Object {"role": String("admin"), "userId": String("user_abc")}})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: H_kB-HdrYTE5d-Mxs7oHs14rJYZiZrrdpgbrWGMgiG7GQ7LDbCtdkjeRadYdlH2sjWo2uBStgs8XXnNIvtJTBQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: utbbMaZPGQ7DazQE_dk7p0GilYkThmbLKGUKjdvtVMPkhCb-ZOz56U5GJ8Bspd2OKqONWHMQCqfz_x5w2KC7PA
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Getting Started with ekoDB (Database)
   2. Introduction to Machine Learning (AI)
   3. Database Design Principles (Database)
   4. Vector Databases Explained (Database)
   5. Natural Language Processing (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   Record({"category": Object({"value": String("AI"), "type": String("String")}), "count": Object({"value": Integer(2), "type": String("Integer")})})
   Record({"count": Object({"value": Integer(3), "type": String("Integer")}), "category": Object({"type": String("String"), "value": String("Database")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_goal_templates`
=== ekoDB Goal Template CRUD Example (Rust) ===

--- Creating goal template ---
Created template: Data Migration (id: y4AGokfGbjULCU7zCEaatvOk-vl6gkHI-03nqXcwloOum7a-8pPCiPGAZjR_Gsm3Z_3f6Mq5EVSMeMBkw61SvA)

--- Listing templates ---
Templates: {"count":1,"items":[{"description":{"type":"String","value":"Template for migrating data between schemas"},"id":"y4AGokfGbjULCU7zCEaatvOk-vl6gkHI-03nqXcwloOum7a-8pPCiPGAZjR_Gsm3Z_3f6Mq5EVSMeMBkw61SvA","steps":{"type":"Array","value":[{"description":"Analyze source schema"},{"description":"Create target schema"},{"description":"Migrate records"},{"description":"Validate results"}]},"title":{"type":"String","value":"Data Migration"}}]}

--- Getting template ---
Fetched: {"type":"String","value":"Data Migration"}

--- Updating template ---
Updated description: {"type":"String","value":"Updated: comprehensive data migration workflow"}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/client_goals_tasks_agents`
=== ekoDB Goals, Tasks & Agents Integration Example (Rust) ===

--- Goal: create ---
Created goal: "Deploy v2.0 to production" (id: D5NVcTWVmpDKFpRUDMnntMipZF_BHTuoPFbIS290sLmjib-KkGyQ9JQSy1m3jnaCunctTyHJ5fa_i60PSXN7hw)

--- Goal: list ---
Goals: {"count":1,"goals":[{"created_at":"2026-09-16T13:24:14.983121+00:00","description":"Full release cycle for version 2.0","id":"D5NVcTWVmpDKFpRUDMnntMipZF_BHTuoPFbIS290sLmjib-KkGyQ9JQSy1m3jnaCunctTyHJ5fa_i60PSXN7hw","status":"pending","steps":"[{\"description\":\"Run integration tests\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]","title":"Deploy v2.0 to production","updated_at":"2026-09-16T13:24:14.983121+00:00"}]}

--- Goal: get ---
Fetched goal: {"type":"String","value":"Deploy v2.0 to production"}

--- Goal: update ---
Updated description: {"type":"String","value":"Updated: full v2.0 release with rollback plan"}

--- Goal: search ---
Search results: {"count":1,"items":[{"_score":12.87,"created_at":{"type":"DateTime","value":"2026-09-16T13:24:14.983121+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"D5NVcTWVmpDKFpRUDMnntMipZF_BHTuoPFbIS290sLmjib-KkGyQ9JQSy1m3jnaCunctTyHJ5fa_i60PSXN7hw","status":{"type":"String","value":"pending"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-09-16T13:24:15.035462+00:00"}}]}

--- Goal step: start (step 0) ---
Step 0 started: {"created_at":{"type":"DateTime","value":"2026-09-16T13:24:14.983121+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"D5NVcTWVmpDKFpRUDMnntMipZF_BHTuoPFbIS290sLmjib-KkGyQ9JQSy1m3jnaCunctTyHJ5fa_i60PSXN7hw","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"status\":\"InProgress\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-09-16T13:24:15.052739+00:00"}}

--- Goal step: complete (step 0) ---
Step 0 completed: {"created_at":{"type":"DateTime","value":"2026-09-16T13:24:14.983121+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"D5NVcTWVmpDKFpRUDMnntMipZF_BHTuoPFbIS290sLmjib-KkGyQ9JQSy1m3jnaCunctTyHJ5fa_i60PSXN7hw","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-09-16T13:24:15.065422+00:00"}}

--- Goal step: start (step 1) ---
Step 1 started

--- Goal step: fail (step 1) ---
Step 1 failed: {"created_at":{"type":"DateTime","value":"2026-09-16T13:24:14.983121+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"D5NVcTWVmpDKFpRUDMnntMipZF_BHTuoPFbIS290sLmjib-KkGyQ9JQSy1m3jnaCunctTyHJ5fa_i60PSXN7hw","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-09-16T13:24:15.086183+00:00"}}

--- Goal: complete ---
Goal completed (pending review): {"completed_at":{"type":"DateTime","value":"2026-09-16T13:24:15.095281+00:00"},"created_at":{"type":"DateTime","value":"2026-09-16T13:24:14.983121+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"D5NVcTWVmpDKFpRUDMnntMipZF_BHTuoPFbIS290sLmjib-KkGyQ9JQSy1m3jnaCunctTyHJ5fa_i60PSXN7hw","status":{"type":"String","value":"pending_review"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"summary":{"type":"String","value":"Partial completion — step 1 failed"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-09-16T13:24:15.095281+00:00"}}

--- Goal: approve ---
Goal approved: {"completed_at":{"type":"DateTime","value":"2026-09-16T13:24:15.095281+00:00"},"created_at":{"type":"DateTime","value":"2026-09-16T13:24:14.983121+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"D5NVcTWVmpDKFpRUDMnntMipZF_BHTuoPFbIS290sLmjib-KkGyQ9JQSy1m3jnaCunctTyHJ5fa_i60PSXN7hw","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"summary":{"type":"String","value":"Partial completion — step 1 failed"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-09-16T13:24:15.104471+00:00"}}

--- Goal: create (for rejection) ---
Created goal2: "Refactor auth module" (id: 4zzaCO6TqzWTWWclqpm9XGpgRcY9h9BI3cEViwxH_J5oimpnB00pMxVZ1ixltgioQFWwSmm3BrIPHKosV1mTHg)

--- Goal: complete (goal2) ---

--- Goal: reject ---
Goal rejected: {"completed_at":{"type":"DateTime","value":"2026-09-16T13:24:15.130807+00:00"},"created_at":{"type":"DateTime","value":"2026-09-16T13:24:15.113443+00:00"},"description":{"type":"String","value":"Rewrite JWT handling"},"id":"4zzaCO6TqzWTWWclqpm9XGpgRcY9h9BI3cEViwxH_J5oimpnB00pMxVZ1ixltgioQFWwSmm3BrIPHKosV1mTHg","reason":{"type":"String","value":"Breaks backward compatibility"},"status":{"type":"String","value":"failed"},"steps":{"type":"String","value":"[]"},"summary":{"type":"String","value":"Completed refactor"},"title":{"type":"String","value":"Refactor auth module"},"updated_at":{"type":"DateTime","value":"2026-09-16T13:24:15.130807+00:00"}}

--- Task: create ---
Created task: Nightly backup (id: 9EzC-4nX2_-60iWre_YoqrVpUKIBOyCbBhGH-i1reOP6LTCjeygW0ReFCen5WjDB5knMujsznVByL0-hqHxjFA)

--- Task: list ---
Tasks: {"count":1,"items":[{"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"9EzC-4nX2_-60iWre_YoqrVpUKIBOyCbBhGH-i1reOP6LTCjeygW0ReFCen5WjDB5knMujsznVByL0-hqHxjFA","max_retries":{"type":"Integer","value":3},"schedule":{"type":"String","value":"0 2 * * *"},"title":{"type":"String","value":"Nightly backup"}}]}

--- Task: get ---
Fetched task: {"type":"String","value":"Nightly backup"}

--- Task: start ---
Task started: {"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"9EzC-4nX2_-60iWre_YoqrVpUKIBOyCbBhGH-i1reOP6LTCjeygW0ReFCen5WjDB5knMujsznVByL0-hqHxjFA","max_retries":{"type":"Integer","value":3},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"running"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-09-16T13:24:15.178359+00:00"}}

--- Task: succeed ---
Task succeeded: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"9EzC-4nX2_-60iWre_YoqrVpUKIBOyCbBhGH-i1reOP6LTCjeygW0ReFCen5WjDB5knMujsznVByL0-hqHxjFA","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-09-16T13:24:15.187065+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-09-16T13:24:15.187065+00:00"}}

--- Task: start (round 2) ---

--- Task: pause ---
Task paused: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"9EzC-4nX2_-60iWre_YoqrVpUKIBOyCbBhGH-i1reOP6LTCjeygW0ReFCen5WjDB5knMujsznVByL0-hqHxjFA","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-09-16T13:24:15.187065+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"paused"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-09-16T13:24:15.205785+00:00"}}

--- Task: resume ---
Task resumed: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"9EzC-4nX2_-60iWre_YoqrVpUKIBOyCbBhGH-i1reOP6LTCjeygW0ReFCen5WjDB5knMujsznVByL0-hqHxjFA","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-09-16T13:24:15.187065+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-09-16T13:24:15.214914+00:00"}}

--- Task: fail ---
Task failed: {"consecutive_failures":{"type":"Integer","value":1},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"9EzC-4nX2_-60iWre_YoqrVpUKIBOyCbBhGH-i1reOP6LTCjeygW0ReFCen5WjDB5knMujsznVByL0-hqHxjFA","last_error":{"type":"String","value":"S3 bucket access denied"},"last_run":{"type":"DateTime","value":"2026-09-16T13:24:15.226393+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-09-16T13:24:15.226393+00:00"}}

--- Task: due ---
Due tasks: {"count":0,"items":[]}

--- Task: delete ---
Task deleted

--- Agent: create ---
Created agent: backup-agent (id: qG3hbvNQv9bXzex89VZzptJy9uIQBHNSS-SdpZlMDIDaCO-STg0Tc0uOpvsV2QrfsABazhvIlmn8SP-By-YFWg)

--- Agent: list ---
Agents: {"count":1,"items":[{"capabilities":{"type":"Array","value":["backup","restore","verify"]},"deployment_id":{"type":"String","value":"deploy-abc-123"},"description":{"type":"String","value":"Handles nightly backups and restores"},"id":"qG3hbvNQv9bXzex89VZzptJy9uIQBHNSS-SdpZlMDIDaCO-STg0Tc0uOpvsV2QrfsABazhvIlmn8SP-By-YFWg","llm_model":{"type":"String","value":"gpt-4.1"},"name":{"type":"String","value":"backup-agent"}}]}

--- Agent: get ---
Fetched agent: {"type":"String","value":"backup-agent"}

--- Agent: get_by_name ---
Found by name: {"type":"String","value":"backup-agent"}

--- Agent: update ---
Updated agent description: {"type":"String","value":"Handles backups, restores, and disaster recovery"}

--- Agent: agents_by_deployment ---
Agents in deployment: {"count":0,"items":[]}
WARNING: agents_by_deployment omitted created agent qG3hbvNQv9bXzex89VZzptJy9uIQBHNSS-SdpZlMDIDaCO-STg0Tc0uOpvsV2QrfsABazhvIlmn8SP-By-YFWg; TODO: check/fix the server-side deployment lookup

--- Agent: delete ---
Agent deleted

--- Goal: delete ---
Goal 1 deleted
Goal 2 deleted

=== All goals, tasks & agents operations completed ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_joins`
=== ekoDB Rust Client - Join Operations Example ===

=== Setting up sample data ===
✓ Sample data created

=== Example 1: Single collection join (users with departments) ===
✓ Found 2 users with department data
  - Alice Johnson: Engineering
  - Bob Smith: Sales

=== Example 2: Join with filtering ===
✓ Found 1 users in Engineering
  - Alice Johnson: Building A

=== Example 3: Join with user profiles ===
✓ Found 2 users with profile data
  - Alice Johnson: Senior Software Engineer
  - Bob Smith: Sales Manager

=== Example 4: Join orders with user data ===
✓ Found 2 completed orders
  - Mouse ($25) by Alice Johnson
  - Laptop ($1200) by Alice Johnson

=== Example 5: Complex join with multiple conditions ===
✓ Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✓ Deleted test collections

✓ Join operations example completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_kv_links`
=== ekoDB KV Document Linking Example (Rust) ===

--- Inserting documents ---
Inserted doc1: Project Alpha (id: j4Opi_fn1na5xnhLFMxxiCJ8ga3ZLd6E18QO_11cpP5h32jRPxQYrTS2lcFJgG23T35Ha3kq7WfX8W3b85G6AQ)
Inserted doc2: Project Beta (id: f9T04Plq9D0m2_5RTNV1owy9E4LSRnQS_ldIW9oJTFPiok6aw1wr7G-WTl2EldfGCIpJwX606CEgAkUp8nNniQ)

--- Setting KV key ---
Set key: kv_links:rs:user:alice:projects

--- Linking documents ---
Linked doc1: null
Linked doc2: null

--- Getting links ---
Links for kv_links:rs:user:alice:projects: [{"collection":"kv_links_example_rs","created_at":"2026-09-16T13:27:01.718838Z","document_id":"j4Opi_fn1na5xnhLFMxxiCJ8ga3ZLd6E18QO_11cpP5h32jRPxQYrTS2lcFJgG23T35Ha3kq7WfX8W3b85G6AQ","field_path":null,"last_accessed":"2026-09-16T13:27:01.722294Z","metadata":{}},{"collection":"kv_links_example_rs","created_at":"2026-09-16T13:27:01.720586Z","document_id":"f9T04Plq9D0m2_5RTNV1owy9E4LSRnQS_ldIW9oJTFPiok6aw1wr7G-WTl2EldfGCIpJwX606CEgAkUp8nNniQ","field_path":null,"last_accessed":"2026-09-16T13:27:01.722294Z","metadata":{}}]

--- Unlinking doc1 ---
Unlinked doc1: null

--- Verifying remaining links ---
Remaining links: [{"collection":"kv_links_example_rs","created_at":"2026-09-16T13:27:01.720586Z","document_id":"f9T04Plq9D0m2_5RTNV1owy9E4LSRnQS_ldIW9oJTFPiok6aw1wr7G-WTl2EldfGCIpJwX606CEgAkUp8nNniQ","field_path":null,"last_accessed":"2026-09-16T13:27:01.725649Z","metadata":{}}]

--- Cleanup ---
Unlinked remaining document
Deleted KV key
Deleted documents
Deleted collection

=== All KV linking operations completed ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Prefix Query (StartsWith) ===
✓ Found 1 users with names starting with A
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
  2. Some(Object({"value": String("David"), "type": String("String")}))
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
    "name": "Venus",
    "diameter_km": 12104
  },
  {
    "name": "Mars",
    "diameter_km": 6779
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a great day.

=== Done ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_schedules`
=== ekoDB Schedule Management Example (Rust) ===

--- Creating schedule ---
Created schedule: "nightly-cleanup" (id: 1192c86d-2c5a-4634-8c82-7bfe045782fe)

--- Listing schedules ---
Schedules: {"count":1,"schedules":[{"created_at":"2026-09-16T13:33:12.904710Z","cron_expression":"0 0 3 * * *","description":"Remove expired sessions and temp files","enabled":true,"function_label":"schedule_noop_rust_97860","id":"1192c86d-2c5a-4634-8c82-7bfe045782fe","last_execution":null,"name":"nightly-cleanup","next_execution":"2026-09-17T03:00:00Z","parameters":{},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-09-16T13:33:12.904710Z"}]}

--- Getting schedule ---
Fetched schedule: "nightly-cleanup"

--- Updating schedule ---
Updated cron: "0 0 4 * * *"
Updated description: "Remove expired sessions, temp files, and orphaned uploads"

--- Triggering schedule ---
Trigger response: {"schedule_id":"1192c86d-2c5a-4634-8c82-7bfe045782fe","status":"triggered"}

--- Pausing schedule ---
Schedule paused: {"created_at":"2026-09-16T13:33:12.904710Z","cron_expression":"0 0 4 * * *","description":"Remove expired sessions, temp files, and orphaned uploads","enabled":false,"function_label":"schedule_noop_rust_97860","id":"1192c86d-2c5a-4634-8c82-7bfe045782fe","last_execution":"2026-09-16T13:33:12.962552Z","name":"nightly-cleanup","next_execution":null,"parameters":{"type":"Object","value":{}},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-09-16T13:33:12.966516Z"}

--- Resuming schedule ---
Schedule resumed: {"created_at":"2026-09-16T13:33:12.904710Z","cron_expression":"0 0 4 * * *","description":"Remove expired sessions, temp files, and orphaned uploads","enabled":true,"function_label":"schedule_noop_rust_97860","id":"1192c86d-2c5a-4634-8c82-7bfe045782fe","last_execution":"2026-09-16T13:33:12.962552Z","name":"nightly-cleanup","next_execution":"2026-09-17T04:00:00Z","parameters":{"type":"Object","value":{}},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-09-16T13:33:12.970539Z"}

--- Deleting schedule ---
Schedule deleted successfully

=== All schedule operations completed ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("M6WEiYR3obpIKwNG_1X1ODOfoJcx4PJAWGRM46DJ-T-q53TuCDRmnPpZMa88wCEBKyQ_QvLw8oe67AvkC1tFrw"))
✓ Inserted user 2: Some(String("GR6XiSZRVTdKxYMmoa9cSm8x5whDCLn-uap6wm6pI8vaR_Mrp132BphCuDweeMgz_1Z-wHy2E5Pie8Wey5ri0w"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - title: String
    (required)
  - email: String
    (required)
  - status: String
  - age: Integer

=== Listing Collections ===
✓ Total collections: 13
  Sample: ["chat_agent_configs__ek0_testing", "agent_function_versions__ek0_testing", "schedules__ek0_testing", "functions__ek0_testing", "chat_configurations__ek0_testing"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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

Execution time: 7ms
=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("GO79zzr3Ba7Sd-VIVQDt8C9muk_HihLXrRrSfuaeAzOvBWUwWCiE45iZYtz4T0msTmaJbadjaryfOmHdFqjr1A")})

=== Find by ID ===
Found: Record({"price": Object({"type": String("Float"), "value": Float(99.99)}), "name": Object({"value": String("Test Record"), "type": String("String")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "id": String("GO79zzr3Ba7Sd-VIVQDt8C9muk_HihLXrRrSfuaeAzOvBWUwWCiE45iZYtz4T0msTmaJbadjaryfOmHdFqjr1A"), "created_at": Object({"type": String("DateTime"), "value": String("2026-09-16T13:36:20.355088+00:00")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "value": Object({"value": Integer(42), "type": String("Integer")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-09-16T13:36:20.355088+00:00")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"created_at": Object({"type": String("DateTime"), "value": String("2026-09-16T13:36:20.355088+00:00")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "name": Object({"type": String("String"), "value": String("Test Record")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "id": String("GO79zzr3Ba7Sd-VIVQDt8C9muk_HihLXrRrSfuaeAzOvBWUwWCiE45iZYtz4T0msTmaJbadjaryfOmHdFqjr1A"), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")})})]

=== Update Document ===
Updated: Record({"embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "price": Object({"type": String("Float"), "value": Float(99.99)}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "name": Object({"value": String("Updated Record"), "type": String("String")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-09-16T13:36:20.355088+00:00")}), "id": String("GO79zzr3Ba7Sd-VIVQDt8C9muk_HihLXrRrSfuaeAzOvBWUwWCiE45iZYtz4T0msTmaJbadjaryfOmHdFqjr1A"), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "value": Object({"type": String("Integer"), "value": Integer(100)})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: zh5nrq4AzUj1_YXq-BTd5-drVdAJEsUyO9ErsqtB9XHu8CY7KrxdCN-xK8rpJ1gMSwD7Dh27tbXCz_pIGX5-vw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  active: true
  value: 42
  name: "WebSocket Test Record"
  id: "zh5nrq4AzUj1_YXq-BTd5-drVdAJEsUyO9ErsqtB9XHu8CY7KrxdCN-xK8rpJ1gMSwD7Dh27tbXCz_pIGX5-vw"

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native_rs (XU85XY54vl1JBrJ3OoQWrlHO7P1tApr_69ukUwSEZlJxDxPJxI3n6BQQJOAiHgb9-fCHuFXRv4oaASabclryzg)

First call (cache miss - will fetch from GitHub API):
  Response time: 174ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 58.0x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit_rs (h-ldBqy5rCwuioQLXEMQsQzGgNQD2jZ5bdMt_74nKevpJW4-lLAF1seRRxmaRDLQ2S7bU2KWN-EruHBcwwhW7w)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_rs' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_rs (B7x5Vd0s0GvEXK6pSrEkQIbl2gkZFM_p2ADkFrK2_PhQ5pJM7vJhogzUUg2D7DgTs7Mm59DICSD0ELLD_EhbiQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_rs' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache_rs (zEVZfSxI6iy14B8xq2mHyjV8uiGbEx77_4gA1VLL8Rc4Qnt5fEMkAvLGiCggUWGDuvMakvbBxnjWIgNcWk-Liw)
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

perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_rs (ZEMbIx8l47DWHbfAiEB0w7twFXE53THoWbwOaGiSKmfzoceXtST5yaLC8LMArYxGzanG4pWjSRXaaXFF3C0zaQ)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 13ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 0PdbEi5cW3p2l9T1v1D4cIS5EA4bZrLQyPSY9E3P4nWVAaTK8dNCms9drZe4G0axfaO-TO0Vg7IdXI5ttekYxw
Created Bob: $500 - ID: CZZA2uw4vrUkbWw7a3kKGwQy6f-T6kxhpRvjLsIYPJ8YGKt3zlDR4zz1xbJWiNYUo_fbtIayAvTgwcNWsgo3xg

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): e7f2b28f-ba2b-4126-9fc2-6360a546a6af

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
New transaction: 7d799d00-ab45-4f26-aef2-d6ad9610628f
Updated Bob: $700 → $600 (in transaction)
Status before rollback: "Active"
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_user_functions`
✓ Client created

=== Create User Function ===
Created user function with ID: Ya5jiKsb6sbJO2CvfmHjKNgh3v_jUPAVpxEtDJniPJkQz_hfC-dfcadQu8Bfe4B4chVbqD37GudED8gH9Hf1ow

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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_websocket_chat_stream`
=== WebSocket Chat Streaming Example (Rust) ===

✓ Authentication successful
✓ Created chat session: 7QFgeFN96a9NlN3pppQPL_xuzwUrB4B-1x4W6m82BLFkBqkffP5ZOX9qfAF93iTpIsrbPh-eJkyuzm1S2kayAg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Sending message: 'What is the capital of France?' ===
✓ Message accepted, streaming...

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: "VHeqDJpq1Asg2WpK62faqwLvoqnsT4GW4Emy3HylrmVD0gPMSgfzC4gVksYG520dC3traVTUhf5syc1XDcOUQA"
Execution time: 1020ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}
Context window: 1000000 tokens

Full response: {"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris....

✓ WebSocket chat streaming example completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_websocket_subscribe`
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_rs' ===
✓ Subscribed (subscription_id: sub_9a52760246f94cb389395a5bfadd802d)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: 3iGCRbRRPP8pyOKcdXOzyhrsf2l1FxA-FzR4WdWztZNRZtQW47Mi_e7NbqM03GBhJebZKrRsrmmT6mTecEJPHQ

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_rs"
     Record IDs: ["3iGCRbRRPP8pyOKcdXOzyhrsf2l1FxA-FzR4WdWztZNRZtQW47Mi_e7NbqM03GBhJebZKrRsrmmT6mTecEJPHQ"]
     Timestamp:  "2026-09-16T13:42:44.956886+00:00"

Inserting record 2...
✓ Inserted: iidsP1x4Hz1KWR_wvOfnS-kuVufcaHgNe35fNpaquy5xSsjEYdtvyb7IFwYIHSVKOGKe2kOeuB25Hd3MPcRFkQ

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["iidsP1x4Hz1KWR_wvOfnS-kuVufcaHgNe35fNpaquy5xSsjEYdtvyb7IFwYIHSVKOGKe2kOeuB25Hd3MPcRFkQ"]

=== Unsubscribing ===
✓ Unsubscribed

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_rs'

✓ WebSocket subscription example completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("baYROqS7A06La9OjSiBMMbkqQaAP9oF3i7ZE6ZgndSHC3mvl11eZgfE_ynn81-EPEfbmECG_YbiKmpKweT0Blw"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/bypass_ripple_example`
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: Record({"id": String("psB7ZwNMa5QlcSJtX3XQ4ulAU78axuVddiYaSaTdMeT4aYyjF4OVfqt4EK8XfjIG6sy03SI6wj8F3I6fO1vwuA")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("s7I2Br5j72DCD1aGcuIH7KsYxeChGxfNz9LaE7sE1wKMEpMWzb4ZNiucBhTEDObQQmJOG_B61uAYFtI4n6I0ZQ")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"name": Object({"value": String("Product 1"), "type": String("String")}), "id": String("psB7ZwNMa5QlcSJtX3XQ4ulAU78axuVddiYaSaTdMeT4aYyjF4OVfqt4EK8XfjIG6sy03SI6wj8F3I6fO1vwuA"), "price": Object({"type": String("Integer"), "value": Integer(150)})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("h3Ciw3daU62OJA2Vy9_TTIWHXsop-n2l2vd8zIatbanW4YEpM6WRvFCjugZkxDOKauVYfb3MDUUHUbU2MKD9Pw")})

✅ All bypass_ripple operations completed successfully!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/projection_example`
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["id", "email", "name"]

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["avatar_url", "name", "id", "bio", "created_at", "status", "email", "age", "user_role"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["user_role", "email", "name", "status", "age", "created_at", "api_key", "bio", "password", "secret_token", "id", "avatar_url"]
  Projected query:
    - 3 fields per record
    - Fields: ["email", "name", "id"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
