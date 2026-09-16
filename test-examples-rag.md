make test-examples-rag
🐍 Building Python client package...
🔧 Ensuring maturin is available in .venv...
🔨 Building wheel...
🍹 Building a mixed python/rust project
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.22s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.26.4-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.26.4-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.26.4
    Uninstalling ekodb_client-0.26.4:
      Successfully uninstalled ekodb_client-0.26.4
Successfully installed ekodb-client-0.26.4
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
    ✓ Generated embedding: 1536 dimensions in 360.470875ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 300.273083ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 404.269167ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 262.820667ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 295.301959ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 305.134333ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 364.563667ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 327.223667ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 277.821916ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 262.9575ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 429.75675ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 251.396625ms
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
    ✓ Generated embedding: 1536 dimensions in 249.024958ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 76.088458ms

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

Writing memory-safe and high-performance database code involves several strategies that leverage best practices in programming, database design, and performance optimization. Here’s a comprehensive approach that combines these aspects:

### 1. Use Appropriate Database Technology
- **Choose the Right DBMS:** Assess your application needs to choose between SQL (for structured, relational data) and NoSQL (for unstructured or semi-structured data). Use NoSQL for flexible schemas and horizontal scaling.
- **Proper Indexing:** Use indexes on frequently queried fields to improve lookup times. Monitor and optimize existing indexes.

### 2. Memory Safety
- **Use Memory-Safe Programming Languages:** Opt for languages that enforce memory safety through built-in features (e.g., Rust, Java) when possible.
- **Handling Pointers Carefully:** In languages like C/C++, ensure pointers are handled with care to avoid memory leaks or segmentation faults.
- **Avoid Buffer Overflows:** Implement bounds-checking and utilize safe libraries for string handling.
- **Garbage Collection:** In languages with garbage collection (e.g., Java, C#), ensure you manage object lifetimes correctly to avoid memory leaks.

### 3. Optimizing Database Queries
- **Optimize Query Logic:** Write efficient, well-structured queries. Use `EXPLAIN` to analyze query plans and find bottlenecks.
- **Batch Operations:** Use batch inserts/updates instead of individual operations to reduce transaction overhead.
- **Connection Pooling:** Reuse database connections instead of opening a new one for each request to decrease latency.

### 4. Error Handling
- **Graceful Degradation:** Implement proper error handling to ensure that your application can recover or gracefully handle unexpected database errors without crashing.
- **Transactions:** Use transactions to ensure data integrity and rollback mechanisms during failures.

### 5. Data Manipulation
- **Prepared Statements:** Use prepared statements to prevent SQL injection and improve performance by allowing the database to cache execution plans.
- **Normalization:** Normalize your database to eliminate redundancy, which can help reduce memory usage and improve data integrity.

### 6. Caching Strategies
- **Query Caching:** Cache frequently accessed data to reduce load on the database. Use in-memory storage (like Redis) for caching.
- **Materialized Views:** Create materialized views for complex queries that are costly to execute.

### 7. Profiling and Monitoring
- **Performance Profiling:** Continuously profile your database interactions to identify slow queries and optimize them.
- **Monitoring Tools:** Use database monitoring tools to keep an eye on memory usage, query performance, and database health.

### 8. Scaling Considerations
- **Partitioning:** For heavily load-balanced applications, consider sharding the database or partitioning data across multiple databases.
- **Load Balancing:** Distribute database load across multiple replicas or instances to enhance response times and reduce latency.

### Example Snippet
Here’s an example of using prepared statements in Python with a PostgreSQL database to ensure memory safety and performance:

```python
import psycopg2
from psycopg2 import sql

# Connection pooling
from psycopg2 import pool
db_pool = pool.SimpleConnectionPool(minconn=1, maxconn=10, user='user', password='password', host='127.0.0.1', port='5432', database='test_db')

def insert_user(name, email):
    conn = db_pool.getconn()
    try:
        with conn.cursor() as cursor:
            cursor.execute("INSERT INTO users (name, email) VALUES (%s, %s)", (name, email))
            conn.commit()  # Ensure atomicity
    except Exception as e:
        print(f"Error occurred: {e}")
        conn.rollback()
    finally:
        db_pool.putconn(conn)  # Return the connection to the pool
```

### In Summary
Maintain a balance between memory safety and performance by choosing the right tools, optimizing your database interactions, and constantly monitoring performance metrics. This multifaceted approach can significantly enhance the reliability and efficiency of your database code.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 274.832083ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 4154 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 316.989375ms
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
  ✓ Text search completed in 44.164792ms

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
    ✓ Generated embedding: 1536 dimensions in 0.261s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.297s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.273s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.572s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.300s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.323s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.238s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.282s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.344s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.406s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.337s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.266s
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
    ✓ Generated embedding: 1536 dimensions in 0.292s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.068s

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

Writing memory-safe, high-performance database code involves several best practices and approaches that span the design of your application, the choice of the database, and the handling of data within your code. Below are some key strategies to consider:

### 1. **Choose the Right Database**

- **NoSQL vs. SQL**: Depending on your needs, choose a suitable database type. SQL databases are great for complex queries and relationships, while NoSQL databases excel in horizontal scaling and working with unstructured data.
- **Database Optimization**: Take advantage of built-in database features such as indexing, caching, and partitioning to improve performance.

### 2. **Connection Management**

- **Use Connection Pools**: Instead of opening a new connection for each request, use connection pooling to manage database connections efficiently.
- **Limit Connection Lifespan**: Occasionally refresh connections to avoid stale connections that can cause memory leaks.

### 3. **Efficient Query Writing**

- **Prepared Statements**: Use prepared statements to prevent SQL injection attacks and reduce the overhead of query parsing.
- **Optimize Queries**: Assess query execution plans and use the right indexes (e.g., composite indexes for multi-field filters) to speed up data access.
- **Batch Operations**: Use batch insert/update queries to reduce round trips to the database.

### 4. **Memory Management**

- **Limit Result Set Size**: Only fetch the necessary data by applying pagination or limits to your queries.
- **Lazy Loading**: Consider implementing lazy loading patterns where related data is only fetched when needed.
- **Free Unused Resources**: Ensure that cursors and result sets are properly closed and cleaned up after use to free memory.

### 5. **Concurrency Handling**

- **Transactions**: Use database transactions to ensure data consistency and integrity, especially when performing multiple related operations.
- **Optimistic Concurrency Control**: Implement optimistic concurrency controls where applicable to handle updates without lengthy locks.

### 6. **Error Handling**

- **Graceful Error Handling**: Implement robust error handling to capture and log exceptions without crashing the application.
- **Graceful Degradation**: Ensure that when failures occur, the system degrades gracefully rather than becoming entirely unresponsive.

### 7. **Performance Monitoring**

- **Evaluate Performance**: Regularly monitor performance metrics using built-in database tools and custom logging to discover slow queries or high resource usage.
- **Profile Database Calls**: Use profiling tools to analyze database calls and identify bottlenecks in your application.

### 8. **Use of Database Tools and Libraries**

- Leverage ORM (Object-Relational Mapping) libraries that support memory safety. Some ORMs automatically handle connection pooling, memory management, and query optimization for you while maintaining a focus on high performance.

### Example (Using ekoDB)

If you are using ekoDB, which provides a full backend platform, you can follow these design principles effectively. For instance:

- Utilize `create_function` to create optimized stored procedures to encapsulate complex operations.
- Employ `query_explain` before executing queries to ensure they are optimized and use indexes appropriately.
- Manage connection lifecycles with provided tools seamlessly without leaking connections.

### Conclusion

By combining efficient database design, robust code practices, and performance monitoring, you can create memory-safe and high-performance database code. Always consider the unique needs of your application and system architecture when adopting these practices.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.288s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3706 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.251s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_new_question: Writing memory-safe, high-performance database code involves several best practices and approaches that span the design of your application, the choice of the database, and the handling of data within your code. Below are some key strategies to consider:

### 1. **Choose the Right Database**

- **NoSQL vs. SQL**: Depending on your needs, choose a suitable database type. SQL databases are great for complex queries and relationships, while NoSQL databases excel in horizontal scaling and working with unstructured data.
- **Database Optimization**: Take advantage of built-in database features such as indexing, caching, and partitioning to improve performance.

### 2. **Connection Management**

- **Use Connection Pools**: Instead of opening a new connection for each request, use connection pooling to manage database connections efficiently.
- **Limit Connection Lifespan**: Occasionally refresh connections to avoid stale connections that can cause memory leaks.

### 3. **Efficient Query Writing**

- **Prepared Statements**: Use prepared statements to prevent SQL injection attacks and reduce the overhead of query parsing.
- **Optimize Queries**: Assess query execution plans and use the right indexes (e.g., composite indexes for multi-field filters) to speed up data access.
- **Batch Operations**: Use batch insert/update queries to reduce round trips to the database.

### 4. **Memory Management**

- **Limit Result Set Size**: Only fetch the necessary data by applying pagination or limits to your queries.
- **Lazy Loading**: Consider implementing lazy loading patterns where related data is only fetched when needed.
- **Free Unused Resources**: Ensure that cursors and result sets are properly closed and cleaned up after use to free memory.

### 5. **Concurrency Handling**

- **Transactions**: Use database transactions to ensure data consistency and integrity, especially when performing multiple related operations.
- **Optimistic Concurrency Control**: Implement optimistic concurrency controls where applicable to handle updates without lengthy locks.

### 6. **Error Handling**

- **Graceful Error Handling**: Implement robust error handling to capture and log exceptions without crashing the application.
- **Graceful Degradation**: Ensure that when failures occur, the system degrades gracefully rather than becoming entirely unresponsive.

### 7. **Performance Monitoring**

- **Evaluate Performance**: Regularly monitor performance metrics using built-in database tools and custom logging to discover slow queries or high resource usage.
- **Profile Database Calls**: Use profiling tools to analyze database calls and identify bottlenecks in your application.

### 8. **Use of Database Tools and Libraries**

- Leverage ORM (Object-Relational Mapping) libraries that support memory safety. Some ORMs automatically handle connection pooling, memory management, and query optimization for you while maintaining a focus on high performance.

### Example (Using ekoDB)

If you are using ekoDB, which provides a full backend platform, you can follow these design principles effectively. For instance:

- Utilize `create_function` to create optimized stored procedures to encapsulate complex operations.
- Employ `query_explain` before executing queries to ensure they are optimized and use indexes appropriately.
- Manage connection lifecycles with provided tools seamlessly without leaking connections.

### Conclusion

By combining efficient database design, robust code practices, and performance monitoring, you can create memory-safe and high-performance database code. Always consider the unique needs of your application and system architecture when adopting these practices.

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
    ✓ Generated embedding: 1536 dimensions in 0.290s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.326s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.298s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.280s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.277s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.288s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.316s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.278s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.261s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.233s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.258s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.231s
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
    ✓ Generated embedding: 1536 dimensions in 0.215s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.079s
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

To write memory-safe high-performance database code, consider the following strategies that combine best practices in database management, programming languages, and system architectures:

### 1. **Use Prepared Statements and Parameterized Queries**
   - **Prevention of SQL Injection**: Ensure that queries are safe from injection attacks by using prepared statements, which separate SQL code from data.
   - **Performance**: Prepared statements can be pre-parsed and compiled, which can lead to performance improvements, especially for repeated queries.

### 2. **Connection Pooling**
   - **Efficient Resource Management**: Utilize connection pooling to manage database connections efficiently. This reduces the overhead of establishing new connections and helps maintain performance.
   - **Concurrency Handling**: Pooling allows multiple requests to share connections, enhancing scalability in applications with high concurrency demands.

### 3. **Memory Management**
   - **Garbage Collection**: In languages like Java or C#, rely on garbage collection for automatic memory management, but also monitor memory usage to avoid leaks.
   - **Manual Memory Management**: In languages like C or C++, ensure to free allocated memory appropriately to prevent memory leaks, utilizing smart pointers where applicable.

### 4. **Optimize Queries**
   - **Indexing**: Create appropriate indexes on database tables to speed up query performance. Analyze query performance using explain plans to understand how indexes are utilized.
   - **Normalization**: Normalize database schemas to reduce redundancy and improve data integrity, but also consider denormalization for read-heavy applications to minimize JOIN operations.

### 5. **Batch Processing**
   - **Bulk Inserts and Updates**: Use batch operations for inserts and updates to reduce the number of database transactions. This is more efficient than executing many individual queries.
   - **Transaction Management**: Wrap multiple operations in transactions to ensure atomicity and to minimize the overhead of commit operations.

### 6. **Asynchronous I/O Operations**
   - **Non-Blocking Calls**: If supported by your database and programming environment, use asynchronous I/O to avoid blocking application threads while waiting for database responses, improving overall application responsiveness.

### 7. **Caching Mechanisms**
   - **In-Memory Caching**: Utilize in-memory caching strategies (like Redis or Memcached) to reduce the load on your database and speed up access to frequently queried data.
   - **Application-Level Caching**: Implement caching at the application level to store results of expensive queries for quick retrieval.

### 8. **Profiling and Monitoring**
   - **Database Monitoring Tools**: Use tools to monitor query performance and system resource usage to identify bottlenecks (e.g., slow queries, high CPU, or memory consumption).
   - **Profile Your Code**: Use profiling tools in your development environment to analyze memory usage and identify performance hotspots.

### 9. **Utilize Connection Options Wisely**
   - **Timeouts and Retries**: Set appropriate connection timeouts and retry policies to handle transient failures gracefully without crashing the application.
   - **Transaction Isolation Levels**: Choose the correct isolation level based on the use case to balance between data integrity and performance.

### 10. **Concurrency Control**
   - **Optimistic Locking**: Implement optimistic locking to avoid blocking database rows during updates, which can improve performance in high-write environments.
   - **Pessimistic Locking**: In scenarios demanding strong consistency, use pessimistic locking judiciously.

By integrating these strategies into your coding practices, you can create database interactions that are both memory-safe and performance-oriented. Always ensure to test under load and review code regularly for performance improvements.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.269s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3950 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.246s
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
  ✓ Text search completed in 0.044s
✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_new_question: To write memory-safe high-performance database code, consider the following strategies that combine best practices in database management, programming languages, and system architectures:

### 1. **Use Prepared Statements and Parameterized Queries**
   - **Prevention of SQL Injection**: Ensure that queries are safe from injection attacks by using prepared statements, which separate SQL code from data.
   - **Performance**: Prepared statements can be pre-parsed and compiled, which can lead to performance improvements, especially for repeated queries.

### 2. **Connection Pooling**
   - **Efficient Resource Management**: Utilize connection pooling to manage database connections efficiently. This reduces the overhead of establishing new connections and helps maintain performance.
   - **Concurrency Handling**: Pooling allows multiple requests to share connections, enhancing scalability in applications with high concurrency demands.

### 3. **Memory Management**
   - **Garbage Collection**: In languages like Java or C#, rely on garbage collection for automatic memory management, but also monitor memory usage to avoid leaks.
   - **Manual Memory Management**: In languages like C or C++, ensure to free allocated memory appropriately to prevent memory leaks, utilizing smart pointers where applicable.

### 4. **Optimize Queries**
   - **Indexing**: Create appropriate indexes on database tables to speed up query performance. Analyze query performance using explain plans to understand how indexes are utilized.
   - **Normalization**: Normalize database schemas to reduce redundancy and improve data integrity, but also consider denormalization for read-heavy applications to minimize JOIN operations.

### 5. **Batch Processing**
   - **Bulk Inserts and Updates**: Use batch operations for inserts and updates to reduce the number of database transactions. This is more efficient than executing many individual queries.
   - **Transaction Management**: Wrap multiple operations in transactions to ensure atomicity and to minimize the overhead of commit operations.

### 6. **Asynchronous I/O Operations**
   - **Non-Blocking Calls**: If supported by your database and programming environment, use asynchronous I/O to avoid blocking application threads while waiting for database responses, improving overall application responsiveness.

### 7. **Caching Mechanisms**
   - **In-Memory Caching**: Utilize in-memory caching strategies (like Redis or Memcached) to reduce the load on your database and speed up access to frequently queried data.
   - **Application-Level Caching**: Implement caching at the application level to store results of expensive queries for quick retrieval.

### 8. **Profiling and Monitoring**
   - **Database Monitoring Tools**: Use tools to monitor query performance and system resource usage to identify bottlenecks (e.g., slow queries, high CPU, or memory consumption).
   - **Profile Your Code**: Use profiling tools in your development environment to analyze memory usage and identify performance hotspots.

### 9. **Utilize Connection Options Wisely**
   - **Timeouts and Retries**: Set appropriate connection timeouts and retry policies to handle transient failures gracefully without crashing the application.
   - **Transaction Isolation Levels**: Choose the correct isolation level based on the use case to balance between data integrity and performance.

### 10. **Concurrency Control**
   - **Optimistic Locking**: Implement optimistic locking to avoid blocking database rows during updates, which can improve performance in high-write environments.
   - **Pessimistic Locking**: In scenarios demanding strong consistency, use pessimistic locking judiciously.

By integrating these strategies into your coding practices, you can create database interactions that are both memory-safe and performance-oriented. Always ensure to test under load and review code regularly for performance improvements.

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
    ✓ Generated embedding: 1536 dimensions in 0.307s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.228s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.269s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.318s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.307s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.271s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.252s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.273s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.302s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.276s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.242s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.260s
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
    ✓ Generated embedding: 1536 dimensions in 0.244s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.055s

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

Writing memory-safe, high-performance database code involves combining best practices in both database interactions and programming language constructs. Here’s a comprehensive approach to achieve this:

### 1. Use Efficient Querying Techniques
- **Prepared Statements**: Use prepared statements to enhance performance by pre-compiling SQL queries. This reduces the overhead of query parsing and execution.
- **Batch Operations**: When inserting or updating multiple records, use batch operations to minimize the number of database calls.
- **Proper Indexing**: Ensure that your database tables are appropriately indexed to speed up query performance. Analyze query plans to identify the need for indexes.

### 2. Optimize Database Design
- **Normalization**: Normalize your database schema to reduce redundancy, but also consider denormalization for read-heavy applications where performance is critical and data integrity can be managed.
- **Use Proper Data Types**: Choose efficient data types for your fields to reduce memory usage and increase performance (e.g., using INTEGER instead of BIGINT if possible).

### 3. Connection Management
- **Connection Pooling**: Utilize connection pooling to manage database connections efficiently, reducing the overhead of establishing database connections for each request.
- **Close Connections**: Ensure that all database connections are closed or returned to the pool promptly to avoid resource leaks.

### 4. Error Handling
- **Graceful Handling**: Implement proper error handling strategies (such as try-catch blocks) to manage exceptions without crashing the application and ensure resources are freed.
- **Use Transactions**: Use transactions to ensure that a series of database operations are completed successfully or rolled back in case of errors, maintaining data integrity.

### 5. Memory Management
- **Pooling Resources**: For languages like C/C++, use memory pool allocators to minimize fragmentation and allocation overhead.
- **Automatic Garbage Collection**: If using languages with garbage collection (e.g., Java, C#), ensure large datasets are processed in small chunks to allow for effective garbage collection.

### 6. Asynchronous Operations
- **Non-blocking I/O**: Consider using non-blocking I/O operations where possible to prevent blocking threads while waiting for database responses, especially in web applications.

### 7. Profiling and Monitoring
- **Performance Monitoring**: Use monitoring tools to analyze query performance, system resource utilization, and identify bottlenecks in the application's interaction with the database.
- **Regular Profiling**: Profile your code regularly to understand memory usage patterns and optimize hotspots.

### 8. Security Considerations
- **Sanitize Inputs**: Always validate and sanitize user inputs to prevent SQL injection attacks and other malicious behaviors.
- **Limit Permissions**: Operate with the least privileges necessary for database accounts to minimize security risks.

### 9. Use of Frameworks and Libraries
- **ORMs**: If using an Object-Relational Mapping (ORM) framework, ensure it’s optimized for performance. Some ORMs can generate inefficient queries if not configured properly.
- **Data Access Layers**: Implement a data access layer that abstracts the database interactions, allowing for better maintenance and reusability.

### Conclusion
Combining these practices will help you write database code that is both memory-safe and performs well. Always keep performance considerations in mind during the design and implementation phases, and continuously evaluate your code as the application and database grow.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.250s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3652 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.291s
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
  ✓ Text search completed in 0.041s

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

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

  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 34 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.313s
    ? Function auto-cleaned up by client
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 169 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.297s
    ? Function auto-cleaned up by client
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 33 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.248s
    ? Function auto-cleaned up by client
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 230 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.256s
    ? Function auto-cleaned up by client
? Stored Rust programming conversation (4 messages)
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 31 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.244s
    ? Function auto-cleaned up by client
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 217 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.296s
    ? Function auto-cleaned up by client
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 33 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.226s
    ? Function auto-cleaned up by client
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 232 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.285s
    ? Function auto-cleaned up by client
? Stored database design conversation (4 messages)
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 36 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.296s
    ? Function auto-cleaned up by client
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 178 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.281s
    ? Function auto-cleaned up by client
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 37 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.272s
    ? Function auto-cleaned up by client
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 213 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.245s
    ? Function auto-cleaned up by client
? Stored performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval ===
User asks: "How do I write memory-safe high-performance database code?"

=== Step 3: Searching Related Context ===
Using hybrid search to find relevant messages from all conversations...


? Generating embedding for user question...
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 58 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.288s
    ? Function auto-cleaned up by client

? Executing hybridSearch()...
  ? Collection: rag_messages
  ? Query text: "How do I write memory-safe high-performance database code?"
  ? Vector dimensions: 1536
  ? Limit: 5 results
  ? Search type: Semantic (vector) + Keyword (text)
  ? Server combines both scores for relevance ranking
  ? Search completed in 0.093s

? Found 5 related messages across all conversations:
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
? Context prepared from search results
? AI would use this context to generate comprehensive response

=== Step 5: Storing New Conversation ===
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 58 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.265s
    ? Function auto-cleaned up by client
? New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


? Executing textSearch()...
  ? Collection: rag_messages
  ? Query: "ownership system"
  ? Limit: 3 results
  ? Search method: Full-text with fuzzy matching & stemming
  ? No vector embeddings needed - pure keyword search
  ? Text search completed in 0.044s

? Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_rust_programming: The borrow checker enforces Rust's ownership rules at compile time. It ensures that references don't outlive the data they point to and prevents data races by allowing either multiple immutable references or one mutable reference.

=== System Statistics ===

? Querying database statistics...
  ? Using findAllWithLimit() helper - simplified query API

? Database Statistics:
  ? Total conversations: 4
  ? Total messages stored: 13
  ? All messages indexed for vector search ?
  ? All messages indexed for text search ?
  ? All messages queryable by metadata ?

=== Step 8: Dynamic Search Configuration ===
Each conversation can have its own search config...

? Conversations can store custom search configurations:
  ? Search type: hybrid, text, or vector
  ? Relevance thresholds
  ? Filter by tags or metadata
  ? Collection-specific settings
  ? Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!

=== Cleanup ===
? Cleanup complete


=== ? Summary: What This Example Showed ===

? ekoDB Native Capabilities Used:
  ? Functions with Embed operation (AI integration)
  ? Hybrid Search (text + vector combined)
  ? Text Search (full-text with stemming)
  ? Automatic embedding generation
  ? Cross-collection queries

? New Client Helper Methods:
  ? client.embed(text, model) - Generate embeddings
  ? client.hybridSearch() - Semantic + keyword search
  ? client.textSearch() - Full-text search
  ? client.findAllWithLimit() - Query all documents

? Key Takeaways:
  1. ekoDB handles AI Functions natively - no external services needed
  2. One-line embedding generation with auto-cleanup
  3. Hybrid search combines semantic understanding + keyword matching
  4. Perfect for RAG: store, search, and retrieve context
  5. All AI capabilities accessible through simple client methods

? Build production RAG systems with ekoDB!
   ? Set OPENAI_API_KEY in your ekoDB server environment
   ? Use these client helpers to make AI integration simple
   ? Scale to millions of documents with native indexing


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

