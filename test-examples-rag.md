make test-examples-rag
🐍 [36mBuilding Python client package...[0m
🔧 [36mEnsuring maturin is available in .venv...[0m
🔨 [36mBuilding wheel...[0m
🍹 Building a mixed python/rust project
🐍 Found CPython 3.14 at /opt/homebrew/opt/python@3.14/bin/python3.14
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.82s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.22.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel into .venv...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.22.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.22.0
    Uninstalling ekodb_client-0.22.0:
      Successfully uninstalled ekodb_client-0.22.0
Successfully installed ekodb-client-0.22.0
✅ [32mPython client package built and installed![0m
📦 [36mEnsuring Python example dependencies in .venv...[0m

🤖 [36mRAG Conversation System Examples[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[33mPrerequisites:[0m
  1. ekoDB server running (make run in ekodb/)
  2. OPENAI_API_KEY set in server environment
  3. API_BASE_URL and API_BASE_KEY exported in your shell

[36mBuilding Rust client library...[0m
✓ Rust client built

✓ Python client built and installed into .venv (via build-python-client prerequisite)

[36mBuilding TypeScript client library...[0m
✓ TypeScript client built

[36mInstalling TypeScript client in examples...[0m
✓ TypeScript client installed

[36mBuilding TypeScript example...[0m
✓ TypeScript example built

[36mBuilding Go client library...[0m
✓ Go client built

[36mBuilding Go RAG example...[0m
✓ Go example built

[36mBuilding Kotlin client library...[0m
✓ Kotlin client built

[36mBuilding Kotlin RAG example...[0m
✓ Kotlin example built

[36mRunning Rust RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 413.999041ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 275.963125ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 283.052958ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 481.219583ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 5.954432708s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 504.697083ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 234.45125ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 229.576834ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 324.24175ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 4.502987541s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 307.708375ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 279.320167ms
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
    ✓ Generated embedding: 1536 dimensions in 275.605ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 94.105042ms

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

Writing memory-safe and high-performance database code involves several principles and best practices. Here’s a comprehensive guide:

### Memory Safety

1. **Use Safe Programming Languages:**
   - Prefer languages that enforce memory safety, like Rust, Swift, or languages with garbage collection (e.g., Java, C#). These languages manage memory allocation and deallocation automatically.

2. **Check for NULL:**
   - Always verify that any pointer or reference is not NULL before dereferencing it, which helps prevent dereference errors.

3. **Bound Checks:**
   - Ensure that any array or collection access does not exceed its bounds to avoid buffer overflows.

4. **Immutable Data Structures:**
   - Utilize immutable data structures where possible, which benefit from being thread-safe and reducing the chance of unintended side effects.

5. **Use RAII (Resource Acquisition Is Initialization):**
   - This C++ idiom manages resource allocation through object lifetimes, ensuring that resources are automatically released when objects go out of scope.

6. **Encapsulation:**
   - Encapsulate database access logic to control and validate state changes, minimizing the risk of unexpected errors.

### High Performance

1. **Efficient Queries:**
   - Write optimized SQL queries. Use `EXPLAIN` to analyze query execution plans and make adjustments based on performance insights.

2. **Indexing:**
   - Utilize appropriate indexes on frequently queried fields, but avoid over-indexing, as it can degrade write performance.

3. **Connection Pooling:**
   - Implement connection pooling to minimize the overhead of establishing database connections.

4. **Batch Operations:**
   - Use batch processing for inserts/updates to minimize round trips between your application and database.

5. **Caching:**
   - Implement caching strategies for repetitive queries using in-memory data stores (e.g., Redis) to alleviate database load.

6. **Optimize Data Types:**
   - Use the smallest data types that can accommodate your data. For example, use `INT` instead of `BIGINT` when appropriate.

7. **Asynchronous Operations:**
   - For I/O-bound applications, use asynchronous database access patterns to avoid blocking your application.

8. **Monitor and Profile:**
   - Regularly monitor your database performance, using profiling tools to identify slow queries or bottlenecks.

9. **Normalization vs. Denormalization:**
   - Normalize your database to reduce redundancy and improve integrity, but consider denormalizing for read-heavy workloads to reduce joins.

10. **Transaction Management:**
   - Use transactions wisely to ensure ACID compliance for critical operations while avoiding long-running transactions that can lock resources.

### Tools and Libraries

1. **ORMs and Query Builders:**
   - Use Object-Relational mappers (ORMs) like Entity Framework, Hibernate, or Sequelize, as they can manage memory safety in object mappings while optimizing queries.
   - Also consider lightweight query builders for more control over SQL.

2. **Profiling Tools:**
   - Use profiling tools or database analyzers (like pgAdmin for PostgreSQL or MySQL Workbench) to evaluate and optimize your queries and schema.

3. **Data Validation Libraries:**
   - Implement validation libraries to ensure that data being inserted or updated meets expected structures and constraints, preventing runtime errors.

### Conclusion

By combining safe programming practices with performance optimizations tailored to your database and application's needs, you can create robust and performant database code. Make sure to continually profile and refine your code as systems and data change.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 352.531084ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3662 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 395.087583ms
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


→ Executing text_search()...
  • Collection: rag_messages
  • Query: "ownership system"
  • Limit: 3 results
  • Search method: Full-text with fuzzy matching & stemming
  • No vector embeddings needed - pure keyword search
  ✓ Text search completed in 65.119458ms

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_new_question: Writing memory-safe and high-performance database code involves several principles and best practices. Here’s a comprehensive guide:

### Memory Safety

1. **Use Safe Programming Languages:**
   - Prefer languages that enforce memory safety, like Rust, Swift, or languages with garbage collection (e.g., Java, C#). These languages manage memory allocation and deallocation automatically.

2. **Check for NULL:**
   - Always verify that any pointer or reference is not NULL before dereferencing it, which helps prevent dereference errors.

3. **Bound Checks:**
   - Ensure that any array or collection access does not exceed its bounds to avoid buffer overflows.

4. **Immutable Data Structures:**
   - Utilize immutable data structures where possible, which benefit from being thread-safe and reducing the chance of unintended side effects.

5. **Use RAII (Resource Acquisition Is Initialization):**
   - This C++ idiom manages resource allocation through object lifetimes, ensuring that resources are automatically released when objects go out of scope.

6. **Encapsulation:**
   - Encapsulate database access logic to control and validate state changes, minimizing the risk of unexpected errors.

### High Performance

1. **Efficient Queries:**
   - Write optimized SQL queries. Use `EXPLAIN` to analyze query execution plans and make adjustments based on performance insights.

2. **Indexing:**
   - Utilize appropriate indexes on frequently queried fields, but avoid over-indexing, as it can degrade write performance.

3. **Connection Pooling:**
   - Implement connection pooling to minimize the overhead of establishing database connections.

4. **Batch Operations:**
   - Use batch processing for inserts/updates to minimize round trips between your application and database.

5. **Caching:**
   - Implement caching strategies for repetitive queries using in-memory data stores (e.g., Redis) to alleviate database load.

6. **Optimize Data Types:**
   - Use the smallest data types that can accommodate your data. For example, use `INT` instead of `BIGINT` when appropriate.

7. **Asynchronous Operations:**
   - For I/O-bound applications, use asynchronous database access patterns to avoid blocking your application.

8. **Monitor and Profile:**
   - Regularly monitor your database performance, using profiling tools to identify slow queries or bottlenecks.

9. **Normalization vs. Denormalization:**
   - Normalize your database to reduce redundancy and improve integrity, but consider denormalizing for read-heavy workloads to reduce joins.

10. **Transaction Management:**
   - Use transactions wisely to ensure ACID compliance for critical operations while avoiding long-running transactions that can lock resources.

### Tools and Libraries

1. **ORMs and Query Builders:**
   - Use Object-Relational mappers (ORMs) like Entity Framework, Hibernate, or Sequelize, as they can manage memory safety in object mappings while optimizing queries.
   - Also consider lightweight query builders for more control over SQL.

2. **Profiling Tools:**
   - Use profiling tools or database analyzers (like pgAdmin for PostgreSQL or MySQL Workbench) to evaluate and optimize your queries and schema.

3. **Data Validation Libraries:**
   - Implement validation libraries to ensure that data being inserted or updated meets expected structures and constraints, preventing runtime errors.

### Conclusion

By combining safe programming practices with performance optimizations tailored to your database and application's needs, you can create robust and performant database code. Make sure to continually profile and refine your code as systems and data change.

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


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.593s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.285s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.321s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 14.264s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.231s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.249s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.292s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.303s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 3.200s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.283s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.302s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.303s
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
    ✓ Generated embedding: 1536 dimensions in 0.336s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.081s

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

Writing memory-safe, high-performance database code involves several best practices across both the programming and database management layers. Here are some strategies:

### 1. Use Prepared Statements
- **SQL Injection Prevention**: Always use prepared statements to ensure that input data is safely handled rather than directly concatenated into queries, which can lead to SQL injection vulnerabilities.
- **Performance**: Prepared statements can also improve performance, as the database can cache the execution plan.

### 2. Transactions
- **Atomic Operations**: Use transactions to group multiple operations together to ensure they either complete successfully or leave the database unchanged. This ensures data integrity.
- **Concurrency Control**: Use optimistic or pessimistic locking as necessary to manage concurrent access to shared resources.

### 3. Connection Management
- **Connection Pooling**: Use a connection pool to handle database connections efficiently. This minimizes the overhead of establishing new connections by reusing existing ones.

### 4. Indexing
- **Optimize Query Performance**: Create appropriate indexes on the database to speed up query performance. Use the `query_explain` tool (in ekoDB) to understand how queries are being executed and improve the indexing strategy.
- **Avoid Over-Indexing**: Too many indexes can slow down write operations, so balance read and write efficiencies.

### 5. Data Modeling
- **Normalization**: Normalize database schema to reduce redundancy, which also helps ensure data integrity.
- **Denormalization When Necessary**: In performance-critical applications, consider selectively denormalizing data for read-heavy operations to reduce joins.

### 6. Asynchronous Operations
- **Async Database Operations**: Utilize asynchronous database calls when possible to avoid blocking the application thread. This enhances the responsiveness of your application.

### 7. Error Handling
- **Graceful Error Management**: Implement robust error handling to catch database connection issues or execution errors and provide meaningful feedback or logging without crashing the application.
- **Retry Logic**: Implement retry logic for transient errors, such as timeout exceptions, to improve resiliency.

### 8. Profiling and Monitoring
- **Performance Monitoring**: Utilize profiling tools to monitor query performance and identify bottlenecks. Monitor active queries and database load to optimize performance over time.
- **Logging**: Implement logging to capture slow queries and errors, which can help diagnose issues and improve performance.

### 9. Memory Management
- **Batch Operations**: Use batch inserts or updates to minimize the number of database calls and optimize memory usage. 
- **Garbage Collection**: Ensure any allocated resources are appropriately disposed of or released, especially in long-running transactions or services.

### 10. Utilize the Right Tools and Technologies
- **Choose the Right Database**: Select a database system that fits your application’s performance, scalability, and complexity needs (e.g., SQL vs. NoSQL).
- **Leverage ORM Tools**: If using an Object-Relational Mapping (ORM) tool, choose one that is efficient and provides caching or lazy loading features.

### Example in EkoDB Context
For a backend built using ekoDB, you can implement these practices by utilizing features like:
- **Stored Functions** to encapsulate database logic and improve transaction safety.
- **Custom Indexes** to optimize query performance for your specific data retrieval patterns.
- **Asynchronous query handling** to improve application responsiveness.

By integrating these practices, you can write high-performance, memory-safe database interactions that improve the reliability and efficiency of your applications.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.296s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3804 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.497s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_new_question: Writing memory-safe, high-performance database code involves several best practices across both the programming and database management layers. Here are some strategies:

### 1. Use Prepared Statements
- **SQL Injection Prevention**: Always use prepared statements to ensure that input data is safely handled rather than directly concatenated into queries, which can lead to SQL injection vulnerabilities.
- **Performance**: Prepared statements can also improve performance, as the database can cache the execution plan.

### 2. Transactions
- **Atomic Operations**: Use transactions to group multiple operations together to ensure they either complete successfully or leave the database unchanged. This ensures data integrity.
- **Concurrency Control**: Use optimistic or pessimistic locking as necessary to manage concurrent access to shared resources.

### 3. Connection Management
- **Connection Pooling**: Use a connection pool to handle database connections efficiently. This minimizes the overhead of establishing new connections by reusing existing ones.

### 4. Indexing
- **Optimize Query Performance**: Create appropriate indexes on the database to speed up query performance. Use the `query_explain` tool (in ekoDB) to understand how queries are being executed and improve the indexing strategy.
- **Avoid Over-Indexing**: Too many indexes can slow down write operations, so balance read and write efficiencies.

### 5. Data Modeling
- **Normalization**: Normalize database schema to reduce redundancy, which also helps ensure data integrity.
- **Denormalization When Necessary**: In performance-critical applications, consider selectively denormalizing data for read-heavy operations to reduce joins.

### 6. Asynchronous Operations
- **Async Database Operations**: Utilize asynchronous database calls when possible to avoid blocking the application thread. This enhances the responsiveness of your application.

### 7. Error Handling
- **Graceful Error Management**: Implement robust error handling to catch database connection issues or execution errors and provide meaningful feedback or logging without crashing the application.
- **Retry Logic**: Implement retry logic for transient errors, such as timeout exceptions, to improve resiliency.

### 8. Profiling and Monitoring
- **Performance Monitoring**: Utilize profiling tools to monitor query performance and identify bottlenecks. Monitor active queries and database load to optimize performance over time.
- **Logging**: Implement logging to capture slow queries and errors, which can help diagnose issues and improve performance.

### 9. Memory Management
- **Batch Operations**: Use batch inserts or updates to minimize the number of database calls and optimize memory usage. 
- **Garbage Collection**: Ensure any allocated resources are appropriately disposed of or released, especially in long-running transactions or services.

### 10. Utilize the Right Tools and Technologies
- **Choose the Right Database**: Select a database system that fits your application’s performance, scalability, and complexity needs (e.g., SQL vs. NoSQL).
- **Leverage ORM Tools**: If using an Object-Relational Mapping (ORM) tool, choose one that is efficient and provides caching or lazy loading features.

### Example in EkoDB Context
For a backend built using ekoDB, you can implement these practices by utilizing features like:
- **Stored Functions** to encapsulate database logic and improve transaction safety.
- **Custom Indexes** to optimize query performance for your specific data retrieval patterns.
- **Asynchronous query handling** to improve application responsiveness.

By integrating these practices, you can write high-performance, memory-safe database interactions that improve the reliability and efficiency of your applications.

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

=== Cleanup ===
✓ Cleanup complete


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


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning TypeScript RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 19.515s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.436s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.310s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.345s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.325s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.311s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.265s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.283s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.289s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.293s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.275s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.265s
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
    ✓ Generated embedding: 1536 dimensions in 0.268s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.080s
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

Writing memory-safe and high-performance database code requires a careful approach to programming practices and database interaction. Here are some key strategies to consider:

### 1. Use Strong Typing and Memory Safety Features
- If you're using a programming language like Rust, employ its ownership and borrowing system to ensure memory safety at compile time.
- In languages like C#, utilize features like `using` statements or `Dispose` patterns to manage resource cleanup automatically.

### 2. Optimize Query Performance
- **Use Prepared Statements**: They can prevent SQL injection attacks and optimize performance by allowing the database to cache query plans.
- **Batch Operations**: Instead of executing numerous single-row operations, group them into a batch transaction to reduce round trips to the database.
  
### 3. Indexing
- Ensure that your database tables are appropriately indexed based on query patterns. This improves the speed of data retrieval significantly.
- Regularly monitor and analyze query performance using tools to identify and create necessary indexes.

### 4. Data Access Patterns
- **Lazy Loading**: Load data as needed rather than all at once, especially large datasets. This can help reduce memory usage.
- **Pagination**: For queries that may return a lot of data, always implement pagination to limit the number of records handled at a time.

### 5. Connection Management
- Use a connection pool to manage database connections efficiently, minimizing the overhead associated with creating and closing connections frequently.
- Ensure proper handling of connection lifecycle events to prevent memory leaks.

### 6. Error Handling
- Implement robust error handling to manage database exceptions gracefully. This can prevent cascading failures and ensure the application remains stable.

### 7. Use ORM Wisely
- When using Object-Relational Mappers (ORMs), be cautious of the overhead they can introduce. Optimize queries generated by the ORM and avoid over-fetching of data.
- Understand the underlying SQL generated by the ORM to identify potential inefficiencies.

### 8. Monitor and Profile
- Regularly profile your application's performance and memory usage. Tools like profilers and monitoring frameworks can help identify bottlenecks.
- Use database performance monitoring tools to identify slow queries or resource contention issues.

### 9. Consider Asynchronous Processing
- In environments where high throughput is necessary, consider using asynchronous programming paradigms to prevent blocking operations when accessing the database.

### 10. Use Transactions Wisely
- For multiple related database operations, wrap them in transactions to ensure atomicity. However, be mindful of transaction duration and locking, which could affect performance.

### Example With ekoDB Tools
In the context of ekoDB, you might focus on:
- Using stored functions (`create_function`) for encapsulating operations.
- Ensuring you make use of `batch_transaction` for multi-record operations to enhance performance.

### Conclusion
Combining these strategies will help you develop database code that is both memory-safe and performant. Always iterate and test under realistic load conditions to ensure your approach meets the demands of your application. If you're interested, I can assist with specific implementations or tool usage for optimizing database operations with ekoDB.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.352s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3411 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.250s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


→ Executing textSearch()...
  • Collection: rag_messages
  • Query: "ownership system"
  • Limit: 3 results
  • Search method: Full-text with fuzzy matching & stemming
  • No vector embeddings needed - pure keyword search
  ✓ Text search completed in 0.052s
✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_new_question: Writing memory-safe and high-performance database code requires a careful approach to programming practices and database interaction. Here are some key strategies to consider:

### 1. Use Strong Typing and Memory Safety Features
- If you're using a programming language like Rust, employ its ownership and borrowing system to ensure memory safety at compile time.
- In languages like C#, utilize features like `using` statements or `Dispose` patterns to manage resource cleanup automatically.

### 2. Optimize Query Performance
- **Use Prepared Statements**: They can prevent SQL injection attacks and optimize performance by allowing the database to cache query plans.
- **Batch Operations**: Instead of executing numerous single-row operations, group them into a batch transaction to reduce round trips to the database.
  
### 3. Indexing
- Ensure that your database tables are appropriately indexed based on query patterns. This improves the speed of data retrieval significantly.
- Regularly monitor and analyze query performance using tools to identify and create necessary indexes.

### 4. Data Access Patterns
- **Lazy Loading**: Load data as needed rather than all at once, especially large datasets. This can help reduce memory usage.
- **Pagination**: For queries that may return a lot of data, always implement pagination to limit the number of records handled at a time.

### 5. Connection Management
- Use a connection pool to manage database connections efficiently, minimizing the overhead associated with creating and closing connections frequently.
- Ensure proper handling of connection lifecycle events to prevent memory leaks.

### 6. Error Handling
- Implement robust error handling to manage database exceptions gracefully. This can prevent cascading failures and ensure the application remains stable.

### 7. Use ORM Wisely
- When using Object-Relational Mappers (ORMs), be cautious of the overhead they can introduce. Optimize queries generated by the ORM and avoid over-fetching of data.
- Understand the underlying SQL generated by the ORM to identify potential inefficiencies.

### 8. Monitor and Profile
- Regularly profile your application's performance and memory usage. Tools like profilers and monitoring frameworks can help identify bottlenecks.
- Use database performance monitoring tools to identify slow queries or resource contention issues.

### 9. Consider Asynchronous Processing
- In environments where high throughput is necessary, consider using asynchronous programming paradigms to prevent blocking operations when accessing the database.

### 10. Use Transactions Wisely
- For multiple related database operations, wrap them in transactions to ensure atomicity. However, be mindful of transaction duration and locking, which could affect performance.

### Example With ekoDB Tools
In the context of ekoDB, you might focus on:
- Using stored functions (`create_function`) for encapsulating operations.
- Ensuring you make use of `batch_transaction` for multi-record operations to enhance performance.

### Conclusion
Combining these strategies will help you develop database code that is both memory-safe and performant. Always iterate and test under realistic load conditions to ensure your approach meets the demands of your application. If you're interested, I can assist with specific implementations or tool usage for optimizing database operations with ekoDB.

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

=== Cleanup ===
✓ Cleanup complete


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


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Go RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.244s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.278s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.356s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.263s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.304s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.368s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.264s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.338s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.225s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.287s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.268s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.258s
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
    ✓ Generated embedding: 1536 dimensions in 0.288s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages
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
2026/06/26 13:06:27 Network error, retrying after 128.760895ms...
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves a combination of best practices at both the database and application levels. Here are some key principles to follow:

### 1. **Query Optimization**
   - **Use Prepared Statements:** Prepared statements help mitigate SQL injection attacks and often improve performance by allowing the database to cache execution plans.
   - **Indexing:** Create indexes on columns that are frequently used in WHERE clauses, JOIN conditions, and sorting. Use composite indexes when multiple fields are involved.
   - **Batch Operations:** Use batch inserts and updates to reduce the number of individual transactions sent to the database.

### 2. **Connection Management**
   - **Connection Pooling:** Use connection pooling to manage database connections efficiently. This reduces the overhead of establishing new connections for each database operation.
   - **Close Connections:** Ensure that database connections are properly closed after use to prevent resource leaks.
   
### 3. **Data Integrity and Safety**
   - **Transactions:** Wrap multiple SQL statements in transactions to ensure atomicity. This ensures that if one part of a transaction fails, the entire transaction can be rolled back.
   - **Validation:** Validate inputs on the application side before executing queries. This can reduce the risk of corrupting the database with invalid data.

### 4. **Memory Management**
   - **Efficient Data Types:** Use the most appropriate and efficient data types for your fields (e.g., avoid using `TEXT` when `VARCHAR` will suffice).
   - **Batch Fetching:** Retrieve only the necessary data when querying and consider pagination for large datasets.
   - **Avoid Memory Leaks:** Keep an eye on memory usage and avoid keeping large datasets in memory when not necessary.

### 5. **Concurrency Control**
   - **Locking Mechanisms:** Understand the locking mechanisms of your database. Use appropriate isolation levels as per your requirement to ensure data consistency while optimizing performance.
   - **Optimistic Concurrency:** When appropriate, use optimistic concurrency control to allow multiple transactions to occur without locking resources and improve throughput.

### 6. **Monitoring and Profiling**
   - **Use Profiling Tools:** Monitor your database's performance regularly using profiling tools. Identify slow queries and many reading/writing operations and optimize them.
   - **Analyze Query Execution Plans:** Use execution plans to understand how the database executes a query. Look for full table scans and other inefficiencies.

### 7. **Programming Language Best Practices**
   - **Safe Libraries:** Use library functions and frameworks that handle memory safety, such as those built into languages like Rust, which emphasize safety by design.
   - **Error Handling:** Implement robust error handling to manage exceptions that may occur during database operations gracefully.

### 8. **Regular Maintenance**
   - **Database Maintenance:** Regularly perform maintenance tasks such as vacuuming (for PostgreSQL), analyzing tables, and optimizing them based on usage patterns to improve performance.

By integrating these practices, you can write database code that is both memory-safe and optimized for performance, leading to a more reliable and efficient application.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.446s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3338 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.414s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


→ Executing TextSearch()...
  • Collection: rag_messages
  • Query: "ownership system"
  • Limit: 3 results
  • Search method: Full-text with fuzzy matching & stemming
  • No vector embeddings needed - pure keyword search
  ✓ Text search completed in 0.051s

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_rust_programming: The borrow checker enforces Rust's ownership rules at compile time. It ensures that references don't outlive the data they point to and prevents data races by allowing either multiple immutable references or one mutable reference.

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

=== Cleanup ===
✓ Cleanup complete


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


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Kotlin RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
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
    ✓ Generated embedding: 1536 dimensions in 0.367s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.589s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.538s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.503s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.486s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.417s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.262s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.31s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.25s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.261s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.353s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.281s
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
    ✓ Generated embedding: 1536 dimensions in 0.262s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.101s

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
    ✓ Generated embedding: 1536 dimensions in 0.36s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


→ Executing textSearch()...
  • Collection: rag_messages
  • Query: "ownership system"
  • Limit: 3 results
  • Search method: Full-text with fuzzy matching & stemming
  • No vector embeddings needed - pure keyword search
  ✓ Text search completed in 0.037s

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

=== Cleanup ===
✓ Cleanup complete


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


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
✅ [32mRAG Examples Complete![0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[32mWhat you just saw across 5 languages:[0m
  ✓ Embeddings generated via ekoDB Functions
  ✓ Hybrid search (semantic + keyword)
  ✓ Text search with stemming
  ✓ Cross-conversation context retrieval
  ✓ Simple client helpers wrapping powerful AI

[36mMission: AI for All 🚀[0m - Making RAG accessible to everyone!

