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
operation ✓ Generated embedding: 1536 dimensions in 322.310666ms • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 169 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 271.825834ms • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 251.234458ms • Function auto-cleaned up
by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 230 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 289.611209ms • Function auto-cleaned up by client ✓ Stored Rust
programming conversation (4 messages) → Calling ekoDB embed() helper... • Using
model: text-embedding-3-small • Text length: 31 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 257.280167ms • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 217
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 350.550208ms • Function auto-cleaned up
by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 33 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 311.860542ms • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 232
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 538.007833ms • Function auto-cleaned up
by client ✓ Stored database design conversation (4 messages) → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 36
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 298.990625ms • Function auto-cleaned up
by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 178 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 291.418375ms • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 37
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 405.823958ms • Function auto-cleaned up
by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 213 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 273.1395ms • Function auto-cleaned up by client ✓ Stored
performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval === User asks: "How do I
write memory-safe high-performance database code?"

=== Step 3: Searching Related Context === Using hybrid search to find relevant
messages from all conversations...

→ Generating embedding for user question... → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 309.944709ms • Function auto-cleaned up by client

→ Executing hybrid_search()... • Collection: rag_messages • Query text: "How do
I write memory-safe high-performance database code?" • Vector dimensions: 1536 •
Limit: 5 results • Search type: Semantic (vector) + Keyword (text) • Server
combines both scores for relevance ranking ✓ Search completed in 103.481625ms

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

Answer: Writing memory-safe, high-performance database code involves several
steps:

1. **Understanding your data**: As mentioned in the context, it's important to
   choose the right database type (SQL or NoSQL) depending on your data's
   structure, relationships, and the type of queries you'll be performing.

2. **Normalize your data**: Normalize your data to reduce redundancy and improve
   data integrity. As stated in Context 3, normalization involves dividing large
   tables into smaller ones and defining relationships between them with foreign
   keys.

3. **Optimize your queries**: Reduce the amount of data that needs to be read
   from the database. This can be done by limiting the number of records
   returned, using WHERE clauses, indexing, and avoiding using wildcards at the
   beginning of a LIKE clause.

4. **Use prepared statements**: Prepared statements not only help prevent SQL
   injection attacks but can also enhance performance, especially when executing
   a single statement repeatedly.

5. **Connection pooling**: Create a pool of database connections and reuse them,
   rather than creating a new connection every time a client makes a request.
   This significantly reduces the overhead of establishing a new connection.

6. **Use efficient data types**: Choose the most efficient data type for your
   columns to reduce the amount of space your data consumes.

7. **Caching**: Cache the results of queries that are requested frequently and
   don't change often.

8. **Memory management**: Regularly check your application for memory leaks and
   ensure all database connections are closed when they are no longer needed.

9. **Concurrency control**: Use appropriate locking mechanisms to control the
   simultaneous access of data by multiple threads.

Remember, the performance of your database code depends largely on the specific
requirements and constraints of your project. It's essential to continuously
monitor and tune your database as those requirements evolve.

=== Step 5: Storing New Conversation === → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 284.626167ms • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 1965
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 281.515041ms • Function auto-cleaned up
by client ✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search === Searching for messages about
'ownership' across ALL conversations...

→ Executing text_search()... • Collection: rag_messages • Query: "ownership
system" • Limit: 3 results • Search method: Full-text with fuzzy matching &
stemming • No vector embeddings needed - pure keyword search ✓ Text search
completed in 62.01475ms

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
operation ✓ Generated embedding: 1536 dimensions in 0.409s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 169 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.316s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.223s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 230 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.267s • Function
auto-cleaned up by client ✓ Stored Rust programming conversation (4 messages) →
Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 31 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.204s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 217 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.233s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.231s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 232 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.311s • Function
auto-cleaned up by client ✓ Stored database design conversation (4 messages) →
Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 36 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.300s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 178 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.586s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 37
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.195s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 213 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.307s • Function
auto-cleaned up by client ✓ Stored performance optimization conversation (4
messages)

=== Step 2: New User Question with Context Retrieval === User asks: "How do I
write memory-safe high-performance database code?"

=== Step 3: Searching Related Context === Using hybrid search to find relevant
messages from all conversations...

→ Generating embedding for user question... → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.295s • Function auto-cleaned up by client

→ Executing hybrid_search()... • Collection: rag_messages • Query text: "How do
I write memory-safe high-performance database code?" • Vector dimensions: 1536 •
Limit: 5 results • Search type: Semantic (vector) + Keyword (text) • Server
combines both scores for relevance ranking ✓ Search completed in 0.091s

✓ Found 5 related messages across all conversations:

1. [Score: 0.000] From conv_database_design Use NoSQL when you need: flexible
   schemas, horizontal scaling, high write throughput, or when working with
   unstructured data. SQL is better for complex queries, ACID transactions, and
   structured data with well-defined relationships.

2. [Score: 0.000] From conv_database_design What is database normalization?

3. [Score: 0.000] From conv_database_design Database normalization is the
   process of organizing data to reduce redundancy and improve data integrity.
   It involves dividing large tables into smaller ones and defining
   relationships between them using foreign keys.

4. [Score: 0.000] From conv_database_design When should I use NoSQL over SQL?

5. [Score: 0.000] From conv_performance How can I optimize database queries?

=== Step 4: Generating Context-Aware Response === ✓ AI Response (with context
from 3 conversations):

Answer: Writing memory-safe and high-performance database code involves several
factors:

1. Choose the Right Database: Depending on your use case, either SQL or NoSQL
   might suit you better. For example, if you need to handle unstructured data,
   have high write throughput, or need horizontal scaling, you might want to
   choose NoSQL (context 1). If your data is structured with well-defined
   relationships, or you need ACID transactions or complex queries, SQL might be
   a better choice (context 1).

2. Database Normalization: Normalization can help improve data integrity and
   reduce redundancy. It involves dividing larger tables into smaller ones and
   defining relationships between them using foreign keys (context 3). This can
   help improve performance by reducing the amount of data that needs to be read
   from or written to the database.

3. Optimize Your Queries: You can optimize your database queries by using
   indexes, using the correct query operators, avoiding full table scans,
   limiting the amount of returned data, and properly utilizing joins,
   subqueries, and other advanced SQL features.

4. Memory Management: Depending on the language you're using to interact with
   the database, there might be specific tools or best practices for managing
   memory. For example, in languages like C++, you need to be careful about
   freeing any memory that you allocate. In more managed languages like Java or
   Python, it's important to understand how the garbage collector works and how
   to write code that doesn't unnecessarily hold onto memory.

5. Error Handling: Proper error handling can prevent memory leaks and other
   potential issues. Always ensure that your code can gracefully handle any
   database errors and clean up any resources it was using.

6. Testing and Profiling: Regularly test your code for performance and memory
   usage. Profiling tools can help you identify any bottlenecks or memory leaks.

Remember, the specifics will depend on your particular use case, the database
you're using, and the language you're writing your code in.

=== Step 5: Storing New Conversation === → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.388s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 2040
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.245s • Function auto-cleaned up by
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
operation ✓ Generated embedding: 1536 dimensions in 0.311s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 169 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.303s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.277s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 230 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.263s • Function
auto-cleaned up by client ✓ Stored Rust programming conversation (4 messages) →
Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 31 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.264s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 217 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.388s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.199s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 232 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.356s • Function
auto-cleaned up by client ✓ Stored database design conversation (4 messages) →
Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 36 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.214s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 178 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.326s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 37
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.309s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 213 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.410s • Function
auto-cleaned up by client ✓ Stored performance optimization conversation (4
messages)

=== Step 2: New User Question with Context Retrieval === User asks: "How do I
write memory-safe high-performance database code?"

=== Step 3: Searching Related Context === Using hybrid search to find relevant
messages from all conversations...

→ Generating embedding for user question... → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.286s • Function auto-cleaned up by client

→ Executing hybridSearch()... • Collection: rag_messages • Query text: "How do I
write memory-safe high-performance database code?" • Vector dimensions: 1536 •
Limit: 5 results • Search type: Semantic (vector) + Keyword (text) • Server
combines both scores for relevance ranking ✓ Search completed in 0.088s ✓ Found
5 related messages across all conversations:

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

=== Step 4: Generating Context-Aware Response === ✓ AI Response (with context
from 3 conversations):

Writing memory-safe, high-performance database code involves a combination of
efficient memory management, well-structured queries, and appropriate database
choice. Here are a few strategies:

1. Database Selection: Choose the right database system based on your needs. As
   mentioned in Context 2, NoSQL databases are suitable for flexible schemas,
   horizontal scaling, high write throughput, and unstructured data. SQL
   databases are better for complex queries, ACID transactions, and structured
   data with well-defined relationships.

2. Query Optimization: You can optimize database queries by using indexes,
   avoiding full table scans, limiting the result set, using joins wisely, and
   avoiding N+1 queries. Also, consider using stored procedures or prepared
   statements to reuse code and minimize database hits.

3. Normalization: Database normalization, as described in Context 5, helps
   improve data integrity and reduces redundancy. This process organizes data by
   dividing large tables into smaller ones and establishing relationships
   between them using foreign keys. Normalization can lead to performance
   improvements due to decreased redundancy.

4. Memory Management: Be mindful of the memory usage in your code. Avoid storing
   large datasets in memory all at once, and instead process data in chunks if
   possible. In addition, ensure that any resources are properly released after
   use to prevent memory leaks.

5. Concurrency: Implement concurrency control mechanisms to handle multiple
   simultaneous operations. This can help to maximize throughput and minimize
   contention for resources.

6. Caching: Use caching techniques to store frequently accessed data in memory,
   reducing the need for expensive database calls.

By combining these strategies, you can write code that is both memory-safe and
high-performance.

=== Step 5: Storing New Conversation === → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.295s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 1816
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.270s • Function auto-cleaned up by
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
operation ✓ Generated embedding: 1536 dimensions in 0.247s • Function
auto-cleaned up by client → Calling ekoDB Embed() helper... • Using model:
text-embedding-3-small • Text length: 169 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.456s • Function auto-cleaned up by client → Calling ekoDB
Embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.240s • Function auto-cleaned up by
client → Calling ekoDB Embed() helper... • Using model: text-embedding-3-small •
Text length: 230 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.266s • Function
auto-cleaned up by client ✓ Stored Rust programming conversation (4 messages) →
Calling ekoDB Embed() helper... • Using model: text-embedding-3-small • Text
length: 31 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.256s • Function
auto-cleaned up by client → Calling ekoDB Embed() helper... • Using model:
text-embedding-3-small • Text length: 217 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.278s • Function auto-cleaned up by client → Calling ekoDB
Embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.278s • Function auto-cleaned up by
client → Calling ekoDB Embed() helper... • Using model: text-embedding-3-small •
Text length: 232 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.289s • Function
auto-cleaned up by client ✓ Stored database design conversation (4 messages) →
Calling ekoDB Embed() helper... • Using model: text-embedding-3-small • Text
length: 36 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.250s • Function
auto-cleaned up by client → Calling ekoDB Embed() helper... • Using model:
text-embedding-3-small • Text length: 178 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.329s • Function auto-cleaned up by client → Calling ekoDB
Embed() helper... • Using model: text-embedding-3-small • Text length: 37
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.289s • Function auto-cleaned up by
client → Calling ekoDB Embed() helper... • Using model: text-embedding-3-small •
Text length: 213 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.281s • Function
auto-cleaned up by client ✓ Stored performance optimization conversation (4
messages)

=== Step 2: New User Question with Context Retrieval === User asks: "How do I
write memory-safe high-performance database code?"

=== Step 3: Searching Related Context === Using hybrid search to find relevant
messages from all conversations...

→ Generating embedding for user question... → Calling ekoDB Embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.482s • Function auto-cleaned up by client

→ Executing HybridSearch()... • Collection: rag_messages • Query text: "How do I
write memory-safe high-performance database code?" • Vector dimensions: 1536 •
Limit: 5 results • Search type: Semantic (vector) + Keyword (text) • Server
combines both scores for relevance ranking ✓ Search completed in 0.082s

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

Answer: Writing memory-safe, high-performance database code involves a
combination of good practices including efficient database design, effective use
of SQL or NoSQL depending on your needs, as well as optimizing your database
queries.

1. Efficient Database Design: This is where concepts like 'Database
   Normalization' come into play. Normalization helps in reducing data
   redundancy and improving data integrity by dividing large tables into smaller
   ones and defining relationships between them using foreign keys.

2. Choice of SQL or NoSQL: Depending on your needs, you may choose either SQL or
   NoSQL databases. Use NoSQL for flexible schemas, horizontal scaling, high
   write throughput, or when working with unstructured data. On the other hand,
   SQL is better for complex queries, ACID transactions, and structured data
   with well-defined relationships.

3. Optimizing Database Queries: To optimize your database queries, ensure that
   you're only retrieving the data you need (limit the use of SELECT \*), make
   good use of indexing to speed up retrieval of data, avoid using heavy
   operations like HAVING and UNION where you can use WHERE and JOIN
   respectively. Also, avoid N+1 queries where you load child entities one by
   one, instead, use join queries to load them in a single trip to your
   database.

4. Memory Management: In terms of memory safety, ensure that you close all
   database connections after operations to free up resources. Also, use
   parameterized queries or prepared statements to guard against SQL injection,
   which can lead to unauthorized use of memory.

5. Regular Monitoring and Maintenance: Regularly monitor the performance of your
   database. Use tools that can help identify slow queries, analyze database
   workload, and suggest indexes. Regular database maintenance like updating
   statistics, rebuilding indexes, and removing old data also helps in
   maintaining high performance.

Remember, what works best can depend on the specific database system you're
using, and the specific use case of your application.

=== Step 5: Storing New Conversation === → Calling ekoDB Embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.332s • Function auto-cleaned up by client → Calling ekoDB
Embed() helper... • Using model: text-embedding-3-small • Text length: 2030
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.229s • Function auto-cleaned up by
client ✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search === Searching for messages about
'ownership' across ALL conversations...

→ Executing TextSearch()... • Collection: rag_messages • Query: "ownership
system" • Limit: 3 results • Search method: Full-text with fuzzy matching &
stemming • No vector embeddings needed - pure keyword search ✓ Text search
completed in 0.054s

✓ Found 3 messages mentioning ownership:

1. From conv_rust_programming: Rust's key features include: memory safety
   without garbage collection, zero-cost abstractions, ownership system,
   powerful type system, and excellent concurrency support.

2. From conv_performance: Rust's ownership system provides zero-cost memory
   management. Use Box for heap allocation, Rc/Arc for shared ownership, and
   avoid cloning large data structures. The compiler optimizes away unnecessary
   allocations.

3. From conv_new_question: Answer: Writing memory-safe, high-performance
   database code involves a combination of good practices including efficient
   database design, effective use of SQL or NoSQL depending on your needs, as
   well as optimizing your database queries.

4. Efficient Database Design: This is where concepts like 'Database
   Normalization' come into play. Normalization helps in reducing data
   redundancy and improving data integrity by dividing large tables into smaller
   ones and defining relationships between them using foreign keys.

5. Choice of SQL or NoSQL: Depending on your needs, you may choose either SQL or
   NoSQL databases. Use NoSQL for flexible schemas, horizontal scaling, high
   write throughput, or when working with unstructured data. On the other hand,
   SQL is better for complex queries, ACID transactions, and structured data
   with well-defined relationships.

6. Optimizing Database Queries: To optimize your database queries, ensure that
   you're only retrieving the data you need (limit the use of SELECT \*), make
   good use of indexing to speed up retrieval of data, avoid using heavy
   operations like HAVING and UNION where you can use WHERE and JOIN
   respectively. Also, avoid N+1 queries where you load child entities one by
   one, instead, use join queries to load them in a single trip to your
   database.

7. Memory Management: In terms of memory safety, ensure that you close all
   database connections after operations to free up resources. Also, use
   parameterized queries or prepared statements to guard against SQL injection,
   which can lead to unauthorized use of memory.

8. Regular Monitoring and Maintenance: Regularly monitor the performance of your
   database. Use tools that can help identify slow queries, analyze database
   workload, and suggest indexes. Regular database maintenance like updating
   statistics, rebuilding indexes, and removing old data also helps in
   maintaining high performance.

Remember, what works best can depend on the specific database system you're
using, and the specific use case of your application.

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
operation ✓ Generated embedding: 1536 dimensions in 0.346s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 169 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.218s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.343s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 230 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.299s • Function
auto-cleaned up by client ✓ Stored Rust programming conversation (4 messages) →
Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 31 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.327s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 217 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.256s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.264s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 232 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.217s • Function
auto-cleaned up by client ✓ Stored database design conversation (4 messages) →
Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 36 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.226s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 178 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.252s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 37
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.559s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 213 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.255s • Function
auto-cleaned up by client ✓ Stored performance optimization conversation (4
messages)

=== Step 2: New User Question with Context Retrieval === User asks: "How do I
write memory-safe high-performance database code?"

=== Step 3: Searching Related Context === Using hybrid search to find relevant
messages from all conversations...

→ Generating embedding for user question... → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.221s • Function auto-cleaned up by client

→ Executing hybridSearch()... • Collection: rag_messages • Query text: "How do I
write memory-safe high-performance database code?" • Vector dimensions: 1536 •
Limit: 5 results • Search type: Semantic (vector) + Keyword (text) • Server
combines both scores for relevance ranking ✓ Search completed in 0.08s

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

=== Step 4: Generating Context-Aware Response === ✓ Context prepared from search
results ✓ AI would use this context to generate comprehensive response

=== Step 5: Storing New Conversation === → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.23s • Function auto-cleaned up by client ✓ New conversation
stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search === Searching for messages about
'ownership' across ALL conversations...

→ Executing textSearch()... • Collection: rag_messages • Query: "ownership
system" • Limit: 3 results • Search method: Full-text with fuzzy matching &
stemming • No vector embeddings needed - pure keyword search ✓ Text search
completed in 0.04s

✓ Found 3 messages mentioning ownership:

1. From conv_rust_programming: Rust's key features include: memory safety
   without garbage collection, zero-cost abstractions, ownership system,
   powerful type system, and excellent concurrency support.

2. From conv_performance: Rust's ownership system provides zero-cost memory
   management. Use Box for heap allocation, Rc/Arc for shared ownership, and
   avoid cloning large data structures. The compiler optimizes away unnecessary
   allocations.

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
