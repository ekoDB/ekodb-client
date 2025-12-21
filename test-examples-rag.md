make test-examples-rag

🤖 [36mRAG Conversation System Examples[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[33mPrerequisites:[0m

1. ekoDB server running (make run in ekodb/)
2. OPENAI_API_KEY set in server environment
3. API_BASE_URL and API_BASE_KEY exported in your shell

[36mBuilding Rust client library...[0m ✓ Rust client built

[36mBuilding Python client bindings...[0m 🍹 Building a mixed python/rust
project 🔗 Found pyo3 bindings with abi3 support 🐍 Not using a specific python
interpreter 💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by
default 📦 Built wheel for abi3 Python ≥ 3.8 to
/Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.4.0-cp38-abi3-macosx_11_0_arm64.whl
✓ Python client built and installed

[36mBuilding TypeScript client library...[0m ✓ TypeScript client built

[36mInstalling TypeScript client in examples...[0m ✓ TypeScript client installed

[36mBuilding TypeScript example...[0m ✓ TypeScript example built

[36mBuilding Go client library...[0m ✓ Go client built

[36mBuilding Go RAG example...[0m ✓ Go example built

[36mBuilding Kotlin client library...[0m ✓ Kotlin client built

[36mBuilding Kotlin RAG example...[0m ✓ Kotlin example built

[36mRunning Rust RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m === ekoDB RAG
Conversation System ===

This example shows how ekoDB can power a self-improving AI system that learns
from its own conversation history.

=== Step 1: Building Conversation History === Storing previous conversations
with embeddings...

→ Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 34 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 655.381459ms • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 169 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 900.51375ms • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 810.318292ms • Function auto-cleaned up
by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 230 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 412.315625ms • Function auto-cleaned up by client ✓ Stored Rust
programming conversation (4 messages) → Calling ekoDB embed() helper... • Using
model: text-embedding-3-small • Text length: 31 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 898.599208ms • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 217
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 707.353916ms • Function auto-cleaned up
by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 33 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 314.216375ms • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 232
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 321.235084ms • Function auto-cleaned up
by client ✓ Stored database design conversation (4 messages) → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 36
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 272.687583ms • Function auto-cleaned up
by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 178 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 473.736ms • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 37
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 303.668542ms • Function auto-cleaned up
by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 213 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 337.012875ms • Function auto-cleaned up by client ✓ Stored
performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval === User asks: "How do I
write memory-safe high-performance database code?"

=== Step 3: Searching Related Context === Using hybrid search to find relevant
messages from all conversations...

→ Generating embedding for user question... → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 319.341042ms • Function auto-cleaned up by client

→ Executing hybrid_search()... • Collection: rag_messages • Query text: "How do
I write memory-safe high-performance database code?" • Vector dimensions: 1536 •
Limit: 5 results • Search type: Semantic (vector) + Keyword (text) • Server
combines both scores for relevance ranking ✓ Search completed in 121.122459ms

✓ Found 5 related messages across all conversations:

1. From conv_database_design Use NoSQL when you need: flexible schemas,
   horizontal scaling, high write throughput, or when working with unstructured
   data. SQL is better for complex queries, ACID transactions, and structured
   data with well-defined relationships.

2. From conv_database_design What is database normalization?

3. From conv_database_design Database normalization is the process of organizing
   data to reduce redundancy and improve data integrity. It involves dividing
   large tables into smaller ones and defining relationships between them using
   foreign keys.

4. From conv_database_design When should I use NoSQL over SQL?

5. From conv_performance How can I optimize database queries?

=== Step 4: Generating Context-Aware Response === ✓ AI Response (with context
from 3 conversations):

Answer: Writing memory-safe, high-performance database code involves a
combination of good practices in both programming and database management. Here
are some tips:

1. **Normalization**: As discussed in Context 2 and 3, database normalization
   can help reduce redundancy and improve data integrity. This can also lead to
   performance benefits because it reduces the amount of data that needs to be
   loaded into memory.

2. **Optimize Queries**: As per Context 5, optimizing your database queries is
   crucial for performance. This could involve using indexes effectively,
   avoiding unnecessary columns in SELECT statements, leveraging the WHERE
   clause to limit data retrieval, and using JOINs effectively.

3. **Choosing the Right Database Type**: Depending on the nature of your data
   and the operations you need to perform, you might choose SQL or NoSQL
   databases (Context 1 and 4). For instance, if you're working with
   unstructured data and require high write throughput, a NoSQL database might
   be more suitable. On the other hand, SQL databases are better for complex
   queries and ACID transactions, which might be important for maintaining data
   integrity and consistency in memory-intensive applications.

4. **Use Efficient Data Structures and Algorithms**: In your code, make sure to
   use efficient data structures and algorithms that minimize memory usage and
   maximize performance. For instance, avoid using data structures that have a
   high memory overhead.

5. **Memory Management**: Depending on the programming language you are using,
   you might need to manually manage memory (e.g., in C or C++) or it might be
   automatically managed for you (e.g., in Python or Java). If you are manually
   managing memory, make sure to free up memory that is no longer needed to
   avoid memory leaks.

6. **Concurrency Control**: If multiple processes or threads are accessing and
   modifying your database, you need to implement proper concurrency control
   mechanisms to avoid conflicts and ensure data integrity.

7. **Caching**: Implement caching to save and quickly retrieve frequently
   accessed data, reducing the need for expensive database operations.

Remember, the best practices can vary depending on the specific use case,
database system, and programming language used.

=== Step 5: Storing New Conversation === → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 371.825792ms • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 2261
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 415.602667ms • Function auto-cleaned up
by client ✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search === Searching for messages about
'ownership' across ALL conversations...

→ Executing text_search()... • Collection: rag_messages • Query: "ownership
system" • Limit: 3 results • Search method: Full-text with fuzzy matching &
stemming • No vector embeddings needed - pure keyword search ✓ Text search
completed in 69.043875ms

✓ Found 3 messages mentioning ownership:

1. From conv_performance: Rust's ownership system provides zero-cost memory
   management. Use Box for heap allocation, Rc/Arc for shared ownership, and
   avoid cloning large data structures. The compiler optimizes away unnecessary
   allocations.

2. From conv_rust_programming: Rust's key features include: memory safety
   without garbage collection, zero-cost abstractions, ownership system,
   powerful type system, and excellent concurrency support.

3. From conv_rust_programming: The borrow checker enforces Rust's ownership
   rules at compile time. It ensures that references don't outlive the data they
   point to and prevents data races by allowing either multiple immutable
   references or one mutable reference.

=== System Statistics ===

→ Querying database statistics... • Using find_all() helper - simplified query
API

📊 Database Statistics: • Total conversations: 4 • Total messages stored: 14 •
All messages indexed for vector search ✓ • All messages indexed for text search
✓ • All messages queryable by metadata ✓

=== Step 8: Dynamic Search Configuration === Each conversation can have its own
search config...

💡 Conversations can store custom search configurations: • Search type: hybrid,
text, or vector • Relevance thresholds • Filter by tags or metadata •
Collection-specific settings • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!

=== Cleanup === Deleting example collections...

✅ All done! RAG system demonstrated successfully.

✓ Using search results to enhance AI responses (RAG) ✓ Building a self-improving
knowledge base ✓ Dynamic search configurations per conversation

ekoDB provides everything needed for AI-powered applications: • Vector search
(semantic similarity) • Text search (keyword matching) • Hybrid search (best of
both) • AI functions (Chat, Embed) • Flexible querying and filtering • All in
one database - no external dependencies!

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m [36mRunning
Python RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m === ekoDB RAG
Conversation System ===

This example shows how ekoDB can power a self-improving AI system that learns
from its own conversation history.

=== Step 1: Building Conversation History === Storing previous conversations
with embeddings...

→ Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 34 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.280s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 169 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.498s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.317s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 230 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.568s • Function
auto-cleaned up by client ✓ Stored Rust programming conversation (4 messages) →
Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 31 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 1.068s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 217 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.279s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.290s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 232 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.209s • Function
auto-cleaned up by client ✓ Stored database design conversation (4 messages) →
Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 36 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.350s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 178 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.243s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 37
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.312s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 213 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.291s • Function
auto-cleaned up by client ✓ Stored performance optimization conversation (4
messages)

=== Step 2: New User Question with Context Retrieval === User asks: "How do I
write memory-safe high-performance database code?"

=== Step 3: Searching Related Context === Using hybrid search to find relevant
messages from all conversations...

→ Generating embedding for user question... → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.545s • Function auto-cleaned up by client

→ Executing hybrid_search()... • Collection: rag_messages • Query text: "How do
I write memory-safe high-performance database code?" • Vector dimensions: 1536 •
Limit: 5 results • Search type: Semantic (vector) + Keyword (text) • Server
combines both scores for relevance ranking ✓ Search completed in 0.091s

✓ Found 5 related messages across all conversations:

1. [Score: 0.000] From conv_database_design Database normalization is the
   process of organizing data to reduce redundancy and improve data integrity.
   It involves dividing large tables into smaller ones and defining
   relationships between them using foreign keys.

2. [Score: 0.000] From conv_database_design Use NoSQL when you need: flexible
   schemas, horizontal scaling, high write throughput, or when working with
   unstructured data. SQL is better for complex queries, ACID transactions, and
   structured data with well-defined relationships.

3. [Score: 0.000] From conv_database_design What is database normalization?

4. [Score: 0.000] From conv_database_design When should I use NoSQL over SQL?

5. [Score: 0.000] From conv_performance How can I optimize database queries?

=== Step 4: Generating Context-Aware Response === ✓ AI Response (with context
from 3 conversations):

Answer: Writing memory-safe, high-performance database code involves a number of
strategies:

1. Normalization: As mentioned in Context 1, database normalization can help
   improve data integrity and reduce redundancy. This can lead to more efficient
   use of memory.

2. Choosing the right database type: Depending on your specific needs and the
   nature of your data, you may want to use SQL or NoSQL. As indicated in
   Context 2, NoSQL is ideal for flexible schemas, high write throughput,
   horizontal scaling, and working with unstructured data. SQL, on the other
   hand, is better for complex queries, ACID transactions, and structured data
   with well-defined relationships.

3. Query Optimization: One key aspect of high-performance database code is
   optimizing your queries. This can involve strategies such as using indexes
   effectively, avoiding full table scans by using WHERE clauses, and minimizing
   the use of wildcards.

4. Efficient Memory Management: To write memory-safe code, always ensure to free
   up any resources that are no longer needed. Avoid memory leaks by ensuring
   every allocated byte of memory is eventually freed.

5. Use of Prepared Statements: Prepared statements can be used to execute the
   same or similar database statements repeatedly with high efficiency. They are
   also beneficial from a security perspective as they assist in preventing SQL
   injection attacks.

6. Regular Monitoring and Tuning: Regularly monitor your database performance
   and tune it as needed. This could involve hardware upgrades, adjusting
   configuration settings, or modifying your database schema.

Remember, the specific methods and techniques for writing high-performance,
memory-safe code can vary depending on the specific database system being used.

=== Step 5: Storing New Conversation === → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.501s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 1747
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.389s • Function auto-cleaned up by
client ✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search === Searching for messages about
'ownership' across ALL conversations...

✓ Found 3 messages mentioning ownership:

1. From conv_performance: Rust's ownership system provides zero-cost memory
   management. Use Box for heap allocation, Rc/Arc for shared ownership, and
   avoid cloning large data structures. The compiler optimizes away unnecessary
   allocations.

2. From conv_rust_programming: Rust's key features include: memory safety
   without garbage collection, zero-cost abstractions, ownership system,
   powerful type system, and excellent concurrency support.

3. From conv_rust_programming: The borrow checker enforces Rust's ownership
   rules at compile time. It ensures that references don't outlive the data they
   point to and prevents data races by allowing either multiple immutable
   references or one mutable reference.

=== System Statistics === Total conversations: 4 Total messages stored: 14 All
messages are indexed for vector search ✓ All messages are indexed for text
search ✓ All messages are queryable by metadata ✓

=== Step 8: Dynamic Search Configuration === Each conversation can have its own
search config...

💡 Conversations can store custom search configurations: • Search type: hybrid,
text, or vector • Relevance thresholds • Filter by tags or metadata •
Collection-specific settings • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!

=== Cleanup === ✓ Cleanup complete

=== 📚 Summary: What This Example Showed ===

🔧 ekoDB Native Capabilities Used: ✓ Functions with Embed operation (AI
integration) ✓ Hybrid Search (text + vector combined) ✓ Text Search (full-text
with stemming) ✓ Automatic embedding generation ✓ Cross-collection queries

🚀 New Client Helper Methods: • client.embed(text, model) - Generate embeddings
• client.hybrid_search() - Semantic + keyword search • client.text_search() -
Full-text search • client.find_all() - Query all documents

💡 Key Takeaways:

1. ekoDB handles AI Functions natively - no external services needed
2. One-line embedding generation with auto-cleanup
3. Hybrid search combines semantic understanding + keyword matching
4. Perfect for RAG: store, search, and retrieve context
5. All AI capabilities accessible through simple client methods

🎯 Build production RAG systems with ekoDB! → Set OPENAI_API_KEY in your ekoDB
server environment → Use these client helpers to make AI integration simple →
Scale to millions of documents with native indexing

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m [36mRunning
TypeScript RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m === ekoDB RAG
Conversation System ===

This example shows how ekoDB can power a self-improving AI system that learns
from its own conversation history.

=== Step 1: Building Conversation History === Storing previous conversations
with embeddings...

→ Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 34 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.452s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 169 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.239s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.270s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 230 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.253s • Function
auto-cleaned up by client ✓ Stored Rust programming conversation (4 messages) →
Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 31 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.207s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 217 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.306s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.298s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 232 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.479s • Function
auto-cleaned up by client ✓ Stored database design conversation (4 messages) →
Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 36 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.262s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 178 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.272s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 37
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.259s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 213 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.244s • Function
auto-cleaned up by client ✓ Stored performance optimization conversation (4
messages)

=== Step 2: New User Question with Context Retrieval === User asks: "How do I
write memory-safe high-performance database code?"

=== Step 3: Searching Related Context === Using hybrid search to find relevant
messages from all conversations...

→ Generating embedding for user question... → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.208s • Function auto-cleaned up by client

→ Executing hybridSearch()... • Collection: rag_messages • Query text: "How do I
write memory-safe high-performance database code?" • Vector dimensions: 1536 •
Limit: 5 results • Search type: Semantic (vector) + Keyword (text) • Server
combines both scores for relevance ranking ✓ Search completed in 0.090s ✓ Found
5 related messages across all conversations:

1. [Score: 0.000] From conv_database_design Database normalization is the
   process of organizing data to reduce redundancy and improve data integrity.
   It involves dividing large tables into smaller ones and defining
   relationships between them using foreign keys.

2. [Score: 0.000] From conv_database_design Use NoSQL when you need: flexible
   schemas, horizontal scaling, high write throughput, or when working with
   unstructured data. SQL is better for complex queries, ACID transactions, and
   structured data with well-defined relationships.

3. [Score: 0.000] From conv_database_design What is database normalization?

4. [Score: 0.000] From conv_database_design When should I use NoSQL over SQL?

5. [Score: 0.000] From conv_performance How can I optimize database queries?

=== Step 4: Generating Context-Aware Response === ✓ AI Response (with context
from 3 conversations):

Answer: Writing memory-safe, high-performance database code involves a
combination of good design principles, effective query optimization, and
appropriate database selection:

1. Database Design: As mentioned earlier, database normalization is a key factor
   for data integrity and reduction of redundancy. A well-structured database
   can greatly enhance both performance and safety.

2. Query Optimization: Optimizing your database queries is fundamental for high
   performance. This might involve using indexes wisely, minimizing the use of
   joins, limiting the returned data by using SELECT instead of SELECT \*, and
   avoiding nested queries as much as possible.

3. Database Selection: Depending on your needs, you might choose SQL or NoSQL.
   SQL databases are typically better for complex queries and structured data
   with well-defined relationships, while NoSQL databases are better suited for
   flexible schemas, high write throughput, and unstructured data.

4. Memory Safety: To ensure memory safety, avoid operations that can cause
   buffer overflow or underflow, null pointer dereferencing, and memory leaks.
   In a high-level language, these are usually handled for you, but in
   lower-level languages, you may need to manage memory manually.

5. Caching: Implement caching strategies to reduce the load on your database,
   this can significantly improve performance and reduce cost.

6. Use Database Features: Most databases come with features to improve
   performance and ensure safety. These may include stored procedures, triggers,
   and views in SQL databases, or sharding and replication in NoSQL databases.

Remember, the choice of database and the specific techniques used will largely
depend on your specific use case and the nature of the data you're working with.

=== Step 5: Storing New Conversation === → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.246s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 1763
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.280s • Function auto-cleaned up by
client ✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search === Searching for messages about
'ownership' across ALL conversations...

→ Executing textSearch()... • Collection: rag_messages • Query: "ownership
system" • Limit: 3 results • Search method: Full-text with fuzzy matching &
stemming • No vector embeddings needed - pure keyword search ✓ Text search
completed in 0.052s ✓ Found 3 messages mentioning ownership:

1. From conv_performance: Rust's ownership system provides zero-cost memory
   management. Use Box for heap allocation, Rc/Arc for shared ownership, and
   avoid cloning large data structures. The compiler optimizes away unnecessary
   allocations.

2. From conv_rust_programming: Rust's key features include: memory safety
   without garbage collection, zero-cost abstractions, ownership system,
   powerful type system, and excellent concurrency support.

3. From conv_rust_programming: The borrow checker enforces Rust's ownership
   rules at compile time. It ensures that references don't outlive the data they
   point to and prevents data races by allowing either multiple immutable
   references or one mutable reference.

=== System Statistics ===

→ Querying database statistics... • Using findAllWithLimit() helper - simplified
query API

📊 Database Statistics: • Total conversations: 4 • Total messages stored: 14 •
All messages indexed for vector search ✓ • All messages indexed for text search
✓ • All messages queryable by metadata ✓

=== Step 8: Dynamic Search Configuration === Each conversation can have its own
search config...

💡 Conversations can store custom search configurations: • Search type: hybrid,
text, or vector • Relevance thresholds • Filter by tags or metadata •
Collection-specific settings • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!

=== Cleanup === ✓ Cleanup complete

=== 📚 Summary: What This Example Showed ===

🔧 ekoDB Native Capabilities Used: ✓ Functions with Embed operation (AI
integration) ✓ Hybrid Search (text + vector combined) ✓ Text Search (full-text
with stemming) ✓ Automatic embedding generation ✓ Cross-collection queries

🚀 New Client Helper Methods: • client.embed(text, model) - Generate embeddings
• client.hybridSearch() - Semantic + keyword search • client.textSearch() -
Full-text search • client.findAllWithLimit() - Query all documents

💡 Key Takeaways:

1. ekoDB handles AI Functions natively - no external services needed
2. One-line embedding generation with auto-cleanup
3. Hybrid search combines semantic understanding + keyword matching
4. Perfect for RAG: store, search, and retrieve context
5. All AI capabilities accessible through simple client methods

🎯 Build production RAG systems with ekoDB! → Set OPENAI_API_KEY in your ekoDB
server environment → Use these client helpers to make AI integration simple →
Scale to millions of documents with native indexing

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m [36mRunning Go
RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m === ekoDB RAG
Conversation System ===

This example shows how ekoDB can power a self-improving AI system that learns
from its own conversation history.

=== Step 1: Building Conversation History === Storing previous conversations
with embeddings...

→ Calling ekoDB Embed() helper... • Using model: text-embedding-3-small • Text
length: 34 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.327s • Function
auto-cleaned up by client → Calling ekoDB Embed() helper... • Using model:
text-embedding-3-small • Text length: 169 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.384s • Function auto-cleaned up by client → Calling ekoDB
Embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.229s • Function auto-cleaned up by
client → Calling ekoDB Embed() helper... • Using model: text-embedding-3-small •
Text length: 230 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.234s • Function
auto-cleaned up by client ✓ Stored Rust programming conversation (4 messages) →
Calling ekoDB Embed() helper... • Using model: text-embedding-3-small • Text
length: 31 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.259s • Function
auto-cleaned up by client → Calling ekoDB Embed() helper... • Using model:
text-embedding-3-small • Text length: 217 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.273s • Function auto-cleaned up by client → Calling ekoDB
Embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.243s • Function auto-cleaned up by
client → Calling ekoDB Embed() helper... • Using model: text-embedding-3-small •
Text length: 232 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.286s • Function
auto-cleaned up by client ✓ Stored database design conversation (4 messages) →
Calling ekoDB Embed() helper... • Using model: text-embedding-3-small • Text
length: 36 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.309s • Function
auto-cleaned up by client → Calling ekoDB Embed() helper... • Using model:
text-embedding-3-small • Text length: 178 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.469s • Function auto-cleaned up by client → Calling ekoDB
Embed() helper... • Using model: text-embedding-3-small • Text length: 37
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 1.015s • Function auto-cleaned up by
client → Calling ekoDB Embed() helper... • Using model: text-embedding-3-small •
Text length: 213 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.294s • Function
auto-cleaned up by client ✓ Stored performance optimization conversation (4
messages)

=== Step 2: New User Question with Context Retrieval === User asks: "How do I
write memory-safe high-performance database code?"

=== Step 3: Searching Related Context === Using hybrid search to find relevant
messages from all conversations...

→ Generating embedding for user question... → Calling ekoDB Embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.256s • Function auto-cleaned up by client

→ Executing HybridSearch()... • Collection: rag_messages • Query text: "How do I
write memory-safe high-performance database code?" • Vector dimensions: 1536 •
Limit: 5 results • Search type: Semantic (vector) + Keyword (text) • Server
combines both scores for relevance ranking ✓ Search completed in 0.085s

✓ Found 5 related messages across all conversations:

1. [Score: 0.000] From conv_database_design Database normalization is the
   process of organizing data to reduce redundancy and improve data integrity.
   It involves dividing large tables into smaller ones and defining
   relationships between them using foreign keys.

2. [Score: 0.000] From conv_database_design What is database normalization?

3. [Score: 0.000] From conv_database_design Use NoSQL when you need: flexible
   schemas, horizontal scaling, high write throughput, or when working with
   unstructured data. SQL is better for complex queries, ACID transactions, and
   structured data with well-defined relationships.

4. [Score: 0.000] From conv_database_design When should I use NoSQL over SQL?

5. [Score: 0.000] From conv_performance How can I optimize database queries?

=== Step 4: Generating Context-Aware Response === ✓ AI Response (with context
from 3 conversations):

Writing memory-safe, high-performance database code involves a variety of
practices. Here are a few key ones:

1. Database Normalization: As mentioned in Context 1, normalization helps to
   organize data, reduce redundancy, and improve data integrity. This can lead
   to improved performance and memory usage as it avoids unnecessary data
   duplication.

2. Correct Database Choice: Depending on your needs, you might choose between
   SQL and NoSQL databases (as discussed in Context 3 and 4). For instance, if
   you're dealing with highly structured data and complex queries, an SQL
   database might be more suitable. On the other hand, for unstructured data and
   high write throughput, a NoSQL database could be more efficient.

3. Query Optimization: Context 5 talks about optimizing database queries, which
   is vital for high-performance code. This could involve techniques such as
   using indexes effectively, avoiding unnecessary columns in SELECT statements,
   minimizing the use of wildcards, and using JOIN operations instead of
   multiple queries when possible.

4. Managing Connections: Open and close connections judiciously to avoid memory
   leaks. Use connection pools to manage connections efficiently and prevent
   system resources from being overwhelmed.

5. Caching: Cache frequently accessed data to avoid unnecessary database hits.
   This can significantly improve performance and reduce memory usage.

Remember, writing memory-safe, high-performance database code is a broad topic
and these are just a few points to get started. Depending on the specific
database system and programming language you are using, there might be more
specific strategies that you can employ.

=== Step 5: Storing New Conversation === → Calling ekoDB Embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.307s • Function auto-cleaned up by client → Calling ekoDB
Embed() helper... • Using model: text-embedding-3-small • Text length: 1666
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.604s • Function auto-cleaned up by
client ✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search === Searching for messages about
'ownership' across ALL conversations...

→ Executing TextSearch()... • Collection: rag_messages • Query: "ownership
system" • Limit: 3 results • Search method: Full-text with fuzzy matching &
stemming • No vector embeddings needed - pure keyword search ✓ Text search
completed in 0.056s

✓ Found 3 messages mentioning ownership:

1. From conv_performance: Rust's ownership system provides zero-cost memory
   management. Use Box for heap allocation, Rc/Arc for shared ownership, and
   avoid cloning large data structures. The compiler optimizes away unnecessary
   allocations.

2. From conv_rust_programming: Rust's key features include: memory safety
   without garbage collection, zero-cost abstractions, ownership system,
   powerful type system, and excellent concurrency support.

3. From conv_rust_programming: The borrow checker enforces Rust's ownership
   rules at compile time. It ensures that references don't outlive the data they
   point to and prevents data races by allowing either multiple immutable
   references or one mutable reference.

=== System Statistics ===

→ Querying database statistics... • Using FindAll() helper - simplified query
API

📊 Database Statistics: • Total conversations: 4 • Total messages stored: 14 •
All messages indexed for vector search ✓ • All messages indexed for text search
✓ • All messages queryable by metadata ✓

=== Step 8: Dynamic Search Configuration === Each conversation can have its own
search config...

💡 Conversations can store custom search configurations: • Search type: hybrid,
text, or vector • Relevance thresholds • Filter by tags or metadata •
Collection-specific settings • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!

=== Cleanup === ✓ Cleanup complete

=== 📚 Summary: What This Example Showed ===

🔧 ekoDB Native Capabilities Used: ✓ Functions with Embed operation (AI
integration) ✓ Hybrid Search (text + vector combined) ✓ Text Search (full-text
with stemming) ✓ Automatic embedding generation ✓ Cross-collection queries

🚀 New Client Helper Methods: • client.Embed(text, model) - Generate embeddings
• client.HybridSearch() - Semantic + keyword search • client.TextSearch() -
Full-text search • client.FindAll() - Query all documents

💡 Key Takeaways:

1. ekoDB handles AI Functions natively - no external services needed
2. One-line embedding generation with auto-cleanup
3. Hybrid search combines semantic understanding + keyword matching
4. Perfect for RAG: store, search, and retrieve context
5. All AI capabilities accessible through simple client methods

🎯 Build production RAG systems with ekoDB! → Set OPENAI_API_KEY in your ekoDB
server environment → Use these client helpers to make AI integration simple →
Scale to millions of documents with native indexing

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m [36mRunning
Kotlin RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m === ekoDB RAG
Conversation System ===

This example shows how ekoDB can power a self-improving AI system that learns
from its own conversation history.

SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to no-operation
(NOP) logger implementation SLF4J(W): See
https://www.slf4j.org/codes.html#noProviders for further details. === Step 1:
Building Conversation History === Storing previous conversations with
embeddings...

→ Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 34 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.35s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 169 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.223s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.351s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 230 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.306s • Function
auto-cleaned up by client ✓ Stored Rust programming conversation (4 messages) →
Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 31 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.324s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 217 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.293s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.29s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 232 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.291s • Function
auto-cleaned up by client ✓ Stored database design conversation (4 messages) →
Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 36 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.503s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 178 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.419s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 37
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.336s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 213 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.296s • Function
auto-cleaned up by client ✓ Stored performance optimization conversation (4
messages)

=== Step 2: New User Question with Context Retrieval === User asks: "How do I
write memory-safe high-performance database code?"

=== Step 3: Searching Related Context === Using hybrid search to find relevant
messages from all conversations...

→ Generating embedding for user question... → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.329s • Function auto-cleaned up by client

→ Executing hybridSearch()... • Collection: rag_messages • Query text: "How do I
write memory-safe high-performance database code?" • Vector dimensions: 1536 •
Limit: 5 results • Search type: Semantic (vector) + Keyword (text) • Server
combines both scores for relevance ranking ✓ Search completed in 0.119s

✓ Found 5 related messages across all conversations:

1. [Score: 0.000] From conv_performance How can I optimize database queries?

2. [Score: 0.000] From conv_database_design Use NoSQL when you need: flexible
   schemas, horizontal scaling, high write throughput, or when working with
   unstructured data. SQL is better for complex queries, ACID transactions, and
   structured data with well-defined relationships.

3. [Score: 0.000] From conv_database_design When should I use NoSQL over SQL?

4. [Score: 0.000] From conv_database_design What is database normalization?

5. [Score: 0.000] From conv_database_design Database normalization is the
   process of organizing data to reduce redundancy and improve data integrity.
   It involves dividing large tables into smaller ones and defining
   relationships between them using foreign keys.

=== Step 4: Generating Context-Aware Response === ✓ Context prepared from search
results ✓ AI would use this context to generate comprehensive response

=== Step 5: Storing New Conversation === → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.698s • Function auto-cleaned up by client ✓ New conversation
stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search === Searching for messages about
'ownership' across ALL conversations...

→ Executing textSearch()... • Collection: rag_messages • Query: "ownership
system" • Limit: 3 results • Search method: Full-text with fuzzy matching &
stemming • No vector embeddings needed - pure keyword search ✓ Text search
completed in 0.055s

✓ Found 3 messages mentioning ownership:

1. From conv_performance: Rust's ownership system provides zero-cost memory
   management. Use Box for heap allocation, Rc/Arc for shared ownership, and
   avoid cloning large data structures. The compiler optimizes away unnecessary
   allocations.

2. From conv_rust_programming: Rust's key features include: memory safety
   without garbage collection, zero-cost abstractions, ownership system,
   powerful type system, and excellent concurrency support.

3. From conv_rust_programming: The borrow checker enforces Rust's ownership
   rules at compile time. It ensures that references don't outlive the data they
   point to and prevents data races by allowing either multiple immutable
   references or one mutable reference.

=== System Statistics ===

→ Querying database statistics... • Using findAllWithLimit() helper - simplified
query API

📊 Database Statistics: • Total conversations: 4 • Total messages stored: 13 •
All messages indexed for vector search ✓ • All messages indexed for text search
✓ • All messages queryable by metadata ✓

=== Step 8: Dynamic Search Configuration === Each conversation can have its own
search config...

💡 Conversations can store custom search configurations: • Search type: hybrid,
text, or vector • Relevance thresholds • Filter by tags or metadata •
Collection-specific settings • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!

=== Cleanup === ✓ Cleanup complete

=== 📚 Summary: What This Example Showed ===

🔧 ekoDB Native Capabilities Used: ✓ Functions with Embed operation (AI
integration) ✓ Hybrid Search (text + vector combined) ✓ Text Search (full-text
with stemming) ✓ Automatic embedding generation ✓ Cross-collection queries

🚀 New Client Helper Methods: • client.embed(text, model) - Generate embeddings
• client.hybridSearch() - Semantic + keyword search • client.textSearch() -
Full-text search • client.findAllWithLimit() - Query all documents

💡 Key Takeaways:

1. ekoDB handles AI Functions natively - no external services needed
2. One-line embedding generation with auto-cleanup
3. Hybrid search combines semantic understanding + keyword matching
4. Perfect for RAG: store, search, and retrieve context
5. All AI capabilities accessible through simple client methods

🎯 Build production RAG systems with ekoDB! → Set OPENAI_API_KEY in your ekoDB
server environment → Use these client helpers to make AI integration simple →
Scale to millions of documents with native indexing

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m ✅ [32mRAG
Examples Complete![0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[32mWhat you just saw across 5 languages:[0m ✓ Embeddings generated via ekoDB
Functions ✓ Hybrid search (semantic + keyword) ✓ Text search with stemming ✓
Cross-conversation context retrieval ✓ Simple client helpers wrapping powerful
AI

[36mMission: AI for All 🚀[0m - Making RAG accessible to everyone!
