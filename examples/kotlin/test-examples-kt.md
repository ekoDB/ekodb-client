make test-examples-kt

🟣 Building Kotlin client library... To honour the JVM settings for this build a
single-use Daemon process will be forked. For more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

BUILD SUCCESSFUL in 3s 2 actionable tasks: 2 up-to-date ✅ Kotlin client built!
🧪 Running Kotlin client library examples... To honour the JVM settings for this
build a single-use Daemon process will be forked. For more on this, please refer
to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Simple CRUD Example ===

=== Create === ✓ Inserted user:
Record(fields={id=StringValue(value=CGqlbKXufElxjJ7zzsSycKXq85imQx2pTDb8YoODqLoB-4XlPgJS9LNvQxWthovI8hVgpw1cqCmifZQ6M5NPwQ)})
User ID:
CGqlbKXufElxjJ7zzsSycKXq85imQx2pTDb8YoODqLoB-4XlPgJS9LNvQxWthovI8hVgpw1cqCmifZQ6M5NPwQ

=== Read === ✓ Found user by ID:
Record(fields={age=ObjectValue(value={value=IntegerValue(value=28),
type=StringValue(value=Integer)}),
active=ObjectValue(value={value=BooleanValue(value=true),
type=StringValue(value=Boolean)}),
email=ObjectValue(value={value=StringValue(value=alice@example.com),
type=StringValue(value=String)}),
id=StringValue(value=CGqlbKXufElxjJ7zzsSycKXq85imQx2pTDb8YoODqLoB-4XlPgJS9LNvQxWthovI8hVgpw1cqCmifZQ6M5NPwQ),
name=ObjectValue(value={value=StringValue(value=Alice Johnson),
type=StringValue(value=String)})})

=== Update === ✓ Updated user: Record(fields={city=StringValue(value=San
Francisco), active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)}),
id=StringValue(value=CGqlbKXufElxjJ7zzsSycKXq85imQx2pTDb8YoODqLoB-4XlPgJS9LNvQxWthovI8hVgpw1cqCmifZQ6M5NPwQ),
email=ObjectValue(value={value=StringValue(value=alice@example.com),
type=StringValue(value=String)}), age=IntegerValue(value=29),
name=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=Alice Johnson)})})

=== Query === ✓ Found 1 users matching query

- Record(fields={city=ObjectValue(value={value=StringValue(value=San Francisco),
  type=StringValue(value=String)}),
  name=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=Alice Johnson)}),
  age=ObjectValue(value={value=IntegerValue(value=29),
  type=StringValue(value=Integer)}),
  email=ObjectValue(value={value=StringValue(value=alice@example.com),
  type=StringValue(value=String)}),
  active=ObjectValue(value={type=StringValue(value=Boolean),
  value=BooleanValue(value=true)}),
  id=StringValue(value=CGqlbKXufElxjJ7zzsSycKXq85imQx2pTDb8YoODqLoB-4XlPgJS9LNvQxWthovI8hVgpw1cqCmifZQ6M5NPwQ)})

=== Delete === ✓ Deleted user with ID:
CGqlbKXufElxjJ7zzsSycKXq85imQx2pTDb8YoODqLoB-4XlPgJS9LNvQxWthovI8hVgpw1cqCmifZQ6M5NPwQ
✓ Confirmed user was deleted

=== Cleanup === ✓ Deleted collection: kotlin_users_example

=== Example Complete ===

BUILD SUCCESSFUL in 9s 2 actionable tasks: 2 executed To honour the JVM settings
for this build a single-use Daemon process will be forked. For more on this,
please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Batch Operations Example ===

=== Batch Insert === ✓ Inserted 5 records IDs:
94a4f9SVjwQz-jctkMwXzJjncAv7XGy3q5qqBWP8cBDGOZLykypsGIIQRYRdpVeub2ABuFwbvdqPfbqyW9K5Qg,
tAFfGosIgxykFUXfTDjEVWm7wbBqnlyC0dPfH5iCePk734ni-QHfffChHKDPgEJCgK2CxTiGl2aLq2VQQCF1tw,
Rlp4r7sp_vuuuJY_pm0QferHPtu3Gu2CmCoir9WXcyszr83RDUdB-\_RGgZNj5b23wl04oWHPfcfx-q17czdyvg...

=== Batch Update === ✓ Updated 3 records

=== Batch Delete === ✓ Deleted 2 records === Cleanup === ✓ Deleted collection:
kotlin_batch_example

=== Example Complete === BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed,
1 up-to-date To honour the JVM settings for this build a single-use Daemon
process will be forked. For more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Query Builder Example ===

=== Inserting Sample Data === ✓ Inserted 5 records

=== Query 1: Equality (city = 'NYC') === ✓ Found 2 records in NYC

=== Query 2: Range (age >= 25 AND age < 32) === ✓ Found 3 records with age 25-31
=== Query 3: Sort by score (descending) === ✓ Top 3 scores:

- Score: ObjectValue(value={value=IntegerValue(value=95),
  type=StringValue(value=Integer)})
- Score: ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=92)})
- Score: ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=88)})

=== Query 4: Complex (score > 80 AND age >= 25) === ✓ Found 4 high-scoring
adults

=== Query 5: IN (city IN ['NYC', 'LA']) === ✓ Found 4 records in NYC or LA

=== Query 6: Pagination (skip 2, limit 2) === ✓ Page 2 (2 records):

- ObjectValue(value={value=StringValue(value=Charlie),
  type=StringValue(value=String)})
- ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=Diana)})

=== Query 7: Contains (name contains 'a') === ✓ Found 2 names containing 'a' ===
Cleanup === ✓ Deleted collection: kotlin_query_example

=== Example Complete ===

BUILD SUCCESSFUL in 4s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - KV Operations Example ===

=== KV Set === ✓ Set key: user:123

=== KV Get === ✓ Retrieved value:
{"name":"Alice","role":"admin","email":"alice@example.com"}

=== KV Set with TTL === ✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key === ✓ Session value:
{"created_at":1760517193904,"user_id":"123"} (Will expire in 10 seconds)

=== Set Multiple Keys === ✓ Set 3 configuration keys

=== KV Delete === ✓ Deleted key: user:123

=== Verify Deletion === ✓ Confirmed key was deleted

=== Cleanup === ✓ Cleaned up test keys

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Collection Management Example ===

=== List Collections === ✓ Found 0 collections

=== Check Collection Existence === Collection 'kotlin_collection_example'
exists: false

=== Create Collection with Schema === ✓ Created collection with schema:
kotlin_collection_example

=== Get Collection Schema === ✓ Schema:
{"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-10-15T08:33:18.029510Z","last_modified":"2025-10-15T08:33:18.029511Z","bypass_ripple":false}

=== Cleanup === ✓ Deleted collection: kotlin_collection_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Document TTL Example ===

=== Insert with TTL === ✓ Inserted document with 10s TTL Document ID:
NwtmqEDuQFiaKBVyufGdlJpcpCiiuVVjPjz9DYaKZTerzt1Td9IsVFb52eRCP_hVwj7qe7NpBjCAV56gE1Y6fA

=== Verify Document Exists === ✓ Document found: [created_at, user_id,
session_id, id, ttl_duration]

=== Insert with Longer TTL === ✓ Inserted document with 1h TTL Document ID:
NAUwQnWhYlfY1u8iTi8PBPLbIXBD92JswxCj-jYQJ3F25yqoilUz9JrDr_MDzW9EzKLrhkorH86heeenimj23w

=== TTL Expiration === ✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document === ✓ Long TTL document still exists: [cache_key,
value, ttl_duration, id]

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection: kotlin_ttl_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client === ✓ WebSocket client created

=== Connect to WebSocket === ✓ Connected to WebSocket

=== Insert Test Record === ✓ Inserted test record

=== Find All via WebSocket === ✓ WebSocket findAll result:
{"payload":{"data":[{"id":"m0BN-DurrwQoHfMcB-xsWu6OMQZRBC2ZWkxPgOyAynu-yn3u_4n5bILts2VuEZ4LA5Pj4nks_zQL-FIoq7k8Kw","name":{"type":"String","value":"Test
User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

=== Close WebSocket === ✓ WebSocket closed

=== Cleanup === ✓ Deleted collection: kotlin_websocket_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Schema Management Example ===

=== Creating Collection with Schema === ✓ Created collection
'kotlin_schema_example' with schema

=== Inserting Valid Documents === ✓ Inserted user 1:
StringValue(value=AiAPCLiTmONsMFUXQmKKVCy9Pi6W4sf5\_-Er9gkOPcnqdw8cOMnxhlCwvHFJvks7Qim-FFAjDyB_oLPEzFAiBw)
✓ Inserted user 2:
StringValue(value=PIcUk5vLgrZWkVTe3q57GkuNrBgInU7EjFMMQ7M3y-Bzri23p8W-86HysIntyIjenip1IKSxVFWTGZRTwXNU1Q)

=== Getting Schema === ✓ Schema for kotlin_schema_example: Fields:
{"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections === ✓ Total collections: 1 Sample:
[kotlin_schema_example]

=== Cleanup === ✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Search Example ===

=== Inserting Sample Documents === ✓ Inserted 5 sample documents

=== Basic Text Search === ✓ Search results for 'programming':
{"results":[{"record":{"title":"Rust
Programming","tags":"programming,rust,tutorial","views":808,"\id":"JLqGPucEi9B4ggxtGyZJEIxGSQmzYVxCuUBYojZy6O6gdW3LiIHYpEId5SZW_KiFKDVAKuS3gIfioyFjGstmoQ","description":"Learn
Rust programming language with hands-on examples and best
practices."},"score":19.8,"matched_fields":["tags","title","description"]},{"record":{"\id":"VJQ6Jc1K503t0FUqbN8arJClWPxVuDvy0D1UzIfGM9uSxdXdguuLdcsEGN7MGua1X6El_ThZD0eOmayB6PIfoQ","views":811,"tags":"programming,javascript,web","title":"JavaScript
Web Development","description":"Build modern web applications using JavaScript,
React, and
Node.js."},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"tags":"programming,python,data-science","\id":"1_zW4CUo_y-pbUjiyOMDR2qZAKYHKitkkmrk09hA1IDCwKlPnC7dQ3DzZLODkUPn8H0RYuzKdtoAPIZotR3QvA","title":"Python
for Data Science","views":12,"description":"Master Python for data analysis,
machine learning, and
visualization."},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' === ✓ Found results for 'machine learning'
{"results":[{"record":{"\id":"1e_EgosnmzsP8hVVoZzO1vU0c5j44Md3TN2tWjC2W0v_1vDgZBecSIhFzItMasG_VxZQv5qduzwDdYe9RMsIqQ","views":271,"description":"Introduction
to machine learning algorithms and neural
networks.","tags":"ai,machine-learning,python","title":"Machine Learning
Basics"},"score":2.7,"matched_fields":["description","tags","title"]},{"record":{"tags":"programming,python,data-science","\id":"1_zW4CUo_y-pbUjiyOMDR2qZAKYHKitkkmrk09hA1IDCwKlPnC7dQ3DzZLODkUPn8H0RYuzKdtoAPIZotR3QvA","title":"Python
for Data Science","views":12,"description":"Master Python for data analysis,
machine learning, and
visualization."},"score":1.0,"matched_fields":["description"]},{"record":{"title":"Database
Design","views":747,"\id":"z7N1tJMzF0A-v2aPwRO-omA8bEdkQgAF6eNjjeUY_Tmk0HP91C-fYvF38doqrLod1gmg8gtgJFrGqc4qeRaIbg","description":"Learn
database design principles, normalization, and query
optimization.","tags":"database,design,sql"},"score":0.5,"matched_fields":["description"]},{"record":{"tags":"programming,rust,tutorial","views":808,"description":"Learn
Rust programming language with hands-on examples and best
practices.","\id":"JLqGPucEi9B4ggxtGyZJEIxGSQmzYVxCuUBYojZy6O6gdW3LiIHYpEId5SZW_KiFKDVAKuS3gIfioyFjGstmoQ","title":"Rust
Programming"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

=== Cleanup === ✓ Deleted collection: kotlin_search_example

✓ All search operations completed successfully

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - WebSocket TTL Example ===

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
StringValue(value=Qci1qqRq3cBviSt6W-ZbWJ4NJcMNfPN9F3yzHCHcsZEhYCvzOx3i7UAKEwbRpuhCG1jkFgheZDAZW7AXH3OdzA)

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved data via
WebSocket:
{"payload":{"data":[{"created_at":{"type":"Integer","value":1760517218576},"id":"Qci1qqRq3cBviSt6W-ZbWJ4NJcMNfPN9F3yzHCHcsZEhYCvzOx3i7UAKEwbRpuhCG1jkFgheZDAZW7AXH3OdzA","name":{"type":"String","value":"WebSocket
TTL
Test"},"ttl_duration":{"type":"String","value":"1h"},"value":{"type":"Integer","value":42}}]},"type":"Success"}

✓ WebSocket closed

=== Cleanup === ✓ Deleted collection: kotlin_websocket_ttl_example

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Join Operations Example ===

=== Setting up sample data === ✓ Sample data created

=== Example 1: Single collection join (users with departments) === ✓ Found 2
users with department data

- Bob Smith
- Alice Johnson

=== Example 2: Join with filtering === ✓ Found 2 users in Engineering

- Bob Smith
- Alice Johnson

=== Example 3: Multi-collection join (users with departments and profiles) === ✓
Found 2 users with department join

- Bob Smith
- Alice Johnson

=== Example 4: Join orders with user data === ✓ Found 3 completed orders

- Mouse ($25)
- Keyboard ($75)
- Laptop ($1200)

=== Example 5: Complex join with multiple conditions === ✓ Found 2 users with
example.com emails

- Bob Smith (bob@example.com)
- Alice Johnson (alice@example.com)

=== Cleanup === ✓ Deleted test collections

✓ Join operations example completed successfully

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
peh3VGEXupNYFARuAzEahoUL1CToF3_MsQDU_x3C7e7IxPK8eMjXJO_Tz6P_5KlE1RzQW9or_5esOnIbOcXO4Q

=== Sending Chat Message === ✓ Chat response: Message ID:
"o-shOqF1I9bpNowkDRuIrG28jtqMyE5ZDUcfrF6z_01tyZPHs1prGAUogddywZ0O7Qab44trVRy36bKQJpSoBA"
Responses: ["Answer: ekoDB is a high-performance database that comes with
several features. It has intelligent caching, real-time capabilities, and AI
integration. One of its notable features is the AI Chat Integration, which lets
you query your database using natural language and receive AI-powered responses.
ekoDB also supports various search types including full-text search, vector
search, and hybrid search with automatic context retrieval."]

=== Cleanup === ✓ Deleted chat session ✓ Deleted collection:
kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 7s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
If3aFrCkjQWowYEtU6PNY21R3EM--i7QhlXdXy7xWdCzBadLHRxGgz8XRuSpGNnvoOrT9GKQrpS1PzdCR-BG4w

=== Sending Messages === ✓ Message 1 sent Responses: ["Based on the provided
context, the available product is ekoDB, which is a high-performance database
product with AI capabilities. It is priced at $99."]

✓ Message 2 sent Responses: ["As an assistant, I'm sorry but I can't provide the
price you're asking for because there's no information given about a specific
product. Could you please specify the product you're interested in?"]

=== Getting Message History === ✓ Retrieved message history Total messages:
[{"chat_id":{"type":"String","value":"If3aFrCkjQWowYEtU6PNY21R3EM--i7QhlXdXy7xWdCzBadLHRxGgz8XRuSpGNnvoOrT9GKQrpS1PzdCR-BG4w"},"content":{"type":"String","value":"What
products are
available?"},"context_snippets":{"type":"Array","value":[{"type":"Object","value":{"collection":{"type":"String","value":"kotlin_chat_sessions_example"},"matched_fields":{"type":"Array","value":[{"type":"String","value":"description"}]},"record":{"type":"Object","value":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"\id":{"type":"String","value":"bjEn9h_SGz3iglpcz56aZg_VL2MgND-mvU7AA8cRkdlRfvJRv68GEl4DTDKwsUshf5BKKuzonYrjIWeY07y0cg"},"price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}}},"score":{"type":"Float","value":0.25}}}]},"created_at":{"type":"DateTime","value":"2025-10-15T08:33:57.709506Z"},"\id":"5QD922PwV9K_cTJLtI6iZsAhcAYCE7ggnVMA61mvmHOa_46TtX9piaPdmNvcOCj-tbzc6ftioL6etNRmaOvfRA","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":32},"prompt_tokens":{"type":"Integer","value":165},"total_tokens":{"type":"Integer","value":197}}},"updated_at":{"type":"DateTime","value":"2025-10-15T08:33:57.709506Z"}},{"chat_id":{"type":"String","value":"If3aFrCkjQWowYEtU6PNY21R3EM--i7QhlXdXy7xWdCzBadLHRxGgz8XRuSpGNnvoOrT9GKQrpS1PzdCR-BG4w"},"content":{"type":"String","value":"Based
on the provided context, the available product is ekoDB, which is a
high-performance database product with AI capabilities. It is priced at
$99."},"context_snippets":{"type":"Array","value":[{"type":"Object","value":{"collection":{"type":"String","value":"kotlin_chat_sessions_example"},"matched_fields":{"type":"Array","value":[{"type":"String","value":"description"}]},"record":{"type":"Object","value":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"\id":{"type":"String","value":"bjEn9h_SGz3iglpcz56aZg_VL2MgND-mvU7AA8cRkdlRfvJRv68GEl4DTDKwsUshf5BKKuzonYrjIWeY07y0cg"},"price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}}},"score":{"type":"Float","value":0.25}}}]},"created_at":{"type":"DateTime","value":"2025-10-15T08:33:57.712466Z"},"\id":"FxzAq_3fTJeQCUEQ5z-C49qD4Q-tnWLgGccODzK51fpd2LBwigrKxBj3GC8pAwLxGWyMARXevtSqe38QM2GnXQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":32},"prompt_tokens":{"type":"Integer","value":165},"total_tokens":{"type":"Integer","value":197}}},"updated_at":{"type":"DateTime","value":"2025-10-15T08:33:57.712466Z"}},{"chat_id":{"type":"String","value":"If3aFrCkjQWowYEtU6PNY21R3EM--i7QhlXdXy7xWdCzBadLHRxGgz8XRuSpGNnvoOrT9GKQrpS1PzdCR-BG4w"},"content":{"type":"String","value":"What
is the
price?"},"created_at":{"type":"DateTime","value":"2025-10-15T08:33:59.633359Z"},"\id":"xYb0tydyGsWE7Hb1K2RwNb-YWxWvxE_g_jovFhrs07iMGoFP0uMSuPqkdqsTYy1Svs-an9kcEdHLJb9KxQPhcA","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":40},"prompt_tokens":{"type":"Integer","value":63},"total_tokens":{"type":"Integer","value":103}}},"updated_at":{"type":"DateTime","value":"2025-10-15T08:33:59.633359Z"}},{"chat_id":{"type":"String","value":"If3aFrCkjQWowYEtU6PNY21R3EM--i7QhlXdXy7xWdCzBadLHRxGgz8XRuSpGNnvoOrT9GKQrpS1PzdCR-BG4w"},"content":{"type":"String","value":"As
an assistant, I'm sorry but I can't provide the price you're asking for because
there's no information given about a specific product. Could you please specify
the product you're interested
in?"},"created_at":{"type":"DateTime","value":"2025-10-15T08:33:59.634829Z"},"\id":"h4Jw5Z-fWJhSOv8mbZkh-UX9i8chwNjYeH0kK9953q653ipiq1Qf52QMgnZMT7RkwSyXSBsZQW_6M2trBeQjKA","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":40},"prompt_tokens":{"type":"Integer","value":63},"total_tokens":{"type":"Integer","value":103}}},"updated_at":{"type":"DateTime","value":"2025-10-15T08:33:59.634829Z"}}]

=== Updating Session === ✓ Updated session system prompt

=== Listing Sessions === ✓ Total sessions: 1

=== Branching Session === ✓ Created branched session:
g81-pH8JNHP_nqu4jFtvKo-pnBPvnNXttskHxo8JV7PYOaX-Oi9NzUfc2xDIIQU68Z7bWs-0EqOys-XpvF39ig

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 7s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
Z45CcicZj-x0fVYNYurK7pVVhuOUA7837PSDS2eoPqqqMY9RPy_wiUTthlMyCAoX9kY0d_7MpaZYRKAkhPbRpQ

=== Sending Initial Message === ✓ Message sent Responses: ["Based on the
information provided, the available product is a high-performance database
product named ekoDB. It is priced at 99."]

✓ Second message sent

=== Regenerating AI Response === ✓ AI response regenerated New responses: ["The
available product based on the given context is ekoDB. It is a high-performance
database product and is priced at 99."]

=== Updating Message === ✓ Updated message content

=== Toggling Forgotten Status === ✓ Marked message as forgotten (excluded from
context)

=== Creating Second Session for Merge === ✓ Created second session:
egjXFuU6z7699cCpZu_Zp5TO47FE_MuLIRlzBeqV6xiMDr8jJyv42n6a4y_kQkCBS6SKEQp28_T1vlIy6zHc0g

=== Merging Sessions === ✓ Merged sessions Merged session ID: null

=== Deleting Message === ✓ Deleted message

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 9s 2 actionable tasks: 1 executed, 1 up-to-date ✅ Kotlin
client examples complete! ✅ All Kotlin integration tests complete!
